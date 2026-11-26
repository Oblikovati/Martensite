// SPDX-License-Identifier: GPL-2.0-only
//! Bind one checked planar cubic to the actual detached 3D curve and pcurve.
use super::{
    AttachmentSource, DetachedPlanarCurve, FaceAttachment, FaceAttachmentCurveError,
    PlanarCubicCertificate, PlanarCubicOptions, PlanarCubicProofFailure, PlanarSupport,
    curve::native, poll,
};
use std::{error::Error, fmt};

/// Source-bound native cubic evidence retained as immutable values.
#[derive(Clone, Copy, Debug)]
pub struct FaceAttachmentCubic {
    source: AttachmentSource,
    support: PlanarSupport,
    world: [[f64; 3]; 4],
    uv: [[f64; 2]; 4],
    certificate: PlanarCubicCertificate,
    tolerance: f64,
}

impl FaceAttachmentCubic {
    /// Read the original captured source. Example: `curve.source()`.
    pub fn source(&self) -> AttachmentSource {
        self.source
    }

    /// Read the complete paired path proof. Example: `curve.certificate()`.
    pub fn certificate(&self) -> PlanarCubicCertificate {
        self.certificate
    }

    /// Read the largest effective native edge and vertex tolerance.
    /// Example: `curve.tolerance()`.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Export a fresh independently owned native cubic edge and its pcurve support.
    /// Example: `curve.to_native()?.edge`.
    pub fn to_native(&self) -> Result<DetachedPlanarCurve, FaceAttachmentCubicError> {
        native::build(
            self.support,
            (self.world, self.uv),
            self.certificate.distance_bound,
            self.certificate.options.distance_limit,
        )
        .map(|(detached, _)| detached)
        .map_err(FaceAttachmentCubicError::Native)
    }
}

/// Why an actual planar cubic pair could not be adopted.
#[derive(Debug)]
#[non_exhaustive]
pub enum FaceAttachmentCubicError {
    /// The native proof, source or operation-local representations were refused.
    Native(FaceAttachmentCurveError),
    /// Whole-span proof could not certify the actual pair.
    Proof(PlanarCubicProofFailure),
}

impl fmt::Display for FaceAttachmentCubicError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Native(cause) => cause.fmt(out),
            Self::Proof(cause) => cause.fmt(out),
        }
    }
}

impl Error for FaceAttachmentCubicError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Native(cause) => Some(cause),
            Self::Proof(cause) => Some(cause),
        }
    }
}

impl FaceAttachment {
    /// Prove and bind the paired retained cubic nets on this selected native planar face.
    /// Example: `face.build_cubic_curve(source, world_poles, uv_poles, options, || false)?`.
    pub fn build_cubic_curve(
        &self,
        source: AttachmentSource,
        world: [[f64; 3]; 4],
        uv: [[f64; 2]; 4],
        options: PlanarCubicOptions,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<FaceAttachmentCubic, FaceAttachmentCubicError> {
        use FaceAttachmentCubicError::{Native, Proof};
        let capture = |cause| Native(FaceAttachmentCurveError::Capture(cause));
        poll(&mut cancelled).map_err(capture)?;
        self.require_source(source).map_err(capture)?;
        let certificate = self
            .proof
            .certify_cubic_curve(world, uv, options, &mut cancelled)
            .map_err(Proof)?;
        let (_, tolerance) = native::build(
            self.support,
            (world, uv),
            certificate.distance_bound,
            options.distance_limit,
        )
        .map_err(Native)?;
        poll(&mut cancelled).map_err(capture)?;
        Ok(FaceAttachmentCubic {
            source,
            support: self.support,
            world,
            uv,
            certificate,
            tolerance,
        })
    }
}
