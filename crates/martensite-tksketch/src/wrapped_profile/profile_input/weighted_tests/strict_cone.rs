//! Compare qualified conical profiles under the explicit chart and actual-body proof.
use super::*;
use martensite_tkbo::body_check::{CheckMode, ValidationStage};
use martensite_tkbo::profile_extrusion::PlanarProofVersion;

const V8_FAMILY_V3: WrappedConeExecutionSelection =
    WrappedConeExecutionSelection::DirectedTransportedAuthoredPlanarV8 {
        domain: WrappedConeDomainPolicy::PositiveXFamilyChartV3,
        arithmetic: WrappedConeArithmeticGrantPolicy::OriginalV1,
        authored_max_components: 64,
        authored_max_subintervals: 100_000,
    };

#[test]
fn strict_cone_family_v3_checks_the_original_full_ellipse() {
    let [major, minor] = [QUARTER_RADIUS, QUARTER_RADIUS / 2.];
    let request = closed_conics::ellipse_request(major, minor);
    let result = normal_extrude_rational_cone_with_selection(&request, 2, V8_FAMILY_V3, || false)
        .unwrap_or_else(|error| panic!("{error}"));
    closed_conics::validate_closed(&result, major, minor);
    assert_independent_self_intersection(&result);
}

#[test]
fn strict_cone_family_v3_retains_the_32_span_work_refusal() {
    let request = closed_conics::ellipse_request(QUARTER_RADIUS, QUARTER_RADIUS / 2.);
    let failure = normal_extrude_rational_cone_with_selection(&request, 3, V8_FAMILY_V3, || false)
        .err()
        .unwrap();
    assert!(
        matches!(failure, RationalWrappedFailure::Wrapped(cause) if cause.class() == WrappedFailureClass::Incomplete && cause.stage() == WrappedStage::Construction)
    );
}

#[test]
fn strict_cone_family_v3_checks_the_original_mixed_curve_region() {
    let mut request = cone();
    request.proof.planar_proof = PlanarProofVersion::GeometricCompositeV6 {
        max_components: 64,
        max_total_subintervals: 100_000,
    };
    request.profile = mixed_curves::half_round(true);
    request.selections[0].outer.uses.pop();
    let result = normal_extrude_rational_cone_with_selection(&request, 3, V8_FAMILY_V3, || false)
        .unwrap_or_else(|error| panic!("{error}"));
    mixed_curves::check_half_round(&result);
    assert_independent_self_intersection(&result);
}

fn assert_independent_self_intersection(result: &RationalWrappedResult) {
    let stage = result.validation().stage(ValidationStage::SelfIntersection);
    assert_eq!(stage.mode, CheckMode::Exact);
    assert_eq!(result.cone_execution_selection(), Some(V8_FAMILY_V3));
}
