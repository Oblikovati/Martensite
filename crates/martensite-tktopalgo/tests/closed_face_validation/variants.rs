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
        .update_vertex(&fixture.vertices[0], Point3::new(9., 9., 9.), 1e-7)
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
