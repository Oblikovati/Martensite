// SPDX-License-Identifier: GPL-2.0-only
//! Certified line contact with a selected finite, full-period native cylindrical wall.
mod capture;
mod point;

pub use super::planar_attachment::AttachmentSource;
use super::{
    face_input_capture::{FaceCaptureLimits, capture_cylindrical_input},
    planar_attachment::{FaceAttachmentError, capture_failure, member},
};
use crate::topods::{Face, Solid};
use martensite_tkmath::predicates::roots::metric_bounds::segment_cylinder::SegmentCylinderWall;
pub use martensite_tkmath::predicates::roots::metric_bounds::segment_cylinder::{
    CylinderContactSense, CylinderLinePlace, CylinderPointChart, CylinderPointError,
    CylinderPointQuery, CylinderSegmentContact, CylinderSegmentError, CylinderSegmentIntersection,
    CylinderWallPlace, SegmentCylinderCut,
};
use std::{error::Error, fmt};

/// A native wall capture or exact directed contact could not complete.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum CylindricalFaceError {
    /// The supplied occurrence, source revision, or coherent native capture was refused.
    Source(FaceAttachmentError),
    /// An actual native chart, edge or trim is outside the certified full-wall scope.
    UnsupportedTrim {
        /// Invocation-local owner of the refused native occurrence.
        owner: usize,
        /// Required representation or incidence.
        required: &'static str,
    },
    /// The trace limit is outside the finite nonnegative spatial domain.
    TraceLimit(f64),
    /// The actual stored trims exceeded the requested model-space bound.
    TraceError {
        /// Achieved model-space L1 error upper bound.
        upper_bound: f64,
        /// Requested maximum model-space L1 error.
        limit: f64,
    },
    /// The exact root, placement, or position evidence was incomplete.
    Numeric(CylinderSegmentError),
    /// A point lift or its certified distance could not be admitted.
    Point(CylinderPointError),
    /// The capture or query was cooperatively cancelled.
    Cancelled,
}

impl fmt::Display for CylindricalFaceError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source(cause) => cause.fmt(out),
            Self::UnsupportedTrim { owner, required } => {
                write!(out, "cylindrical face owner {owner} requires {required}")
            }
            Self::TraceLimit(limit) => {
                write!(
                    out,
                    "cylindrical face trace limit {limit} must be finite and nonnegative"
                )
            }
            Self::TraceError { upper_bound, limit } => {
                write!(out, "cylindrical face trace {upper_bound} exceeds {limit}")
            }
            Self::Numeric(cause) => cause.fmt(out),
            Self::Point(cause) => cause.fmt(out),
            Self::Cancelled => out.write_str("cylindrical face query cancelled"),
        }
    }
}

impl Error for CylindricalFaceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Source(cause) => Some(cause),
            Self::Numeric(cause) => Some(cause),
            Self::Point(cause) => Some(cause),
            _ => None,
        }
    }
}

/// Immutable exact support and trim evidence for the selected native occurrence.
#[derive(Clone, Debug)]
pub struct CylindricalSplitDomain {
    source: AttachmentSource,
    proof: SegmentCylinderWall,
    trace: CylindricalTrimTrace,
}

/// Spatial model-space bound for the retained native rims, paired seam and any hole edges.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CylindricalTrimTrace {
    /// Original requested L1 error limit.
    pub requested_limit: f64,
    /// Achieved outward L1 bound over the certified trim representations and joins.
    pub upper_bound: f64,
}

impl CylindricalSplitDomain {
    /// Read the exact source stamp captured under the host transaction.
    /// Example: `let current = domain.source();`.
    pub fn source(&self) -> AttachmentSource {
        self.source
    }

    /// Read the actual retained trim metric and its separately requested limit.
    /// Example: `domain.trim_trace().upper_bound`.
    pub fn trim_trace(&self) -> CylindricalTrimTrace {
        self.trace
    }

    /// Classify a directed segment in the frozen closed wall trim.
    /// Example: `domain.intersect(current, endpoints, || false)?`.
    pub fn intersect(
        &self,
        current: AttachmentSource,
        endpoints: [[f64; 3]; 2],
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<CylinderSegmentIntersection, CylindricalFaceError> {
        poll(&mut cancelled)?;
        self.require_current(current)?;
        self.proof
            .intersect(endpoints, cancelled)
            .map_err(numeric_failure)
    }

    fn require_current(&self, current: AttachmentSource) -> Result<(), CylindricalFaceError> {
        if current != self.source {
            return Err(CylindricalFaceError::Source(
                FaceAttachmentError::StaleSource {
                    captured: self.source,
                    current,
                },
            ));
        }
        Ok(())
    }
}

/// Capture a uniquely selected full-period native cylindrical wall from a solid.
/// The host keeps the supplied topology occurrence stable throughout capture and provides its stamp.
/// The supported outer trim is one paired seam and two complete circular rims with an exact
/// rectangular UV domain. One strictly interior genuine rectangular UV hole is admitted.
/// The trace limit bounds stored geometry independently of the incidence decisions.
/// Example: `capture_cylindrical_split_domain(&body, &face, source, limits, 1e-9, || false)?`.
pub fn capture_cylindrical_split_domain(
    body: &Solid,
    face: &Face,
    source: AttachmentSource,
    limits: FaceCaptureLimits,
    max_trace: f64,
    mut cancelled: impl FnMut() -> bool,
) -> Result<CylindricalSplitDomain, CylindricalFaceError> {
    poll(&mut cancelled)?;
    if !max_trace.is_finite() || max_trace < 0. {
        return Err(CylindricalFaceError::TraceLimit(max_trace));
    }
    member(body, face, limits.owners, &mut cancelled).map_err(source_failure)?;
    let captured = capture_cylindrical_input(face, limits, &mut cancelled)
        .map_err(capture_failure)
        .map_err(source_failure)?;
    let (proof, upper_bound) = capture::prove(&captured, max_trace, &mut cancelled)?;
    member(body, face, limits.owners, &mut cancelled).map_err(source_failure)?;
    poll(&mut cancelled)?;
    Ok(CylindricalSplitDomain {
        source,
        proof,
        trace: CylindricalTrimTrace {
            requested_limit: max_trace,
            upper_bound,
        },
    })
}

fn poll(cancelled: &mut impl FnMut() -> bool) -> Result<(), CylindricalFaceError> {
    if cancelled() {
        return Err(CylindricalFaceError::Cancelled);
    }
    Ok(())
}

fn source_failure(cause: FaceAttachmentError) -> CylindricalFaceError {
    match cause {
        FaceAttachmentError::Cancelled => CylindricalFaceError::Cancelled,
        cause => CylindricalFaceError::Source(cause),
    }
}

fn numeric_failure(cause: CylinderSegmentError) -> CylindricalFaceError {
    match cause {
        CylinderSegmentError::Cancelled => CylindricalFaceError::Cancelled,
        cause => CylindricalFaceError::Numeric(cause),
    }
}
