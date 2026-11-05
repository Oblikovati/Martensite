//! Input order breaks outcome ties; an actual cause outranks a report-only outcome.
use crate::bop_algo::{ProgressCompletion, ProgressOperation, ProgressStage, ProgressWorksite};
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum FeatureOutcomeKind {
    Completed,
    Cancelled,
    Incomplete,
    Failed,
}
pub(in crate::bop_algo::remove_features) fn retain_feature_outcome(
    earlier: ProgressCompletion,
    later: ProgressCompletion,
) -> ProgressCompletion {
    if feature_outcome_priority(&later) > feature_outcome_priority(&earlier) {
        return later;
    }
    earlier
}
fn feature_outcome_priority(completion: &ProgressCompletion) -> (bool, FeatureOutcomeKind) {
    match completion {
        ProgressCompletion::Completed => (false, FeatureOutcomeKind::Completed),
        ProgressCompletion::Cancelled { .. } => (false, FeatureOutcomeKind::Cancelled),
        ProgressCompletion::Incomplete { cause, .. } => {
            (cause.is_some(), FeatureOutcomeKind::Incomplete)
        }
        ProgressCompletion::Failed { cause, .. } => (cause.is_some(), FeatureOutcomeKind::Failed),
    }
}
pub(super) fn feature_ordinal(
    mut completion: ProgressCompletion,
    ordinal: usize,
) -> ProgressCompletion {
    let context = match &mut completion {
        ProgressCompletion::Completed => return completion,
        ProgressCompletion::Cancelled { context, .. }
        | ProgressCompletion::Failed { context, .. }
        | ProgressCompletion::Incomplete { context, .. } => context,
    };
    context
        .get_or_insert(ProgressWorksite {
            label: "RemoveFeatures.Feature",
            input_ordinal: None,
            entity_key: None,
        })
        .input_ordinal
        .get_or_insert(ordinal);
    completion
}
pub(super) fn stopped_feature_parent(
    label: &'static str,
    ordinal: Option<usize>,
) -> ProgressCompletion {
    ProgressCompletion::Cancelled {
        operation: ProgressOperation::RemoveFeatures,
        stage: ProgressStage::Worker,
        context: Some(ProgressWorksite {
            label,
            input_ordinal: ordinal,
            entity_key: None,
        }),
    }
}
#[cfg(test)]
#[path = "feature_outcome_tests.rs"]
mod tests;
