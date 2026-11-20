//! Frozen conical face selection uses the same graph and history checks as cylinder attachment.
use super::*;
use martensite_tkmath::geom_abs::SurfaceType;
use std::sync::Arc;
mod chart;
mod failure;
pub use chart::{ConeAuthoringChart, ConeParameterRequest, DevelopedConeHit};
pub use failure::ConeAttachmentFailure;
type ConeChecked<T> = Result<T, ConeAttachmentFailure>;

/// Persistence payload for an explicitly selected native conical face.
#[derive(Clone, PartialEq)]
pub struct ConeAttachmentRecord {
    /// Logical selected-face identity supplied by the application.
    pub id: FaceAttachmentId,
    /// Revision associated with these exact frozen bytes.
    pub revision: SupportRevision,
    /// Validation settings, including the tolerance-cap policy.
    pub resolution: Resolution,
    /// Complete native support body with the selected face explicitly referenced.
    pub brep: Vec<u8>,
}

/// Immutable selected conical face and ordered validation of its support body.
#[derive(Clone)]
pub struct ConeFaceAttachment {
    frozen: Arc<FrozenCone>,
}

struct FrozenCone {
    record: ConeAttachmentRecord,
    validation: ValidationReport,
}

/// Detached native support and its original selected-face occurrence.
pub struct DetachedConeSupport {
    /// Independently decoded support body.
    pub body: Shape,
    /// Selected native face belonging to the decoded body.
    pub face: Face,
    id: FaceAttachmentId,
    revision: SupportRevision,
}

impl ConeFaceAttachment {
    /// Freeze a selected cone face. The caller synchronizes concurrent native mutations.
    /// Example: `ConeFaceAttachment::capture(id, revision, &body, &face, resolution)?`.
    pub fn capture(
        id: FaceAttachmentId,
        revision: SupportRevision,
        body: &Shape,
        face: &Face,
        resolution: Resolution,
    ) -> ConeChecked<Self> {
        let attempt = Attempt::new(AttachmentOperation::Capture, id, revision);
        freeze(attempt, body, face, resolution)
    }

    /// Decode and validate a persisted body and its selected conical face.
    /// Example: `ConeFaceAttachment::restore(saved_record)?`.
    pub fn restore(record: ConeAttachmentRecord) -> ConeChecked<Self> {
        let attempt = Attempt::new(AttachmentOperation::Restore, record.id, record.revision);
        restored(attempt, record)
    }

    /// Borrow persistence data. Example: `attachment.record().revision`.
    pub fn record(&self) -> &ConeAttachmentRecord {
        &self.frozen.record
    }

    /// Ordered native validation. Example: `attachment.validation().is_valid()`.
    pub fn validation(&self) -> &ValidationReport {
        &self.frozen.validation
    }

    /// Decode fresh owners for the identified current revision.
    /// Example: `attachment.export_native(revision)?`.
    pub fn export_native(&self, revision: SupportRevision) -> ConeChecked<DetachedConeSupport> {
        let attempt = self.attempt(AttachmentOperation::Export);
        let DecodedFace { body, face } = self.resolve(&attempt, revision)?;
        Ok(DetachedConeSupport {
            body,
            face,
            id: attempt.id,
            revision,
        })
    }

    /// Follow a unique native history image and validate the new conical support.
    /// Example: `attachment.reattach(&source, &changed_body, revision, &history)?`.
    pub fn reattach(
        &self,
        source: &DetachedConeSupport,
        body: &Shape,
        revision: SupportRevision,
        history: &History,
    ) -> ConeChecked<Self> {
        let mut attempt = self.attempt(AttachmentOperation::Reattach);
        let face = reattach::resolve(
            &attempt,
            source.use_ref(),
            &self.record().brep,
            body,
            revision,
            history,
        )?;
        attempt.revision = revision;
        freeze(attempt, body, &face, self.record().resolution)
    }

    fn attempt(&self, operation: AttachmentOperation) -> Attempt {
        Attempt::new(operation, self.record().id, self.record().revision)
    }

    fn resolve(&self, attempt: &Attempt, revision: SupportRevision) -> ConeChecked<DecodedFace> {
        attempt.revision(revision)?;
        // Restore verified the entire immutable graph and the explicit face reference.
        Ok(codec::decode_face(attempt, &self.record().brep)?)
    }
}

impl DetachedConeSupport {
    fn use_ref(&self) -> reattach::DetachedUse<'_> {
        reattach::DetachedUse {
            id: self.id,
            revision: self.revision,
            body: &self.body,
            face: &self.face,
        }
    }
}

fn freeze(
    attempt: Attempt,
    body: &Shape,
    face: &Face,
    resolution: Resolution,
) -> ConeChecked<ConeFaceAttachment> {
    select(&attempt, body, face)?;
    let brep = codec::encode(&attempt, body, face)?;
    restored(
        attempt,
        ConeAttachmentRecord {
            id: attempt.id,
            revision: attempt.revision,
            resolution,
            brep,
        },
    )
}

fn restored(attempt: Attempt, record: ConeAttachmentRecord) -> ConeChecked<ConeFaceAttachment> {
    let DecodedFace { body, face } = codec::decode_face(&attempt, &record.brep)?;
    select(&attempt, &body, &face)?;
    let validation =
        codec::validate_frozen(&attempt, &record.brep, &body, &face, &record.resolution)?;
    Ok(ConeFaceAttachment {
        frozen: Arc::new(FrozenCone { record, validation }),
    })
}

fn select(attempt: &Attempt, body: &Shape, face: &Face) -> ConeChecked<()> {
    admission::face_use(attempt, body, face)?;
    let actual = admission::surface_type(attempt, face)?;
    if actual != SurfaceType::Cone {
        return Err(ConeAttachmentFailure::Surface {
            operation: attempt.operation,
            attachment: attempt.id,
            revision: attempt.revision,
            actual,
        });
    }
    Ok(())
}

impl std::fmt::Debug for ConeAttachmentRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConeAttachmentRecord")
            .field("id", &self.id)
            .field("revision", &self.revision)
            .field("resolution", &self.resolution)
            .field("encoded_bytes", &self.brep.len())
            .finish()
    }
}

#[cfg(test)]
mod tests;
