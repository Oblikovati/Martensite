//! Actual draft-solid collection and classification share the native two-unit scope.
use super::*;
use crate::bop_algo::{
    ProgressCancellation, ProgressCompletion, ProgressOperation,
    builder_area::progress::{classify_area_progress, observe_user_break},
    tools::{FaceClassificationJob, classify_faces_with_progress},
};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
struct SolidClassificationInputs {
    boxes: ShapeBoxMap,
    faces: List<Shape>,
    solids: List<Shape>,
    internal: SolidInternalFaces,
    drafts: DraftSolids,
}
impl BOPAlgoBuilder {
    /// Classify actual source faces against draft solids; `builder.fill_in_3d_parts_progress_raw(ds, drafts, range, observed)?`.
    /// Returned maps are raw intermediates until the enclosing operation qualifies publication.
    pub(crate) fn fill_in_3d_parts_progress_raw(
        &mut self,
        structure: &mut DataStructure,
        draft_solids: &mut SameDomainShapes,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<ProgressCompletion, Failure> {
        let scope = ProgressScope::unnamed(range, 2., false);
        let mut boxes = DataMap::with_hasher(ShapeMapHasher, 1);
        let Some(faces) = self.all_face_splits(structure, &mut boxes, &scope, observed)? else {
            return Ok(classify_area_progress(
                ProgressOperation::Builder,
                self.options(),
                Ok(()),
                observed,
            ));
        };
        let mut inputs = SolidClassificationInputs {
            boxes,
            faces,
            solids: List::default(),
            internal: DataMap::with_hasher(ShapeMapHasher, 1),
            drafts: IndexedDataMap::with_hasher(ShapeMapHasher, 1),
        };
        if !self.collect_classification_drafts(structure, &mut inputs, &scope, observed)? {
            return Ok(classify_area_progress(
                ProgressOperation::Builder,
                self.options(),
                Ok(()),
                observed,
            ));
        }
        self.classify_and_bind_drafts(inputs, draft_solids, &scope, observed)
    }
    fn collect_classification_drafts(
        &mut self,
        structure: &mut DataStructure,
        inputs: &mut SolidClassificationInputs,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<bool, Failure> {
        for index in 0..structure.nb_source_shapes() {
            if structure.shape_info(index)?.shape_type() != ShapeEnum::Solid {
                continue;
            }
            if observe_user_break(self.options(), scope, observed) {
                return Ok(false);
            }
            let bounds = self.solid_box(structure, index)?;
            let solid = structure.shape_info(index)?.shape().clone();
            let mut internal = List::default();
            let draft = self.build_draft_solid(&solid, &mut internal)?;
            inputs.solids.append(draft.clone());
            inputs.internal.bind(draft.clone(), internal);
            inputs.boxes.bind(draft.clone(), bounds);
            inputs.drafts.add(solid, draft);
        }
        Ok(true)
    }
    fn classify_and_bind_drafts(
        &mut self,
        inputs: SolidClassificationInputs,
        drafts: &mut SameDomainShapes,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<ProgressCompletion, Failure> {
        let handle = self.context.clone();
        let mut parts = IndexedDataMap::with_hasher(ShapeMapHasher, 1);
        let child = classify_faces_with_progress(
            FaceClassificationJob {
                faces: &inputs.faces,
                solids: &inputs.solids,
                context: required_context(&handle)?,
                parts: &mut parts,
                boxes: &inputs.boxes,
                internal: &inputs.internal,
                parallel: self.options().run_parallel(),
            },
            &scope.next(1.),
        );
        if matches!(
            child,
            ProgressCompletion::Failed { .. } | ProgressCompletion::Incomplete { .. }
        ) {
            return Ok(child);
        }
        if matches!(child, ProgressCompletion::Cancelled { .. }) {
            observed.record_cancelled();
        }
        self.bind_in_parts(
            &inputs.drafts,
            &parts,
            &inputs.internal,
            drafts,
            scope,
            observed,
        )?;
        if !matches!(child, ProgressCompletion::Completed) {
            return Ok(child);
        }
        Ok(classify_area_progress(
            ProgressOperation::Builder,
            self.options(),
            Ok(()),
            observed,
        ))
    }
}
#[cfg(test)]
#[path = "progress_contract.rs"]
mod tests;
