//! Extended-face intersection and trimming retain one actual receiver and source order.
use super::{ExtendedFaces, FillGap, TrimInputs};
use crate::bop_algo::{
    BOPAlgoBuilder, BuilderHooks, ProgressCancellation, ProgressCompletion, ProgressOperation,
    ProgressStage, ProgressWorksite, builder_area::progress::classify_area_progress,
    progress_result::retain_intersection_completion,
};
use martensite_tkbrep::toptools::{IndexedDataMapOfShapeListOfShape, IndexedMapOfShape};
use martensite_tkbrep::{
    brep_tools::History,
    topexp,
    topods::{self, Shape},
};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;

struct TrimMaps {
    extended_edge_faces: IndexedDataMapOfShapeListOfShape,
    feature_edges: IndexedMapOfShape,
}
impl TrimMaps {
    fn collect(feature: &Shape, result: &Shape) -> Result<Self, Failure> {
        let mut extended_edge_faces = IndexedDataMapOfShapeListOfShape::default();
        topexp::map_shapes_and_ancestors(
            result,
            ShapeEnum::Edge,
            ShapeEnum::Face,
            &mut extended_edge_faces,
        )?;
        let mut feature_edges = IndexedMapOfShape::default();
        topexp::map_shapes_of_type(feature, ShapeEnum::Edge, &mut feature_edges)?;
        Ok(Self {
            extended_edge_faces,
            feature_edges,
        })
    }
    fn inputs<'a>(&'a self, intersector: &'a BOPAlgoBuilder) -> TrimInputs<'a> {
        TrimInputs {
            feature_edges: &self.feature_edges,
            extended_edge_faces: &self.extended_edge_faces,
            intersector,
        }
    }
}
fn trim_intersector(extended: &ExtendedFaces, parallel: bool) -> Result<BOPAlgoBuilder, Failure> {
    let mut intersector = BOPAlgoBuilder::new();
    for position in 1..=extended.extent() {
        intersector.add_argument(extended.find_from_index(position)?);
    }
    intersector.options_mut().set_run_parallel(parallel);
    Ok(intersector)
}
fn run_trim_intersector(
    intersector: &mut BOPAlgoBuilder,
    range: &ProgressRange<'_>,
) -> ProgressCompletion {
    match BuilderHooks::perform_with_progress(intersector, range) {
        Ok(completion) => completion,
        Err(cause) => classify_area_progress(
            ProgressOperation::Builder,
            intersector.options(),
            Err(cause),
            &ProgressCancellation::default(),
        ),
    }
}
fn intersect_extended_on_progress(
    intersector: &mut BOPAlgoBuilder,
    history: &History,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> Result<(Option<Shape>, ProgressCompletion), Failure> {
    if intersector.arguments().extent() <= 1 {
        let first = intersector
            .arguments()
            .first()
            .cloned()
            .ok_or_else(no_extended_faces)?;
        return Ok((Some(first), ProgressCompletion::Completed));
    }
    let completion = run_trim_intersector(intersector, &scope.next(1.));
    if matches!(completion, ProgressCompletion::Cancelled { .. }) {
        observed.record_cancelled();
    }
    if intersector.options().has_errors() {
        return Ok((None, completion));
    }
    if let Some(intersection) = intersector.history() {
        history.merge_handle(&intersection)?;
    }
    Ok((Some(intersector.shape().clone()), completion))
}
fn no_extended_faces() -> Failure {
    Failure::with_type(
        "Standard_NoSuchObject",
        "TrimExtendedFaces received 0 extended faces; expected at least one face",
        "",
    )
}
impl FillGap {
    /// Intersects the extended faces with each other, then trims each by the bounds of its
    /// original face (`TrimExtendedFaces`).
    #[cfg(test)]
    pub(super) fn trim_extended_faces(
        &mut self,
        extended: &ExtendedFaces,
        history: &History,
    ) -> Result<(), Failure> {
        self.trim_extended_faces_on_progress(
            extended,
            history,
            &ProgressRange::default(),
            &ProgressCancellation::default(),
        )
        .map(|_| ())
    }

    /// Trim through the actual ranged intersection while retaining child completion.
    /// Example: `gap.trim_extended_faces_on_progress(extended, history, range, observed)?` is raw execution, not publication.
    pub(super) fn trim_extended_faces_on_progress(
        &mut self,
        extended: &ExtendedFaces,
        history: &History,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<ProgressCompletion, Failure> {
        let mut intersector = trim_intersector(extended, self.run_parallel)?;
        intersector
            .options_mut()
            .set_face_grouping_version(self.face_grouping_version);
        let units = if intersector.arguments().extent() > 1 {
            2.
        } else {
            1.
        };
        let outer = ProgressScope::unnamed(range, units, false);
        let (result, intersection) =
            intersect_extended_on_progress(&mut intersector, history, &outer, observed)?;
        let Some(result) = result else {
            return Ok(intersection);
        };
        let maps = TrimMaps::collect(&self.feature, &result)?;
        let trimmed = self.trim_extended_inputs_on_progress(
            extended,
            history,
            &maps.inputs(&intersector),
            &outer.next(1.),
            observed,
        )?;
        Ok(retain_intersection_completion(intersection, trimmed))
    }

    fn trim_extended_inputs_on_progress(
        &mut self,
        extended: &ExtendedFaces,
        history: &History,
        trim: &TrimInputs<'_>,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<ProgressCompletion, Failure> {
        let scope =
            ProgressScope::with_literal(range, "Trimming faces", extended.extent() as f64, false);
        for position in 1..=extended.extent() {
            if !scope.more() {
                observed.record_cancelled();
                return Ok(cancelled_trim_input(position - 1));
            }
            let original = topods::face(extended.find_key(position)?)?;
            let extension = topods::face(extended.find_from_index(position)?)?;
            self.trim_face(&extension, &original, trim, history)?;
            drop(scope.next(1.));
        }
        Ok(ProgressCompletion::Completed)
    }
}
#[cfg(test)]
#[path = "trimming_tests.rs"]
mod tests;

fn cancelled_trim_input(ordinal: usize) -> ProgressCompletion {
    ProgressCompletion::Cancelled {
        operation: ProgressOperation::RemoveFeatures,
        stage: ProgressStage::Images,
        context: Some(ProgressWorksite {
            label: "TrimExtendedFaces",
            input_ordinal: Some(ordinal),
            entity_key: None,
        }),
    }
}
