//! Face preparation remains on the actual receiver; the base only choreographs its hooks.
use super::BuilderFace;
use crate::bop_algo::builder_area::progress::{
    BuilderAreaHooks, classify_area_progress, perform_area_stages_in_scope,
};
use crate::bop_algo::{BuilderArea, ProgressCancellation, ProgressCompletion, ProgressOperation};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::Failure,
};

struct FaceStageReceiver<'a> {
    face: &'a mut BuilderFace,
    observed: &'a ProgressCancellation,
}

impl BuilderFace {
    /// Build faces in the caller's progress interval and return raw execution completion.
    /// Example: `let outcome = face.perform_with_progress(&indicator.start());`.
    /// Cancelled or failed areas remain diagnostic state, not successful output. This raw
    /// builder shares input topology; completion does not establish deep isolation or validity.
    pub fn perform_with_progress(&mut self, range: &ProgressRange<'_>) -> ProgressCompletion {
        let observed = ProgressCancellation::default();
        let result = self.perform_observed(range, &observed);
        classify_area_progress(
            ProgressOperation::BuilderFace,
            self.options(),
            result,
            &observed,
        )
    }

    fn perform_observed(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let scope = ProgressScope::unnamed(range, 100., false);
        let mut receiver = FaceStageReceiver {
            face: self,
            observed,
        };
        receiver.area_mut().options_mut().clear();
        receiver.face.check_data();
        if receiver.area().options().has_errors() {
            return Ok(());
        }
        perform_area_stages_in_scope(&mut receiver, &scope)
    }
}

impl BuilderAreaHooks for FaceStageReceiver<'_> {
    fn area(&self) -> &BuilderArea {
        // Report stops must inspect this Face receiver, never a detached base operation.
        &self.face.area
    }

    fn area_mut(&mut self) -> &mut BuilderArea {
        // Receiver-specific preparation retains the same Options and Context identities.
        &mut self.face.area
    }

    fn perform_shapes_to_avoid(&mut self, range: &ProgressRange<'_>) -> Result<(), Failure> {
        let face: &mut BuilderFace = self.face;
        face.perform_shapes_to_avoid(range, self.observed)
    }

    fn perform_loops(&mut self, range: &ProgressRange<'_>) -> Result<(), Failure> {
        let face: &mut BuilderFace = self.face;
        face.perform_loops(range, self.observed)
    }

    fn perform_areas(&mut self, range: &ProgressRange<'_>) -> Result<(), Failure> {
        let face: &mut BuilderFace = self.face;
        face.perform_areas(range, self.observed)
    }

    fn perform_internal_shapes(&mut self, range: &ProgressRange<'_>) -> Result<(), Failure> {
        let face: &mut BuilderFace = self.face;
        face.perform_internal_shapes(range, self.observed)
    }
}

#[cfg(test)]
#[path = "progress_contract.rs"]
mod contract;
