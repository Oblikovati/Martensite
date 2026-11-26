// SPDX-License-Identifier: GPL-2.0-only
use super::PlanarFaceRevision;
use crate::{EditFailure, FitSplineId, PointId};
use martensite_tkbrep::brep::planar_attachment::{
    FaceAttachmentCubicError, FaceAttachmentCurveError, FaceAttachmentError, PlanarAttachmentError,
    PlanarAttachmentLocation, PlanarCubicProofFailure,
};
use martensite_tkmath::predicates::roots::CubicFitNumericError;
use std::{error::Error, fmt};

/// Why a fitted Curve On Face could not publish its source-bound spline.
#[derive(Debug)]
#[non_exhaustive]
pub enum PlanarFaceSplineFailure {
    /// The host no longer names the exact original face capture.
    StaleSource {
        /// Recipe's original reservation.
        expected: PlanarFaceRevision,
        /// Observed reservation.
        actual: PlanarFaceRevision,
    },
    /// A declared existing point is missing from the owned snapshot.
    MissingPoint(PointId),
    /// A named existing point differs from its original authored chart lift.
    PointChanged {
        /// Named point identity.
        point: PointId,
        /// Expected nominal world position of the authored UV.
        expected: [f64; 3],
        /// Actual owned point position.
        actual: [f64; 3],
    },
    /// An authored station lies in excluded trim.
    StationTrim {
        /// Declared station identity.
        point: PointId,
        /// Exact original chart classification.
        location: PlanarAttachmentLocation,
    },
    /// The named normal fit is missing from the immutable snapshot.
    MissingSpline(FitSplineId),
    /// The named fit has a different station identity, parameter, or policy recipe.
    SplineRecipe(FitSplineId),
    /// The ordinary immutable sketch edit refused the candidate.
    Edit(EditFailure),
    /// The original fit operator could not materialize the same UV station controls.
    FitNumeric(CubicFitNumericError),
    /// The original native face capture or station lift refused the operation.
    Capture(FaceAttachmentError),
    /// Original trim, pair, or native builder evidence could not complete.
    Native(FaceAttachmentCubicError),
    /// The operation observed local cancellation.
    Cancelled,
}

impl fmt::Display for PlanarFaceSplineFailure {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StaleSource { expected, actual } => write!(
                out,
                "face spline source {actual:?} differs from {expected:?}"
            ),
            Self::MissingPoint(point) => write!(
                out,
                "face spline station {point:?} requires an owned sketch point"
            ),
            Self::PointChanged {
                point,
                expected,
                actual,
            } => write!(
                out,
                "face spline station {point:?} at {actual:?} requires its original chart lift {expected:?}"
            ),
            Self::StationTrim { point, location } => write!(
                out,
                "face spline station {point:?} lies at {location:?}; expected retained material or rim"
            ),
            Self::MissingSpline(id) => {
                write!(out, "face spline {id:?} requires an owned fitted cubic")
            }
            Self::SplineRecipe(id) => write!(
                out,
                "face spline {id:?} differs from its original directed fit recipe"
            ),
            Self::Edit(cause) => cause.fmt(out),
            Self::FitNumeric(cause) => cause.fmt(out),
            Self::Capture(cause) => cause.fmt(out),
            Self::Native(cause) => cause.fmt(out),
            Self::Cancelled => out.write_str("fitted Curve On Face was cancelled"),
        }
    }
}

impl Error for PlanarFaceSplineFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Edit(cause) => Some(cause),
            Self::FitNumeric(cause) => Some(cause),
            Self::Capture(cause) => Some(cause),
            Self::Native(cause) => Some(cause),
            _ => None,
        }
    }
}

pub(super) fn require_source(
    expected: PlanarFaceRevision,
    actual: PlanarFaceRevision,
) -> Result<(), PlanarFaceSplineFailure> {
    if expected != actual {
        return Err(PlanarFaceSplineFailure::StaleSource { expected, actual });
    }
    Ok(())
}

pub(super) fn poll(cancelled: &mut impl FnMut() -> bool) -> Result<(), PlanarFaceSplineFailure> {
    if cancelled() {
        return Err(PlanarFaceSplineFailure::Cancelled);
    }
    Ok(())
}

pub(super) fn edit_failure(cause: EditFailure) -> PlanarFaceSplineFailure {
    match cause {
        EditFailure::Cancelled(_) => PlanarFaceSplineFailure::Cancelled,
        cause => PlanarFaceSplineFailure::Edit(cause),
    }
}

pub(super) fn capture_failure(cause: FaceAttachmentError) -> PlanarFaceSplineFailure {
    match cause {
        FaceAttachmentError::Cancelled => PlanarFaceSplineFailure::Cancelled,
        cause => PlanarFaceSplineFailure::Capture(cause),
    }
}

pub(super) fn native_failure(cause: FaceAttachmentCubicError) -> PlanarFaceSplineFailure {
    match cause {
        FaceAttachmentCubicError::Native(FaceAttachmentCurveError::Capture(
            FaceAttachmentError::Cancelled,
        ))
        | FaceAttachmentCubicError::Proof(PlanarCubicProofFailure::Attachment(
            PlanarAttachmentError::Cancelled,
        )) => PlanarFaceSplineFailure::Cancelled,
        cause => PlanarFaceSplineFailure::Native(cause),
    }
}
