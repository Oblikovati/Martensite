//! Whole removal owns history initialization, weighted stages and its final exception boundary.
use super::BOPAlgoRemoveFeatures;
use super::feature::outcome::retain_feature_outcome;
use crate::bop_algo::{
    AlertRemoveFeaturesFailed, ProgressCancellation, ProgressCompletion, ProgressOperation,
    alerts::alert, builder_area::progress::classify_area_progress,
};
use martensite_tkbrep::brep_tools::History;
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::{Failure, Handle},
};
#[derive(Default)]
struct RemovalExecution {
    observed: ProgressCancellation,
    completion: Option<ProgressCompletion>,
}
impl RemovalExecution {
    fn retain(&mut self, completion: ProgressCompletion) {
        if matches!(completion, ProgressCompletion::Cancelled { .. }) {
            self.observed.record_cancelled();
        }
        let earlier = self
            .completion
            .take()
            .unwrap_or(ProgressCompletion::Completed);
        self.completion = Some(retain_feature_outcome(earlier, completion));
    }
    fn finish(self, completion: ProgressCompletion) -> ProgressCompletion {
        retain_feature_outcome(
            self.completion.unwrap_or(ProgressCompletion::Completed),
            completion,
        )
    }
}
type RemovalProgressStep = fn(
    &mut BOPAlgoRemoveFeatures,
    &ProgressRange<'_>,
    &ProgressCancellation,
) -> Result<ProgressCompletion, Failure>;
impl BOPAlgoRemoveFeatures {
    /// Reserves preparation, removal, history and simplification without clearing other slots.
    /// Example: `remover.fill_pi_constants(2000.0, &mut steps);` reserves 100/1600/100/200 units.
    pub fn fill_pi_constants(&self, whole: f64, steps: &mut crate::bop_algo::PISteps) {
        for (index, weight) in removal_constant_weights(whole).into_iter().enumerate() {
            steps.set_step(index as i32, weight);
        }
    }

    /// Removes features, updates history, simplifies and restores the declared input type.
    /// A thrown failure records AlertRemoveFeaturesFailed after scope close; `remover.perform()`.
    pub fn perform(&mut self) {
        let _ = self.perform_with_progress(&ProgressRange::default());
    }
    /// Runs the complete receiver in the caller range and returns owned execution completion.
    /// Raw geometry still needs publication checks; `remover.perform_with_progress(&indicator.start())`.
    pub fn perform_with_progress(&mut self, range: &ProgressRange<'_>) -> ProgressCompletion {
        let mut execution = RemovalExecution::default();
        let result = self.perform_removal_range(range, &mut execution);
        let completion = self.classify_removal_boundary(result, &execution.observed);
        execution.finish(completion)
    }
    fn perform_removal_range(
        &mut self,
        range: &ProgressRange<'_>,
        execution: &mut RemovalExecution,
    ) -> Result<(), Failure> {
        if self.base.fill_history {
            self.base.history = Handle::new(History::new());
        }
        self.check_data()?;
        if self.base.options.has_errors() {
            return Ok(());
        }
        let scope = ProgressScope::with_literal(range, "Removing features", 100., false);
        self.run_removal_scopes(&scope, execution)?;
        if !self.base.options.has_errors() {
            self.post_treat()?;
        }
        Ok(())
    }
    fn run_removal_scopes(
        &mut self,
        scope: &ProgressScope<'_>,
        execution: &mut RemovalExecution,
    ) -> Result<(), Failure> {
        for (weight, step) in removal_progress_steps() {
            let completion = step(self, &scope.next(weight), &execution.observed)?;
            execution.retain(completion);
            if self.base.options.has_errors() {
                return Ok(());
            }
        }
        Ok(())
    }
    fn classify_removal_boundary(
        &self,
        result: Result<(), Failure>,
        observed: &ProgressCancellation,
    ) -> ProgressCompletion {
        if result.is_err() {
            // The actual catch runs after perform_removal_range has closed every scope.
            self.base
                .options
                .add_error(&alert(AlertRemoveFeaturesFailed));
        }
        classify_area_progress(
            ProgressOperation::RemoveFeatures,
            &self.base.options,
            result,
            observed,
        )
    }
    fn prepare_removal_entry(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<ProgressCompletion, Failure> {
        self.prepare_features_observed(range, observed)?;
        Ok(self.local_removal_completion(observed))
    }
    fn remove_removal_entry(
        &mut self,
        range: &ProgressRange<'_>,
        _: &ProgressCancellation,
    ) -> Result<ProgressCompletion, Failure> {
        self.remove_features_on_progress(range)
    }
    fn history_removal_entry(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<ProgressCompletion, Failure> {
        self.update_history_observed(range, observed)?;
        Ok(self.local_removal_completion(observed))
    }
    fn simplify_removal_entry(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<ProgressCompletion, Failure> {
        self.simplify_result_observed(range, observed)?;
        Ok(self.local_removal_completion(observed))
    }
    fn local_removal_completion(&self, observed: &ProgressCancellation) -> ProgressCompletion {
        classify_area_progress(
            ProgressOperation::RemoveFeatures,
            &self.base.options,
            Ok(()),
            observed,
        )
    }
}
fn removal_progress_steps() -> [(f64, RemovalProgressStep); 4] {
    let [prepare, remove, history, simplify] = removal_constant_weights(100.);
    [
        (prepare, BOPAlgoRemoveFeatures::prepare_removal_entry),
        (remove, BOPAlgoRemoveFeatures::remove_removal_entry),
        (history, BOPAlgoRemoveFeatures::history_removal_entry),
        (simplify, BOPAlgoRemoveFeatures::simplify_removal_entry),
    ]
}

fn removal_constant_weights(whole: f64) -> [f64; 4] {
    let preparation = 0.05 * whole;
    let removal = 0.8 * whole;
    let history = 0.05 * whole;
    let simplification = 0.1 * whole;
    [preparation, removal, history, simplification]
}

#[test]
fn removal_constant_weights_preserve_the_complete_hundred_unit_program() {
    let weights = removal_constant_weights(100.);
    assert_eq!(weights, [5., 80., 5., 10.]);
}
#[cfg(test)]
#[path = "entry_tests.rs"]
mod tests;
