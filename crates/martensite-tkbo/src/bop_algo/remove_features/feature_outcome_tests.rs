//! Stable input order selects equal outcomes; real causes survive later report clearing.
use super::*;
use crate::bop_algo::ProgressFailureClass;
use martensite_tkernel::standard::Failure;
fn feature_failure(cause: Option<Failure>) -> ProgressCompletion {
    ProgressCompletion::Failed {
        operation: ProgressOperation::RemoveFeatures,
        stage: ProgressStage::Worker,
        class: ProgressFailureClass::KernelFailure,
        cause,
        context: None,
    }
}
#[test]
fn feature_outcome_precedence_keeps_actual_causes_then_failure_incomplete_and_stop() {
    let incomplete = ProgressCompletion::Incomplete {
        operation: ProgressOperation::RemoveFeatures,
        stage: ProgressStage::Worker,
        class: ProgressFailureClass::UnqualifiedStage,
        cause: None,
        context: None,
    };
    let ordered = [
        ProgressCompletion::Completed,
        stopped_feature_parent("parent", None),
        incomplete,
        feature_failure(None),
        feature_failure(Some(Failure::new("owned cause"))),
    ];
    for (first_index, first) in ordered.iter().enumerate() {
        for (second_index, second) in ordered.iter().enumerate() {
            let expected = &ordered[first_index.max(second_index)];
            assert_eq!(
                &retain_feature_outcome(first.clone(), second.clone()),
                expected
            );
        }
    }
}
#[test]
fn equal_cause_rank_keeps_first_input_and_existing_child_provenance() {
    let first = feature_ordinal(feature_failure(Some(Failure::new("first cause"))), 3);
    let later = feature_ordinal(feature_failure(Some(Failure::new("later cause"))), 1);
    assert_eq!(retain_feature_outcome(first.clone(), later), first);
    assert_eq!(feature_ordinal(first.clone(), 99), first);
    assert_eq!(
        feature_ordinal(ProgressCompletion::Completed, 7),
        ProgressCompletion::Completed
    );
}
