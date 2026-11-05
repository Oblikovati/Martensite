//! Join independent gap tasks before collecting outcomes and geometry in original feature order.
use super::super::fill_gap::{Connections, FillGap};
use super::outcome::{feature_ordinal, retain_feature_outcome};
use crate::bop_algo::{ProgressCompletion, run_context_tasks};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::Failure,
};
struct FeatureGapTask<'a> {
    ordinal: usize,
    gap: FillGap,
    range: Option<ProgressRange<'a>>,
    completion: Option<ProgressCompletion>,
}
impl FeatureGapTask<'_> {
    fn perform(&mut self, connections: &Connections<'_>) -> Result<(), Failure> {
        let range = self
            .range
            .take()
            .ok_or_else(|| missing_gap_terminal(self.ordinal, "one unused range"))?;
        self.completion = Some(self.gap.perform_on_progress(connections, &range));
        Ok(())
    }
}
pub(super) fn fill_feature_gaps(
    gaps: Vec<FillGap>,
    connections: &Connections<'_>,
    scope: &ProgressScope<'_>,
    parallel: bool,
) -> Result<(Vec<FillGap>, ProgressCompletion), Failure> {
    let mut tasks: Vec<_> = gaps
        .into_iter()
        .enumerate()
        .map(|(ordinal, gap)| FeatureGapTask {
            ordinal,
            gap,
            range: Some(scope.next(1.)),
            completion: None,
        })
        .collect();
    let pooled = parallel && tasks.len() != 1;
    run_context_tasks(
        &mut tasks,
        &(),
        pooled,
        || (),
        |task, _| task.perform(connections),
    )?;
    collect_feature_gaps(tasks)
}
fn collect_feature_gaps(
    tasks: Vec<FeatureGapTask<'_>>,
) -> Result<(Vec<FillGap>, ProgressCompletion), Failure> {
    let mut gaps = Vec::with_capacity(tasks.len());
    let mut outcome = ProgressCompletion::Completed;
    for task in tasks {
        let completion = task
            .completion
            .ok_or_else(|| missing_gap_terminal(task.ordinal, "one terminal worker outcome"))?;
        outcome = retain_feature_outcome(outcome, feature_ordinal(completion, task.ordinal));
        gaps.push(task.gap);
    }
    Ok((gaps, outcome))
}
fn missing_gap_terminal(ordinal: usize, expected: &str) -> Failure {
    Failure::with_type(
        "Standard_ProgramError",
        &format!("feature gap task={ordinal}; expected {expected}"),
        "",
    )
}
#[cfg(test)]
#[path = "feature_task_tests.rs"]
mod tests;
