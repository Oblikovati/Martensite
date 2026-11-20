//! Revision-bound cylinder selection with owned replay data and explicit history images (#420).
use super::encoding;
use martensite_tkbo::{body_check::ValidationReport, profile_extrusion::Resolution};
use martensite_tkbrep::{
    brep_tools::History,
    topods::{Face, Shape},
};
mod admission;
mod chart;
pub use chart::{CylinderParameterRequest, DevelopedCylinderHit, OutOfSupportPolicy};
mod codec;
mod failure;
mod failure_class;
mod reattach;
mod support;
pub use failure::*;
pub use failure_class::AttachmentFailureClass;
type Checked<T> = Result<T, AttachmentFailure>;
use failure::Attempt;

/// Caller-owned logical identity; never derived from geometry or native traversal order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FaceAttachmentId(pub u64);
/// Caller-owned revision token, scoped to this attachment's support body.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SupportRevision(pub u64);
/// Persistence payload; native bytes contain a root and one explicit selected-face reference.
#[derive(Clone, PartialEq)]
pub struct CylinderAttachmentRecord {
    /// Logical selected-face identity supplied by the application.
    pub id: FaceAttachmentId,
    /// Support revision associated with these exact frozen bytes.
    pub revision: SupportRevision,
    /// Explicit native validation settings, including the tolerance-cap policy.
    pub resolution: Resolution,
    /// BinTools encoding with a selected-face reference following the root reference.
    pub brep: Vec<u8>,
}
/// Immutable selected cylindrical face and its validated support body.
/// Capture requires synchronization against concurrent native-input mutation.
#[derive(Clone)]
pub struct CylinderFaceAttachment {
    record: CylinderAttachmentRecord,
    validation: ValidationReport,
}
/// Fresh native owners for an operation; their mutation follows native synchronization rules.
pub struct DetachedCylinderSupport {
    /// Detached support body.
    pub body: Shape,
    /// Selected face belonging to this detached body.
    pub face: Face,
    id: FaceAttachmentId,
    revision: SupportRevision,
}
impl CylinderFaceAttachment {
    /// Freeze a selected cylinder face by exact membership, without a proximity search.
    /// Example: `CylinderFaceAttachment::capture(id, revision, &body, &face, resolution)?`.
    pub fn capture(
        id: FaceAttachmentId,
        revision: SupportRevision,
        body: &Shape,
        face: &Face,
        resolution: Resolution,
    ) -> Checked<Self> {
        let attempt = Attempt::new(AttachmentOperation::Capture, id, revision);
        admission::selection(&attempt, body, face)?;
        let brep = codec::encode(&attempt, body, face)?;
        let record = CylinderAttachmentRecord {
            id,
            revision,
            resolution,
            brep,
        };
        codec::restore(record, attempt)
    }
    /// Revalidate persisted metadata and geometry before publishing an attachment.
    /// Example: `CylinderFaceAttachment::restore(saved_record)?`.
    pub fn restore(record: CylinderAttachmentRecord) -> Checked<Self> {
        let attempt = Attempt::new(AttachmentOperation::Restore, record.id, record.revision);
        codec::restore(record, attempt)
    }
    /// Immutable persistence data; `attachment.record().revision`.
    pub fn record(&self) -> &CylinderAttachmentRecord {
        &self.record
    }
    /// Ordered validation with its actual coverage modes; `attachment.validation().is_valid()`.
    pub fn validation(&self) -> &ValidationReport {
        &self.validation
    }
    /// Decode fresh owners only when the caller's current revision agrees.
    /// Example: `attachment.export_native(current_revision)?`.
    pub fn export_native(&self, expected: SupportRevision) -> Checked<DetachedCylinderSupport> {
        let attempt = Attempt::new(
            AttachmentOperation::Export,
            self.record.id,
            self.record.revision,
        );
        attempt.revision(expected)?;
        codec::decode(&attempt, &self.record.brep)
    }
    /// Follow explicit native history; missing or multiple face images refuse reattachment.
    /// Example: `attachment.reattach(&source, &changed_body, revision, &history)?`.
    pub fn reattach(
        &self,
        source: &DetachedCylinderSupport,
        body: &Shape,
        revision: SupportRevision,
        history: &History,
    ) -> Checked<Self> {
        reattach::apply(self, source, body, revision, history)
    }
}
impl std::fmt::Debug for CylinderAttachmentRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CylinderAttachmentRecord")
            .field("id", &self.id)
            .field("revision", &self.revision)
            .field("resolution", &self.resolution)
            .field("encoded_bytes", &self.brep.len())
            .finish()
    }
}
#[cfg(test)]
mod tests;

#[cfg(test)]
mod refusal_tests;

#[cfg(test)]
mod boolean_history_tests;
