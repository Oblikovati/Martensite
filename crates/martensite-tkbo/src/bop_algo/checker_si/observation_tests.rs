//! Public observations preserve actual receiver behavior and its coverage limits.
use super::*;
use crate::bop_algo::{BOPAlgoBOP, Operation};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::List;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPINurbsConvert};

fn box_shape() -> Shape {
    BRepPrimAPIMakeBox::from_sizes(1., 1., 1.)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

fn checker(shape: &Shape) -> BOPAlgoCheckerSI {
    let mut checker = BOPAlgoCheckerSI::new();
    checker.set_arguments(&[shape.clone()].into_iter().collect::<List<Shape>>());
    checker.options_mut().set_run_parallel(false);
    checker
}

#[test]
fn immutable_observation_has_owned_worker_traits() {
    fn worker_value<T: Send + Sync>() {}
    worker_value::<SelfIntersectionObservation>();
    worker_value::<SelfIntersectionFace>();
}

#[test]
fn observed_box_matches_public_default_execution() {
    let shape = box_shape();
    let mut baseline = checker(&shape);
    let expected = baseline.perform_with_progress(&ProgressRange::default());
    let mut observed = checker(&shape);
    let receipt = observed.perform_with_progress_observed(&ProgressRange::default());
    assert_eq!(receipt.completion(), &expected);
    assert_eq!(receipt.pairs(), snapshot_pairs(&baseline));
    assert!(receipt.diagnostics().is_empty());
    assert_eq!(receipt.faces().len(), 6);
    assert!(
        receipt
            .faces()
            .iter()
            .all(|face| face.status() == SelfIntersectionFaceStatus::PolicyExcluded)
    );
}

#[test]
fn receipt_survives_reuse_without_stale_faces_or_pairs() {
    let mut actual = checker(&box_shape());
    let first = actual.perform_with_progress_observed(&ProgressRange::default());
    actual.set_arguments(&List::default());
    let rejected = actual.perform_with_progress_observed(&ProgressRange::default());
    assert_eq!(rejected.rejected_arguments(), Some(0));
    assert!(!rejected.diagnostics().is_empty());
    assert!(rejected.faces().is_empty());
    assert_eq!(first.faces().len(), 6);
    assert_eq!(first.rejected_arguments(), None);
}

#[test]
fn actual_nurbs_termination_preserves_typed_incomplete() {
    let mut conversion = BRepBuilderAPINurbsConvert::with_shape(&box_shape(), false).unwrap();
    let mut actual = checker(conversion.shape().unwrap());
    let receipt = actual.perform_with_progress_observed(&ProgressRange::default());
    assert!(matches!(
        receipt.completion(),
        ProgressCompletion::Incomplete { .. }
    ));
    assert_eq!(receipt.faces().len(), 6);
    assert!(receipt.faces().iter().all(|face| matches!(
        face.status(),
        SelfIntersectionFaceStatus::Terminated {
            has_contacts: false
        }
    )));
}

#[test]
fn history_availability_does_not_create_an_empty_history() {
    let mut operation = BOPAlgoBOP::new();
    assert!(!operation.has_filled_history());
    operation.set_operation(Operation::Fuse);
    operation.set_to_fill_history(false);
    assert!(!operation.has_filled_history());
    assert!(operation.history().is_none());
    operation.set_to_fill_history(true);
    assert!(!operation.has_filled_history());
    assert!(operation.history().is_some());
    assert!(operation.has_filled_history());
    operation.clear();
    assert!(!operation.has_filled_history());
}

#[test]
fn actual_cancelled_observation_preserves_the_callers_range() {
    use crate::bop_algo::progress_test_support::indicator;
    let shape = box_shape();
    let (baseline_range, _) = indicator(1);
    let expected = checker(&shape).perform_with_progress(&baseline_range.start());
    let (observed_range, _) = indicator(1);
    let receipt = checker(&shape).perform_with_progress_observed(&observed_range.start());
    assert_eq!(receipt.completion(), &expected);
    assert!(matches!(
        receipt.completion(),
        ProgressCompletion::Cancelled { .. }
    ));
}

#[test]
fn actual_failed_observation_owns_the_original_kernel_cause() {
    let mut receiver = checker(&Shape::default());
    let receipt = receiver.perform_with_progress_observed(&ProgressRange::default());
    receiver.set_arguments(&List::default());
    receiver.perform();
    let ProgressCompletion::Failed {
        cause: Some(cause), ..
    } = receipt.completion()
    else {
        panic!(
            "actual null checker={:?}; expected original typed cause",
            receipt.completion()
        );
    };
    assert_eq!(cause.exception_type(), "Standard_NullObject");
    assert!(!cause.what().is_empty());
}
