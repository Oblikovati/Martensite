//! Attachment failures carry the operation, caller identity, revision and owned cause.
use super::*;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::geom_abs::SurfaceType;
/// Attachment operation that did not complete.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttachmentOperation {
    /// Capture mutable native input into a validated immutable record.
    Capture,
    /// Convert native selected-face parameters into developed coordinates.
    Develop,
    /// Restore and validate a persistence record.
    Restore,
    /// Resolve a revision and decode detached native owners.
    Export,
    /// Resolve a selected face through recorded history.
    Reattach,
}
/// Typed reason no attachment or native export was returned.
#[derive(Debug)]
pub enum AttachmentFailureCause {
    /// Invalid or unrepresentable analytic chart value.
    ChartValue {
        /// Failed chart quantity.
        field: &'static str,
        /// Supplied or computed value.
        value: f64,
    },
    /// Native trim classification did not satisfy the requested authoring policy.
    ChartTrim {
        /// Actual native state, including unresolved states.
        state: martensite_tkg3d::top_abs::State,
    },
    /// Native geometry or topology query failed.
    Native {
        /// Native action that failed.
        step: &'static str,
        /// Original toolkit failure.
        cause: Box<Failure>,
    },
    /// Native serialization or checked decoding did not complete.
    Encoding(Box<super::super::SurfaceFailureCause>),
    /// Selected face was missing or repeated in the support body.
    Membership {
        /// Number of exact identity occurrences.
        occurrences: usize,
    },
    /// Selected orientation disagreed with the actual body's face use.
    Orientation {
        /// Supplied orientation.
        actual: Orientation,
        /// Body face orientation.
        expected: Option<Orientation>,
    },
    /// The selected surface was not cylindrical.
    Surface {
        /// Actual analytic surface classification.
        actual: SurfaceType,
    },
    /// The support body failed or did not complete its ordered checks.
    Validation(Box<ValidationReport>),
    /// Invalid explicit validation setting.
    Resolution {
        /// Rejected field.
        field: &'static str,
        /// Offending value.
        value: f64,
    },
    /// The requested support revision is stale or otherwise mismatched.
    Revision {
        /// Requested current revision.
        expected: SupportRevision,
        /// Frozen revision.
        actual: SupportRevision,
    },
    /// A changed support requires a distinct revision token.
    UnchangedRevision,
    /// Supplied detached owners do not belong to this recorded attachment.
    SourceIdentity {
        /// Actual detached attachment identity.
        actual: FaceAttachmentId,
        /// Actual detached source revision.
        revision: SupportRevision,
    },
    /// History supplied zero or multiple distinct current face images.
    HistoryImages {
        /// Number of exact distinct face images in the current body.
        count: usize,
    },
    /// Geometry or semantic references did not replay byte for byte.
    Replay {
        /// First mismatching byte, or the end of the shorter encoding.
        offset: usize,
        /// Original length.
        expected_length: usize,
        /// Replayed length.
        actual_length: usize,
        /// Original byte at the mismatch.
        expected: Option<u8>,
        /// Replayed byte at the mismatch.
        actual: Option<u8>,
    },
}
/// No partial attachment geometry accompanies this owned diagnostic.
#[derive(Debug)]
pub struct AttachmentFailure {
    /// Failed operation.
    pub operation: AttachmentOperation,
    /// Caller-selected face identity.
    pub attachment: FaceAttachmentId,
    /// Source support revision.
    pub revision: SupportRevision,
    /// Actual typed failure evidence.
    pub cause: AttachmentFailureCause,
}
impl std::fmt::Display for AttachmentFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} attachment={:?} revision={:?}: {:?}; expected {}",
            self.operation,
            self.attachment,
            self.revision,
            self.cause,
            self.cause.expected_shape()
        )
    }
}
impl std::error::Error for AttachmentFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.cause {
            AttachmentFailureCause::Native { cause, .. } => Some(cause.as_ref()),
            _ => None,
        }
    }
}
#[derive(Clone, Copy)]
pub(super) struct Attempt {
    pub operation: AttachmentOperation,
    pub id: FaceAttachmentId,
    pub revision: SupportRevision,
}
impl Attempt {
    pub fn new(
        operation: AttachmentOperation,
        id: FaceAttachmentId,
        revision: SupportRevision,
    ) -> Self {
        Self {
            operation,
            id,
            revision,
        }
    }
    pub fn reject(&self, cause: AttachmentFailureCause) -> AttachmentFailure {
        AttachmentFailure {
            operation: self.operation,
            attachment: self.id,
            revision: self.revision,
            cause,
        }
    }
    pub fn native<T>(&self, step: &'static str, result: Result<T, Failure>) -> Checked<T> {
        result.map_err(|cause| {
            self.reject(AttachmentFailureCause::Native {
                step,
                cause: Box::new(cause),
            })
        })
    }
    pub fn encoding<T>(
        &self,
        result: Result<T, super::super::failure::SurfaceFailureCause>,
    ) -> Checked<T> {
        result.map_err(|cause| self.reject(AttachmentFailureCause::Encoding(Box::new(cause))))
    }
    pub fn revision(&self, expected: SupportRevision) -> Checked<()> {
        if expected != self.revision {
            return Err(self.reject(AttachmentFailureCause::Revision {
                expected,
                actual: self.revision,
            }));
        }
        Ok(())
    }
}
