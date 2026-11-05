//! Post-treatment advances only at the two actual tolerance correction boundaries.
use super::BOPAlgoBuilder;
use crate::bop_ds::DataStructure;
use crate::bop_tools::{correct_shape_tolerances, correct_tolerances};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::Failure,
};
use martensite_tkg3d::top_abs::ShapeEnum;

impl BOPAlgoBuilder {
    /// Keeps the legacy unbound selection on the same two correction calls.
    /// Example: `builder.post_treat(ds)?;` protects unchanged inputs in non-destructive mode.
    pub(crate) fn post_treat(&mut self, structure: &DataStructure) -> Result<(), Failure> {
        let range = ProgressRange::default();
        self.post_treat_progress_raw(structure, &range)
    }

    /// Reports actual correction boundaries without inventing native cancellation polls.
    /// Example: `builder.post_treat_progress_raw(ds, range)?;` is not completion classification.
    pub(crate) fn post_treat_progress_raw(
        &mut self,
        structure: &DataStructure,
        range: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let avoided = self.post_treatment_avoided(structure)?;
        let scope =
            ProgressScope::with_literal(range, "Post treatment of result shape", 2.0, false);
        let parallel = self.options().run_parallel();
        correct_tolerances(&self.shape, &avoided, 0.05, parallel)?;
        drop(scope.next(1.0));
        correct_shape_tolerances(&self.shape, &avoided, parallel)
    }

    fn post_treatment_avoided(
        &self,
        structure: &DataStructure,
    ) -> Result<IndexedMapOfShape, Failure> {
        let mut avoided = IndexedMapOfShape::default();
        if !self.non_destructive {
            return Ok(avoided);
        }
        for index in 0..structure.nb_source_shapes() {
            let info = structure.shape_info(index)?;
            if matches!(
                info.shape_type(),
                ShapeEnum::Vertex | ShapeEnum::Edge | ShapeEnum::Face
            ) {
                avoided.add(info.shape().clone());
            }
        }
        Ok(avoided)
    }
}
