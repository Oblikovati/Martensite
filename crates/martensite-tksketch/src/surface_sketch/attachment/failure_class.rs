//! Stable failure categories supplement the exact owned diagnostic evidence.
use super::{AttachmentFailure, AttachmentFailureCause};

/// Why an attachment operation could not publish its result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttachmentFailureClass {
    /// Invalid settings, wrong selected entity or mismatched source ownership.
    InvalidInput,
    /// The supplied current revision differs from the frozen revision.
    StaleRevision,
    /// Recorded history contains no unique current selected-face image.
    UnresolvedHistory,
    /// Encoding failed or did not replay the frozen graph exactly.
    Persistence,
    /// Required native body checks failed or remained incomplete.
    Validation,
    /// A native geometry or topology query failed.
    Native,
}
impl AttachmentFailure {
    /// Categorize a refusal without discarding its detailed cause.
    /// Example: `let category = failure.class();`.
    pub fn class(&self) -> AttachmentFailureClass {
        use AttachmentFailureCause as Cause;
        match &self.cause {
            Cause::Native { .. } => AttachmentFailureClass::Native,
            Cause::Encoding(_) | Cause::Replay { .. } => AttachmentFailureClass::Persistence,
            Cause::Validation(_) => AttachmentFailureClass::Validation,
            Cause::Revision { .. } => AttachmentFailureClass::StaleRevision,
            Cause::HistoryImages { .. } => AttachmentFailureClass::UnresolvedHistory,
            Cause::Membership { .. }
            | Cause::Orientation { .. }
            | Cause::Surface { .. }
            | Cause::Resolution { .. }
            | Cause::UnchangedRevision
            | Cause::SourceIdentity { .. } => AttachmentFailureClass::InvalidInput,
        }
    }
}

impl AttachmentFailureCause {
    pub(super) fn expected_shape(&self) -> &'static str {
        match self {
            Self::Native { .. } => "a completed native geometry/topology query",
            Self::Encoding(_) => "a complete checked root and selected-face encoding",
            Self::Membership { .. } => "exactly one selected-face use in the support body",
            Self::Orientation { .. } => "the orientation of that body's selected-face use",
            Self::Surface { .. } => "a cylindrical analytic surface",
            Self::Validation(_) => "all required ordered body checks completed and passed",
            Self::Resolution { .. } => "a finite strictly positive resolution value",
            Self::Revision { .. } => "the supplied revision equal to the frozen support revision",
            Self::UnchangedRevision => "a target revision distinct from the frozen source revision",
            Self::SourceIdentity { .. } => "the recorded attachment identity and source revision",
            Self::HistoryImages { .. } => {
                "exactly one distinct current face image from recorded history"
            }
            Self::Replay { .. } => "byte-identical frozen root and selected-face encoding",
        }
    }
}
