//! Keep the cylinder's existing failure contract and identify cone-specific admission failures.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::ConicalDevelopmentFailure;
use std::fmt;

/// A conical attachment or chart refused to publish.
#[derive(Debug)]
#[non_exhaustive]
pub enum ConeAttachmentFailure {
    /// Shared attachment diagnostics, including exact membership and revision failures.
    Attachment(Box<AttachmentFailure>),
    /// The selected face is not conical.
    Surface {
        /// Failed operation.
        operation: AttachmentOperation,
        /// Caller-owned face identity.
        attachment: FaceAttachmentId,
        /// Revision associated with the attempted selection.
        revision: SupportRevision,
        /// Actual native surface classification.
        actual: SurfaceType,
    },
    /// The native cone chart or supplied parameters could not be developed.
    Development {
        /// Caller-owned selected face.
        attachment: FaceAttachmentId,
        /// Frozen support revision used for the computation.
        revision: SupportRevision,
        /// Failed metric constraint.
        cause: Box<ConicalDevelopmentFailure>,
    },
}

impl From<AttachmentFailure> for ConeAttachmentFailure {
    fn from(cause: AttachmentFailure) -> Self {
        Self::Attachment(Box::new(cause))
    }
}
impl fmt::Display for ConeAttachmentFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Attachment(cause) => cause.fmt(f),
            Self::Surface {
                operation,
                attachment,
                revision,
                actual,
            } => write!(
                f,
                "{operation:?} attachment={attachment:?} revision={revision:?}: surface={actual:?}, expected an analytic cone"
            ),
            Self::Development {
                attachment,
                revision,
                cause,
            } => write!(
                f,
                "Develop attachment={attachment:?} revision={revision:?}: {cause}"
            ),
        }
    }
}
impl std::error::Error for ConeAttachmentFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Attachment(cause) => Some(cause.as_ref()),
            Self::Surface { .. } => None,
            Self::Development { cause, .. } => Some(cause.as_ref()),
        }
    }
}
