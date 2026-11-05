//! Only prepared input and terminal state cross workers; the Rc kernel never does.
use super::{BOPAlgoFaceFaceTask, BoundingBox3, Face, Transform3};
use crate::bop_algo::{
    Options, ProgressCancellation, ProgressCompletion, ProgressFailureClass, ProgressOperation,
    ProgressStage, ProgressWorksite,
};
use crate::int_tools::FaceFacePostprocessing;
use martensite_tkgeomalgo::int_surf::IntSurfPntOn2S;

/// Owns the exact prepared arguments before a worker constructs its actual task.
/// Example: `let task = input.into_task();` entirely inside the worker.
pub(in crate::bop_algo::pave_filler) struct FaceFaceTaskInput {
    pub(in crate::bop_algo::pave_filler) indices: [i32; 2],
    pub(in crate::bop_algo::pave_filler) faces: [Face; 2],
    pub(in crate::bop_algo::pave_filler) boxes: [BoundingBox3; 2],
    pub(in crate::bop_algo::pave_filler) tolerance: f64,
    pub(in crate::bop_algo::pave_filler) seeds: Vec<IntSurfPntOn2S>,
    pub(in crate::bop_algo::pave_filler) parameters: [bool; 3],
    pub(in crate::bop_algo::pave_filler) fuzzy: f64,
    pub(in crate::bop_algo::pave_filler) parallel: bool,
}

/// Owns terminal geometry, native reports and the separately classified task completion.
/// Example: `let task = packet.into_task();` before existing coordinator postprocessing.
pub(in crate::bop_algo::pave_filler) struct FaceFaceTaskOutput {
    options: Options,
    intersector: FaceFacePostprocessing,
    indices: [i32; 2],
    faces: [Face; 2],
    tolerance: f64,
    transform: Transform3,
    failure: Option<super::Failure>,
    pub(in crate::bop_algo::pave_filler) completion: ProgressCompletion,
}

impl FaceFaceTaskInput {
    /// Preserves the existing setter order and lazy task-local context creation.
    /// Example: `let mut task = input.into_task(); task.perform_parallel()?;`.
    pub(in crate::bop_algo::pave_filler) fn into_task(self) -> BOPAlgoFaceFaceTask<'static> {
        let mut task = BOPAlgoFaceFaceTask::default();
        task.set_run_parallel(self.parallel);
        task.set_indices(self.indices[0], self.indices[1]);
        task.set_faces(&self.faces[0], &self.faces[1]);
        task.set_boxes(&self.boxes[0], &self.boxes[1]);
        task.set_tol_ff(self.tolerance);
        if !self.seeds.is_empty() {
            task.set_list(&self.seeds);
        }
        let [spatial, first, second] = self.parameters;
        task.set_parameters(spatial, first, second, 1.0e-7);
        task.set_fuzzy_value(self.fuzzy);
        task
    }
}

impl BOPAlgoFaceFaceTask<'static> {
    /// Drops the actual worker-local kernel and context before transferring terminal owners.
    /// `observed` belongs to this task; merge it into a parent latch separately.
    /// A captured kernel failure takes precedence over cancellation (shared result v1).
    /// Example: `let packet = task.into_output(observed, ordinal);` at the worker's end.
    pub(in crate::bop_algo::pave_filler) fn into_output(
        self,
        observed: &ProgressCancellation,
        input_ordinal: usize,
    ) -> FaceFaceTaskOutput {
        let completion = self.task_completion(observed, input_ordinal);
        FaceFaceTaskOutput {
            options: self.parallel.options().clone(),
            intersector: self.intersector.into_postprocessing(),
            indices: self.indices,
            faces: self.faces,
            tolerance: self.tolerance,
            transform: self.transform,
            failure: self.failure,
            completion,
        }
    }

    fn task_completion(
        &self,
        observed: &ProgressCancellation,
        ordinal: usize,
    ) -> ProgressCompletion {
        let context = face_worksite(ordinal);
        if self.failure.is_some() {
            return self.failed_task_completion(context);
        }
        if observed.is_cancelled() {
            return cancelled_face_task(context);
        }
        if self.parallel.options().has_errors() || !self.intersector.is_done() {
            return self.failed_task_completion(context);
        }
        ProgressCompletion::Completed
    }

    fn failed_task_completion(&self, context: Option<ProgressWorksite>) -> ProgressCompletion {
        let class = if self.failure.is_some() {
            ProgressFailureClass::KernelFailure
        } else {
            ProgressFailureClass::AlgorithmReport
        };
        ProgressCompletion::Failed {
            operation: ProgressOperation::PaveFiller,
            stage: ProgressStage::Worker,
            class,
            cause: self.failure.clone(),
            context,
        }
    }
}

impl FaceFaceTaskOutput {
    #[cfg(test)]
    /// Restores the same terminal object for unchanged preparation, transform and ordered store.
    /// Example: `let mut task = packet.into_task(); task.intersector_mut().prepare_lines_3d(false)?;`.
    pub(in crate::bop_algo::pave_filler) fn into_task(self) -> BOPAlgoFaceFaceTask<'static> {
        let (task, _completion) = self.into_parts();
        task
    }

    /// Moves terminal state and its independent completion together into coordinator ownership.
    /// Example: `let (task, completion) = packet.into_parts();` before native result storage.
    pub(in crate::bop_algo::pave_filler) fn into_parts(
        self,
    ) -> (BOPAlgoFaceFaceTask<'static>, ProgressCompletion) {
        let mut task = BOPAlgoFaceFaceTask::default();
        *task.parallel.options_mut() = self.options;
        task.intersector = self.intersector.into_intersector();
        task.indices = self.indices;
        task.faces = self.faces;
        task.tolerance = self.tolerance;
        task.transform = self.transform;
        task.failure = self.failure;
        (task, self.completion)
    }
}

fn face_worksite(ordinal: usize) -> Option<ProgressWorksite> {
    let worksite = ProgressWorksite {
        label: "FaceFace",
        input_ordinal: Some(ordinal),
        entity_key: None,
    };
    Some(worksite)
}

fn cancelled_face_task(context: Option<ProgressWorksite>) -> ProgressCompletion {
    ProgressCompletion::Cancelled {
        operation: ProgressOperation::PaveFiller,
        stage: ProgressStage::Worker,
        context,
    }
}
