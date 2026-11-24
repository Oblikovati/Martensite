//! Global planar proof cancellation retains its typed work evidence (#526).
use super::*;
use martensite_tkbo::profile_extrusion::{OperationStage, PlanarProofWorkLimit};

fn request() -> WrappedNormalRequest {
    let mut input = operation_tests::request();
    input.options.planar_work = PlanarProofWorkLimit::PairBisections(8192);
    input
}
fn cancel_on(input: &WrappedNormalRequest, stop: usize) -> (WrappedNormalFailure, usize) {
    let mut polls = 0;
    let refusal = normal_extrude_profile_with_cancel(input, || {
        polls += 1;
        polls >= stop
    })
    .err()
    .unwrap();
    (refusal, polls)
}
#[test]
fn cylinder_cancels_inside_global_planar_certification() {
    let input = request();
    let before = input.clone();
    // Five handoff/admission polls precede graph certification, besides each source read.
    for certification_poll in [1, 7] {
        let stop = input.selections.len() + 5 + certification_poll;
        let (refusal, polls) = cancel_on(&input, stop);
        assert_eq!(refusal.class(), WrappedFailureClass::Cancelled);
        assert_eq!(refusal.stage(), WrappedStage::Construction);
        assert_planar_failure(&refusal, input.options.planar_work, certification_poll > 1);
        assert_eq!(polls, stop, "failed proof cannot reach later handoffs");
    }
    assert_eq!(input, before);
}
fn assert_planar_failure(
    refusal: &WrappedNormalFailure,
    work: PlanarProofWorkLimit,
    observed: bool,
) {
    let planar = refusal
        .planar_work_failure()
        .expect("cancelled planar proof keeps its cause");
    assert!(planar.is_cancelled());
    assert_eq!(planar.decline().stage, OperationStage::Certification);
    assert_eq!(planar.requested_work(), work);
    assert_eq!(planar.proof_work().is_some(), observed);
    if let Some(receipt) = planar.proof_work() {
        assert_eq!(receipt.requested_pair_bisections, 8192);
        assert_eq!(receipt.effective_pair_bisections, 8192);
        assert!(receipt.consumed_pair_bisections <= 8192);
    }
}
#[test]
fn late_cancellation_cannot_replace_a_real_cylinder_refusal() {
    let mut input = request();
    input.support.radius = 0.;
    let before_failure = input.selections.len() + 4;
    let (refusal, polls) = cancel_on(&input, before_failure + 1);
    assert_eq!(refusal.class(), WrappedFailureClass::InvalidInput);
    assert_eq!(refusal.stage(), WrappedStage::Admission);
    assert!(refusal.planar_work_failure().is_none());
    assert_eq!(
        polls, before_failure,
        "failed stage returns before another poll"
    );
}
#[test]
fn false_cancellation_keeps_successful_bytes_receipts_and_sources() {
    let input = request();
    let before = input.clone();
    let plain = normal_extrude_profile(&input).unwrap();
    let observed = normal_extrude_profile_with_cancel(&input, || false).unwrap();
    assert_eq!(plain.encoded_brep(), observed.encoded_brep());
    assert_eq!(plain.budget(), observed.budget());
    assert_eq!(
        format!("{:?}", plain.reports()),
        format!("{:?}", observed.reports())
    );
    assert_eq!(plain.history(), observed.history());
    assert_eq!(input, before);
}
