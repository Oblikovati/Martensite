//! Raw execution classification neither polls nor certifies geometry for publication.
use super::*;
use crate::bop_algo::{ProgressOperation, builder_area::progress::classify_area_progress};
use martensite_tkernel::message::ProgressRange;
impl BOPAlgoPaveFiller {
    /// Run the base receiver through the real weighted pipeline and preserve the native catch.
    /// Example: `let completion = filler.perform_on_progress(range);` still requires validation.
    pub(crate) fn perform_on_progress(&mut self, range: &ProgressRange<'_>) -> ProgressCompletion {
        self.perform_on_progress_observed(range, &ProgressCancellation::default())
    }
    /// Share actual cancellation evidence through recursive section work; `nested.perform_on_progress_observed(range, observed)`.
    pub(in crate::bop_algo::pave_filler) fn perform_on_progress_observed(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> ProgressCompletion {
        self.perform_on_progress_with_initializer(range, observed, Self::initialize_with_progress)
    }
    /// Retain the actual derived initializer and native catch; `filler.perform_on_progress_with_initializer(range, observed, initialize)`.
    pub(crate) fn perform_on_progress_with_initializer(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
        initialize: impl FnOnce(&mut Self, &ProgressRange<'_>) -> Result<(), Failure>,
    ) -> ProgressCompletion {
        let result = super::pipeline::run_intersection_program(self, range, observed, initialize);
        match result {
            Err(cause) => {
                let alert: AlertHandle = Arc::new(AlertIntersectionFailed);
                self.options.add_error(&alert);
                classify_area_progress(
                    ProgressOperation::PaveFiller,
                    &self.options,
                    Err(cause),
                    observed,
                )
            }
            Ok(outcomes) => intersection_completion(self, outcomes, observed),
        }
    }
}
fn intersection_completion(
    filler: &BOPAlgoPaveFiller,
    outcomes: Vec<ProgressCompletion>,
    observed: &ProgressCancellation,
) -> ProgressCompletion {
    let reported = classify_area_progress(
        ProgressOperation::PaveFiller,
        filler.options(),
        Ok(()),
        observed,
    );
    outcomes
        .iter()
        .find(|outcome| matches!(outcome, ProgressCompletion::Failed { .. }))
        .or_else(|| {
            outcomes
                .iter()
                .find(|outcome| matches!(outcome, ProgressCompletion::Incomplete { .. }))
        })
        .or_else(|| {
            outcomes
                .iter()
                .find(|outcome| matches!(outcome, ProgressCompletion::Cancelled { .. }))
        })
        .cloned()
        .unwrap_or(reported)
}
