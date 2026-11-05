//! Coordinator reservations are exclusive task inputs; borrowed ranges are created on workers.
use crate::bop_algo::{
    BuilderFace, ProgressCancellation, ProgressCompletion, ProgressFailureClass, ProgressOperation,
    ProgressStage, run_context_tasks,
};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::Failure,
};

struct SplitFaceTask<'a> {
    face: BuilderFace,
    progress: Option<ProgressScope<'a>>,
    completion: Option<ProgressCompletion>,
}

impl SplitFaceTask<'_> {
    fn perform(&mut self, observed: &ProgressCancellation) -> Result<(), Failure> {
        let scope = self.progress.take().ok_or_else(|| {
            Failure::with_message(
                "Standard_ProgramError",
                "face task progress=None; expected one unconsumed task scope",
            )
        })?;
        if !scope.more() {
            observed.record_cancelled();
            self.completion = Some(worker_cancelled());
            return Ok(());
        }
        let completion = self.face.perform_with_progress(&scope.next(1.0));
        self.record_completion(completion, observed)
    }

    fn record_completion(
        &mut self,
        completion: ProgressCompletion,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        if matches!(completion, ProgressCompletion::Cancelled { .. }) {
            observed.record_cancelled();
        }
        let thrown = original_kernel_failure(&completion);
        self.completion = Some(completion);
        match thrown {
            Some(cause) => Err(cause),
            None => Ok(()),
        }
    }
}

/// Execute actual Face receivers without a shared geometric context or stored range.
/// Example: `perform_face_tasks(builders, parallel, range, observed)?` returns ordered raw builders.
/// The returned completion is child evidence, not permission to publish these mutable builders.
pub(super) fn perform_face_tasks(
    builders: Vec<BuilderFace>,
    parallel: bool,
    range: &ProgressRange<'_>,
    observed: &ProgressCancellation,
) -> Result<(Vec<BuilderFace>, ProgressCompletion), Failure> {
    let scope = ProgressScope::with_literal(range, "Splitting faces", builders.len() as f64, false);
    let mut tasks = reserve_face_tasks(builders, &scope);
    // OSDParallel::For at native runs one input inline; this hint does not alter Face options.
    let pooled = parallel && tasks.len() != 1;
    run_context_tasks(
        &mut tasks,
        &(),
        pooled,
        || (),
        |task, ()| task.perform(observed),
    )?;
    Ok(collect_task_outcomes(tasks))
}

fn reserve_face_tasks<'a>(
    builders: Vec<BuilderFace>,
    scope: &'a ProgressScope<'a>,
) -> Vec<SplitFaceTask<'a>> {
    builders
        .into_iter()
        .map(|face| SplitFaceTask {
            face,
            progress: Some(ProgressScope::unnamed(&scope.next(1.0), 1.0, false)),
            completion: None,
        })
        .collect()
}

fn collect_task_outcomes(tasks: Vec<SplitFaceTask<'_>>) -> (Vec<BuilderFace>, ProgressCompletion) {
    let mut first_stop = ProgressCompletion::Completed;
    let mut builders = Vec::with_capacity(tasks.len());
    for task in tasks {
        let completion = task.completion.unwrap_or_else(unexecuted_task);
        first_stop = prefer_confirmed_failure(first_stop, completion);
        builders.push(task.face);
    }
    (builders, first_stop)
}

fn worker_cancelled() -> ProgressCompletion {
    ProgressCompletion::Cancelled {
        operation: ProgressOperation::BuilderFace,
        stage: ProgressStage::Worker,
        context: None,
    }
}

fn unexecuted_task() -> ProgressCompletion {
    ProgressCompletion::Incomplete {
        operation: ProgressOperation::BuilderFace,
        stage: ProgressStage::Worker,
        class: ProgressFailureClass::UnqualifiedStage,
        cause: None,
        context: None,
    }
}

fn prefer_confirmed_failure(
    current: ProgressCompletion,
    next: ProgressCompletion,
) -> ProgressCompletion {
    if is_confirmed_failure(&current) {
        return current;
    }
    if current == ProgressCompletion::Completed || is_confirmed_failure(&next) {
        return next;
    }
    current
}

#[cfg(test)]
#[path = "tasks_contract.rs"]
mod contract;

/// Recognize an existing causal failure before interpreting a later cancellation observation.
/// Example: `is_confirmed_failure(&completion)` does not poll or reinterpret raw flags.
pub(super) fn is_confirmed_failure(completion: &ProgressCompletion) -> bool {
    matches!(
        completion,
        ProgressCompletion::Failed { .. } | ProgressCompletion::Incomplete { .. }
    )
}

fn original_kernel_failure(completion: &ProgressCompletion) -> Option<Failure> {
    match completion {
        ProgressCompletion::Failed {
            class: ProgressFailureClass::KernelFailure,
            cause: Some(cause),
            ..
        } => Some(cause.clone()),
        _ => None,
    }
}
