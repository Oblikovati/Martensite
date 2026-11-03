//! Independent actual-carrier checks retain the coincident replay contract.
use super::{check, fixture};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::face_validation::*;

#[test]
fn nonconstant_cap_displacement_has_two_independent_proofs() {
    let mut actual = fixture::ROUND_NET;
    actual[1][0] += 1. / 1_073_741_824.;
    let shape = fixture::strip_with_carrier(1e-7, actual);
    let report = check(&shape, ClosedFacePolicy::piecewise_boundary());
    assert!(report.certificate().is_some(), "{:?}", report.completion());
    assert_eq!(report.work().support_obligations, 20);
    assert_eq!(report.work().support_finished, 20);
    let actual_proof = report.boundary().unwrap().proof().unwrap();
    assert!(actual_proof.certificate().is_some());
    assert!(
        check(&shape, ClosedFacePolicy::bounded())
            .certificate()
            .is_none()
    );
}

#[test]
fn collapsed_and_folded_actual_carriers_refuse_even_with_large_tolerance() {
    let collapsed = [[1., 0.]; 9];
    let mut folded = fixture::ROUND_NET;
    folded[2] = [0., -2.];
    folded[6] = [0., 2.];
    for actual in [collapsed, folded] {
        let shape = fixture::strip_with_carrier(100., actual);
        let report = check(&shape, ClosedFacePolicy::piecewise_boundary());
        assert!(report.coverage().geometry, "{:?}", report.completion());
        assert!(report.certificate().is_none(), "{:?}", report.completion());
    }
}

#[test]
fn vertex_displacement_within_raw_tolerance_cannot_inherit_carrier_proof() {
    let shape = fixture::strip(1e-7, 0.);
    BRepBuilder::default()
        .update_vertex_point(
            &shape.vertices[0],
            Point3::new(1., 0., -1. + 1. / 1_073_741_824.),
            1e-7,
        )
        .unwrap();
    let report = check(&shape, ClosedFacePolicy::piecewise_boundary());
    assert!(report.coverage().geometry, "{:?}", report.completion());
    assert!(matches!(
        report.completion(),
        ClosedFaceCompletion::Incomplete(ClosedFaceIncomplete::BoundaryRelation { .. })
    ));
    assert!(report.certificate().is_none());
}

#[test]
fn both_supports_consume_one_global_obligation_and_span_allowance() {
    let shape = fixture::strip(1e-7, 0.);
    for (spans, obligations) in [(7, 20), (8, 19)] {
        let mut policy = ClosedFacePolicy::piecewise_boundary();
        policy.support.max_spans = spans;
        policy.support.max_obligations = obligations;
        let report = check(&shape, policy);
        assert!(report.certificate().is_none(), "{:?}", report.completion());
        assert!(matches!(
            report.completion(),
            ClosedFaceCompletion::Incomplete(_)
        ));
    }
}

#[test]
fn reversed_face_preserves_explicit_piecewise_proof() {
    let mut shape = fixture::strip(1e-7, 0.);
    shape.face.set_orientation(Orientation::Reversed);
    let report = check(&shape, ClosedFacePolicy::piecewise_boundary());
    assert!(report.certificate().is_some(), "{:?}", report.completion());
}

#[test]
fn broken_cap_join_is_incomplete_without_changing_raw_tolerance() {
    let shape = fixture::strip(1e-7, 1. / 1_073_741_824.);
    let report = check(&shape, ClosedFacePolicy::piecewise_boundary());
    assert!(report.coverage().geometry);
    assert!(report.certificate().is_none());
    assert!(matches!(
        report.completion(),
        ClosedFaceCompletion::Incomplete(ClosedFaceIncomplete::BoundaryRelation { .. })
    ));
}

#[test]
fn explicit_piecewise_cancellation_retains_owned_input_without_certificate() {
    use martensite_tkbrep::brep::face_input_capture::{
        FaceCaptureLimits, capture_closed_extrusion_face,
    };
    let shape = fixture::strip(1e-7, 0.);
    let input =
        capture_closed_extrusion_face(&shape.face, FaceCaptureLimits::bounded(), || false).unwrap();
    let report =
        certify_closed_extrusion_face(input, ClosedFacePolicy::piecewise_boundary(), || true);
    assert!(matches!(
        report.completion(),
        ClosedFaceCompletion::Incomplete(ClosedFaceIncomplete::Cancelled)
    ));
    assert!(report.certificate().is_none());
}

#[test]
fn piecewise_published_witnesses_are_send_and_sync() {
    fn require_owned<T: Send + Sync>() {}
    require_owned::<ClosedFaceBoundaryReport>();
    require_owned::<ClosedFaceReport>();
    require_owned::<ClosedFaceCertificate>();
}

#[test]
fn tilted_or_wrong_level_actual_caps_cannot_use_a_planar_carrier() {
    for tilted in [false, true] {
        let shape = fixture::strip(1e-7, 0.);
        for (edge, level) in [(&shape.edges[1], 2.), (&shape.edges[2], -1.)] {
            let points = shifted_cap(level, tilted);
            fixture::replace_cap_points(edge, points, 1e-7);
        }
        let report = check(&shape, ClosedFacePolicy::piecewise_boundary());
        assert!(report.coverage().geometry, "{:?}", report.completion());
        assert!(report.certificate().is_none(), "{:?}", report.completion());
    }
}
fn shifted_cap(level: f64, tilted: bool) -> [Point3; 9] {
    let delta = 1. / 1_073_741_824.;
    std::array::from_fn(|index| {
        let xy = fixture::ROUND_NET[index];
        let offset = if tilted && index != 1 { 0. } else { delta };
        Point3::new(xy[0], xy[1], level + offset)
    })
}
