//! The source-aware allowance admits wide curves before proof work begins (#425).
use super::*;
use crate::surface_sketch::WrappedConeArithmeticGrantPolicy::{InputBoundV1, PhysicalAngleBoundV2};
use martensite_tkbo::profile_extrusion::PlanarProofVersion;

#[test]
fn narrow_cone_two_ovals_use_an_explicit_angle_scaled_grant() {
    let request = two_ovals_request();
    let legacy = WrappedConeExecutionSelection::DirectedFamilyChartRequireCertifiedV2 {
        arithmetic: InputBoundV1,
    };
    let refusal = normal_extrude_cone_profile_with_selection(&request, legacy)
        .err()
        .expect("the earlier grant ceiling for 20 original spans is one million");
    assert_eq!(refusal.stage(), WrappedStage::Admission);
    let selection = WrappedConeExecutionSelection::DirectedFamilyChartRequireCertifiedV2 {
        arithmetic: PhysicalAngleBoundV2,
    };
    let result = normal_extrude_cone_profile_with_selection(&request, selection).unwrap();
    check_grant(&result);
    check_members(&result);
}

fn two_ovals_request() -> WrappedConeNormalRequest {
    let mut request = fixture::two_ovals_and_dot();
    request.proof.arithmetic.max_operations = 2_200_000;
    request.proof.integration.behavior_version = 2;
    request.proof.planar_proof = PlanarProofVersion::GeometricCompositeV6 {
        max_components: 64,
        max_total_subintervals: u64::from(request.proof.integration.max_total_subintervals),
    };
    request
}

fn check_grant(result: &WrappedConeNormalResult) {
    let grant = result.arithmetic_grant_receipt().unwrap();
    assert_eq!(grant.policy, PhysicalAngleBoundV2);
    assert_eq!(grant.source_spans, 20);
    assert_eq!(grant.operations_per_span_level, 2048);
    assert_eq!(grant.requested_operations, 2_200_000);
    assert_eq!(grant.hard_ceiling, 16_000_000);
    assert_eq!(grant.primitive_ceiling, 1_000_000);
    assert!((2_200_000..2_600_000).contains(&grant.input_ceiling));
    assert_eq!(result.budget().arithmetic_operations, 2_001_936);
}

fn check_members(result: &WrappedConeNormalResult) {
    assert_eq!(result.reports().len(), 3);
    let coverage = result.actual_self_intersection_coverage().unwrap();
    assert_eq!(coverage.side_self_domains, 276);
    assert_eq!(coverage.cap_self_domains, 6);
    for (region, report) in result.reports() {
        assert!(report.validation.is_valid());
        assert_eq!(
            report.coverage.euler_characteristic,
            if region.0 == 2 { 2 } else { 0 }
        );
    }
}
