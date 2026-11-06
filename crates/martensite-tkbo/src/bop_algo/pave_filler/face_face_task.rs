//! Private BOPAlgoFaceFace wrapper used by the PaveFiller task phase.
use super::super::{
    AlertIntersectionFailed, Options, ParallelAlgo, ParallelAlgoBase, ProgressCancellation,
};
use super::progress_observation::native_user_break;
#[cfg(test)]
use crate::int_tools::Context;
use crate::int_tools::FaceFace;
use martensite_tkbrep::topods::Face;
use martensite_tkernel::{
    message::{AlertHandle, ProgressRange, ProgressScope},
    standard::Failure,
};
use martensite_tkmath::{
    bnd::BoundingBox3,
    gp::{Point3, Transform3, Vector3},
    toploc::Location,
};
use std::sync::Arc;

#[path = "face_face_task/transport.rs"]
mod transport;
pub(super) use transport::{FaceFaceTaskInput, FaceFaceTaskOutput};

/// Face-face task state from BOPAlgoPaveFiller_6; `task.perform_parallel()?`.
/// The worker-local task is neither Send nor Sync: its released intersector owns
/// an Rc-backed line constructor, and its range uses Cell. Shared
/// coordinator scopes lend worker-local ranges; no borrowed range escapes a worker.
pub(crate) struct BOPAlgoFaceFaceTask<'a> {
    parallel: ParallelAlgoBase<'a>,
    intersector: FaceFace,
    indices: [i32; 2],
    faces: [Face; 2],
    boxes: [BoundingBox3; 2],
    tolerance: f64,
    transform: Transform3,
    failure: Option<Failure>,
}

impl Default for BOPAlgoFaceFaceTask<'_> {
    fn default() -> Self {
        Self {
            parallel: ParallelAlgoBase::default(),
            intersector: FaceFace::default(),
            indices: [-1; 2],
            faces: std::array::from_fn(|_| Face::default()),
            boxes: std::array::from_fn(|_| BoundingBox3::default()),
            tolerance: 1e-7,
            transform: Transform3::default(),
            failure: None,
        }
    }
}

impl<'a> BOPAlgoFaceFaceTask<'a> {
    pub(crate) fn set_run_parallel(&mut self, enabled: bool) {
        self.parallel.options_mut().set_run_parallel(enabled);
    }

    pub(crate) fn set_indices(&mut self, first: i32, second: i32) {
        self.indices = [first, second];
    }

    pub(crate) fn indices(&self) -> [i32; 2] {
        self.indices
    }

    pub(crate) fn set_faces(&mut self, first: &Face, second: &Face) {
        self.faces = [first.clone(), second.clone()];
    }

    pub(crate) fn set_boxes(&mut self, first: &BoundingBox3, second: &BoundingBox3) {
        self.boxes = [first.clone(), second.clone()];
    }

    pub(crate) fn set_tol_ff(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
    }

    pub(crate) fn tol_ff(&self) -> f64 {
        self.tolerance
    }

    #[cfg(test)]
    pub(crate) fn trsf(&self) -> &Transform3 {
        &self.transform
    }

    pub(crate) fn set_parameters(&mut self, spatial: bool, first: bool, second: bool, tol: f64) {
        self.intersector.set_parameters(spatial, first, second, tol);
    }

    /// Starting points of the walking (`IntToolsFaceFace::SetList`).
    pub(crate) fn set_list(&mut self, points: &[martensite_tkgeomalgo::int_surf::IntSurfPntOn2S]) {
        self.intersector.set_list(points);
    }

    pub(crate) fn set_fuzzy_value(&mut self, fuzzy: f64) {
        self.intersector.set_fuzzy_value(fuzzy);
    }

    #[cfg(test)]
    pub(crate) fn set_context(&mut self, context: &martensite_tkernel::standard::Handle<Context>) {
        self.intersector.set_context(context);
    }

    pub(crate) fn face1(&self) -> &Face {
        &self.faces[0]
    }

    pub(crate) fn face2(&self) -> &Face {
        &self.faces[1]
    }

    pub(crate) fn intersector(&self) -> &FaceFace {
        &self.intersector
    }

    pub(crate) fn intersector_mut(&mut self) -> &mut FaceFace {
        &mut self.intersector
    }

    pub(crate) fn apply_trsf(&mut self) -> Result<(), Failure> {
        self.intersector.apply_result_transform(self.transform)
    }

    /// Executes one lexical worker-local range without storing it in reusable task state.
    /// Example: `task.perform_with_range(&range, observed)?` inside the scoped worker.
    pub(super) fn perform_with_range(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let scope = ProgressScope::unnamed(range, 1.0, false);
        self.perform_in_scope(&scope, observed)
    }

    fn perform_in_scope(
        &mut self,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        self.failure = None;
        if native_user_break(self.parallel.options(), scope, observed) {
            return Ok(());
        }
        self.execute_face_intersection();
        Ok(())
    }

    fn execute_face_intersection(&mut self) {
        match transform_boxes_to_origin(&self.boxes) {
            Ok(Some(transform)) => self.perform_on_translated_faces(transform),
            Ok(None) => self.perform_on_input_faces(),
            Err(cause) => self.record_task_failure(cause),
        }
    }

    fn perform_on_input_faces(&mut self) {
        if let Err(cause) =
            self.intersector
                .perform(&self.faces[0], &self.faces[1], self.run_parallel())
        {
            self.record_task_failure(cause);
        }
    }

    fn perform_on_translated_faces(&mut self, transform: Transform3) {
        let result = self.translated_intersection_inputs(transform);
        match result {
            Ok([first, second]) => {
                if let Err(cause) = self
                    .intersector
                    .perform(&first, &second, self.run_parallel())
                {
                    self.record_task_failure(cause);
                }
            }
            Err(cause) => self.record_task_failure(cause),
        }
    }

    fn translated_intersection_inputs(
        &mut self,
        transform: Transform3,
    ) -> Result<[Face; 2], Failure> {
        let faces = self.translated_input_faces(transform)?;
        self.transform = transform.inverted()?;
        Ok(faces)
    }

    fn translated_input_faces(&self, transform: Transform3) -> Result<[Face; 2], Failure> {
        let location = Location::new(transform);
        let mut first = self.faces[0].clone();
        let mut second = self.faces[1].clone();
        first.move_by(&location, false)?;
        second.move_by(&location, false)?;
        Ok([first, second])
    }

    fn record_task_failure(&mut self, cause: Failure) {
        self.failure = Some(cause);
        self.parallel.options().add_error(&failure_alert());
    }

    fn run_parallel(&self) -> bool {
        self.parallel_options().run_parallel()
    }
}

impl<'a> ParallelAlgo<'a> for BOPAlgoFaceFaceTask<'a> {
    fn parallel_options(&self) -> &Options {
        self.parallel.options()
    }

    fn parallel_base(&self) -> &ParallelAlgoBase<'a> {
        &self.parallel
    }

    fn parallel_base_mut(&mut self) -> &mut ParallelAlgoBase<'a> {
        &mut self.parallel
    }

    fn perform_parallel(&mut self) -> Result<(), Failure> {
        let scope = ProgressScope::unnamed(self.parallel.progress_range(), 1.0, false);
        let observed = ProgressCancellation::default();
        self.perform_in_scope(&scope, &observed)
    }

    fn fill_parallel_constants(&self, _whole: f64, _steps: &mut super::super::PISteps) {}

    fn fill_parallel_steps(&self, _steps: &mut super::super::PISteps) {}
}

fn transform_boxes_to_origin(boxes: &[BoundingBox3; 2]) -> Result<Option<Transform3>, Failure> {
    let mut union = boxes[0].clone();
    union.add_box(&boxes[1]);
    let minimum = union.corner_min()?;
    let maximum = union.corner_max()?;
    let center = midpoint(minimum, maximum);
    let origin = Point3::default();
    let distance = center.distance(&origin);
    if distance < 1e5 || union.square_extent().sqrt() / distance > 1e-5 {
        return Ok(None);
    }
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::between(minimum, origin));
    Ok(Some(transform))
}

fn midpoint(first: Point3, second: Point3) -> Point3 {
    Point3::new(
        (first.x() + second.x()) * 0.5,
        (first.y() + second.y()) * 0.5,
        (first.z() + second.z()) * 0.5,
    )
}

fn failure_alert() -> AlertHandle {
    Arc::new(AlertIntersectionFailed) as AlertHandle
}
