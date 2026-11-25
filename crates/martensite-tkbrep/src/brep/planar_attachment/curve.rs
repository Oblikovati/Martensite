// SPDX-License-Identifier: GPL-2.0-only
//! Freeze the paired degree-one BRep representations on a detached original chart.
mod native;

use super::{
    AttachmentSource, FaceAttachment, FaceAttachmentError, PlanarCurveCertificate,
    PlanarCurveProofFailure, numeric_failure, poll,
};
use crate::topods::{Edge, Face};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Axis3, Transform3};
use std::{error::Error, fmt};

#[derive(Clone, Copy, Debug)]
pub(super) struct PlanarSupport {
    pub(super) frame: Axis3,
    pub(super) position: Transform3,
}

/// An independently owned native edge and the located support of its stored pcurve.
pub struct DetachedPlanarCurve {
    /// Degree-one 3D and 2D Bézier representations use the same [0, 1] parameter.
    pub edge: Edge,
    /// An untrimmed, isolated copy of the accepted original support chart.
    pub support: Face,
}

/// A completed source-bound straight curve and its actual retained representations.
pub struct FaceAttachmentCurve {
    source: AttachmentSource,
    support: PlanarSupport,
    native: DetachedPlanarCurve,
    certificate: PlanarCurveCertificate,
    tolerance: f64,
    limit: f64,
}

impl FaceAttachmentCurve {
    /// Read the named original source. Example: `curve.source()`.
    pub fn source(&self) -> AttachmentSource {
        self.source
    }

    /// Read proof of both affine paths and the achieved discrepancy. Example: `curve.certificate()`.
    pub fn certificate(&self) -> PlanarCurveCertificate {
        self.certificate
    }

    /// Read the largest effective native edge/vertex tolerance. Example: `curve.tolerance()`.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Export fresh native ownership whose edits cannot affect the retained result.
    /// Example: `let edge = curve.to_native()?.edge;`.
    pub fn to_native(&self) -> Result<DetachedPlanarCurve, FaceAttachmentCurveError> {
        let controls = self.native.controls()?;
        native::build(
            self.support,
            controls,
            self.certificate.distance_bound,
            self.limit,
        )
        .map(|(native, _)| native)
    }
}

/// Why a planar face curve could not adopt its native representation.
#[derive(Debug)]
pub enum FaceAttachmentCurveError {
    /// The captured source or nominal chart could not be used.
    Capture(FaceAttachmentError),
    /// The mathematical whole-interval proof refused one paired affine path.
    Certificate(PlanarCurveProofFailure),
    /// An operation-local BRep builder or reader failed.
    Native(Box<Failure>),
    /// Retained parameter range, controls, placement, or topology differed from the proof.
    Representation,
    /// Effective native edge or vertex tolerance exceeds the requested acceptance limit.
    Tolerance {
        /// Largest native tolerance found on the result.
        effective: f64,
        /// Caller-supplied inclusive model limit.
        limit: f64,
    },
}

impl fmt::Display for FaceAttachmentCurveError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Capture(cause) => cause.fmt(output),
            Self::Certificate(cause) => cause.fmt(output),
            Self::Native(cause) => write!(output, "planar face curve BRep failure: {cause}"),
            Self::Representation => {
                output.write_str("retained planar curve differs from the certified pair")
            }
            Self::Tolerance { effective, limit } => write!(
                output,
                "planar face curve effective tolerance {effective} exceeds {limit}"
            ),
        }
    }
}

impl Error for FaceAttachmentCurveError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Capture(cause) => Some(cause),
            Self::Certificate(cause) => Some(cause),
            Self::Native(cause) => Some(cause.as_ref()),
            _ => None,
        }
    }
}

impl From<FaceAttachmentError> for FaceAttachmentCurveError {
    fn from(cause: FaceAttachmentError) -> Self {
        Self::Capture(cause)
    }
}

impl From<Failure> for FaceAttachmentCurveError {
    fn from(cause: Failure) -> Self {
        Self::Native(Box::new(cause))
    }
}

impl FaceAttachment {
    /// Build a straight curve from ordered original UV controls on this actual trimmed source.
    /// Example: `proof.build_straight_curve(source, [[-2., 1.], [2., 1.]], 1e-6, || false)`.
    pub fn build_straight_curve(
        &self,
        source: AttachmentSource,
        uv: [[f64; 2]; 2],
        limit: f64,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<FaceAttachmentCurve, FaceAttachmentCurveError> {
        poll(&mut cancelled)?;
        self.require_source(source)?;
        let world = self.lift_curve_controls(uv)?;
        let certificate = self
            .proof
            .certify_straight_curve(world, uv, limit, &mut cancelled)
            .map_err(FaceAttachmentCurveError::Certificate)?;
        let (native, tolerance) =
            native::build(self.support, (world, uv), certificate.distance_bound, limit)?;
        poll(&mut cancelled)?;
        Ok(FaceAttachmentCurve {
            source,
            support: self.support,
            native,
            certificate,
            tolerance,
            limit,
        })
    }

    fn lift_curve_controls(
        &self,
        [first, last]: [[f64; 2]; 2],
    ) -> Result<[[f64; 3]; 2], FaceAttachmentError> {
        Ok([
            self.proof
                .solver_chart(first)
                .map_err(numeric_failure)?
                .world,
            self.proof
                .solver_chart(last)
                .map_err(numeric_failure)?
                .world,
        ])
    }
}
