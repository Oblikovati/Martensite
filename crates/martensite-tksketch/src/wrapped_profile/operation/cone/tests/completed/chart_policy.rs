//! Explicit V2 qualification reuses the accepted A profile and its independent complete checks.
use super::*;
use crate::surface_sketch::{
    WrappedConeDomainPolicy, normal_extrude_cone_profile_with_domain_policy,
};
#[test]
fn explicit_v2_curved_holed_cone_completes_with_independent_bounds_and_owned_exports() {
    accepted(NormalSense::Outward);
}
#[test]
fn explicit_v2_inward_curved_holed_cone_completes_with_independent_bounds_and_caller_history() {
    accepted(NormalSense::Inward);
}
fn accepted(sense: NormalSense) {
    let mut request = request();
    request.support.normal = sense;
    let before = request.clone();
    let result = execute_public(&request);
    publication::assert_publication(&result, &before);
    assert_eq!(request, before);
    assert_eq!(result.support(), before.support);
    assert_eq!(result.proof_options(), before.proof);
    assert_chart_binding(&result, &before);
    check_measure(&result);
    check_coverage(&result);
    check_export_isolation(&result, &mut request, &before);
}
fn execute_public(request: &WrappedConeNormalRequest) -> WrappedConeNormalResult {
    let started = std::time::Instant::now();
    let outcome = normal_extrude_cone_profile_with_domain_policy(
        request,
        WrappedConeDomainPolicy::PositiveXCommonChartV2,
    );
    eprintln!(
        r#"{{"cone_execute_seconds":{:.6},"domain_policy":"PositiveXCommonChartV2","normal":"{:?}"}}"#,
        started.elapsed().as_secs_f64(),
        request.support.normal
    );
    outcome.unwrap_or_else(|cause| panic!("explicit V2 A profile refused: {cause:?}"))
}
fn assert_chart_binding(result: &WrappedConeNormalResult, request: &WrappedConeNormalRequest) {
    assert_eq!(
        result.domain_policy(),
        WrappedConeDomainPolicy::PositiveXCommonChartV2
    );
    let proof = result
        .chart_domain_receipt()
        .expect("V2 must retain its domain receipt");
    let mut inputs = input::capture(request, &mut || false).unwrap();
    assert_query(proof, request, &inputs, true);
    inputs.originals[0].loops[0][0].poles[0][0] =
        inputs.originals[0].loops[0][0].poles[0][0].next_up();
    assert_query(proof, request, &inputs, false);
    assert_metadata(proof, request);
}
fn assert_metadata(proof: &ConicalChartDomainEvidence, request: &WrappedConeNormalRequest) {
    assert_eq!(
        proof.chart_origin().to_bits(),
        request.support.chart_origin.to_bits()
    );
    assert_eq!(
        proof.fit_center().to_bits(),
        request.support.chart_origin.to_bits()
    );
    assert_eq!([proof.developed_turn(), proof.physical_turn()], [0, 0]);
    assert_eq!(
        proof.options().fractional_bits,
        request.proof.arithmetic.fractional_bits
    );
    assert!(proof.options().max_operations <= request.proof.arithmetic.max_operations);
}
fn assert_query(
    proof: &ConicalChartDomainEvidence,
    request: &WrappedConeNormalRequest,
    inputs: &Inputs,
    expected: bool,
) {
    let spans: Vec<_> = inputs
        .originals
        .iter()
        .flat_map(|source| source.loops.iter().flatten())
        .map(|piece| piece.poles.as_slice())
        .collect();
    let query = ConicalChartDomainQuery {
        spans: &spans,
        semi_angle: request.support.semi_angle,
        chart_origin: request.support.chart_origin,
        fit_center: request.support.chart_origin,
        developed_turn: 0,
        physical_turn: 0,
        options: proof.options(),
    };
    assert_eq!(proof.matches(&query), expected);
}
