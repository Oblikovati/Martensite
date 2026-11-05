//! VF task geometry is read-only; only the coordinator writes the intersection structure.
use super::{BOPAlgoPaveFiller, VertexFaceSolver, missing_structure};
use crate::bop_algo::pave_filler::progress_observation::native_user_break;
use crate::bop_algo::{
    Options, ProgressCancellation, ProgressCompletion, ProgressFailureClass, ProgressOperation,
    ProgressStage, ProgressWorksite, run_launcher_context_tasks,
};
use crate::int_tools::Context;
use martensite_tkbrep::topods::{Face, Vertex};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::Failure,
};

// Lexical owned Range is Send; it is taken once and is never borrowed across workers.
struct VertexFaceWork<'a, 'p> {
    ordinal: usize,
    vertex: Vertex,
    face: Face,
    fuzzy: f64,
    solver: &'a mut VertexFaceSolver,
    progress: Option<ProgressRange<'p>>,
}

/// Dispatches actual VF work without lending the mutable DS to any worker.
/// Example: `execute_vertex_face_tasks(filler, solvers, scope, observed)?;`.
pub(super) fn execute_vertex_face_tasks(
    filler: &BOPAlgoPaveFiller,
    solvers: &mut [VertexFaceSolver],
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> Result<Vec<ProgressCompletion>, Failure> {
    let mut work = prepare_vertex_face_work(filler, solvers, scope)?;
    dispatch_vertex_face_work(filler, &mut work, observed)?;
    Ok(work.iter().map(vertex_face_completion).collect())
}

fn dispatch_vertex_face_work(
    filler: &BOPAlgoPaveFiller,
    work: &mut [VertexFaceWork<'_, '_>],
    observed: &ProgressCancellation,
) -> Result<(), Failure> {
    let context = filler.context.get().ok_or_else(missing_structure)?;
    run_launcher_context_tasks(
        work,
        context,
        filler.options.run_parallel(),
        Context::default,
        |task, context| compute_vertex_face_work(task, context, observed),
    )
}

fn prepare_vertex_face_work<'a, 'p>(
    filler: &BOPAlgoPaveFiller,
    solvers: &'a mut [VertexFaceSolver],
    scope: &'p ProgressScope<'p>,
) -> Result<Vec<VertexFaceWork<'a, 'p>>, Failure> {
    solvers
        .iter_mut()
        .enumerate()
        .map(|(ordinal, solver)| vertex_face_work_item(filler, solver, ordinal, scope.next(1.0)))
        .collect()
}

fn vertex_face_work_item<'a, 'p>(
    filler: &BOPAlgoPaveFiller,
    solver: &'a mut VertexFaceSolver,
    ordinal: usize,
    range: ProgressRange<'p>,
) -> Result<VertexFaceWork<'a, 'p>, Failure> {
    let structure = filler
        .data_structure
        .as_ref()
        .ok_or_else(missing_structure)?;
    Ok(VertexFaceWork {
        ordinal,
        vertex: Vertex::unchecked(structure.shape(solver.vertex)?.clone()),
        face: Face::unchecked(structure.shape(solver.face)?.clone()),
        fuzzy: filler.options.fuzzy_value(),
        solver,
        progress: Some(range),
    })
}

pub(super) fn vertex_face_intersection_scope<'a>(
    outer: &'a ProgressScope<'a>,
    count: usize,
) -> ProgressScope<'a> {
    let range = outer.next(9.0);
    ProgressScope::with_literal(
        &range,
        "Performing Vertex-Face intersection",
        count as f64,
        false,
    )
}

fn compute_vertex_face_work(
    work: &mut VertexFaceWork<'_, '_>,
    context: &Context,
    observed: &ProgressCancellation,
) -> Result<(), Failure> {
    let range = work.progress.take().ok_or_else(|| {
        Failure::new(&format!(
            "VF work item {}, expected one unconsumed worker range",
            work.ordinal
        ))
    })?;
    let task_scope = ProgressScope::unnamed(&range, 1.0, false);
    if cancel_vertex_face_work(work, &task_scope, observed) {
        return Ok(());
    }
    compute_vertex_face_projection(work, context);
    Ok(())
}

fn compute_vertex_face_projection(work: &mut VertexFaceWork<'_, '_>, context: &Context) {
    let solver = &mut work.solver;
    // An exception of ComputeVF becomes the solver's error; its flag stays -1.
    match context.compute_vf(
        &work.vertex,
        &work.face,
        &mut solver.uv.0,
        &mut solver.uv.1,
        &mut solver.new_tolerance,
        work.fuzzy,
    ) {
        Ok(flag) => solver.flag = flag,
        Err(cause) => {
            // Native catches ComputeVF locally and leaves flag=-1; the parent emits a warning.
            solver.failed = true;
            solver.cause = Some(cause);
        }
    }
}

fn vertex_face_completion(work: &VertexFaceWork<'_, '_>) -> ProgressCompletion {
    let operation = ProgressOperation::PaveFiller;
    let stage = ProgressStage::Worker;
    let context = vertex_face_worksite(work);
    if work.solver.cancelled {
        return ProgressCompletion::Cancelled {
            operation,
            stage,
            context,
        };
    }
    if work.solver.failed {
        return failed_vertex_face_completion(work, context);
    }
    ProgressCompletion::Completed
}

fn vertex_face_worksite(work: &VertexFaceWork<'_, '_>) -> Option<ProgressWorksite> {
    Some(ProgressWorksite {
        label: "PerformVF",
        input_ordinal: Some(work.ordinal),
        entity_key: Some(format!(
            "vertex={},face={}",
            work.solver.vertex, work.solver.face
        )),
    })
}

fn failed_vertex_face_completion(
    work: &VertexFaceWork<'_, '_>,
    context: Option<ProgressWorksite>,
) -> ProgressCompletion {
    ProgressCompletion::Failed {
        operation: ProgressOperation::PaveFiller,
        stage: ProgressStage::Worker,
        class: ProgressFailureClass::KernelFailure,
        cause: work.solver.cause.clone(),
        context,
    }
}

fn cancel_vertex_face_work(
    work: &mut VertexFaceWork<'_, '_>,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> bool {
    let options = Options::new();
    if !native_user_break(&options, scope, observed) {
        return false;
    }
    work.solver.cancelled = true;
    work.solver.failed = true;
    true
}
