// SPDX-License-Identifier: GPL-2.0-only
use super::super::*;
use martensite_tkmath::predicates::roots::DyadicBound;
#[path = "vertex_fixture.rs"]
mod fixture;

fn proof(packet: &OwnedFacePacket) -> OwnedVertexBinding {
    let observation = inspect_owned_strip(packet, || false);
    assert_eq!(
        observation.completion,
        BindingIncomplete::ReleasedSupportCertificateNotBound
    );
    assert_eq!(observation.source_atomicity, SourceAtomicity::Unestablished);
    assert_eq!(
        observation.topology_observation,
        TopologyObservation::OwnedVertexEndpointIncidenceBound
    );
    match observation.vertex_binding {
        VertexBindingObservation::Bound(proof) => proof,
        other => panic!("{other:?}; expected bound owned endpoints"),
    }
}
fn refuses(packet: &OwnedFacePacket) {
    let observed = inspect_owned_strip(packet, || false);
    assert!(
        observed.prerequisites.is_ok(),
        "{:?}",
        observed.prerequisites
    );
    assert!(
        matches!(
            observed.vertex_binding,
            VertexBindingObservation::Incomplete(_)
        ),
        "{:?}",
        observed.vertex_binding
    );
    assert_eq!(
        observed.topology_observation,
        TopologyObservation::EdgeOccurrencesOnlyVerticesUnbound
    );
}
#[test]
fn named_closed_strip_endpoints_complete_only_the_owned_vertex_obligation() {
    let proof = proof(&fixture::packet(0.25, 0.));
    assert_eq!(proof.endpoints.len(), 8);
    assert_eq!(proof.identity_pairs_checked, 28);
    assert!(
        proof
            .endpoints
            .iter()
            .all(|p| p.curve.certified_squared_upper == DyadicBound::from_f64(0.).unwrap())
    );
}
#[test]
fn nonzero_endpoint_error_has_independent_exact_squared_bound() {
    let proof = proof(&fixture::packet(0.25, 0.125));
    let expected = DyadicBound::from_f64(0.015625).unwrap();
    for endpoint in proof.endpoints {
        assert_eq!(endpoint.curve.certified_squared_upper, expected);
        assert_eq!(endpoint.support.certified_squared_upper, expected);
        assert_eq!(endpoint.curve.requested_spatial_limit, 0.25);
        assert!(endpoint.curve.threshold_queries <= 65);
    }
}
#[test]
fn unchanged_raw_limit_refuses_error_that_compatibility_floor_would_admit() {
    let packet = fixture::packet(1e-9, 1e-8);
    assert_eq!(packet.edges[0].vertices[0].effective_tolerance, 1e-7);
    refuses(&packet);
}
#[test]
fn subnormal_squared_endpoint_error_stays_nonzero_without_float_square() {
    let tiny = f64::from_bits(1);
    let proof = proof(&fixture::packet(f64::from_bits(2), tiny));
    let expected = super::super::exact::squared_limit(tiny, &mut || false).unwrap();
    assert!(expected > DyadicBound::from_f64(0.).unwrap());
    assert_eq!(proof.endpoints[0].curve.certified_squared_upper, expected);
}
#[test]
fn raw_zero_limit_is_exact_and_never_raised() {
    proof(&fixture::packet(-0., 0.));
    refuses(&fixture::packet(0., f64::from_bits(1)));
}
#[test]
fn same_tshape_with_different_location_remains_a_distinct_endpoint_class() {
    let packet = fixture::packet(0.25, 0.);
    let a = &packet.edges[0].vertices[0];
    let b = &packet.edges[0].vertices[1];
    assert_eq!(a.intrinsic.tshape_token, b.intrinsic.tshape_token);
    assert_ne!(a.intrinsic.location_datums, b.intrinsic.location_datums);
    proof(&packet);
}
#[test]
fn missing_and_duplicate_endpoint_uses_refuse_before_geometric_checks() {
    let mut packet = fixture::packet(0.25, 0.);
    packet.edges[0].vertices.pop();
    refuses(&packet);
    packet = fixture::packet(0.25, 0.);
    packet.edges[0].vertices[1] = packet.edges[0].vertices[0].clone();
    refuses(&packet);
}
#[test]
fn parameter_mismatch_is_not_hidden_by_spatial_tolerance() {
    let mut packet = fixture::packet(0.25, 0.);
    let EndpointParameterObservation::TopologicalRangeEnd { returned, .. } =
        &mut packet.edges[0].vertices[0].parameter
    else {
        panic!("expected range parameter")
    };
    *returned = Ok(Some(f64::from_bits(1)));
    refuses(&packet);
}
#[test]
fn missing_parameter_and_uninspected_point_records_remain_incomplete() {
    let mut packet = fixture::packet(0.25, 0.);
    packet.edges[0].vertices[0].point_representation_count = 1;
    refuses(&packet);
    packet = fixture::packet(0.25, 0.);
    let EndpointParameterObservation::TopologicalRangeEnd { returned, .. } =
        &mut packet.edges[0].vertices[0].parameter
    else {
        panic!("expected range parameter")
    };
    *returned = Ok(None);
    refuses(&packet);
}
#[test]
fn modified_flag_change_is_an_incomplete_endpoint_observation() {
    let mut packet = fixture::packet(0.25, 0.);
    let EndpointParameterObservation::TopologicalRangeEnd {
        modified_before,
        modified_after,
        ..
    } = &mut packet.edges[0].vertices[0].parameter
    else {
        panic!("expected range parameter")
    };
    *modified_before = false;
    *modified_after = true;
    refuses(&packet);
}
#[test]
fn wrong_quotient_pair_and_missing_pair_receipt_refuse() {
    let mut packet = fixture::packet(0.25, 0.);
    packet.vertex_relations[0].same = !packet.vertex_relations[0].same;
    refuses(&packet);
    packet = fixture::packet(0.25, 0.);
    packet.vertex_relations.pop();
    refuses(&packet);
}
#[test]
fn reversed_boundary_metadata_cannot_change_intrinsic_parameter_side() {
    let mut packet = fixture::packet(0.25, 0.);
    packet.edges[0].vertices[0].boundary.orientation =
        martensite_tkg3d::top_abs::Orientation::Reversed;
    refuses(&packet);
}
#[test]
fn coherent_endpoint_reindexing_does_not_create_or_change_identity() {
    let mut packet = fixture::packet(0.25, 0.);
    for edge in &mut packet.edges {
        edge.vertices.swap(0, 1);
    }
    for relation in &mut packet.vertex_relations {
        let a = relation.left ^ 1;
        let b = relation.right ^ 1;
        relation.left = a.min(b);
        relation.right = a.max(b);
    }
    proof(&packet);
}
#[test]
fn negative_nonfinite_raw_tolerances_do_not_use_effective_replacements() {
    let mut packet = fixture::packet(0.25, 0.);
    for edge in &mut packet.edges {
        for vertex in &mut edge.vertices {
            vertex.raw_tolerance = -1.;
        }
    }
    refuses(&packet);
    packet = fixture::packet(0.25, 0.);
    packet.raw_face_tolerance = f64::NAN;
    refuses(&packet);
}

#[test]
fn coincident_points_with_distinct_vertex_identity_never_form_a_quotient() {
    refuses(&fixture::split_vertex());
}
#[test]
fn equal_matrices_with_distinct_location_datum_identity_never_form_a_quotient() {
    refuses(&fixture::split_location());
}
#[test]
fn cancellation_during_vertex_stage_never_publishes_owned_vertex_binding() {
    let packet = fixture::packet(0.25, 0.);
    let prerequisite = inspect_owned_strip(&packet, || false).prerequisites;
    assert!(matches!(
        super::super::vertex_binding::inspect(&packet, &prerequisite, &mut || true),
        VertexBindingObservation::Incomplete(BindingFailure::Cancelled)
    ));
}
