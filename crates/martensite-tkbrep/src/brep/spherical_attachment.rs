// SPDX-License-Identifier: GPL-2.0-only
//! Source-bound original native full-period spherical band without pole coedges.
use super::{
    cylindrical_split::{CylindricalFaceError, capture::sphere},
    face_input_capture::{FaceCaptureLimits, capture_spherical_input},
    planar_attachment::{FaceAttachmentError, capture_failure, member},
};
use crate::topods::{Face, Solid};
use martensite_tkmath::predicates::roots::metric_bounds::spherical_band::{
    SphericalBand, SphericalBandError,
};
use std::{error::Error, fmt};

pub use super::planar_attachment::AttachmentSource;

/// Immutable proof for the selected original native spherical band.
#[derive(Clone, Debug)]
pub struct SphericalFaceDomain {
    source: AttachmentSource,
    band: SphericalBand,
    trace_limit: f64,
}

impl SphericalFaceDomain {
    /// Named source from the original capture. Example: `domain.source()`.
    pub fn source(&self) -> AttachmentSource {
        self.source
    }

    /// Literal closed original latitude bounds. Example: `domain.latitude()`.
    pub fn latitude(&self) -> [f64; 2] {
        self.band.latitude()
    }

    /// Achieved native L1 trim trace and original requested cap.
    /// Example: `let [achieved, requested] = domain.trace()`.
    pub fn trace(&self) -> [f64; 2] {
        [self.band.trace_bound(), self.trace_limit]
    }
}

/// A native spherical face could not be admitted with coherent complete trace evidence.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum SphericalFaceError {
    /// The source occurrence, stamp or bounded original capture was refused.
    Source(FaceAttachmentError),
    /// The actual directed native rectangle and coedges could not be decoded.
    Rectangle(Box<CylindricalFaceError>),
    /// The stored spherical proof or query did not complete.
    Numeric(SphericalBandError),
    /// The operation was cooperatively cancelled.
    Cancelled,
}

impl fmt::Display for SphericalFaceError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source(cause) => cause.fmt(out),
            Self::Rectangle(cause) => cause.fmt(out),
            Self::Numeric(cause) => cause.fmt(out),
            Self::Cancelled => out.write_str("spherical face capture cancelled"),
        }
    }
}

impl Error for SphericalFaceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Source(cause) => Some(cause),
            Self::Rectangle(cause) => Some(cause.as_ref()),
            Self::Numeric(cause) => Some(cause),
            Self::Cancelled => None,
        }
    }
}

/// Bind the uniquely selected native full non-pole band under the host's source transaction.
/// Example: `capture_spherical_band(&body, &face, source, limits, 1e-8, || false)?`.
pub fn capture_spherical_band(
    body: &Solid,
    face: &Face,
    source: AttachmentSource,
    limits: FaceCaptureLimits,
    trace_limit: f64,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<SphericalFaceDomain, SphericalFaceError> {
    poll(&mut cancelled)?;
    if !trace_limit.is_finite() || trace_limit < 0. {
        return Err(SphericalFaceError::Numeric(SphericalBandError::Limit(
            trace_limit,
        )));
    }
    member(body, face, limits.owners, &mut cancelled).map_err(source_failure)?;
    let observed = capture_spherical_input(face, limits, &mut cancelled)
        .map_err(capture_failure)
        .map_err(source_failure)?;
    let copied = sphere::decode(&observed, &mut cancelled).map_err(rectangle_failure)?;
    let band =
        SphericalBand::capture(copied, trace_limit, &mut cancelled).map_err(numeric_failure)?;
    member(body, face, limits.owners, &mut cancelled).map_err(source_failure)?;
    poll(&mut cancelled)?;
    Ok(SphericalFaceDomain {
        source,
        band,
        trace_limit,
    })
}

fn poll(cancelled: &mut impl FnMut() -> bool) -> Result<(), SphericalFaceError> {
    if cancelled() {
        return Err(SphericalFaceError::Cancelled);
    }
    Ok(())
}

fn source_failure(cause: FaceAttachmentError) -> SphericalFaceError {
    match cause {
        FaceAttachmentError::Cancelled => SphericalFaceError::Cancelled,
        cause => SphericalFaceError::Source(cause),
    }
}

fn rectangle_failure(cause: CylindricalFaceError) -> SphericalFaceError {
    match cause {
        CylindricalFaceError::Cancelled => SphericalFaceError::Cancelled,
        CylindricalFaceError::Source(cause) => source_failure(cause),
        cause => SphericalFaceError::Rectangle(Box::new(cause)),
    }
}

fn numeric_failure(cause: SphericalBandError) -> SphericalFaceError {
    match cause {
        SphericalBandError::Cancelled => SphericalFaceError::Cancelled,
        cause => SphericalFaceError::Numeric(cause),
    }
}
