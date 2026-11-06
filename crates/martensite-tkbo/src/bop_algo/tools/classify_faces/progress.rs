//! Scoped classification preserves serial task order; parallel BVH queries stay coordinator-owned.
use super::{FaceBoxes, InParts, ShapeBoxMap, SolidFaceCandidates, SolidInternalFaces};
use super::{classify_candidates_with_progress, prepare_candidates, solid_box};
use crate::{
    bop_algo::{
        ProgressCancellation, ProgressCompletion, ProgressFailureClass, ProgressOperation,
        ProgressStage,
    },
    int_tools::Context,
};
use martensite_tkbrep::topods::{Shape, Solid};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    ncollection::List,
    standard::Failure,
};

#[path = "execution.rs"]
mod execution;
use execution::ClassificationExecution;

/// The arguments of `BOPAlgoTools::ClassifyFaces`, borrowed for one run; no range is stored.
/// Example: `classify_faces_with_progress(FaceClassificationJob { faces, solids, context, parts, boxes, internal, parallel: false }, &range)`.
pub struct FaceClassificationJob<'a> {
    /// The faces to classify (`theFaces`).
    pub faces: &'a List<Shape>,
    /// The solids to classify against (`theSolids`).
    pub solids: &'a List<Shape>,
    /// The intersection context (`theContext`).
    pub context: &'a Context,
    /// Receives each solid with the faces inside it (`theInParts`).
    pub parts: &'a mut InParts,
    /// Bounding boxes already known for the shapes (`theShapeBoxMap`).
    pub boxes: &'a ShapeBoxMap,
    /// The own INTERNAL faces of the solids (`theSolidsIF`).
    pub internal: &'a SolidInternalFaces,
    /// Whether the solids are classified in parallel (`theRunParallel`).
    pub parallel: bool,
}

/// Classifies the faces against the solids in the caller's range (`ClassifyFaces`) and returns raw
/// execution completion; a non-completed run leaves `parts` as diagnostic state only.
/// Example: `let status = classify_faces_with_progress(job, &indicator.start());`.
pub fn classify_faces_with_progress(
    job: FaceClassificationJob<'_>,
    range: &ProgressRange<'_>,
) -> ProgressCompletion {
    let execution = ClassificationExecution::default();
    let _: &(dyn Send + Sync) = &execution;
    classify_faces_observed(job, range, &execution)
}

fn classify_faces_observed(
    job: FaceClassificationJob<'_>,
    range: &ProgressRange<'_>,
    execution: &ClassificationExecution<'_>,
) -> ProgressCompletion {
    let _: &(dyn Send + Sync) = &job;
    let observed = ProgressCancellation::default();
    let result = classify_job(job, range, &observed, execution);
    classification_result(result, &observed)
}

fn classification_result(
    result: Result<ProgressCompletion, Failure>,
    observed: &ProgressCancellation,
) -> ProgressCompletion {
    match result {
        Err(cause) => ProgressCompletion::Failed {
            operation: ProgressOperation::ClassifyFaces,
            stage: ProgressStage::Completion,
            class: ProgressFailureClass::KernelFailure,
            cause: Some(cause),
            context: None,
        },
        Ok(ProgressCompletion::Completed) if observed.is_cancelled() => cancelled_completion(),
        Ok(completion) => completion,
    }
}

struct SolidTaskInput {
    solid: Solid,
    bounds: martensite_tkmath::bnd::BoundingBox3,
    own_internal: List<Shape>,
}

struct FaceClassificationTask<'a, 'p> {
    input: SolidTaskInput,
    scope: &'a ProgressScope<'p>,
    prepared: Option<Result<Option<SolidFaceCandidates>, Failure>>,
    inside: List<Shape>,
    completion: ProgressCompletion,
}

fn classify_job(
    mut job: FaceClassificationJob<'_>,
    range: &ProgressRange<'_>,
    observed: &ProgressCancellation,
    execution: &ClassificationExecution<'_>,
) -> Result<ProgressCompletion, Failure> {
    let scope = ProgressScope::unnamed(range, 10., false);
    let Some((mut boxes, inputs)) = prepared_job(&job, &scope, observed)? else {
        return Ok(cancelled_completion());
    };
    drop(scope.next(1.));
    let workers = classification_scope(&scope, inputs.len());
    execute_classifications(&mut job, inputs, &mut boxes, &workers, observed, execution)
}

fn prepared_job(
    job: &FaceClassificationJob<'_>,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> Result<Option<(FaceBoxes, Vec<SolidTaskInput>)>, Failure> {
    let Some(boxes) = prepared_face_boxes(job, scope, observed)? else {
        return Ok(None);
    };
    let inputs = solid_inputs(job)?;
    if observed.is_cancelled() {
        return Ok(None);
    }
    Ok(Some((boxes, inputs)))
}

fn prepared_face_boxes(
    job: &FaceClassificationJob<'_>,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> Result<Option<FaceBoxes>, Failure> {
    let mut boxes = super::empty_face_boxes(job.faces.len());
    for (position, face) in job.faces.iter().enumerate() {
        if scope_cancelled(scope, observed) {
            return Ok(None);
        }
        super::append_face_box(&mut boxes, position, face, job.boxes)?;
    }
    boxes.tree.build()?;
    Ok(Some(boxes))
}

fn solid_inputs(job: &FaceClassificationJob<'_>) -> Result<Vec<SolidTaskInput>, Failure> {
    let mut inputs = Vec::new();
    for shape in job.solids.iter() {
        let solid = Solid::unchecked(shape.clone());
        let bounds = solid_box(&solid, job.boxes)?;
        let own_internal = job.internal.seek(&solid).cloned().unwrap_or_default();
        inputs.push(SolidTaskInput::new(solid, bounds, own_internal));
    }
    Ok(inputs)
}

fn execute_classifications(
    job: &mut FaceClassificationJob<'_>,
    inputs: Vec<SolidTaskInput>,
    boxes: &mut FaceBoxes,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
    execution: &ClassificationExecution<'_>,
) -> Result<ProgressCompletion, Failure> {
    let scopes = classification_scopes(scope, inputs.len());
    let mut tasks = classification_tasks(inputs, &scopes);
    run_prepared_tasks(&mut tasks, boxes, job, observed, execution)?;
    merge_tasks(job.parts, tasks, observed)
}

fn run_prepared_tasks(
    tasks: &mut [FaceClassificationTask<'_, '_>],
    boxes: &mut FaceBoxes,
    job: &FaceClassificationJob<'_>,
    observed: &ProgressCancellation,
    execution: &ClassificationExecution<'_>,
) -> Result<(), Failure> {
    if job.parallel {
        return parallel_tasks(tasks, boxes, job.context, observed, execution);
    }
    serial_tasks(tasks, boxes, job.context, observed, execution)
}

fn serial_tasks(
    tasks: &mut [FaceClassificationTask<'_, '_>],
    boxes: &mut FaceBoxes,
    context: &Context,
    observed: &ProgressCancellation,
    execution: &ClassificationExecution<'_>,
) -> Result<(), Failure> {
    // Native context-dependent Perform still catches through Launcher(max0) in serial mode.
    let launcher = crate::bop_algo::context_tasks::context_task_launcher(false, tasks.len());
    let serial = ClassificationExecution {
        observer: execution.observer,
        launcher: Some(&launcher),
    };
    serial_with_launcher(tasks, boxes, context, observed, &serial)
}

struct SerialTask<'a, 'task, 'progress> {
    task: &'a mut FaceClassificationTask<'task, 'progress>,
    boxes: &'a mut FaceBoxes,
}

fn serial_with_launcher(
    tasks: &mut [FaceClassificationTask<'_, '_>],
    boxes: &mut FaceBoxes,
    context: &Context,
    observed: &ProgressCancellation,
    execution: &ClassificationExecution<'_>,
) -> Result<(), Failure> {
    for task in tasks {
        let mut packet = [SerialTask { task, boxes }];
        let _: &dyn Send = &packet[0];
        execution.run(&mut packet, context, |packet, context| {
            serial_task(packet, context, observed, execution)
        })?;
    }
    Ok(())
}

fn serial_task(
    packet: &mut SerialTask<'_, '_, '_>,
    context: &Context,
    observed: &ProgressCancellation,
    execution: &ClassificationExecution<'_>,
) -> Result<(), Failure> {
    let _close = CloseTaskScope(packet.task.scope);
    if cancel_task(packet.task, observed) {
        return Ok(());
    }
    select_task_candidates(packet.task, packet.boxes);
    classify_task(packet.task, context, observed, execution)
}

fn prepare_parallel_tasks(tasks: &mut [FaceClassificationTask<'_, '_>], boxes: &mut FaceBoxes) {
    // Coordinator selection changes phase only; it adds no native cancellation poll or advance.
    for task in tasks {
        select_task_candidates(task, boxes);
    }
}

fn select_task_candidates(task: &mut FaceClassificationTask<'_, '_>, boxes: &mut FaceBoxes) {
    // A native initial cancellation must suppress a computed selection failure.
    let prepared = task.input.candidates(boxes);
    task.prepared = Some(prepared);
}

fn classify_task(
    task: &mut FaceClassificationTask<'_, '_>,
    context: &Context,
    observed: &ProgressCancellation,
    execution: &ClassificationExecution<'_>,
) -> Result<(), Failure> {
    let Some(prepared) = task.prepared.take() else {
        return Ok(());
    };
    let prepared = prepared?;
    (execution.observer)(context)?;
    let local = ProgressCancellation::default();
    classify_task_candidates(task, prepared, context, &local)?;
    finish_task(task, &local, observed);
    Ok(())
}

fn classify_task_candidates(
    task: &mut FaceClassificationTask<'_, '_>,
    prepared: Option<SolidFaceCandidates>,
    context: &Context,
    local: &ProgressCancellation,
) -> Result<(), Failure> {
    let Some(input) = prepared else {
        return Ok(());
    };
    if !input.empty_solid {
        drop(task.scope.next(1.));
    }
    task.inside = classify_candidates_with_progress(&input, context, task.scope, local)?;
    Ok(())
}

fn merge_tasks(
    parts: &mut InParts,
    tasks: Vec<FaceClassificationTask<'_, '_>>,
    observed: &ProgressCancellation,
) -> Result<ProgressCompletion, Failure> {
    let mut completion = ProgressCompletion::Completed;
    for task in tasks {
        if matches!(task.completion, ProgressCompletion::Cancelled { .. }) {
            observed.record_cancelled();
        }
        if !matches!(task.completion, ProgressCompletion::Completed) {
            completion = task.completion;
        }
        if !parts.contains(&task.input.solid) {
            parts.add(task.input.solid.into_shape(), task.inside);
        }
    }
    Ok(completion)
}

/// Latch an actual task poll without manufacturing an Options report; `scope_cancelled(scope, observed)`.
pub(super) fn scope_cancelled(scope: &ProgressScope<'_>, observed: &ProgressCancellation) -> bool {
    if scope.more() {
        return false;
    }
    observed.record_cancelled();
    true
}

fn cancelled_completion() -> ProgressCompletion {
    ProgressCompletion::Cancelled {
        operation: ProgressOperation::ClassifyFaces,
        stage: ProgressStage::Completion,
        context: None,
    }
}

fn pending_completion() -> ProgressCompletion {
    ProgressCompletion::Incomplete {
        operation: ProgressOperation::ClassifyFaces,
        stage: ProgressStage::Worker,
        class: ProgressFailureClass::UnqualifiedStage,
        cause: None,
        context: None,
    }
}

struct CloseTaskScope<'a, 'p>(&'a ProgressScope<'p>);
impl Drop for CloseTaskScope<'_, '_> {
    fn drop(&mut self) {
        // Native task scopes close on every exit, including unwinding inside a worker.
        self.0.close();
    }
}

fn classification_scope<'a>(scope: &'a ProgressScope<'a>, count: usize) -> ProgressScope<'a> {
    ProgressScope::with_literal(
        &scope.next(9.),
        "Classification of faces relatively solids",
        count as f64,
        false,
    )
}

fn classification_scopes<'a>(scope: &'a ProgressScope<'a>, count: usize) -> Vec<ProgressScope<'a>> {
    (0..count)
        .map(|_| ProgressScope::unnamed(&scope.next(1.), 2., false))
        .collect()
}

fn classification_tasks<'a, 'p>(
    inputs: Vec<SolidTaskInput>,
    scopes: &'a [ProgressScope<'p>],
) -> Vec<FaceClassificationTask<'a, 'p>> {
    inputs
        .into_iter()
        .zip(scopes)
        .map(|(input, scope)| FaceClassificationTask::new(input, scope))
        .collect()
}

fn parallel_tasks(
    tasks: &mut [FaceClassificationTask<'_, '_>],
    boxes: &mut FaceBoxes,
    context: &Context,
    observed: &ProgressCancellation,
    execution: &ClassificationExecution<'_>,
) -> Result<(), Failure> {
    prepare_parallel_tasks(tasks, boxes);
    execution.run(tasks, context, |task, context| {
        parallel_task(task, context, observed, execution)
    })
}

fn parallel_task(
    task: &mut FaceClassificationTask<'_, '_>,
    context: &Context,
    observed: &ProgressCancellation,
    execution: &ClassificationExecution<'_>,
) -> Result<(), Failure> {
    let _close = CloseTaskScope(task.scope);
    if cancel_task(task, observed) {
        return Ok(());
    }
    classify_task(task, context, observed, execution)
}

fn cancel_task(task: &mut FaceClassificationTask<'_, '_>, observed: &ProgressCancellation) -> bool {
    if !scope_cancelled(task.scope, observed) {
        return false;
    }
    task.completion = cancelled_completion();
    true
}

fn finish_task(
    task: &mut FaceClassificationTask<'_, '_>,
    local: &ProgressCancellation,
    observed: &ProgressCancellation,
) {
    task.completion = ProgressCompletion::Completed;
    if local.is_cancelled() {
        observed.record_cancelled();
        task.completion = cancelled_completion();
    }
}

impl SolidTaskInput {
    fn new(
        solid: Solid,
        bounds: martensite_tkmath::bnd::BoundingBox3,
        own_internal: List<Shape>,
    ) -> Self {
        Self {
            solid,
            bounds,
            own_internal,
        }
    }

    fn candidates(&self, boxes: &mut FaceBoxes) -> Result<Option<SolidFaceCandidates>, Failure> {
        // Reuse the built coordinator selector without copying or rebuilding its tree.
        prepare_candidates(&self.solid, &self.bounds, &self.own_internal, boxes)
    }
}

impl<'a, 'p> FaceClassificationTask<'a, 'p> {
    fn new(input: SolidTaskInput, scope: &'a ProgressScope<'p>) -> Self {
        Self {
            input,
            scope,
            prepared: None,
            inside: List::default(),
            completion: pending_completion(),
        }
    }
}

#[cfg(test)]
#[path = "progress_tests.rs"]
mod tests;
