//! Native face collection, task dispatch and ordered report/image merge remain one operation path.
use super::tasks::{is_confirmed_failure, perform_face_tasks};
use super::{BOPAlgoBuilder, PreparedFaceSplits};
use crate::{
    bop_algo::{
        ProgressCancellation, ProgressCompletion, ProgressOperation, ProgressStage,
        builder_area::progress::observe_user_break,
    },
    bop_ds::DataStructure,
};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::Failure,
};

impl BOPAlgoBuilder {
    /// Carry actual Face outcomes through the native preparation and ordered raw merge.
    /// Example: `builder.build_split_faces_progress_raw(ds, range, observed)?;` then guard publication.
    pub(crate) fn build_split_faces_progress_raw(
        &mut self,
        structure: &DataStructure,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<ProgressCompletion, Failure> {
        let scope = ProgressScope::unnamed(range, 10.0, false);
        let Some((splits, builders)) = self.collect_face_splits(structure, |actual| {
            observe_user_break(actual.options(), &scope, observed)
        })?
        else {
            return Ok(collection_cancelled());
        };
        drop(scope.next(1.0));
        self.run_prepared_face_tasks(structure, (splits, builders), &scope, observed)
    }

    fn run_prepared_face_tasks(
        &mut self,
        structure: &DataStructure,
        prepared: PreparedFaceSplits,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<ProgressCompletion, Failure> {
        let (splits, builders) = prepared;
        let parallel = self.options().run_parallel();
        let (builders, completion) =
            perform_face_tasks(builders, parallel, &scope.next(9.0), observed)?;
        if observe_user_break(self.options(), scope, observed) {
            return Ok(cancelled_after_children(completion));
        }
        self.merge_prepared_face_splits(structure, splits, &builders)?;
        Ok(completion)
    }
}

fn collection_cancelled() -> ProgressCompletion {
    ProgressCompletion::Cancelled {
        operation: ProgressOperation::Builder,
        stage: ProgressStage::Images,
        context: None,
    }
}

/// Preserve an already-confirmed child failure when a later native poll observes cancellation.
/// Example: `cancelled_after_children(child)` never drops an owned exception cause.
pub(super) fn cancelled_after_children(completion: ProgressCompletion) -> ProgressCompletion {
    if is_confirmed_failure(&completion) {
        return completion;
    }
    collection_cancelled()
}
