//! One weighted face-image pipeline composes actual child outcomes before publication.
use super::BOPAlgoBuilder;
use crate::bop_algo::{
    ProgressCancellation, ProgressCompletion, ProgressOperation,
    builder_area::progress::classify_area_progress,
};
use crate::bop_ds::DataStructure;
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::Failure,
};
type FaceImageStage = fn(
    &mut BOPAlgoBuilder,
    &DataStructure,
    &ProgressRange<'_>,
    &ProgressCancellation,
) -> Result<ProgressCompletion, Failure>;

impl BOPAlgoBuilder {
    /// Dispatch real face stages with native weights; `builder.fill_images_faces_progress_raw(ds, range, observed)?`.
    /// Raw images remain diagnostic until the enclosing operation validates publication.
    pub(crate) fn fill_images_faces_progress_raw(
        &mut self,
        structure: &DataStructure,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<ProgressCompletion, Failure> {
        let scope = ProgressScope::with_literal(range, "Filling splits of faces", 10., false);
        let stages: [(f64, FaceImageStage); 3] = [
            (9., Self::build_split_faces_progress_raw),
            (0.5, Self::fill_same_domain_faces_progress_raw),
            (0.5, Self::fill_internal_vertices_progress_raw),
        ];
        let mut outcome = ProgressCompletion::Completed;
        for (weight, stage) in stages {
            let child = stage(self, structure, &scope.next(weight), observed)?;
            outcome = merge_face_outcomes(outcome, child);
            if self.options().has_errors() || terminal_face_stop(&outcome) {
                break;
            }
        }
        if matches!(outcome, ProgressCompletion::Completed) {
            return Ok(classify_area_progress(
                ProgressOperation::Builder,
                self.options(),
                Ok(()),
                observed,
            ));
        }
        Ok(outcome)
    }
}
fn terminal_face_stop(completion: &ProgressCompletion) -> bool {
    matches!(
        completion,
        ProgressCompletion::Failed { .. } | ProgressCompletion::Incomplete { .. }
    )
}
fn merge_face_outcomes(
    previous: ProgressCompletion,
    next: ProgressCompletion,
) -> ProgressCompletion {
    if terminal_face_stop(&next) || matches!(previous, ProgressCompletion::Completed) {
        return next;
    }
    previous
}

#[cfg(test)]
#[path = "dispatch_contract.rs"]
mod tests;
