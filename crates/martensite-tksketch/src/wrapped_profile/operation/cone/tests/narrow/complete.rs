//! Each public narrow-cone operation supplies every acceptance check from the same owned result.
use super::*;
use crate::surface_sketch::{
    WrappedConeDomainPolicy, normal_extrude_cone_profile_with_domain_policy,
};
#[test]
fn outward_narrow_polynomial_punctuation_completes_with_independent_bounds() {
    accepted(NormalSense::Outward);
}
#[test]
fn inward_narrow_polynomial_punctuation_completes_with_independent_bounds() {
    accepted(NormalSense::Inward);
}
fn accepted(sense: NormalSense) {
    let mut request = fixture::request(sense, fixture::Quadrant::PositiveX);
    let before = request.clone();
    let started = std::time::Instant::now();
    let outcome = normal_extrude_cone_profile_with_domain_policy(
        &request,
        WrappedConeDomainPolicy::PositiveXCommonChartV2,
    );
    eprintln!(
        r#"{{"event":"narrow_cone_api","sense":"{sense:?}","seconds":{:.6}}}"#,
        started.elapsed().as_secs_f64()
    );
    let result = outcome.unwrap_or_else(|cause| panic!("narrow cone complete refusal: {cause:?}"));
    report_work(&result);
    assert_eq!(request, before);
    check_publication(&result, &before);
    check_members(&result);
    check_isolation(&result, &mut request, &before);
}
fn check_publication(result: &WrappedConeNormalResult, request: &WrappedConeNormalRequest) {
    publication::assert_publication_core(result, request);
    assert_eq!(result.support(), request.support);
    assert_eq!(
        result.domain_policy(),
        WrappedConeDomainPolicy::PositiveXCommonChartV2
    );
    check_chart_binding(result, request);
}
fn check_chart_binding(result: &WrappedConeNormalResult, request: &WrappedConeNormalRequest) {
    let inputs = input::capture(request, &mut || false).unwrap();
    let spans: Vec<_> = inputs
        .originals
        .iter()
        .flat_map(|source| source.loops.iter().flatten())
        .map(|piece| piece.poles.as_slice())
        .collect();
    let proof = result.chart_domain_receipt().unwrap();
    let query = chart_query(request, &spans, proof);
    assert!(proof.matches(&query));
    assert_eq!(
        proof.options().fractional_bits,
        request.proof.arithmetic.fractional_bits
    );
    assert!(proof.options().max_operations <= request.proof.arithmetic.max_operations);
    assert_eq!([proof.developed_turn(), proof.physical_turn()], [0, 0]);
}
fn chart_query<'a>(
    request: &WrappedConeNormalRequest,
    spans: &'a [&'a [[f64; 2]]],
    proof: &ConicalChartDomainEvidence,
) -> ConicalChartDomainQuery<'a> {
    ConicalChartDomainQuery {
        spans,
        semi_angle: request.support.semi_angle,
        chart_origin: request.support.chart_origin,
        fit_center: request.support.chart_origin,
        developed_turn: 0,
        physical_turn: 0,
        options: proof.options(),
    }
}
fn check_members(result: &WrappedConeNormalResult) {
    assert_eq!(result.reports().len(), 2);
    for (region, report) in result.reports() {
        check_measures(*region, report, result.support().normal);
        assert_eq!(
            report.coverage.euler_characteristic,
            if region.0 == 1 { 0 } else { 2 }
        );
        assert!(report.native_volume.value.is_finite() && report.native_volume.value > 0.);
        assert!(
            report.native_volume.integration_error.is_finite()
                && report.native_volume.integration_error >= 0.
        );
    }
}
fn check_measures(region: SurfaceRegionId, report: &WrappedConeMemberReport, sense: NormalSense) {
    let area = if region.0 == 1 { 366. / 5. } else { 4. };
    assert!(
        report.planar_area.encloses(area),
        "{region:?}: {:?}",
        report.planar_area
    );
    let [lower, upper] = super::volume::bounds(region, sense);
    assert!(
        report.authored_volume.lower >= lower && report.authored_volume.upper <= upper,
        "{region:?}: {:?} outside[{lower},{upper}]",
        report.authored_volume
    );
    assert!(
        report.boundary_flux.lower > 0. && report.boundary_flux.upper >= report.boundary_flux.lower
    );
}
fn check_isolation(
    result: &WrappedConeNormalResult,
    request: &mut WrappedConeNormalRequest,
    before: &WrappedConeNormalRequest,
) {
    check_isolation_with_vertices(result, request, before, check_authored_vertices);
}
fn check_isolation_with_vertices(
    result: &WrappedConeNormalResult,
    request: &mut WrappedConeNormalRequest,
    before: &WrappedConeNormalRequest,
    check_vertices: impl FnOnce(&WrappedConeNormalRequest, &DetachedWrappedBodies),
) {
    let bytes = result.encoded_brep().to_vec();
    request.profile.vertices.clear();
    request.profile.curves.clear();
    let first = result.export_native().unwrap();
    let second = result.export_native().unwrap();
    check_export_owners(&first, &second, result.history().len());
    super::normals::check(before, result, &first);
    super::super::completed::occurrences::check_requested(result, before, &first);
    super::super::completed::native_flux::check(result, &first);
    check_vertices(before, &first);
    assert_eq!(result.encoded_brep(), bytes);
}
fn check_export_owners(
    first: &DetachedWrappedBodies,
    second: &DetachedWrappedBodies,
    history_count: usize,
) {
    assert!(!first.bodies.is_partner(&second.bodies));
    assert_eq!(
        first.entities.keys().collect::<Vec<_>>(),
        second.entities.keys().collect::<Vec<_>>()
    );
    assert_eq!(first.entities.len(), history_count);
    for (key, shape) in &first.entities {
        assert!(!shape.is_partner(&second.entities[key]));
    }
}
fn check_authored_vertices(request: &WrappedConeNormalRequest, detached: &DetachedWrappedBodies) {
    let checked: usize = detached
        .entities
        .iter()
        .map(|(key, shape)| super::super::inward::check_vertex(request, shape, *key))
        .sum();
    assert_eq!(checked, request.profile.vertices.len() * 2);
}
fn report_work(result: &WrappedConeNormalResult) {
    let spans: usize = result
        .reports()
        .values()
        .map(|report| report.coverage.faces - 2)
        .sum();
    eprintln!(
        r#"{{"event":"narrow_cone_completed_work","arithmetic_operations":{},"integration_subintervals":{},"retained_spans":{spans}}}"#,
        result.budget().arithmetic_operations,
        result.budget().integration_subintervals
    );
    assert!(
        result.budget().arithmetic_operations <= result.proof_options().arithmetic.max_operations
    );
}

mod directed;
