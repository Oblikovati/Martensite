// SPDX-License-Identifier: GPL-2.0-only
use super::PlanarFaceRevision;
use crate::constraints::ConstraintFailure;
use martensite_tkbrep::brep::planar_attachment::{FaceAttachmentError, PlanarAttachmentLocation};
use std::{error::Error, fmt};

/// Why the attached sketch did not publish a new accepted state.
#[derive(Debug)]
#[non_exhaustive]
pub enum PlanarFaceConstraintFailure {
    /// The ordinary hard sketch or prepared solver refused the state.
    Sketch(Box<ConstraintFailure>),
    /// The source capture or exact chart check could not complete.
    Attachment(Box<FaceAttachmentError>),
    /// The actual world point or actual UV lift is outside the admissible trim.
    Trim(PlanarAttachmentLocation),
    /// The two parameter step scales were not finite and positive.
    InvalidChartScale,
    /// The literal selected boundary has no exact binary64 parameter pair.
    BoundaryParameters([[f64; 2]; 2]),
    /// The host-provided revision is different from the bound one.
    StaleSource {
        /// Source expected by the attachment.
        expected: PlanarFaceRevision,
        /// Actual source version supplied for this update.
        current: PlanarFaceRevision,
    },
    /// Rebinding requires the same named face and a fresh input version.
    RebindSource {
        /// Prior attachment source.
        prior: PlanarFaceRevision,
        /// Proposed replacement source.
        next: PlanarFaceRevision,
    },
    /// Cancellation was observed before publication.
    Cancelled,
}

impl From<ConstraintFailure> for PlanarFaceConstraintFailure {
    fn from(cause: ConstraintFailure) -> Self {
        match cause {
            ConstraintFailure::Cancelled => Self::Cancelled,
            cause => Self::Sketch(Box::new(cause)),
        }
    }
}

impl From<FaceAttachmentError> for PlanarFaceConstraintFailure {
    fn from(cause: FaceAttachmentError) -> Self {
        match cause {
            FaceAttachmentError::Cancelled => Self::Cancelled,
            cause => Self::Attachment(Box::new(cause)),
        }
    }
}

impl fmt::Display for PlanarFaceConstraintFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sketch(cause) => cause.fmt(output),
            Self::Attachment(cause) => cause.fmt(output),
            Self::Trim(class) => write!(
                output,
                "attached point is {class:?}, expected the closed face trim"
            ),
            Self::InvalidChartScale => {
                output.write_str("attached point requires two finite positive chart scales")
            }
            Self::BoundaryParameters(uv) => write!(
                output,
                "closed attachment boundary {uv:?} requires two exact solver coordinates"
            ),
            Self::StaleSource { expected, current } => write!(
                output,
                "attached source {current:?} must match {expected:?}"
            ),
            Self::RebindSource { prior, next } => write!(
                output,
                "attachment source {next:?} must be a later input of {prior:?}"
            ),
            Self::Cancelled => output.write_str("point attachment was cancelled"),
        }
    }
}

impl Error for PlanarFaceConstraintFailure {}
