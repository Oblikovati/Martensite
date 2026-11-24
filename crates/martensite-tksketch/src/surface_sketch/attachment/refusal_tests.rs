//! Refusals preserve the frozen source and require explicit, unambiguous history.
use super::tests::{body, capture};
use super::*;
use martensite_tkbrep::brep::BRepBuilder;

fn source(attachment: &CylinderFaceAttachment) -> DetachedCylinderSupport {
    attachment.export_native(SupportRevision(3)).unwrap()
}
#[test]
fn a_changed_body_requires_a_distinct_revision() {
    let attachment = capture();
    let source = source(&attachment);
    let error = attachment
        .reattach(&source, &source.body, SupportRevision(3), &History::new())
        .err()
        .unwrap();
    assert!(error.to_string().contains("target revision distinct"));
    assert!(error.to_string().contains("SupportRevision(3)"));
    assert!(matches!(
        error.cause,
        AttachmentFailureCause::UnchangedRevision
    ));
}
#[test]
fn removal_history_does_not_reselect_a_surviving_native_identity() {
    let attachment = capture();
    let source = source(&attachment);
    let history = History::new();
    history.remove(&source.face).unwrap();
    let error = attachment
        .reattach(&source, &source.body, SupportRevision(4), &history)
        .err()
        .unwrap();
    assert!(matches!(
        error.cause,
        AttachmentFailureCause::HistoryImages { count: 0 }
    ));
}
#[test]
fn unchanged_face_uses_exact_identity_when_history_has_no_modification() {
    let attachment = capture();
    let source = source(&attachment);
    let rebound = attachment
        .reattach(&source, &source.body, SupportRevision(4), &History::new())
        .unwrap();
    assert_eq!(rebound.record().brep, attachment.record().brep);
    assert_eq!(rebound.record().revision, SupportRevision(4));
}
#[test]
fn mutation_of_operation_source_refuses_rebinding_without_mutating_snapshot() {
    let attachment = capture();
    let bytes = attachment.record().brep.clone();
    let source = source(&attachment);
    BRepBuilder::default()
        .update_face_tolerance(&source.face, 0.1)
        .unwrap();
    let (target, _) = body(12.);
    let error = attachment
        .reattach(&source, &target, SupportRevision(4), &History::new())
        .err()
        .unwrap();
    assert!(matches!(error.cause, AttachmentFailureCause::Replay { .. }));
    assert_eq!(attachment.record().brep, bytes);
    assert!(CylinderFaceAttachment::restore(attachment.record().clone()).is_ok());
}
#[test]
fn trailing_persistence_bytes_are_not_silently_discarded() {
    let mut record = capture().record().clone();
    record.brep.push(0x7f);
    let error = CylinderFaceAttachment::restore(record).err().unwrap();
    assert!(matches!(
        error.cause,
        AttachmentFailureCause::Replay {
            expected: Some(0x7f),
            actual: None,
            ..
        }
    ));
}
#[test]
fn truncated_persistence_is_an_owned_failure() {
    let mut record = capture().record().clone();
    record.brep.truncate(16);
    let error = CylinderFaceAttachment::restore(record).err().unwrap();
    assert_eq!(error.operation, AttachmentOperation::Restore);
    assert_eq!(error.class(), AttachmentFailureClass::Persistence);
    assert_eq!(error.attachment, FaceAttachmentId(7));
    assert!(matches!(error.cause, AttachmentFailureCause::Encoding(_)));
}
#[test]
fn split_face_history_refuses_ambiguous_attachment() {
    let attachment = capture();
    let source = source(&attachment);
    let (compound, history) = split_images(&source.face);
    let error = attachment
        .reattach(&source, &compound, SupportRevision(4), &history)
        .err()
        .unwrap();
    assert!(matches!(
        error.cause,
        AttachmentFailureCause::HistoryImages { count: 2 }
    ));
}

#[test]
fn revision_failure_retains_a_typed_class_and_context() {
    let error = capture().export_native(SupportRevision(99)).err().unwrap();
    assert_eq!(error.class(), AttachmentFailureClass::StaleRevision);
    assert_eq!(error.operation, AttachmentOperation::Export);
    assert_eq!(error.attachment, FaceAttachmentId(7));
    assert_eq!(error.revision, SupportRevision(3));
    fn send_sync<T: Send + Sync>() {}
    send_sync::<AttachmentFailureClass>();
}

fn split_images(source: &Face) -> (Shape, History) {
    use martensite_tkbrep::topods::{Builder, Compound};
    let (first, first_face) = body(12.);
    let (second, second_face) = body(14.);
    let mut compound = Compound::default();
    Builder.make_compound(&mut compound);
    Builder.add(&mut compound, &first).unwrap();
    Builder.add(&mut compound, &second).unwrap();
    let history = History::new();
    history.add_modified(source, &first_face).unwrap();
    history.add_modified(source, &second_face).unwrap();
    (compound.into_shape(), history)
}

#[test]
fn original_input_mutation_does_not_change_the_owned_attachment() {
    let (body, face) = body(10.);
    let attachment = CylinderFaceAttachment::capture(
        FaceAttachmentId(7),
        SupportRevision(3),
        &body,
        &face,
        super::tests::resolution(),
    )
    .unwrap();
    let before = attachment.record().clone();
    BRepBuilder::default()
        .update_face_tolerance(&face, 0.1)
        .unwrap();
    assert_eq!(attachment.record(), &before);
    let restored = CylinderFaceAttachment::restore(before).unwrap();
    assert!(restored.validation().is_valid());
    assert_eq!(restored.record(), attachment.record());
}

#[test]
fn invalid_resolution_message_names_the_value_and_required_domain() {
    let mut record = capture().record().clone();
    record.resolution.sew = -0.25;
    let error = CylinderFaceAttachment::restore(record).err().unwrap();
    let message = error.to_string();
    assert!(message.contains("sew"));
    assert!(message.contains("-0.25"));
    assert!(message.contains("finite strictly positive"));
    assert_eq!(error.class(), AttachmentFailureClass::InvalidInput);
}
