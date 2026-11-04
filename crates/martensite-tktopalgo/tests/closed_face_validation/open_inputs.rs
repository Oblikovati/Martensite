//! Input binding does not discharge represented seam, cap, metric or face-validity checks.
use super::{fixture, placed};
use martensite_tkbrep::brep::face_input_capture::*;
use martensite_tkmath::predicates::roots::open_extrusion::*;
use martensite_tktopalgo::face_validation::*;
fn snapshot(shape: &fixture::StripFixture) -> FaceInputSnapshot {
    capture_closed_extrusion_face(&shape.face, FaceCaptureLimits::bounded(), || false).unwrap()
}
fn capture(shape: &fixture::StripFixture) -> OpenSupportInputReport {
    capture_placed_open_support_inputs(
        snapshot(shape),
        ClosedFacePolicy::placed_piecewise_boundary(),
        || false,
    )
}
#[test]
fn open_endpoint_inputs_bind_without_becoming_a_face_certificate() {
    let mut net = fixture::ROUND_NET;
    net[8][0] = f64::from_bits(1f64.to_bits() + 1);
    let shape = fixture::strip_from_net(1e-7, 0., net);
    let report = capture(&shape);
    let inputs = report.inputs().unwrap();
    for input in [inputs.support(), inputs.carrier()] {
        let proof = certify_open_quadratic_extrusion(
            input.numeric().clone(),
            OpenExtrusionPolicy::bounded(),
            || false,
        );
        assert!(proof.certificate().is_some(), "{:?}", proof.completion());
        assert_ne!(proof.input().poles().first(), proof.input().poles().last());
    }
    let face = super::check(&shape, ClosedFacePolicy::placed_piecewise_boundary());
    assert!(face.certificate().is_none());
}
#[test]
fn reflected_actual_frame_selects_local_direction_only_after_exact_world_check() {
    let shape = placed::reflected_carrier();
    let report = capture(&shape);
    let inputs = report.inputs().unwrap();
    assert_eq!(inputs.support().numeric().direction(), [0., 0., 1.]);
    assert_eq!(inputs.carrier().numeric().direction(), [0., 0., -1.]);
    assert_eq!(inputs.reference_parameter(), -1.);
    assert!(report.work().queries > 0);
}
#[test]
fn rounded_inverse_reconstruction_does_not_pass_input_binding() {
    use martensite_tkmath::gp::{Point3, Transform3};
    let shape = fixture::strip(1e-7, 0.);
    let transform = Transform3::scaling(Point3::default(), 3.).unwrap();
    for edge in [&shape.edges[1], &shape.edges[2]] {
        placed::place_spatial_curve(edge, transform);
    }
    let report = capture(&shape);
    assert!(matches!(
        report.inputs(),
        Err(OpenSupportInputFailure::Face(
            ClosedFaceCompletion::Incomplete(ClosedFaceIncomplete::BoundaryRelation { .. })
        ))
    ));
}
#[test]
fn cancellation_and_query_limits_are_retained_without_published_inputs() {
    let shape = fixture::strip(1e-7, 0.);
    let mut limits = ClosedFacePolicy::placed_piecewise_boundary();
    let cancelled = capture_placed_open_support_inputs(snapshot(&shape), limits, || true);
    assert!(matches!(
        cancelled.inputs(),
        Err(OpenSupportInputFailure::Face(
            ClosedFaceCompletion::Incomplete(ClosedFaceIncomplete::Cancelled)
        ))
    ));
    limits.max_queries = 0;
    let limited = capture_placed_open_support_inputs(snapshot(&shape), limits, || false);
    assert!(matches!(
        limited.inputs(),
        Err(OpenSupportInputFailure::Face(
            ClosedFaceCompletion::Incomplete(ClosedFaceIncomplete::Limit { .. })
        ))
    ));
}
#[test]
fn earlier_face_policies_cannot_select_this_input_binding_implicitly() {
    let shape = fixture::strip(1e-7, 0.);
    for policy in [
        ClosedFacePolicy::bounded(),
        ClosedFacePolicy::piecewise_boundary(),
    ] {
        let report = capture_placed_open_support_inputs(snapshot(&shape), policy, || false);
        assert!(matches!(
            report.inputs(),
            Err(OpenSupportInputFailure::Face(
                ClosedFaceCompletion::Unsupported(_)
            ))
        ));
        assert_eq!(report.work().queries, 0);
    }
}
#[test]
fn owned_inputs_are_send_sync_and_survive_later_live_source_mutation() {
    fn owned<T: Send + Sync>() {}
    owned::<OpenSupportInputReport>();
    owned::<PlacedOpenSupportInputs>();
    let shape = fixture::strip(1e-7, 0.);
    let report = capture(&shape);
    let before = format!("{report:?}");
    fixture::set_cache(&shape.edges[0], [[9.; 2]; 2], Some([[9.; 2]; 2]));
    assert_eq!(format!("{report:?}"), before);
    let changed = capture(&shape);
    assert!(changed.inputs().is_err());
}
