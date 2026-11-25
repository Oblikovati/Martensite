use super::*;
mod replay;

fn request() -> WrappedConeNormalRequest {
    operation::cone::tests::request()
}
fn selections() -> [WrappedConeExecutionSelection; 3] {
    use WrappedConeDomainPolicy::{LegacySectorV1, PositiveXCommonChartV2};
    use WrappedConeExecutionSelection::*;
    [
        RoundedProxyNativeSearchV1 {
            domain: LegacySectorV1,
        },
        RoundedProxyNativeSearchV1 {
            domain: PositiveXCommonChartV2,
        },
        DirectedCommonChartRequireCertifiedV1,
    ]
}
#[test]
fn execution_selection_is_shared_and_returned_by_value() {
    fn shared<T: Send + Sync>() {}
    shared::<WrappedConeExecutionSelection>();
    let _: fn(&WrappedConeNormalResult) -> WrappedConeExecutionSelection =
        WrappedConeNormalResult::execution_selection;
}
#[test]
fn selected_entries_cancel_before_source_capture_without_mutating_input() {
    let input = directed_request();
    let original = input.clone();
    for selection in selections() {
        let failure =
            normal_extrude_cone_profile_with_selection_and_cancel(&input, selection, || true)
                .err()
                .unwrap();
        assert_eq!(failure.class(), WrappedFailureClass::Cancelled);
        assert_eq!(failure.stage(), WrappedStage::Admission);
        assert_eq!(input, original);
    }
}
#[test]
fn selected_admission_preserves_original_failure_before_later_cancel() {
    let mut input = request();
    input.support.semi_angle = 0.;
    let expected = normal_extrude_cone_profile(&input).err().unwrap();
    for selection in selections() {
        assert_admission(&input, selection, &expected);
    }
}
fn assert_admission(
    input: &WrappedConeNormalRequest,
    selection: WrappedConeExecutionSelection,
    expected: &WrappedNormalFailure,
) {
    let mut polls = 0;
    let failure = normal_extrude_cone_profile_with_selection_and_cancel(input, selection, || {
        polls += 1;
        polls > 1
    })
    .err()
    .unwrap();
    assert_eq!(failure.class(), expected.class());
    assert_eq!(failure.stage(), expected.stage());
    assert_eq!(failure.entities(), expected.entities());
    assert_eq!(failure.to_string(), expected.to_string());
    assert_eq!(polls, 1);
}

fn directed_request() -> WrappedConeNormalRequest {
    let mut input = request();
    input.proof.planar_proof =
        martensite_tkbo::profile_extrusion::PlanarProofVersion::FilteredCompositeBernsteinV5 {
            max_components: 64,
            max_total_subintervals: u64::from(input.proof.integration.max_total_subintervals),
        };
    input
}
#[test]
fn directed_selection_refuses_original_v4_before_later_cancellation() {
    let input = request();
    let original = input.clone();
    let mut polls = 0;
    let failure = normal_extrude_cone_profile_with_selection_and_cancel(
        &input,
        WrappedConeExecutionSelection::DirectedCommonChartRequireCertifiedV1,
        || {
            polls += 1;
            polls > 1
        },
    )
    .err()
    .unwrap();
    assert_eq!(failure.class(), WrappedFailureClass::Unsupported);
    assert_eq!(failure.stage(), WrappedStage::Admission);
    assert_eq!(polls, 1);
    assert_eq!(input, original);
}
