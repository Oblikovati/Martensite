//! Move each allocated range into its actual worker; never insert a coordinator wrapper scope.
use super::EdgeEdgeTask;
use crate::bop_algo::{ProgressCancellation, ProgressCompletion, run_context_tasks};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::Failure,
};

struct EdgeEdgeInvocation<'a> {
    ordinal: usize,
    worksite: &'static str,
    task: EdgeEdgeTask,
    range: Option<ProgressRange<'a>>,
    completion: Option<ProgressCompletion>,
}

/// Executes each owned task once and retains ordered terminal packets.
/// Example: `execute_edge_tasks(tasks, task_scope, parallel, "PerformEE")?`.
pub(super) fn execute_edge_tasks(
    tasks: Vec<EdgeEdgeTask>,
    scope: &ProgressScope<'_>,
    parallel: bool,
    worksite: &'static str,
) -> Result<Vec<(EdgeEdgeTask, ProgressCompletion)>, Failure> {
    let mut work = allocated_edge_invocations(tasks, scope, worksite);
    let pooled = parallel && work.len() != 1;
    run_context_tasks(&mut work, &(), pooled, || (), |work, _| work.perform())?;
    work.into_iter()
        .map(EdgeEdgeInvocation::into_output)
        .collect()
}

fn allocated_edge_invocations<'a>(
    tasks: Vec<EdgeEdgeTask>,
    scope: &'a ProgressScope<'a>,
    worksite: &'static str,
) -> Vec<EdgeEdgeInvocation<'a>> {
    // Reservation stays sequential because scope.next is not an atomic range allocator.
    tasks
        .into_iter()
        .enumerate()
        .map(|(ordinal, task)| EdgeEdgeInvocation {
            ordinal,
            worksite,
            task,
            range: Some(scope.next(1.0)),
            completion: None,
        })
        .collect()
}

impl EdgeEdgeInvocation<'_> {
    fn perform(&mut self) -> Result<(), Failure> {
        let range = self.range.take().ok_or_else(|| {
            Failure::new(&format!(
                "EE task {} has no range; expected one unused owned range",
                self.ordinal
            ))
        })?;
        let observed = ProgressCancellation::default();
        self.completion =
            Some(
                self.task
                    .perform_with_range(&range, &observed, self.ordinal, self.worksite),
            );
        Ok(())
    }

    fn into_output(self) -> Result<(EdgeEdgeTask, ProgressCompletion), Failure> {
        let completion = self.completion.ok_or_else(|| {
            Failure::new(&format!(
                "EE task {} has no completion; expected one terminal worker result",
                self.ordinal
            ))
        })?;
        Ok((self.task, completion))
    }
}

/// Keep this coordinator scope alive through result storage and the new-vertex tail.
/// Example: `let scope = edge_intersection_scope(outer, count);`.
pub(super) fn edge_intersection_scope<'a>(
    outer: &'a ProgressScope<'a>,
    count: usize,
) -> ProgressScope<'a> {
    let range = outer.next(9.0);
    ProgressScope::with_literal(
        &range,
        "Performing Edge-edge intersection",
        count as f64,
        false,
    )
}

#[cfg(test)]
#[path = "execution_tests.rs"]
mod tests;
