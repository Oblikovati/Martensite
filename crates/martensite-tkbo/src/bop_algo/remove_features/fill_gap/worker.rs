//! One feature-gap worker owns its history and final failure boundary.
use super::{Connections, FillGap};
use crate::bop_algo::{
    ProgressCancellation, ProgressCompletion, ProgressFailureClass, ProgressOperation,
    ProgressStage, ProgressWorksite, progress_result::retain_intersection_completion,
};
use martensite_tkbrep::brep_tools::History;
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::{Failure, Handle},
};

impl FillGap {
    /// Finds, extends and trims the adjacent faces; a failure marks the feature as having
    /// adjacent faces without reconstruction, so the removal warns (`Perform`).
    /// Example: `gap.perform(connections)` runs the same worker without an active indicator.
    #[cfg(test)]
    pub(in crate::bop_algo::remove_features) fn perform(&mut self, connections: &Connections<'_>) {
        let _ = self.perform_on_progress(connections, &ProgressRange::default());
    }

    /// Run this complete worker in its caller range and retain actual child completion.
    /// Example: `gap.perform_on_progress(connections, range)` still requires parent/publication checks.
    pub(in crate::bop_algo::remove_features) fn perform_on_progress(
        &mut self,
        connections: &Connections<'_>,
        range: &ProgressRange<'_>,
    ) -> ProgressCompletion {
        let observed = ProgressCancellation::default();
        let outcome = self.fill_in_range(connections, range, &observed);
        self.finish_gap_worker(outcome)
    }

    fn fill_in_range(
        &mut self,
        connections: &Connections<'_>,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<ProgressCompletion, Failure> {
        let scope = ProgressScope::unnamed(range, 3., false);
        self.history = Handle::new(History::new());
        let owner = self.history.clone();
        let history = owner.get().ok_or_else(|| {
            Failure::with_type(
                "Standard_NullObject",
                "FillGap history=null; expected freshly allocated non-null history",
                "",
            )
        })?;
        self.fill_in_scope(connections, history, &scope, observed)
    }

    fn fill_in_scope(
        &mut self,
        connections: &Connections<'_>,
        history: &History,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<ProgressCompletion, Failure> {
        let adjacent =
            self.find_adjacent_faces_on_progress(connections, &scope.next(1.), observed)?;
        if !gap_parent_more(scope, observed) {
            return Ok(stopped_gap_parent("FillGap.AfterAdjacency"));
        }
        self.has_adjacent_faces = adjacent.extent() > 0;
        if !self.has_adjacent_faces {
            return Ok(local_gap_completion(observed));
        }
        let extended =
            self.extend_adjacent_faces_on_progress(&adjacent, history, &scope.next(1.), observed)?;
        if !gap_parent_more(scope, observed) {
            return Ok(stopped_gap_parent("FillGap.AfterExtension"));
        }
        let earlier = local_gap_completion(observed);
        let trimmed =
            self.trim_extended_faces_on_progress(&extended, history, &scope.next(1.), observed)?;
        Ok(retain_intersection_completion(earlier, trimmed))
    }

    fn finish_gap_worker(
        &mut self,
        outcome: Result<ProgressCompletion, Failure>,
    ) -> ProgressCompletion {
        let cause = match outcome {
            Ok(completion) => return completion,
            Err(cause) => cause,
        };
        // The source catch runs after the worker scope closes and preserves partial history.
        self.has_adjacent_faces = true;
        self.faces.clear(false);
        ProgressCompletion::Failed {
            operation: ProgressOperation::RemoveFeatures,
            stage: ProgressStage::Worker,
            class: ProgressFailureClass::KernelFailure,
            cause: Some(cause),
            context: Some(ProgressWorksite {
                label: "FillGap",
                input_ordinal: None,
                entity_key: None,
            }),
        }
    }
}
fn gap_parent_more(scope: &ProgressScope<'_>, observed: &ProgressCancellation) -> bool {
    let more = scope.more();
    if !more {
        observed.record_cancelled();
    }
    more
}
fn stopped_gap_parent(label: &'static str) -> ProgressCompletion {
    ProgressCompletion::Cancelled {
        operation: ProgressOperation::RemoveFeatures,
        stage: ProgressStage::Worker,
        context: Some(ProgressWorksite {
            label,
            input_ordinal: None,
            entity_key: None,
        }),
    }
}
fn local_gap_completion(observed: &ProgressCancellation) -> ProgressCompletion {
    if !observed.is_cancelled() {
        return ProgressCompletion::Completed;
    }
    ProgressCompletion::Cancelled {
        operation: ProgressOperation::RemoveFeatures,
        stage: ProgressStage::Worker,
        context: None,
    }
}
#[cfg(test)]
#[path = "worker_tests.rs"]
mod tests;
