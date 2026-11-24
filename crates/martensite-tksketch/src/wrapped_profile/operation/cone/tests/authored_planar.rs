//! The authored graph has one family issuance before the per-member UV proofs.
use super::*;
use crate::surface_sketch::normal_extrude_cone_profile_with_selection;
use martensite_tkbo::profile_extrusion::{PlanarProofVersion, PlanarProofWorkLimit};

const LIMIT: u16 = 8192;
const AUTHORED_CAP: u64 = 17;
const LEGACY: WrappedConeExecutionSelection =
    WrappedConeExecutionSelection::DirectedTransportedPlacementV5 {
        domain: WrappedConeDomainPolicy::PositiveXCommonChartV2,
        arithmetic: WrappedConeArithmeticGrantPolicy::OriginalV1,
    };

fn selected(components: u16) -> WrappedConeExecutionSelection {
    WrappedConeExecutionSelection::DirectedTransportedAuthoredPlanarV8 {
        domain: WrappedConeDomainPolicy::PositiveXCommonChartV2,
        arithmetic: WrappedConeArithmeticGrantPolicy::OriginalV1,
        authored_max_components: components,
        authored_max_subintervals: AUTHORED_CAP,
    }
}

fn input() -> WrappedConeNormalRequest {
    let mut request = supported_common_chart_request();
    request.options.planar_work = PlanarProofWorkLimit::PairBisections(LIMIT);
    request
}

#[test]
fn authored_planar_v8_replays_two_members_with_one_source_issuance() {
    let request = input();
    let legacy = normal_extrude_cone_profile_with_selection(&request, LEGACY).unwrap();
    let current = normal_extrude_cone_profile_with_selection(&request, selected(64)).unwrap();
    assert_eq!(legacy.execution_selection(), LEGACY);
    assert!(legacy.authored_planar_receipt().is_none());
    assert_eq!(current.execution_selection(), selected(64));
    assert_eq!(current.encoded_brep(), legacy.encoded_brep());
    assert_eq!(current.history(), legacy.history());
    assert_eq!(current.proof_options(), request.proof);
    assert_source_issuance(&current, &request);
    for result in [&legacy, &current] {
        assert_pair_loans(result);
        assert_integration_loans(result, &request);
    }
}

fn assert_source_issuance(result: &WrappedConeNormalResult, request: &WrappedConeNormalRequest) {
    let family = &result.family.family;
    let program = result.authored_planar_receipt().unwrap();
    assert_eq!(family.selection.planar.len(), 2);
    assert_eq!(
        program.requested_proof_version(),
        PlanarProofVersion::GeometricCompositeV6 {
            max_components: 64,
            max_total_subintervals: AUTHORED_CAP,
        }
    );
    assert_eq!(
        program.entry_allowance(),
        u64::from(request.proof.integration.max_total_subintervals)
    );
    assert_eq!(program.granted_subintervals(), AUTHORED_CAP);
    assert_eq!(program.integration_subintervals(), 0);
    for (&region, member) in &family.selection.planar {
        assert!(program.shares_issuance(member.integral_program_evidence().unwrap()));
        assert!(!program.shares_issuance(result.planar_integral_receipt(region).unwrap()));
    }
}

fn assert_pair_loans(result: &WrappedConeNormalResult) {
    let first = result.budget().source_planar_work;
    assert_eq!(first.requested_pair_bisections, LIMIT);
    assert_eq!(first.effective_pair_bisections, LIMIT);
    let mut consumed = first.consumed_pair_bisections;
    for prepared in &result.family.family.selection.prepared {
        let work = result.reports()[&prepared.source.region]
            .material
            .planar_work;
        let remaining = u64::from(LIMIT) - consumed;
        assert_eq!(u64::from(work.requested_pair_bisections), remaining);
        assert_eq!(u64::from(work.effective_pair_bisections), remaining);
        consumed += work.consumed_pair_bisections;
    }
    assert_eq!(
        result.budget().planar_work.consumed_pair_bisections,
        consumed
    );
}

fn assert_integration_loans(result: &WrappedConeNormalResult, request: &WrappedConeNormalRequest) {
    let maximum = u64::from(request.proof.integration.max_total_subintervals);
    let mut consumed = result
        .authored_planar_receipt()
        .map_or(0, |proof| proof.integration_subintervals());
    for prepared in &result.family.family.selection.prepared {
        let region = prepared.source.region;
        let decoded = result.planar_integral_receipt(region).unwrap();
        assert_eq!(
            decoded.requested_proof_version(),
            request.proof.planar_proof
        );
        assert_eq!(decoded.entry_allowance(), maximum - consumed);
        assert_eq!(decoded.granted_subintervals(), maximum - consumed);
        consumed += decoded.integration_subintervals();
        consumed += result
            .reference_volume_receipt(region)
            .unwrap()
            .integration_subintervals();
        consumed += u64::from(
            result
                .flux_receipt(region)
                .unwrap()
                .integration_subintervals(),
        );
    }
    assert!(consumed > 0);
    assert_eq!(
        u64::from(result.budget().integration_subintervals),
        consumed
    );
}

#[test]
fn authored_planar_v8_refuses_a_counter_above_its_explicit_component_cap() {
    let request = input();
    let refusal = normal_extrude_cone_profile_with_selection(&request, selected(1))
        .err()
        .unwrap();
    let work = refusal.planar_work_failure().unwrap();
    let program = work.integral_program_failure().unwrap();
    assert_eq!(program.granted_subintervals(), AUTHORED_CAP);
    let cause = &program.numerical_cause().unwrap().cause;
    assert!(matches!(
        cause,
        IntegralProgramFailure::ComponentCount {
            actual: 2,
            limit: 1
        }
    ));
}

#[test]
fn authored_planar_v8_does_not_blame_first_member_for_a_later_stationary_join() {
    let mut request = input();
    let id = ProfileCurveId(9);
    let slot = request
        .profile
        .curves
        .iter()
        .position(|curve| curve.id() == id)
        .unwrap();
    let [start, end] = request.profile.curves[slot].ends().unwrap();
    let point = |id| {
        request
            .profile
            .vertices
            .iter()
            .find(|vertex| vertex.id == id)
            .unwrap()
            .point
    };
    request.profile.curves[slot] = ProfileCurve::Spline {
        id,
        start,
        end,
        degree: 2,
        poles: vec![point(start), point(start), point(end)],
        weights: None,
        knots: vec![0., 0., 0., 1., 1., 1.],
    };
    for selection in [LEGACY, selected(64)] {
        let failure = normal_extrude_cone_profile_with_selection(&request, selection)
            .err()
            .unwrap();
        assert!(failure.entities().is_empty());
        assert!(failure.to_string().contains("curve=SurfaceCurveId(9)"));
    }
}
