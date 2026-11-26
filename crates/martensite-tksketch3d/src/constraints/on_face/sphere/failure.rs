// SPDX-License-Identifier: GPL-2.0-only
use super::FaceAttachmentRevision;
use crate::constraints::ConstraintFailure;
use martensite_tkbrep::brep::spherical_attachment::{SphericalBandPlace, SphericalFaceError};
use std::{error::Error, fmt};

/// Typed failure of the native periodic point relation or its prepared solve.
#[derive(Debug)]
#[non_exhaustive]
pub enum SphericalFaceFailure {
    /// The ordinary hard sketch refused the current or candidate state.
    Sketch(Box<ConstraintFailure>),
    /// Native capture, periodic branch, or bounded point qualification failed.
    Attachment(Box<SphericalFaceError>),
    /// The retained world point or UV lies outside the closed latitude trim.
    Trim(SphericalBandPlace),
    /// The original chart does not provide two finite positive parameter step scales.
    InvalidChartScale,
    /// Whole-band normalized tangent area does not clear the prepared solve's rank guard.
    SingularChart {
        /// Certified lower bound of tangent area divided by radius squared.
        lower: f64,
        /// Required strict lower bound.
        limit: f64,
    },
    /// The host's actual input no longer matches the bound captured face.
    StaleSource {
        /// Original expected input.
        expected: FaceAttachmentRevision,
        /// Currently active input.
        current: FaceAttachmentRevision,
    },
    /// Rebinding requires the same face incarnation and a new input revision and reservation.
    RebindSource {
        /// Prior accepted input.
        prior: FaceAttachmentRevision,
        /// Proposed replacement input.
        next: FaceAttachmentRevision,
    },
    /// Cooperative cancellation prevented publication.
    Cancelled,
}

impl fmt::Display for SphericalFaceFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sketch(cause) => cause.fmt(output),
            Self::Attachment(cause) => cause.fmt(output),
            Self::Trim(place) => write!(
                output,
                "attached point lies {place:?}, expected the closed native rims"
            ),
            Self::InvalidChartScale => output.write_str(
                "spherical point requires finite positive longitude and latitude scales",
            ),
            Self::SingularChart { lower, limit } => write!(
                output,
                "normalized tangent area {lower} must exceed {limit}"
            ),
            Self::StaleSource { expected, current } => {
                write!(output, "source {current:?} must match {expected:?}")
            }
            Self::RebindSource { prior, next } => write!(
                output,
                "source {next:?} must be a fresh revision of {prior:?}"
            ),
            Self::Cancelled => output.write_str("spherical point attachment was cancelled"),
        }
    }
}

impl Error for SphericalFaceFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Sketch(cause) => Some(cause.as_ref()),
            Self::Attachment(cause) => Some(cause.as_ref()),
            _ => None,
        }
    }
}

impl From<ConstraintFailure> for SphericalFaceFailure {
    fn from(cause: ConstraintFailure) -> Self {
        match cause {
            ConstraintFailure::Cancelled => Self::Cancelled,
            cause => Self::Sketch(Box::new(cause)),
        }
    }
}
impl From<SphericalFaceError> for SphericalFaceFailure {
    fn from(cause: SphericalFaceError) -> Self {
        match cause {
            SphericalFaceError::Cancelled => Self::Cancelled,
            cause => Self::Attachment(Box::new(cause)),
        }
    }
}
