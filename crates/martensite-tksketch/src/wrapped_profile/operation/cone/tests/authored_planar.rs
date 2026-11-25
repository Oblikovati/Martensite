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
    assert_eq!(family.selection.planar.len(), request.selections.len());
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
        consumed += result
            .flux_receipt(region)
            .unwrap()
            .integration_subintervals();
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

const DIRECTED_CAPS: WrappedConeExecutionSelection =
    WrappedConeExecutionSelection::DirectedTransportedAuthoredPlanarAndCapCyclesV9 {
        arithmetic: WrappedConeArithmeticGrantPolicy::PhysicalAngleBoundV2,
        authored_max_components: 64,
        authored_max_subintervals: AUTHORED_CAP,
    };

const FAMILY_AUTHORED: WrappedConeExecutionSelection =
    WrappedConeExecutionSelection::DirectedTransportedAuthoredPlanarV8 {
        domain: WrappedConeDomainPolicy::PositiveXFamilyChartV3,
        arithmetic: WrappedConeArithmeticGrantPolicy::PhysicalAngleBoundV2,
        authored_max_components: 64,
        authored_max_subintervals: AUTHORED_CAP,
    };

#[test]
fn directed_cap_v9_replays_legacy_native_publication_with_distinct_work() {
    let mut request = input();
    let published = narrow::fixture::two_ovals_and_dot();
    request.profile = published.profile;
    request.selections = published.selections;
    request.proof.arithmetic.max_operations = 2_200_000;
    let old = normal_extrude_cone_profile_with_selection(&request, FAMILY_AUTHORED).unwrap();
    let new = normal_extrude_cone_profile_with_selection(&request, DIRECTED_CAPS).unwrap();
    assert_eq!(old.encoded_brep(), new.encoded_brep());
    assert_eq!(old.history(), new.history());
    assert_eq!(old.domain_policy(), new.domain_policy());
    assert_eq!(old.proof_options(), new.proof_options());
    assert_eq!(
        new.actual_self_intersection_coverage(),
        old.actual_self_intersection_coverage()
    );
    assert_eq!(old.budget().volume, new.budget().volume);
    assert_source_issuance(&old, &request);
    check_directed_cap_members(&old, &new);
    assert_pair_loans(&new);
    assert_directed_integration_loans(&new, &request);
}

fn check_directed_cap_members(old: &WrappedConeNormalResult, new: &WrappedConeNormalResult) {
    assert_eq!(old.reports().len(), 3);
    assert_eq!(old.reports().len(), new.reports().len());
    for (region, current) in new.reports() {
        let prior = &old.reports()[region];
        assert_eq!(prior.coverage, current.coverage);
        assert_eq!(prior.validation.census, current.validation.census);
        assert_eq!(prior.validation.stages, current.validation.stages);
        assert_eq!(
            prior.validation.tolerance_budget,
            current.validation.tolerance_budget
        );
        assert!(prior.validation.is_valid() && current.validation.is_valid());
        assert!(old.planar_integral_receipt(*region).is_some());
        assert!(new.planar_integral_receipt(*region).is_none());
        assert!(old.directed_cap_receipt(*region).is_none());
        let work = new.directed_cap_receipt(*region).unwrap();
        assert_eq!(work.pair, current.material.planar_work);
        assert_eq!(work.loops, current.material.planar.components);
        assert!(work.pair_domains > 0);
        assert_eq!(
            current.material.planar_proof,
            PlanarProofVersion::DirectedSelectedCapV1 {
                max_components: work.requested.max_components,
                max_total_subintervals: work.effective_integration_cells,
            },
        );
    }
}

fn assert_directed_integration_loans(
    result: &WrappedConeNormalResult,
    request: &WrappedConeNormalRequest,
) {
    let maximum = u64::from(request.proof.integration.max_total_subintervals);
    let mut consumed = result
        .authored_planar_receipt()
        .unwrap()
        .integration_subintervals();
    for prepared in &result.family.family.selection.prepared {
        let region = prepared.source.region;
        let cap = result.directed_cap_receipt(region).unwrap();
        assert_eq!(cap.requested.requested_integration_cells, maximum);
        assert_eq!(
            cap.requested.remaining_integration_cells,
            maximum - consumed
        );
        assert_eq!(cap.effective_integration_cells, maximum - consumed);
        consumed += cap.integration_cells;
        consumed += result
            .reference_volume_receipt(region)
            .unwrap()
            .integration_subintervals();
        consumed += result
            .flux_receipt(region)
            .unwrap()
            .integration_subintervals();
    }
    assert_eq!(
        u64::from(result.budget().integration_subintervals),
        consumed
    );
}

#[test]
fn directed_cap_v9_refuses_a_complete_counter_face_above_its_component_cap() {
    use martensite_tkbo::body_check::{DirectedCapCause, DirectedCapFailure};
    use std::error::Error;
    let mut request = input();
    request.proof.planar_proof = PlanarProofVersion::GeometricCompositeV6 {
        max_components: 1,
        max_total_subintervals: u64::from(request.proof.integration.max_total_subintervals),
    };
    let failure = normal_extrude_cone_profile_with_selection(&request, DIRECTED_CAPS)
        .err()
        .unwrap();
    let mut item: &dyn Error = &failure;
    let retained = loop {
        if let Some(cause) = item.downcast_ref::<DirectedCapFailure>() {
            break cause;
        }
        item = item
            .source()
            .unwrap_or_else(|| panic!("missing directed cap failure: {failure:?}"));
    };
    assert!(matches!(
        retained.cause.as_ref(),
        DirectedCapCause::Components {
            actual: 2,
            limit: 1
        }
    ));
    assert!(retained.observed.is_none());
}
