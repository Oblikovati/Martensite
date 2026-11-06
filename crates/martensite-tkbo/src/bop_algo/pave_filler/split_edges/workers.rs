//! Each split worker owns one range; shared DS stays read-only until the launcher joins.
use super::*;
use crate::bop_algo::{
    Options, ProgressFailureClass, ProgressOperation, ProgressStage, ProgressWorksite,
    run_launcher_context_tasks,
};
use crate::bop_ds::DataStructure;
use crate::int_tools::Context;
use martensite_tkernel::standard::Handle;
struct SplitEdgeInvocation<'a> {
    ordinal: usize,
    task: SplitEdgeTask,
    range: Option<ProgressRange<'a>>,
    completion: Option<ProgressCompletion>,
}
/// Execute the actual context-dependent launcher and return ordered terminal packets.
/// Example: `execute_split_edge_tasks(tasks, scope, ds, context, parallel)?`.
pub(super) fn execute_split_edge_tasks(
    tasks: Vec<SplitEdgeTask>,
    scope: &ProgressScope<'_>,
    structure: &DataStructure,
    context: &Handle<Context>,
    parallel: bool,
) -> Result<Vec<(SplitEdgeTask, ProgressCompletion)>, Failure> {
    let mut work = allocated_split_invocations(tasks, scope);
    run_launcher_context_tasks(
        &mut work,
        context,
        parallel,
        || Handle::new(Context::default()),
        |item, context| item.perform(structure, context),
    )?;
    work.into_iter()
        .map(SplitEdgeInvocation::into_output)
        .collect()
}
fn allocated_split_invocations<'a>(
    tasks: Vec<SplitEdgeTask>,
    scope: &'a ProgressScope<'a>,
) -> Vec<SplitEdgeInvocation<'a>> {
    tasks
        .into_iter()
        .enumerate()
        .map(|(ordinal, task)| SplitEdgeInvocation {
            ordinal,
            task,
            range: Some(scope.next(1.)),
            completion: None,
        })
        .collect()
}
impl SplitEdgeInvocation<'_> {
    fn perform(
        &mut self,
        structure: &DataStructure,
        context: &Handle<Context>,
    ) -> Result<(), Failure> {
        let range = self.range.take().ok_or_else(|| {
            Failure::new(&format!(
                "split task {} has no range; expected one unused owned range",
                self.ordinal
            ))
        })?;
        let scope = ProgressScope::unnamed(&range, 1., false);
        let observed = ProgressCancellation::default();
        let outcome = if native_user_break(&Options::default(), &scope, &observed) {
            Ok(())
        } else {
            self.task.perform_on(structure, context)
        };
        self.completion = Some(split_worker_completion(
            &outcome,
            observed.is_cancelled(),
            self.ordinal,
        ));
        outcome
    }
    fn into_output(self) -> Result<(SplitEdgeTask, ProgressCompletion), Failure> {
        let completion = self.completion.ok_or_else(|| {
            Failure::new(&format!(
                "split task {} has no completion; expected one terminal worker result",
                self.ordinal
            ))
        })?;
        Ok((self.task, completion))
    }
}
impl SplitEdgeTask {
    /// Native BOPAlgoSplitEdge::Perform computes tolerance before constructing its owned edge.
    fn perform_on(
        &mut self,
        structure: &DataStructure,
        context: &Handle<Context>,
    ) -> Result<(), Failure> {
        self.tolerance = compute_tolerance_of_cb(self.common.as_ref(), structure, context.get())?;
        make_split_edge(
            &self.edge,
            &self.vertices[0],
            self.parameters[0],
            &self.vertices[1],
            self.parameters[1],
            &mut self.split,
        )?;
        BRepBndLib::add(&self.split, &mut self.bounds, true)?;
        self.bounds
            .set_gap(self.bounds.gap() + precision::confusion());
        Ok(())
    }
}
fn split_worker_completion(
    outcome: &Result<(), Failure>,
    cancelled: bool,
    ordinal: usize,
) -> ProgressCompletion {
    let context = Some(ProgressWorksite {
        label: "MakeSplitEdges",
        input_ordinal: Some(ordinal),
        entity_key: None,
    });
    if let Err(cause) = outcome {
        return ProgressCompletion::Failed {
            operation: ProgressOperation::PaveFiller,
            stage: ProgressStage::Worker,
            class: ProgressFailureClass::KernelFailure,
            cause: Some(cause.clone()),
            context,
        };
    }
    if cancelled {
        return ProgressCompletion::Cancelled {
            operation: ProgressOperation::PaveFiller,
            stage: ProgressStage::Worker,
            context,
        };
    }
    ProgressCompletion::Completed
}
#[cfg(test)]
#[path = "worker_contract.rs"]
mod tests;
