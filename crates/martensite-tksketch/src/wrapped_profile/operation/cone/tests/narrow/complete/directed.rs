//! The same complete O. fixture qualifies the explicitly directed certified behavior.
use super::*;
use crate::surface_sketch::{
    WrappedConeExecutionSelection, normal_extrude_cone_profile_with_selection,
};
use martensite_tkbo::body_check::{CheckMode, ValidationStage};
mod endpoints;
mod mutations;
#[test]
fn directed_outward_narrow_polynomial_punctuation_requires_complete_actual_si() {
    accepted(NormalSense::Outward);
}
#[test]
fn directed_inward_narrow_polynomial_punctuation_requires_complete_actual_si() {
    accepted(NormalSense::Inward);
}
fn accepted(sense: NormalSense) {
    run_request(fixture::request(sense, fixture::Quadrant::PositiveX));
}
fn run_request(mut request: WrappedConeNormalRequest) {
    let sense = request.support.normal;
    let before = request.clone();
    let selected = WrappedConeExecutionSelection::DirectedCommonChartRequireCertifiedV1;
    let started = std::time::Instant::now();
    let outcome = normal_extrude_cone_profile_with_selection(&request, selected);
    eprintln!(
        r#"{{"event":"directed_narrow_cone_api","sense":"{sense:?}","seconds":{:.6}}}"#,
        started.elapsed().as_secs_f64()
    );
    let result =
        outcome.unwrap_or_else(|cause| panic!("directed complete operation refused: {cause:?}"));
    check_result(&result, &mut request, &before, selected);
}
fn check_result(
    result: &WrappedConeNormalResult,
    request: &mut WrappedConeNormalRequest,
    before: &WrappedConeNormalRequest,
    selected: WrappedConeExecutionSelection,
) {
    assert_eq!(*request, *before);
    assert_eq!(result.execution_selection(), selected);
    report_work(result);
    check_publication(result, before);
    check_members(result);
    check_certified(result);
    if before.proof.integration.behavior_version == 2 {
        check_accumulation_receipts(result);
    }
    mutations::check(result);
    check_isolation_with_vertices(result, request, before, |source, detached| {
        endpoints::check(result, source, detached)
    });
}
fn check_certified(result: &WrappedConeNormalResult) {
    let body = result.family.budget.actual_certificate.as_ref().unwrap();
    assert!(body.certificate().operations() > 0);
    check_census(body.native_census());
    check_public_coverage(result);
    for report in result.reports().values() {
        assert!(report.validation.is_valid());
        assert!(!report.validation.is_exact());
        assert_eq!(
            report
                .validation
                .stage(ValidationStage::SelfIntersection)
                .mode,
            CheckMode::Exact
        );
        assert_eq!(
            report.validation.stage(ValidationStage::Geometry).mode,
            CheckMode::Sampled
        );
    }
}

fn check_public_coverage(result: &WrappedConeNormalResult) {
    let coverage = result.actual_self_intersection_coverage().unwrap();
    assert_eq!(coverage.side_self_domains, 148);
    assert_eq!(coverage.adjacent_unions, 148);
    assert_eq!(coverage.nonincident_pairs, 10_730);
    assert_eq!(coverage.side_cap_pairs, 592);
    assert_eq!(coverage.cap_self_domains, 4);
    assert_eq!(coverage.cap_pairs, 6);
}

fn check_census(census: martensite_tkbo::body_check::ConicalNativeCensus) {
    assert_eq!(
        [census.members, census.sides, census.caps, census.cap_wires],
        [2, 148, 4, 6]
    );
    assert_eq!(
        [
            census.cap_edge_uses,
            census.cap_uv_controls,
            census.shared_rulings
        ],
        [296, 1184, 148]
    );
}

#[test]
fn geometric_v6_outward_requires_complete_actual_si() {
    run_geometric(NormalSense::Outward);
}
#[test]
fn geometric_v6_inward_requires_complete_actual_si() {
    run_geometric(NormalSense::Inward);
}
fn run_geometric(sense: NormalSense) {
    let mut request = fixture::request(sense, fixture::Quadrant::PositiveX);
    request.proof.planar_proof =
        martensite_tkbo::profile_extrusion::PlanarProofVersion::GeometricCompositeV6 {
            max_components: 64,
            max_total_subintervals: u64::from(request.proof.integration.max_total_subintervals),
        };
    run_request(request);
}

#[test]
fn outward_accumulation_v2_preserves_complete_actual_si_and_original_caps() {
    run_accumulation_v2(NormalSense::Outward);
}
#[test]
fn inward_accumulation_v2_preserves_complete_actual_si_and_original_caps() {
    run_accumulation_v2(NormalSense::Inward);
}
fn run_accumulation_v2(sense: NormalSense) {
    let mut request = fixture::request(sense, fixture::Quadrant::PositiveX);
    assert_eq!(request.proof.arithmetic.max_operations, 1_000_000);
    request.proof.integration.behavior_version = 2;
    request.proof.planar_proof =
        martensite_tkbo::profile_extrusion::PlanarProofVersion::GeometricCompositeV6 {
            max_components: 64,
            max_total_subintervals: u64::from(request.proof.integration.max_total_subintervals),
        };
    run_request(request);
}

fn check_accumulation_receipts(result: &WrappedConeNormalResult) {
    assert_eq!(result.proof_options().integration.behavior_version, 2);
    assert_eq!(result.budget().integration_subintervals, 444);
    for member in result.family.family.members.values() {
        let geometry = &member.geometry;
        assert_eq!(geometry.flux.requested_options().behavior_version, 2);
        assert_eq!(geometry.flux.effective_options().behavior_version, 2);
        assert_eq!(
            geometry
                .reference_volume
                .common()
                .unwrap()
                .requested_options()
                .behavior_version,
            2
        );
        assert_eq!(
            geometry
                .reference_volume
                .common()
                .unwrap()
                .effective_options()
                .behavior_version,
            2
        );
    }
}
