//! Raw local stages retain native scope names, polling sites and loop advances.
//! Result::Ok is only choreography: callers must classify the actual report/stop reason.
use super::helpers::{compound_of, explore};
use super::{AlertNoFacesToRemove, BOPAlgoRemoveFeatures, alert};
use crate::bop_algo::{ProgressCancellation, builder_area::progress::observe_user_break};
use crate::bop_tools::make_connexity_blocks;
use martensite_tkbrep::brep_tools::History;
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::{IndexedDataMapOfShapeListOfShape, IndexedMapOfShape};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkshhealing::shape_upgrade::ShapeUpgradeUnifySameDomain;

// Preserve the literal native label, including its missing space.
const SIMPLIFY_SCOPE: &str = "Simplifyingthe result";

impl BOPAlgoRemoveFeatures {
    /// Runs actual collection/grouping; Ok alone does not establish completion.
    /// Example: `remover.prepare_features_progress_raw(&range)?;` then inspect its report.
    #[cfg(test)]
    pub(super) fn prepare_features_progress_raw(
        &mut self,
        range: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        self.prepare_features_observed(range, &ProgressCancellation::default())
    }

    pub(super) fn prepare_features_observed(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        topexp::map_shapes(&self.input_shape, &mut self.inputs_map, true, true);
        let outer = ProgressScope::with_literal(range, "Preparing features", 2.0, false);
        let collect = outer.next(1.0);
        let count = self.faces_to_remove.len() as f64;
        let scope =
            ProgressScope::with_literal(&collect, "Preparing the faces to remove", count, false);
        let Some(faces) = self.collect_requested_faces(&scope, observed)? else {
            return Ok(());
        };
        self.group_requested_faces(&faces)
    }

    fn collect_requested_faces(
        &self,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Option<Vec<Shape>>, Failure> {
        let mut faces = Vec::new();
        for shape in self.faces_to_remove.iter() {
            if !self.collect_input_faces(shape, &mut faces, scope, observed)? {
                return Ok(None);
            }
            drop(scope.next(1.0));
        }
        Ok(Some(faces))
    }

    fn collect_input_faces(
        &self,
        shape: &Shape,
        faces: &mut Vec<Shape>,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<bool, Failure> {
        for face in explore(shape, ShapeEnum::Face)? {
            if observe_user_break(&self.base.options, scope, observed) {
                return Ok(false);
            }
            if self.inputs_map.contains(&face) {
                faces.push(face);
            }
        }
        Ok(true)
    }

    fn group_requested_faces(&mut self, faces: &[Shape]) -> Result<(), Failure> {
        if faces.is_empty() {
            self.base.options.add_error(&alert(AlertNoFacesToRemove));
            return Ok(());
        }
        self.features = requested_feature_blocks(faces)?;
        Ok(())
    }

    /// Updates the same input history and preserves the native post-filter polling site.
    /// Example: `remover.update_history_progress_raw(&range)?;` then classify the report.
    #[cfg(test)]
    pub(super) fn update_history_progress_raw(
        &mut self,
        range: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        self.update_history_observed(range, &ProgressCancellation::default())
    }

    /// Marks input sub-shapes without a result trace as removed, retaining confirmed stops.
    /// Example: `remover.update_history_observed(range, observed)?`.
    pub(super) fn update_history_observed(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let handle = self.filled_history();
        let Some(history) = handle.get() else {
            return Ok(());
        };
        self.rebuild_result_map();
        let Some(removals) = self.history_input_removals(history, range, observed)? else {
            return Ok(());
        };
        history.merge(&removals)
    }

    fn rebuild_result_map(&mut self) {
        self.base.map_shape.clear(false);
        let mut result_shapes = IndexedMapOfShape::default();
        topexp::map_shapes(&self.base.shape, &mut result_shapes, true, true);
        for shape in result_shapes.iter() {
            self.base.map_shape.add(shape.clone());
        }
    }

    fn history_input_removals(
        &self,
        history: &History,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Option<History>, Failure> {
        let removals = History::new();
        let count = self.inputs_map.extent() as f64;
        let scope = ProgressScope::with_literal(range, "Updating history", count, false);
        for shape in self.inputs_map.iter() {
            if !self.record_input_removals(shape, history, &removals, &scope, observed)? {
                return Ok(None);
            }
            drop(scope.next(1.0));
        }
        Ok(Some(removals))
    }

    fn record_input_removals(
        &self,
        shape: &Shape,
        history: &History,
        removals: &History,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<bool, Failure> {
        if !History::is_supported_type(shape)? || history.is_removed(shape)? {
            return Ok(true);
        }
        if observe_user_break(&self.base.options, scope, observed) {
            return Ok(false);
        }
        self.record_missing_images(shape, history, removals)?;
        Ok(true)
    }

    fn record_missing_images(
        &self,
        shape: &Shape,
        history: &History,
        removals: &History,
    ) -> Result<(), Failure> {
        let images = history.modified(shape)?;
        if images.is_empty() && !self.base.map_shape.contains(shape) {
            removals.remove(shape)?;
        }
        for image in images
            .iter()
            .filter(|image| !self.base.map_shape.contains(image))
        {
            removals.remove(image)?;
        }
        Ok(())
    }

    /// Runs the same unifier after native keep-shape polling, without adding a build range.
    /// Example: `remover.simplify_result_progress_raw(&range)?;` then classify the report.
    #[cfg(test)]
    pub(super) fn simplify_result_progress_raw(
        &mut self,
        range: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        self.simplify_result_observed(range, &ProgressCancellation::default())
    }

    /// Unifies rebuilt faces and edges while keeping input sub-shapes still held by the result.
    /// Example: `remover.simplify_result_observed(range, observed)?`.
    pub(super) fn simplify_result_observed(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        if self.base.shape.is_same(&self.input_shape) {
            return Ok(());
        }
        let outer = ProgressScope::with_literal(range, SIMPLIFY_SCOPE, 2.0, false);
        let unifier = self.configured_unifier()?;
        if self.base.map_shape.is_empty() {
            self.rebuild_result_map();
        }
        if !self.keep_input_shapes(&unifier, &outer.next(1.0), observed)? {
            return Ok(());
        }
        unifier.build()?;
        drop(outer.next(1.0));
        self.finish_unification(&unifier)
    }

    fn configured_unifier(&self) -> Result<ShapeUpgradeUnifySameDomain, Failure> {
        let unifier = ShapeUpgradeUnifySameDomain::empty();
        unifier.initialize(&self.base.shape, true, true, false)?;
        unifier.allow_internal_edges(false);
        Ok(unifier)
    }

    fn keep_input_shapes(
        &self,
        unifier: &ShapeUpgradeUnifySameDomain,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<bool, Failure> {
        let scope = ProgressScope::unnamed(range, self.inputs_map.extent() as f64, false);
        for input in self.inputs_map.iter() {
            if observe_user_break(&self.base.options, &scope, observed) {
                return Ok(false);
            }
            if self.base.map_shape.contains(input) {
                unifier.keep_shape(input)?;
            }
            drop(scope.next(1.0));
        }
        Ok(true)
    }

    fn finish_unification(&mut self, unifier: &ShapeUpgradeUnifySameDomain) -> Result<(), Failure> {
        self.base.shape = unifier.shape();
        if let Some(history) = self.filled_history().get() {
            history.merge_handle(&unifier.history())?;
        }
        Ok(())
    }
}

fn requested_feature_blocks(faces: &[Shape]) -> Result<Vec<Shape>, Failure> {
    let mut ancestors = IndexedDataMapOfShapeListOfShape::default();
    let mut blocks = Vec::new();
    let compound = compound_of(faces)?;
    make_connexity_blocks(
        &compound,
        ShapeEnum::Edge,
        ShapeEnum::Face,
        &mut blocks,
        &mut ancestors,
    )?;
    blocks.iter().map(|block| compound_of(block)).collect()
}

#[cfg(test)]
#[path = "local_progress_contract.rs"]
mod contract;
