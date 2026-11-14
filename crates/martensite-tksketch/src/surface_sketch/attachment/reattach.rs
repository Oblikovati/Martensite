//! History supplies candidate faces; absent history never triggers geometric matching.
use super::*;
use martensite_tkbrep::{topods, toptools::IndexedMapOfShape};
use martensite_tkg3d::top_abs::ShapeEnum;
pub(super) fn apply(
    attachment: &CylinderFaceAttachment,
    source: &DetachedCylinderSupport,
    body: &Shape,
    revision: SupportRevision,
    history: &History,
) -> Checked<CylinderFaceAttachment> {
    let attempt = Attempt::new(
        AttachmentOperation::Reattach,
        attachment.record.id,
        attachment.record.revision,
    );
    source_matches(attachment, source, &attempt)?;
    if revision == attachment.record.revision {
        return Err(attempt.reject(AttachmentFailureCause::UnchangedRevision));
    }
    let faces = images(&attempt, source, body, history)?;
    let face = unique_face(&attempt, &faces)?;
    publish(attachment, body, &face, revision)
}
fn unique_face(attempt: &Attempt, faces: &IndexedMapOfShape) -> Checked<Face> {
    if faces.extent() != 1 {
        return Err(attempt.reject(AttachmentFailureCause::HistoryImages {
            count: faces.extent(),
        }));
    }
    let shape = attempt.native("unique history face", faces.find_key(1))?;
    attempt.native("history face", topods::face(shape))
}
fn publish(
    attachment: &CylinderFaceAttachment,
    body: &Shape,
    face: &Face,
    revision: SupportRevision,
) -> Checked<CylinderFaceAttachment> {
    let target = Attempt::new(
        AttachmentOperation::Reattach,
        attachment.record.id,
        revision,
    );
    let brep = codec::encode(&target, body, face)?;
    let record = CylinderAttachmentRecord {
        id: attachment.record.id,
        revision,
        resolution: attachment.record.resolution,
        brep,
    };
    codec::restore(record, target)
}

fn source_matches(
    attachment: &CylinderFaceAttachment,
    source: &DetachedCylinderSupport,
    attempt: &Attempt,
) -> Checked<()> {
    if source.id != attachment.record.id || source.revision != attachment.record.revision {
        return Err(attempt.reject(AttachmentFailureCause::SourceIdentity {
            actual: source.id,
            revision: source.revision,
        }));
    }
    let actual = codec::encode(attempt, &source.body, &source.face)?;
    codec::compare(attempt, &attachment.record.brep, &actual)
}
fn images(
    attempt: &Attempt,
    source: &DetachedCylinderSupport,
    body: &Shape,
    history: &History,
) -> Checked<IndexedMapOfShape> {
    let mut images = IndexedMapOfShape::default();
    let modified = attempt.native("modified faces", history.modified(&source.face))?;
    append(attempt, body, &mut images, modified)?;
    let generated = attempt.native("generated faces", history.generated(&source.face))?;
    append(attempt, body, &mut images, generated)?;
    if !attempt.native("deleted support face", history.is_removed(&source.face))?
        && admission::membership(attempt, body, &source.face)?.0 == 1
    {
        images.add(source.face.clone().into_shape());
    }
    Ok(images)
}
fn append(
    attempt: &Attempt,
    body: &Shape,
    images: &mut IndexedMapOfShape,
    candidates: Vec<Shape>,
) -> Checked<()> {
    for candidate in candidates {
        if attempt.native("history image kind", candidate.shape_type())? != ShapeEnum::Face {
            continue;
        }
        let face = attempt.native("history face image", topods::face(&candidate))?;
        admission::face_use(attempt, body, &face)?;
        images.add(candidate);
    }
    Ok(())
}
