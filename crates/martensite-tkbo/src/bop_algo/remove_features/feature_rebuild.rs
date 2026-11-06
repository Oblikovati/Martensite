//! Rebuild and history boundaries of one prepared feature removal.
use super::super::BOPAlgoRemoveFeatures;
use super::super::fill_gap::FillGap;
use super::super::helpers::{
    compound_of, explore, find_internals, make_removed, remove_internal_wires,
};
use super::super::solids::{
    FeatureInputs, OriginalFaces, fill_solids_history, get_original_faces, get_valid_solids,
};
use crate::bop_algo::{
    AlertUnableToRemoveTheFeature, BOPAlgoMakerVolume, BuilderHooks, ProgressCompletion,
    ProgressOperation, ProgressStage, ProgressWorksite,
    progress_result::retain_intersection_completion,
};
use martensite_tkbrep::brep_tools::History;
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::{IndexedMapOfShape, MapOfShape};
use martensite_tkernel::message::{AlertHandle, ProgressRange, ProgressScope};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use std::sync::Arc;

/// The validated rebuild of the solids of one feature.
struct Rebuilt {
    maker: BOPAlgoMakerVolume,
    faces: OriginalFaces,
    solids: Vec<Shape>,
    /// The faces and solids dropped from the maker-volume result.
    removed: Vec<Shape>,
}

impl BOPAlgoRemoveFeatures {
    /// Rebuilds the solids of the feature from their kept faces and the reconstructed
    /// adjacent faces with the maker-volume algorithm; any invalid outcome only warns
    /// (`RemoveFeature`).
    #[cfg(test)]
    pub(super) fn remove_feature(
        &mut self,
        gap: &FillGap,
        solids_history_needed: bool,
    ) -> Result<(), Failure> {
        self.remove_feature_on_progress(gap, solids_history_needed, &ProgressRange::default())?;
        Ok(())
    }

    /// Rebuild one prepared feature in its caller range, retaining actual child completion.
    /// Example: `remover.remove_feature_on_progress(gap, true, range)?` requires publication checks.
    pub(in crate::bop_algo::remove_features) fn remove_feature_on_progress(
        &mut self,
        gap: &FillGap,
        solids_history_needed: bool,
        range: &ProgressRange<'_>,
    ) -> Result<ProgressCompletion, Failure> {
        let handle = self.base.history.clone();
        let history = handle.get().ok_or_else(null_history)?;
        if gap.faces.extent() == 0 && gap.has_adjacent_faces {
            return Ok(self.unavailable_feature(gap));
        }
        let scope = ProgressScope::unnamed(range, 100., false);
        let (candidate, completion) = self.rebuild_solids(gap, history, &scope)?;
        if let Some(mut rebuilt) = candidate {
            self.finish_rebuilt_feature(gap, &mut rebuilt, history, solids_history_needed, &scope)?;
        }
        Ok(completion)
    }

    fn finish_rebuilt_feature(
        &mut self,
        gap: &FillGap,
        rebuilt: &mut Rebuilt,
        history: &History,
        solids_history_needed: bool,
        scope: &ProgressScope<'_>,
    ) -> Result<(), Failure> {
        drop(scope.next(3.));
        remove_internal_wires(&rebuilt.solids, Some(&mut rebuilt.faces.internals))?;
        self.merge_rebuilt_history(gap, rebuilt, history)?;
        drop(scope.next(3.));
        if solids_history_needed {
            merge_rebuilt_solids_history(gap, rebuilt, history)?;
        }
        rebuilt
            .solids
            .extend(rebuilt.faces.untouched.iter().cloned());
        self.base.shape = compound_of(&rebuilt.solids)?;
        Ok(())
    }

    fn merge_rebuilt_history(
        &self,
        gap: &FillGap,
        rebuilt: &mut Rebuilt,
        history: &History,
    ) -> Result<(), Failure> {
        history.merge_handle(&gap.history)?;
        if let Some(maker_history) = rebuilt.maker.history() {
            history.merge_handle(&maker_history)?;
        }
        let removals = rebuilt
            .faces
            .internals
            .iter()
            .chain(&rebuilt.removed)
            .cloned()
            .collect();
        self.record_removals(&rebuilt.solids, removals, history)
    }

    /// The solids of the feature rebuilt by the maker-volume algorithm and validated;
    /// None when the removal must only warn: adjacent faces not rebuilt, no solid, a face
    /// to keep lost, or not one result solid per input solid.
    fn rebuild_solids(
        &self,
        gap: &FillGap,
        history: &History,
        scope: &ProgressScope<'_>,
    ) -> Result<(Option<Rebuilt>, ProgressCompletion), Failure> {
        let (faces, original, mut maker) = self.prepare_solid_rebuild(gap, history, scope)?;
        let completion = retain_feature_volume_boundary(&mut maker, &scope.next(90.))?;
        let kept = self.check_rebuilt_faces(gap, &maker, &faces, history, scope)?;
        let candidate = if kept == Some(true) {
            self.select_rebuilt_solids(gap, maker, faces, &original)?
        } else {
            None
        };
        let completion = if kept.is_none() {
            retain_intersection_completion(completion, stopped_feature_validation())
        } else {
            completion
        };
        Ok((candidate, completion))
    }

    fn check_rebuilt_faces(
        &self,
        gap: &FillGap,
        maker: &BOPAlgoMakerVolume,
        faces: &OriginalFaces,
        history: &History,
        scope: &ProgressScope<'_>,
    ) -> Result<Option<bool>, Failure> {
        if maker.options().has_errors() || explore(maker.shape(), ShapeEnum::Solid)?.is_empty() {
            self.warn_unable_to_remove(&gap.feature);
            return Ok(Some(false));
        }
        let kept = keeps_every_face(maker, faces, gap, history, scope)?;
        if kept == Some(false) {
            self.warn_unable_to_remove(&gap.feature);
        }
        Ok(kept)
    }

    fn prepare_solid_rebuild(
        &self,
        gap: &FillGap,
        history: &History,
        scope: &ProgressScope<'_>,
    ) -> Result<(OriginalFaces, Shape, BOPAlgoMakerVolume), Failure> {
        // Without adjacent faces an internal part is removed: no fuse is needed.
        let fuse = gap.faces.extent() > 0;
        let inputs = FeatureInputs {
            solids: &gap.solids,
            feature_faces: &gap.feature_faces,
            adjacent_faces: &gap.faces,
        };
        let mut faces = get_original_faces(&self.base.shape, &inputs, history)?;
        drop(scope.next(3.));
        let kept: Vec<Shape> = faces.to_be_kept.iter().cloned().collect();
        let original = compound_of(&kept)?;
        let mut maker = self.volume_maker(fuse, &original);
        self.add_adjacent_faces(gap, &mut maker, &mut faces)?;
        Ok((faces, original, maker))
    }

    fn select_rebuilt_solids(
        &self,
        gap: &FillGap,
        maker: BOPAlgoMakerVolume,
        faces: OriginalFaces,
        original: &Shape,
    ) -> Result<Option<Rebuilt>, Failure> {
        let mut removed = Vec::new();
        let solids = get_valid_solids(&maker, &faces, original, gap.solids.extent(), &mut removed)?;
        if solids.len() != gap.solids.extent() {
            self.warn_unable_to_remove(&gap.feature);
            return Ok(None);
        }
        Ok(Some(Rebuilt {
            maker,
            faces,
            solids,
            removed,
        }))
    }

    fn unavailable_feature(&self, gap: &FillGap) -> ProgressCompletion {
        self.warn_unable_to_remove(&gap.feature);
        ProgressCompletion::Completed
    }

    /// The maker-volume tool on the original faces, avoiding internal shapes and keeping
    /// the inputs.
    fn volume_maker(&self, fuse: bool, original: &Shape) -> BOPAlgoMakerVolume {
        let mut maker = BOPAlgoMakerVolume::new();
        maker
            .options_mut()
            .set_face_grouping_version(self.base.options.face_grouping_version());
        maker
            .options_mut()
            .set_run_parallel(self.base.options.run_parallel());
        maker.set_avoid_internal_shapes(true);
        maker.set_intersect(fuse);
        maker.set_non_destructive(true);
        maker.add_argument(original);
        maker
    }

    /// Adds the reconstructed adjacent faces (several images of one face as a compound, so
    /// they do not intersect) to the maker and to the faces to keep; with the history, the
    /// internal parts of the original adjacent faces are collected.
    fn add_adjacent_faces(
        &self,
        gap: &FillGap,
        maker: &mut BOPAlgoMakerVolume,
        faces: &mut OriginalFaces,
    ) -> Result<(), Failure> {
        for position in 1..=gap.faces.extent() {
            let images: Vec<Shape> = gap
                .faces
                .find_from_index(position)?
                .iter()
                .cloned()
                .collect();
            let argument = match images.as_slice() {
                [single] => single.clone(),
                _ => compound_of(&images)?,
            };
            maker.add_argument(&argument);
            faces.to_be_kept.add(argument);
            if self.base.fill_history {
                find_internals(gap.faces.find_key(position)?, &mut faces.internals);
            }
        }
        Ok(())
    }

    /// With the history, the internal shapes and the dropped faces and solids without trace
    /// in the solids are removed.
    fn record_removals(
        &self,
        solids: &[Shape],
        removals: Vec<Shape>,
        history: &History,
    ) -> Result<(), Failure> {
        if !self.base.fill_history {
            return Ok(());
        }
        let mut result_shapes = IndexedMapOfShape::default();
        for solid in solids {
            topexp::map_shapes(solid, &mut result_shapes, true, true);
        }
        let removed = History::new();
        make_removed(&removals, &removed, &result_shapes)?;
        history.merge(&removed)
    }

    fn warn_unable_to_remove(&self, feature: &Shape) {
        let alert: AlertHandle = Arc::new(AlertUnableToRemoveTheFeature::new(feature));
        self.base.options.add_warning(&alert);
    }
}

/// Whether every face to keep, except the earlier splits of the adjacent faces, keeps a
/// face in the maker-volume result.
fn keeps_every_face(
    maker: &BOPAlgoMakerVolume,
    faces: &OriginalFaces,
    gap: &FillGap,
    history: &History,
    scope: &ProgressScope<'_>,
) -> Result<Option<bool>, Failure> {
    let Some(adjacent_splits) = feature_adjacent_splits(gap, history, scope)? else {
        return Ok(None);
    };
    for shape in faces
        .to_be_kept
        .iter()
        .filter(|shape| !adjacent_splits.contains(shape))
    {
        if !scope.more() {
            return Ok(None);
        }
        if !retains_any_face(maker, shape)? {
            return Ok(Some(false));
        }
    }
    Ok(Some(true))
}
fn feature_adjacent_splits(
    gap: &FillGap,
    history: &History,
    scope: &ProgressScope<'_>,
) -> Result<Option<MapOfShape>, Failure> {
    let mut adjacent_splits = MapOfShape::default();
    for face in (1..=gap.faces.extent()).map(|position| gap.faces.find_key(position)) {
        if !scope.more() {
            return Ok(None);
        }
        let face = face?;
        let images = history.modified(face)?;
        if images.is_empty() {
            adjacent_splits.add(face.clone());
        }
        for image in images {
            adjacent_splits.add(image);
        }
    }
    Ok(Some(adjacent_splits))
}
fn retains_any_face(maker: &BOPAlgoMakerVolume, shape: &Shape) -> Result<bool, Failure> {
    for face in explore(shape, ShapeEnum::Face)? {
        if !maker.is_deleted(&face)? {
            return Ok(true);
        }
    }
    Ok(false)
}
fn merge_rebuilt_solids_history(
    gap: &FillGap,
    rebuilt: &mut Rebuilt,
    history: &History,
) -> Result<(), Failure> {
    let solids_history = History::new();
    fill_solids_history(
        &rebuilt.faces.solids_to_rebuild,
        &mut rebuilt.solids,
        &gap.faces,
        &rebuilt.maker,
        &solids_history,
    )?;
    history.merge(&solids_history)
}
fn stopped_feature_validation() -> ProgressCompletion {
    ProgressCompletion::Cancelled {
        operation: ProgressOperation::RemoveFeatures,
        stage: ProgressStage::Classification,
        context: Some(ProgressWorksite {
            label: "RemoveFeature.ValidateFaces",
            input_ordinal: None,
            entity_key: None,
        }),
    }
}

/// The failure of a removal run without the history `RemoveFeatures` creates.
fn null_history() -> Failure {
    Failure::with_type(
        "Standard_NullObject",
        "BOPAlgoRemoveFeatures::RemoveFeature: the history is null; expected RemoveFeatures to create it",
        "",
    )
}

#[cfg(test)]
#[path = "feature_rebuild_tests.rs"]
mod tests;

// The generic receiver returns its owned cause after its real PerformInternal catch adds
// BuilderFailed. Removal consumes that caught result, warns, and proceeds to later features.
fn retain_feature_volume_boundary(
    maker: &mut BOPAlgoMakerVolume,
    range: &ProgressRange<'_>,
) -> Result<ProgressCompletion, Failure> {
    use crate::bop_algo::{AlertBuilderFailed, ProgressFailureClass};
    use martensite_tkernel::standard::RuntimeTyped;
    match maker.perform_with_progress(range) {
        Err(cause) if maker.options().has_error(AlertBuilderFailed::static_type()) => {
            Ok(ProgressCompletion::Failed {
                operation: ProgressOperation::MakerVolume,
                stage: ProgressStage::Completion,
                class: ProgressFailureClass::KernelFailure,
                cause: Some(cause),
                context: None,
            })
        }
        outcome => outcome,
    }
}
