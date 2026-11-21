//! Actual cylinder bodies pin membership, replay and revision behavior at the public interface.
use super::*;
use martensite_tkbrep::{
    brep::BRepBuilder, brep_adaptor::BRepAdaptorSurface, topexp::Explorer, topods,
};
use martensite_tkg3d::{adaptor3d::Adaptor3dSurface, top_abs::ShapeEnum};
use martensite_tkmath::{
    geom_abs::SurfaceType,
    gp::{Axis2, Direction3, Point3},
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
pub(super) fn resolution() -> Resolution {
    use martensite_tkbo::profile_extrusion::{DisplayTolerance, ToleranceCap};
    Resolution::new(
        1e-6,
        1e-7,
        1e-9,
        DisplayTolerance {
            chordal: 1e-3,
            angular: 0.1,
        },
        ToleranceCap::Refuse(1e-5),
    )
}
pub(super) fn body(radius: f64) -> (Shape, Face) {
    let mut maker = BRepPrimAPIMakeCylinder::new(radius, 4.);
    let body = maker.shape().unwrap().clone();
    let face = cylinder_face(&body);
    (body, face)
}
fn cylinder_face(body: &Shape) -> Face {
    let mut explorer = Explorer::new(body, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        let face = topods::face(explorer.current()).unwrap();
        if BRepAdaptorSurface::with_face(&face, false)
            .unwrap()
            .get_type()
            .unwrap()
            == SurfaceType::Cylinder
        {
            return face;
        }
        explorer.next().unwrap();
    }
    panic!("fixture must contain one cylindrical lateral face")
}
pub(super) fn capture() -> CylinderFaceAttachment {
    let (body, face) = body(10.);
    CylinderFaceAttachment::capture(
        FaceAttachmentId(7),
        SupportRevision(3),
        &body,
        &face,
        resolution(),
    )
    .unwrap()
}
#[test]
fn selected_face_roundtrips_with_its_body_and_caller_identity() {
    let attachment = capture();
    let restored = CylinderFaceAttachment::restore(attachment.record().clone()).unwrap();
    assert_eq!(attachment.record(), restored.record());
    assert!(restored.validation().is_valid());
    assert!(!restored.validation().is_exact());
    let first = attachment.export_native(SupportRevision(3)).unwrap();
    let second = restored.export_native(SupportRevision(3)).unwrap();
    assert!(!first.body.is_partner(&second.body));
    assert_unique_membership(&first);
}
fn assert_unique_membership(source: &DetachedCylinderSupport) {
    let attempt = Attempt::new(AttachmentOperation::Export, source.id, source.revision);
    assert_eq!(
        admission::membership(&attempt, &source.body, &source.face)
            .unwrap()
            .0,
        1
    );
}
#[test]
fn foreign_face_is_rejected_even_when_it_has_identical_geometry() {
    let (body, _) = body(10.);
    let (_, foreign) = self::body(10.);
    let error = CylinderFaceAttachment::capture(
        FaceAttachmentId(7),
        SupportRevision(3),
        &body,
        &foreign,
        resolution(),
    )
    .err()
    .unwrap();
    assert!(matches!(
        error.cause,
        AttachmentFailureCause::Membership { occurrences: 0 }
    ));
}
#[test]
fn stale_revision_cannot_export_a_current_support() {
    let error = capture().export_native(SupportRevision(4)).err().unwrap();
    assert!(matches!(
        error.cause,
        AttachmentFailureCause::Revision {
            expected: SupportRevision(4),
            actual: SupportRevision(3),
        }
    ));
}
#[test]
fn export_mutation_does_not_change_frozen_geometry() {
    let attachment = capture();
    let bytes = attachment.record().brep.clone();
    let detached = attachment.export_native(SupportRevision(3)).unwrap();
    BRepBuilder::default()
        .update_face_tolerance(&detached.face, 0.1)
        .unwrap();
    assert_eq!(attachment.record().brep, bytes);
    assert!(
        CylinderFaceAttachment::restore(attachment.record().clone())
            .unwrap()
            .validation()
            .is_valid()
    );
}
#[test]
fn history_rebinds_explicitly_changed_radius_without_searching_for_a_nearest_face() {
    let attachment = capture();
    let source = attachment.export_native(SupportRevision(3)).unwrap();
    let (changed, target) = body(12.);
    let history = History::new();
    history.add_modified(&source.face, &target).unwrap();
    let result = attachment
        .reattach(&source, &changed, SupportRevision(4), &history)
        .unwrap();
    assert_eq!(result.record().id, FaceAttachmentId(7));
    assert_eq!(result.record().revision, SupportRevision(4));
    let exported = result.export_native(SupportRevision(4)).unwrap();
    assert_eq!(radius(&exported.face), 12.);
}
fn radius(face: &Face) -> f64 {
    BRepAdaptorSurface::with_face(face, false)
        .unwrap()
        .cylinder()
        .unwrap()
        .radius()
}
#[test]
fn missing_history_refuses_an_identical_but_unrelated_support_face() {
    let attachment = capture();
    let source = attachment.export_native(SupportRevision(3)).unwrap();
    let (changed, _) = body(10.);
    let error = attachment
        .reattach(&source, &changed, SupportRevision(4), &History::new())
        .err()
        .unwrap();
    assert!(matches!(
        error.cause,
        AttachmentFailureCause::HistoryImages { count: 0 }
    ));
}
#[test]
fn oblique_intrinsic_cylinder_replays_its_native_coefficients() {
    let (body, face) = oblique_body();
    let attachment = CylinderFaceAttachment::capture(
        FaceAttachmentId(7),
        SupportRevision(3),
        &body,
        &face,
        resolution(),
    )
    .unwrap();
    assert_eq!(
        attachment.record(),
        CylinderFaceAttachment::restore(attachment.record().clone())
            .unwrap()
            .record()
    );
}
fn oblique_body() -> (Shape, Face) {
    let axes = Axis2::with_x_direction(
        Point3::new(1.25, -2.5, 3.75),
        Direction3::new(1., 2., 3.).unwrap(),
        Direction3::new(3., -1., 2.).unwrap(),
    )
    .unwrap();
    let mut maker = BRepPrimAPIMakeCylinder::with_axes(axes, 10., 4.);
    let body = maker.shape().unwrap().clone();
    let face = cylinder_face(&body);
    (body, face)
}
#[test]
fn public_attachment_values_are_send_sync() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<FaceAttachmentId>();
    send_sync::<SupportRevision>();
    send_sync::<CylinderAttachmentRecord>();
    send_sync::<CylinderFaceAttachment>();
    send_sync::<DetachedCylinderSupport>();
    send_sync::<AttachmentOperation>();
    send_sync::<AttachmentFailureCause>();
    send_sync::<AttachmentFailure>();
}
