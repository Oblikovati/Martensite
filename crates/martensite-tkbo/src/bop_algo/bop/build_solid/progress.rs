//! The concrete fuse-solid builder retains its two parent polls and actual eight-unit child.
use super::*;
use crate::bop_algo::{
    ProgressCancellation, ProgressCompletion, builder_area::progress::observe_user_break,
};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
impl BOPAlgoBOP {
    /// Rebuild fused solids through their actual range; `bop.build_solid_progress_raw(range, observed)?`.
    pub(in crate::bop_algo::bop) fn build_solid_progress_raw(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let scope = ProgressScope::unnamed(range, 10., false);
        let (solids, containers, mut face_solids) = self.collect_argument_solids()?;
        if observe_user_break(self.builder().options(), &scope, observed) {
            return Ok(());
        }
        let shared = shared_argument_solids(&face_solids)?;
        let untouched = self.select_fuse_solids(&solids, &shared, &mut face_solids)?;
        if observe_user_break(self.builder().options(), &scope, observed) {
            return Ok(());
        }
        let mut result = Shape::default();
        make_container(ShapeEnum::Compound, &mut result);
        if !self.add_ranged_fuse_solids(&face_solids, &mut result, &scope, observed)? {
            return Ok(());
        }
        self.finish_fuse_result(result, &untouched, &containers)
    }
    fn add_ranged_fuse_solids(
        &mut self,
        face_solids: &IndexedDataMapOfShapeListOfShape,
        result: &mut Shape,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<bool, Failure> {
        let faces = boundary_fuse_faces(face_solids)?;
        if faces.is_empty() {
            return Ok(true);
        }
        let mut builder = self.fuse_solid_builder(&faces);
        let completion = builder.perform_with_progress(&scope.next(8.));
        if let ProgressCompletion::Failed {
            cause: Some(cause), ..
        } = &completion
        {
            return Err(cause.clone());
        }
        if matches!(completion, ProgressCompletion::Cancelled { .. }) {
            observed.record_cancelled();
        }
        self.retain_fuse_solids(&builder, result)
    }
}
