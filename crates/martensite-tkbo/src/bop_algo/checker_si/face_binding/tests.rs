// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkmath::predicates::roots::DyadicBound;
#[path = "capture_tests.rs"]
mod capture_tests;
#[path = "fixture.rs"]
mod fixture;
#[path = "vertex_tests.rs"]
mod vertex_tests;

#[test]
fn independently_closed_strip_prerequisites_never_complete_self_search() {
    let observation = inspect_owned_strip(&fixture::packet(), || false);
    assert!(
        observation.prerequisites.is_ok(),
        "{:?}",
        observation.prerequisites
    );
    assert_eq!(
        observation.completion,
        BindingIncomplete::ReleasedSupportCertificateNotBound
    );
    assert_eq!(observation.source_atomicity, SourceAtomicity::Unestablished);
    assert_eq!(
        observation.topology_observation,
        TopologyObservation::EdgeOccurrencesOnlyVerticesUnbound
    );
}
#[test]
fn affine_coefficient_bound_is_separate_from_recorded_requested_tolerance() {
    let mut packet = fixture::packet();
    let OwnedSpaceCurve::Spline(curve) = &mut packet.edges[2].curve else {
        panic!("expected spline")
    };
    curve.poles[2][0] += 0.125;
    let proof = inspect_owned_strip(&packet, || false)
        .prerequisites
        .unwrap();
    assert_eq!(proof.conformance[2].existing_spatial_limit, 0.25);
    assert_eq!(
        proof.conformance[2].proven_squared_upper,
        DyadicBound::from_f64(0.015625).unwrap()
    );
    assert_eq!(
        proof.conformance[2].existing_squared_limit,
        DyadicBound::from_f64(0.0625).unwrap()
    );
    assert!(proof.conformance[2].bound_queries <= 65 * 9);
}
#[test]
fn above_recorded_tolerance_refuses_without_widening() {
    let mut packet = fixture::packet();
    let OwnedSpaceCurve::Spline(curve) = &mut packet.edges[2].curve else {
        panic!("expected spline")
    };
    curve.poles[2][0] += 0.5;
    assert!(matches!(
        inspect_owned_strip(&packet, || false).prerequisites,
        Err(BindingFailure::Conformance {
            occurrence: 2,
            limit: 0.25,
            ..
        })
    ));
}
#[test]
fn same_positive_but_different_weights_are_not_a_common_convex_basis() {
    let mut packet = fixture::packet();
    let OwnedSpaceCurve::Spline(curve) = &mut packet.edges[2].curve else {
        panic!("expected spline")
    };
    curve.weights[2] = 2.;
    assert!(
        inspect_owned_strip(&packet, || false)
            .prerequisites
            .is_err()
    );
}
#[test]
fn seam_identity_and_branch_are_both_required() {
    let mut packet = fixture::packet();
    packet.relations[0].same = false;
    assert!(
        inspect_owned_strip(&packet, || false)
            .prerequisites
            .is_err()
    );
    packet.relations[0].same = true;
    packet.relations[0].location_equal = false;
    assert!(
        inspect_owned_strip(&packet, || false)
            .prerequisites
            .is_err()
    );
    packet.relations[0].location_equal = true;
    packet.edges[1].stored_branch = 1;
    assert!(
        inspect_owned_strip(&packet, || false)
            .prerequisites
            .is_err()
    );
}
#[test]
fn partial_domain_missing_occurrence_and_duplicate_pair_refuse() {
    let mut packet = fixture::packet();
    packet.uv_bounds[1] = 3.;
    assert!(
        inspect_owned_strip(&packet, || false)
            .prerequisites
            .is_err()
    );
    packet = fixture::packet();
    packet.edges.pop();
    assert!(
        inspect_owned_strip(&packet, || false)
            .prerequisites
            .is_err()
    );
    packet = fixture::packet();
    packet.relations[1] = packet.relations[0].clone();
    assert!(
        inspect_owned_strip(&packet, || false)
            .prerequisites
            .is_err()
    );
}
#[test]
fn reversed_chart_side_without_topology_reversal_breaks_boundary_cycle() {
    let mut packet = fixture::packet();
    packet.edges[2].canonical_orientation = martensite_tkg3d::top_abs::Orientation::Forward;
    assert!(
        inspect_owned_strip(&packet, || false)
            .prerequisites
            .is_err()
    );
}
#[test]
fn singular_nonfinite_and_zero_extrusion_domains_refuse() {
    let mut packet = fixture::packet();
    packet.support_placement.linear[2] = [0.; 3];
    assert!(
        inspect_owned_strip(&packet, || false)
            .prerequisites
            .is_err()
    );
    packet = fixture::packet();
    packet.basis.poles[3][0] = f64::NAN;
    assert!(
        inspect_owned_strip(&packet, || false)
            .prerequisites
            .is_err()
    );
    packet = fixture::packet();
    packet.direction = [0.; 3];
    assert!(
        inspect_owned_strip(&packet, || false)
            .prerequisites
            .is_err()
    );
}
#[test]
fn zero_signed_zero_negative_and_nonfinite_tolerances_are_explicit() {
    let mut packet = fixture::packet();
    packet.face_tolerance = -0.;
    assert!(inspect_owned_strip(&packet, || false).prerequisites.is_ok());
    packet.face_tolerance = -1.;
    assert!(
        inspect_owned_strip(&packet, || false)
            .prerequisites
            .is_err()
    );
    packet.face_tolerance = f64::INFINITY;
    assert!(
        inspect_owned_strip(&packet, || false)
            .prerequisites
            .is_err()
    );
}
#[test]
fn cancellation_preserves_incomplete_status_without_certificate() {
    let observation = inspect_owned_strip(&fixture::packet(), || true);
    assert!(matches!(
        observation.prerequisites,
        Err(BindingFailure::Cancelled)
    ));
    assert_eq!(
        observation.completion,
        BindingIncomplete::ReleasedSupportCertificateNotBound
    );
}
#[test]
fn owned_packets_and_reports_are_send_sync_without_reachable_geometry_handles() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<OwnedFacePacket>();
    assert_send_sync::<FaceBindingObservation>();
}

#[test]
fn exact_subnormal_squared_error_never_uses_a_rounded_float_square() {
    let tiny = f64::from_bits(1);
    let limit = f64::from_bits(2);
    let input = super::conformance::ResidualInput {
        curve: OwnedLine {
            origin: [tiny, 0., 0.],
            direction: [0.; 3],
        },
        curve_placement: fixture::placement(),
        support_origin: [0.; 3],
        direction: [0.; 3],
        support_placement: fixture::placement(),
        v_map: OwnedLine {
            origin: [0.; 2],
            direction: [0.; 2],
        },
        interval: [0., 1.],
    };
    let (upper, work) = super::refine::certified_upper(&input, 0, limit, &mut || false).unwrap();
    assert_eq!(
        upper,
        super::exact::squared_limit(tiny, &mut || false).unwrap()
    );
    assert!(upper > DyadicBound::from_f64(0.).unwrap());
    assert!(work <= 65);
    assert!(super::refine::certified_upper(&input, 0, 0., &mut || false).is_err());
}
#[test]
fn one_ulp_wrong_trim_endpoint_is_not_admitted_by_spatial_tolerance() {
    let mut packet = fixture::packet();
    packet.edges[0].range[1] = f64::from_bits(2f64.to_bits() + 1);
    packet.edges[0].curve_range = packet.edges[0].range;
    assert!(
        inspect_owned_strip(&packet, || false)
            .prerequisites
            .is_err()
    );
}
#[test]
fn affine_rotation_translation_and_reflection_keep_exact_conformance() {
    let mut packet = fixture::packet();
    let linear = [[0., -1., 0.], [1., 0., 0.], [0., 0., -1.]];
    packet.support_placement.linear = linear;
    packet.support_placement.origin = [3., 4., 0.];
    for edge in &mut packet.edges {
        edge.placement.linear = linear;
        edge.placement.origin = [3., 4., -edge.placement.origin[2]];
    }
    assert!(inspect_owned_strip(&packet, || false).prerequisites.is_ok());
    packet.edges[3].placement.origin = [4., 3., 2.];
    assert!(
        inspect_owned_strip(&packet, || false)
            .prerequisites
            .is_err()
    );
}
#[test]
fn periodic_and_open_layouts_refuse_prerequisites() {
    let mut packet = fixture::packet();
    packet.basis.periodic = true;
    assert!(
        inspect_owned_strip(&packet, || false)
            .prerequisites
            .is_err()
    );
    packet = fixture::packet();
    packet.basis.poles[8][0] = 2.;
    assert!(
        inspect_owned_strip(&packet, || false)
            .prerequisites
            .is_err()
    );
}
#[test]
fn degenerate_and_nonclamped_layouts_refuse_prerequisites() {
    let mut packet = fixture::packet();
    packet.uv_bounds[2] = 0.;
    assert!(
        inspect_owned_strip(&packet, || false)
            .prerequisites
            .is_err()
    );
    packet = fixture::packet();
    packet.basis.multiplicities[0] = 2;
    assert!(
        inspect_owned_strip(&packet, || false)
            .prerequisites
            .is_err()
    );
}
