//! Observable evaluator forms and retained-input isolation are independent obligations.
use super::{check, fixture};
use martensite_tkbrep::brep::{BRepBuilder, face_input_capture::*};
use martensite_tkmath::{
    gp::{Point3, Transform3, TransformForm},
    toploc::Location,
};
use martensite_tktopalgo::face_validation::*;
#[test]
fn form_only_mutation_refuses_the_stored_evaluator_disagreement() {
    let mut fixture = fixture::strip(1e-7, 0.);
    let mut transform = Transform3::translation([7., 0., 0.]);
    transform.set_form(TransformForm::Identity);
    fixture
        .face
        .set_location(&Location::new(transform), false)
        .unwrap();
    let report = check(&fixture, ClosedFacePolicy::bounded());
    assert!(
        matches!(report.completion(), ClosedFaceCompletion::Unsupported(_)),
        "{:?}",
        report.completion()
    );
    assert!(report.certificate().is_none());
}
#[test]
fn coherent_translation_preserves_the_complete_face_certificate() {
    let mut fixture = fixture::strip(1e-7, 0.);
    fixture
        .face
        .set_location(
            &Location::new(Transform3::translation([7., -3., 2.])),
            false,
        )
        .unwrap();
    let report = check(&fixture, ClosedFacePolicy::bounded());
    assert!(report.certificate().is_some(), "{:?}", report.completion());
}
#[test]
fn later_live_vertex_mutation_does_not_change_the_captured_certificate() {
    let fixture = fixture::strip(1e-7, 0.);
    let input =
        capture_closed_extrusion_face(&fixture.face, FaceCaptureLimits::bounded(), || false)
            .unwrap();
    let before = format!("{input:?}");
    BRepBuilder::default()
        .update_vertex_point(&fixture.vertices[0], Point3::new(9., 9., 9.), 1e-7)
        .unwrap();
    let report = certify_closed_extrusion_face(input, ClosedFacePolicy::bounded(), || false);
    assert!(report.certificate().is_some(), "{:?}", report.completion());
    assert_eq!(format!("{:?}", report.input()), before);
    assert!(
        check(&fixture, ClosedFacePolicy::bounded())
            .certificate()
            .is_none()
    );
}
#[test]
fn public_owned_report_and_borrowed_certificate_are_send_and_sync() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<ClosedFaceReport>();
    send_sync::<ClosedFaceCertificate<'static>>();
}
#[test]
fn reversed_face_orientation_preserves_the_same_owned_domain() {
    let mut fixture = fixture::strip(1e-7, 0.);
    fixture.face.reverse();
    let report = check(&fixture, ClosedFacePolicy::bounded());
    assert!(report.certificate().is_some(), "{:?}", report.completion());
}
#[test]
fn collapsed_and_backtracking_supports_never_certify() {
    let collapsed = [[1., 0.]; 9];
    let backtracking = [
        [1., 0.],
        [2., 0.],
        [3., 0.],
        [2., 0.],
        [1., 0.],
        [0., 0.],
        [-1., 0.],
        [0., 0.],
        [1., 0.],
    ];
    for net in [collapsed, backtracking] {
        let fixture = fixture::strip_from_net(1e-7, 0., net);
        let report = check(&fixture, ClosedFacePolicy::bounded());
        assert!(report.certificate().is_none(), "{:?}", report.completion());
    }
}
#[test]
fn a_crossing_coherent_boundary_has_no_self_exclusion_certificate() {
    let net = [
        [1., 0.],
        [1., 1.],
        [-1., 1.],
        [-1., 0.],
        [1., -1.],
        [1., 0.],
        [-1., -1.],
        [0., 0.],
        [1., 0.],
    ];
    let fixture = fixture::strip_from_net(1e-7, 0., net);
    let report = check(&fixture, ClosedFacePolicy::bounded());
    assert!(report.coverage().geometry, "{:?}", report.completion());
    assert!(report.certificate().is_none(), "{:?}", report.completion());
}
#[test]
fn support_and_location_limits_remain_incomplete() {
    let mut fixture = fixture::strip(1e-7, 0.);
    let mut policy = ClosedFacePolicy::bounded();
    policy.support.max_obligations = 0;
    assert!(check(&fixture, policy).certificate().is_none());
    fixture
        .face
        .set_location(&Location::new(Transform3::translation([1., 0., 0.])), false)
        .unwrap();
    let policy = ClosedFacePolicy {
        max_location_steps: 0,
        ..ClosedFacePolicy::bounded()
    };
    let report = check(&fixture, policy);
    assert!(matches!(
        report.completion(),
        ClosedFaceCompletion::Incomplete(_)
    ));
}

#[test]
fn query_allowance_is_global_and_keeps_actual_unfinished_work() {
    let fixture = fixture::strip(1e-7, 0.);
    let full = check(&fixture, ClosedFacePolicy::bounded());
    let count = full.work().queries;
    let exact = ClosedFacePolicy {
        max_queries: count,
        ..ClosedFacePolicy::bounded()
    };
    assert!(check(&fixture, exact).certificate().is_some());
    let short = ClosedFacePolicy {
        max_queries: count - 1,
        ..exact
    };
    let refused = check(&fixture, short);
    assert!(matches!(
        refused.completion(),
        ClosedFaceCompletion::Incomplete(_)
    ));
    assert_eq!(refused.work().queries, count);
    assert!(refused.certificate().is_none());
}
#[test]
fn a_spent_support_proof_budget_never_becomes_a_face_pass() {
    let fixture = fixture::strip(1e-7, 0.);
    let mut policy = ClosedFacePolicy::bounded();
    policy.support.max_proof_steps = 0;
    let report = check(&fixture, policy);
    assert!(report.coverage().geometry);
    assert!(!report.coverage().self_exclusion);
    assert!(matches!(
        report.completion(),
        ClosedFaceCompletion::Incomplete(_)
    ));
    assert!(report.certificate().is_none());
}
