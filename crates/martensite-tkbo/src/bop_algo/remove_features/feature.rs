//! `BOPAlgoRemoveFeatures::RemoveFeatures` and `RemoveFeature`: the gaps of all features
//! filled, then each feature removed by rebuilding its solids with the reconstructed faces.
#[path = "feature_connections.rs"]
mod connections;
#[path = "feature_outcome.rs"]
pub(in crate::bop_algo::remove_features) mod outcome;
#[path = "feature_rebuild.rs"]
mod rebuild;
#[path = "feature_tasks.rs"]
mod tasks;
use super::BOPAlgoRemoveFeatures;
use super::fill_gap::FillGap;
use crate::bop_algo::ProgressCompletion;
use connections::FeatureIncidences;
use martensite_tkbrep::brep_tools::History;
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::{Failure, Handle},
};
use outcome::{feature_ordinal, retain_feature_outcome, stopped_feature_parent};

impl BOPAlgoRemoveFeatures {
    /// Fills every gap, then removes features one by one so an inability warning does not
    /// stop later removals (`RemoveFeatures`). Ranges and outcomes follow input order.
    #[cfg(test)]
    pub(super) fn remove_features(&mut self) -> Result<(), Failure> {
        self.remove_features_on_progress(&ProgressRange::default())?;
        Ok(())
    }

    /// Dispatch gaps in caller ranges, join them, and rebuild features in original order.
    /// Example: `remover.remove_features_on_progress(range)?` still requires publication checks.
    pub(super) fn remove_features_on_progress(
        &mut self,
        range: &ProgressRange<'_>,
    ) -> Result<ProgressCompletion, Failure> {
        let outer = ProgressScope::with_literal(range, "Removing features", 2., false);
        let incidences = FeatureIncidences::new(&self.input_shape)?;
        let filling_range = outer.next(1.);
        // Native keeps this scope alive through sequential removal and closes it last.
        let filling = ProgressScope::with_literal(
            &filling_range,
            "Filling gaps",
            self.features.len() as f64,
            false,
        );
        let (gaps, completion) = tasks::fill_feature_gaps(
            self.new_feature_gaps(),
            &incidences.connections(),
            &filling,
            self.base.options.run_parallel(),
        )?;
        if self.base.options.user_break(&outer) {
            return Ok(retain_feature_outcome(
                completion,
                stopped_feature_parent("RemoveFeatures.AfterGaps", None),
            ));
        }
        self.ensure_feature_history();
        let removing_range = outer.next(1.);
        let removing = ProgressScope::with_literal(
            &removing_range,
            "Removing features one by one",
            gaps.len() as f64,
            false,
        );
        self.remove_filled_feature_gaps(&gaps, &removing, completion)
    }

    fn new_feature_gaps(&self) -> Vec<FillGap> {
        self.features
            .iter()
            .map(|feature| {
                let mut gap = FillGap::new(feature, self.base.options.run_parallel());
                gap.face_grouping_version = self.base.options.face_grouping_version();
                gap
            })
            .collect()
    }

    fn ensure_feature_history(&mut self) {
        // The history tracks the solids and faces between removals even when not requested.
        if self.base.history.is_null() {
            self.base.history = Handle::new(History::new());
        }
    }

    fn remove_filled_feature_gaps(
        &mut self,
        gaps: &[FillGap],
        scope: &ProgressScope<'_>,
        mut completion: ProgressCompletion,
    ) -> Result<ProgressCompletion, Failure> {
        for (ordinal, gap) in gaps.iter().enumerate() {
            if self.base.options.user_break(scope) {
                return Ok(retain_feature_outcome(
                    completion,
                    stopped_feature_parent("RemoveFeatures.BeforeFeature", Some(ordinal)),
                ));
            }
            // The last removal needs no solid history unless the history is requested.
            let solids_history_needed = self.base.fill_history || ordinal + 1 < gaps.len();
            let child =
                self.remove_feature_on_progress(gap, solids_history_needed, &scope.next(1.))?;
            completion = retain_feature_outcome(completion, feature_ordinal(child, ordinal));
        }
        Ok(completion)
    }
}

#[cfg(test)]
#[path = "feature_fixture.rs"]
pub(in crate::bop_algo::remove_features) mod fixture;
#[cfg(test)]
#[path = "feature_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "feature_census.rs"]
mod census;
