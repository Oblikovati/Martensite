//! Reserve ranges sequentially, then move each sole range onto its actual context worker.
use super::EdgeFaceTask;
use crate::bop_algo::{ProgressCancellation, ProgressCompletion, run_launcher_context_tasks};
use crate::int_tools::Context;
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::{Failure, Handle},
};

struct EdgeFaceInvocation<'a> {
    ordinal: usize,
    worksite: &'static str,
    task: EdgeFaceTask,
    range: Option<ProgressRange<'a>>,
    completion: Option<ProgressCompletion>,
}

/// Uses the native context-dependent launcher even for serial and single-task execution.
/// Example: `execute_edge_face_tasks(tasks, scope, context, false, "PerformEF")?`.
pub(super) fn execute_edge_face_tasks(
    tasks: Vec<EdgeFaceTask>,
    scope: &ProgressScope<'_>,
    context: &Handle<Context>,
    parallel: bool,
    worksite: &'static str,
) -> Result<Vec<(EdgeFaceTask, ProgressCompletion)>, Failure> {
    let mut work = allocated_edge_face_invocations(tasks, scope, worksite);
    run_launcher_context_tasks(
        &mut work,
        context,
        parallel,
        || Handle::new(Context::default()),
        |work, context| work.perform(context),
    )?;
    work.into_iter()
        .map(EdgeFaceInvocation::into_output)
        .collect()
}

fn allocated_edge_face_invocations<'a>(
    tasks: Vec<EdgeFaceTask>,
    scope: &'a ProgressScope<'a>,
    worksite: &'static str,
) -> Vec<EdgeFaceInvocation<'a>> {
    // scope.next is a sequential reservation, never a concurrent worker allocator.
    tasks
        .into_iter()
        .enumerate()
        .map(|(ordinal, task)| EdgeFaceInvocation {
            ordinal,
            worksite,
            task,
            range: Some(scope.next(1.0)),
            completion: None,
        })
        .collect()
}

impl EdgeFaceInvocation<'_> {
    fn perform(&mut self, context: &Handle<Context>) -> Result<(), Failure> {
        let range = self.range.take().ok_or_else(|| {
            Failure::new(&format!(
                "EF task {} has no range; expected one unused owned range",
                self.ordinal
            ))
        })?;
        let observed = ProgressCancellation::default();
        self.completion = Some(self.task.perform_with_range(
            &range,
            &observed,
            self.ordinal,
            context,
            self.worksite,
        ));
        Ok(())
    }

    fn into_output(self) -> Result<(EdgeFaceTask, ProgressCompletion), Failure> {
        let completion = self.completion.ok_or_else(|| {
            Failure::new(&format!(
                "EF task {} has no completion; expected one terminal worker result",
                self.ordinal
            ))
        })?;
        Ok((self.task, completion))
    }
}

/// Keeps the native named scope alive through analysis and the complete EF tail.
/// Example: `let scope = edge_face_intersection_scope(outer, count);`.
pub(super) fn edge_face_intersection_scope<'a>(
    outer: &'a ProgressScope<'a>,
    count: usize,
) -> ProgressScope<'a> {
    let range = outer.next(9.0);
    ProgressScope::with_literal(
        &range,
        "Performing Edge-Face intersection",
        count as f64,
        false,
    )
}

#[cfg(test)]
#[path = "execution_contract.rs"]
mod tests;
