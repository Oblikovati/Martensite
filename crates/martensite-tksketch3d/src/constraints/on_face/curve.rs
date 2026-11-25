// SPDX-License-Identifier: GPL-2.0-only
//! Source-qualified replay of one straight planar Curve On Face recipe.
use super::{PlanarFaceBinding, PlanarFaceRevision};
use martensite_tkbrep::brep::planar_attachment::{
    DetachedPlanarCurve, FaceAttachmentCurve, FaceAttachmentCurveError, PlanarCurveCertificate,
};
use std::{error::Error, fmt};

/// Persisted source and directed chart controls for one straight Curve On Face.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanarFaceCurveRecipe {
    /// Exact captured host face and input reservation.
    pub source: PlanarFaceRevision,
    /// Authored endpoints in the ordered original UV chart.
    pub uv: [[f64; 2]; 2],
    /// Inclusive world distance and effective native tolerance limit.
    pub distance_limit: f64,
}

/// Immutable, certified native materialization of a straight face curve recipe.
pub struct PlanarFaceCurve {
    recipe: PlanarFaceCurveRecipe,
    native: FaceAttachmentCurve,
}

impl PlanarFaceCurve {
    /// Read the persisted controls and exact source. Example: `curve.recipe()`.
    pub fn recipe(&self) -> PlanarFaceCurveRecipe {
        self.recipe
    }

    /// Read the whole-interval measured result. Example: `curve.certificate().distance_bound`.
    pub fn certificate(&self) -> PlanarCurveCertificate {
        self.native.certificate()
    }

    /// Read the largest effective native tolerance. Example: `curve.tolerance()`.
    pub fn tolerance(&self) -> f64 {
        self.native.tolerance()
    }

    /// Export independently owned native edge and pcurve support. Example: `curve.to_native()?.edge`.
    pub fn to_native(&self) -> Result<DetachedPlanarCurve, PlanarFaceCurveFailure> {
        self.native
            .to_native()
            .map_err(PlanarFaceCurveFailure::Native)
    }
}

/// Why the straight face curve recipe could not be adopted.
#[derive(Debug)]
pub enum PlanarFaceCurveFailure {
    /// Named face or host input reservation changed.
    StaleSource {
        /// Recipe's original source reservation.
        expected: PlanarFaceRevision,
        /// Actual current reservation.
        actual: PlanarFaceRevision,
    },
    /// Operation cancelled before publication.
    Cancelled,
    /// Native source, trim proof, or paired BRep representation refused.
    Native(FaceAttachmentCurveError),
}

impl fmt::Display for PlanarFaceCurveFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StaleSource { expected, actual } => write!(
                output,
                "straight face curve source {actual:?} differs from {expected:?}"
            ),
            Self::Cancelled => output.write_str("straight face curve was cancelled"),
            Self::Native(cause) => cause.fmt(output),
        }
    }
}

impl Error for PlanarFaceCurveFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Native(cause) => Some(cause),
            _ => None,
        }
    }
}

impl PlanarFaceBinding {
    /// Author or replay one directed straight UV span against the actual current source.
    /// Example: `binding.replay_straight_curve(recipe, || current_source, || false)`.
    pub fn replay_straight_curve(
        &self,
        recipe: PlanarFaceCurveRecipe,
        mut current_source: impl FnMut() -> PlanarFaceRevision,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<PlanarFaceCurve, PlanarFaceCurveFailure> {
        use PlanarFaceCurveFailure::Cancelled;
        if cancelled() {
            return Err(Cancelled);
        }
        require_source(recipe.source, self.revision)?;
        require_source(recipe.source, current_source())?;
        let native = self
            .proof
            .build_straight_curve(
                recipe.source.face,
                recipe.uv,
                recipe.distance_limit,
                &mut cancelled,
            )
            .map_err(native_failure)?;
        if cancelled() {
            return Err(Cancelled);
        }
        require_source(recipe.source, current_source())?;
        Ok(PlanarFaceCurve { recipe, native })
    }
}

fn require_source(
    expected: PlanarFaceRevision,
    actual: PlanarFaceRevision,
) -> Result<(), PlanarFaceCurveFailure> {
    if expected != actual {
        return Err(PlanarFaceCurveFailure::StaleSource { expected, actual });
    }
    Ok(())
}

fn native_failure(cause: FaceAttachmentCurveError) -> PlanarFaceCurveFailure {
    use martensite_tkbrep::brep::planar_attachment::{
        FaceAttachmentError, PlanarAttachmentError, PlanarCurveProofFailure,
    };
    match cause {
        FaceAttachmentCurveError::Capture(FaceAttachmentError::Cancelled)
        | FaceAttachmentCurveError::Certificate(PlanarCurveProofFailure::Attachment(
            PlanarAttachmentError::Cancelled,
        )) => PlanarFaceCurveFailure::Cancelled,
        cause => PlanarFaceCurveFailure::Native(cause),
    }
}
