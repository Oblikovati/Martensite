//! The native FF executor moves inputs/results while constructing each nonSend kernel locally.
use super::face_face_task::{FaceFaceTaskInput, FaceFaceTaskOutput};
use crate::bop_algo::{ProgressCancellation, run_context_tasks};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::Failure,
};

// Lexical task envelope: the owned Range is Send, never shared as Sync state.
struct FaceFaceWork<'p> {
    ordinal: usize,
    input: Option<FaceFaceTaskInput>,
    output: Option<FaceFaceTaskOutput>,
    progress: Option<ProgressRange<'p>>,
}

impl<'p> FaceFaceWork<'p> {
    fn new((ordinal, input): (usize, FaceFaceTaskInput), progress: ProgressRange<'p>) -> Self {
        Self {
            ordinal,
            input: Some(input),
            output: None,
            progress: Some(progress),
        }
    }

    fn into_output(self) -> Result<FaceFaceTaskOutput, Failure> {
        self.output.ok_or_else(|| {
            Failure::new(&format!(
                "FF work item {} has no output; expected one terminal callback result",
                self.ordinal
            ))
        })
    }
}

/// Retain the standalone test call's assigning scope without duplicating execution.
/// Example: `let outputs = execute_face_inputs(inputs, outer, parallel)?;`.
#[cfg(test)]
pub(super) fn execute_face_inputs(
    inputs: Vec<FaceFaceTaskInput>,
    outer: &ProgressScope<'_>,
    parallel: bool,
) -> Result<Vec<FaceFaceTaskOutput>, Failure> {
    let scope = face_intersection_scope(outer, inputs.len());
    execute_face_inputs_in_scope(inputs, &scope, parallel)
}

/// Borrow the real caller-owned named scope through joined worker execution.
/// Example: `execute_face_inputs_in_scope(inputs, &scope, parallel)?` before storing results.
pub(super) fn execute_face_inputs_in_scope(
    inputs: Vec<FaceFaceTaskInput>,
    scope: &ProgressScope<'_>,
    parallel: bool,
) -> Result<Vec<FaceFaceTaskOutput>, Failure> {
    let mut work = prepared_face_work(inputs, scope);
    run_face_work(&mut work, parallel)?;
    work.into_iter().map(FaceFaceWork::into_output).collect()
}

fn run_face_work(work: &mut [FaceFaceWork<'_>], parallel: bool) -> Result<(), Failure> {
    let pooled = parallel && work.len() != 1;
    run_context_tasks(work, &(), pooled, || (), |work, _| execute_face_work(work))
}

/// Keep the native assigning scope alive through post-join polls and result storage.
/// Example: `let tasks = face_intersection_scope(outer, inputs.len());` in the coordinator.
pub(super) fn face_intersection_scope<'a>(
    outer: &'a ProgressScope<'a>,
    count: usize,
) -> ProgressScope<'a> {
    let range = outer.next(1.0);
    ProgressScope::with_literal(
        &range,
        "Performing Face-Face intersection",
        count as f64,
        false,
    )
}

fn prepared_face_work<'p>(
    inputs: Vec<FaceFaceTaskInput>,
    scope: &'p ProgressScope<'p>,
) -> Vec<FaceFaceWork<'p>> {
    // Reserve native task ranges sequentially; no coordinator-created task scope exists.
    inputs
        .into_iter()
        .enumerate()
        .map(|input| FaceFaceWork::new(input, scope.next(1.0)))
        .collect()
}

fn execute_face_work(work: &mut FaceFaceWork<'_>) -> Result<(), Failure> {
    let input = work.input.take().ok_or_else(|| {
        Failure::new(&format!(
            "FF work item {} has no input; expected exactly one owned prepared input",
            work.ordinal
        ))
    })?;
    let range = work.progress.take().ok_or_else(|| {
        Failure::new(&format!(
            "FF work item {}, expected one unconsumed worker range",
            work.ordinal
        ))
    })?;
    let mut task = input.into_task();
    let observed = ProgressCancellation::default();
    task.perform_with_range(&range, &observed)?;
    work.output = Some(task.into_output(&observed, work.ordinal));
    Ok(())
}

#[cfg(test)]
#[path = "face_face_tasks_contract.rs"]
mod contract;
