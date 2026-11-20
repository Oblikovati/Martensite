//! Keep source owner, depth row and the original typed error at the attachment boundary.
use super::*;

/// No trim-qualified cone tool accompanies this refusal.
#[derive(Debug)]
#[non_exhaustive]
pub enum ConeFootprintFailure {
    /// The selected conical face or current revision did not match.
    Attachment(Box<ConeAttachmentFailure>),
    /// The solved request does not use the selected chart's extrusion frame.
    SupportMismatch {
        /// Caller-supplied cone and chart.
        actual: Box<ConeSupport>,
        /// Support estimated from this frozen face and seam.
        expected: Box<ConeSupport>,
    },
    /// Required native trim structure could not be established.
    UnsupportedTrim {
        /// Required form of the native trim or occurrence.
        expected: &'static str,
    },
    /// Native trim capture failed.
    Native(Box<Failure>),
    /// A requested region has no matching authored or fitted result.
    MissingRegion(SurfaceRegionId),
    /// A whole-region enclosure touches a bounded interior support trim.
    InnerTrimOverlap {
        /// Owning region of the authored or fitted footprint.
        region: SurfaceRegionId,
        /// One-based inner wire ordinal local to this frozen support only.
        wire: usize,
        /// Caller depth row when checking a retained fitted cap.
        depth_row: Option<usize>,
    },
    /// The source or one retained cap enclosure could not be proved inside the trim.
    Metric {
        /// Authored region, when available.
        region: Option<SurfaceRegionId>,
        /// First or last depth index when checking a retained cap.
        depth_row: Option<usize>,
        /// Original numerical refusal.
        cause: Box<ConicalDevelopmentFailure>,
    },
    /// The underlying extrusion failed or was cancelled.
    Extrusion(Box<WrappedNormalFailure>),
}

impl From<ConeAttachmentFailure> for ConeFootprintFailure {
    fn from(cause: ConeAttachmentFailure) -> Self {
        Self::Attachment(Box::new(cause))
    }
}
impl From<WrappedNormalFailure> for ConeFootprintFailure {
    fn from(cause: WrappedNormalFailure) -> Self {
        Self::Extrusion(Box::new(cause))
    }
}
impl From<band::BandFailure> for ConeFootprintFailure {
    fn from(cause: band::BandFailure) -> Self {
        match cause {
            band::BandFailure::Native(cause) => Self::Native(cause),
            band::BandFailure::Unsupported(expected) => Self::UnsupportedTrim { expected },
        }
    }
}

pub(super) fn metric(
    region: Option<SurfaceRegionId>,
    depth_row: Option<usize>,
    cause: ConicalDevelopmentFailure,
) -> ConeFootprintFailure {
    ConeFootprintFailure::Metric {
        region,
        depth_row,
        cause: Box::new(cause),
    }
}

impl std::fmt::Display for ConeFootprintFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Attachment(cause) => cause.fmt(f),
            Self::SupportMismatch { actual, expected } => write!(
                f,
                "cone support={actual:?}, expected selected chart {expected:?}"
            ),
            Self::UnsupportedTrim { expected } => {
                write!(f, "unsupported native cone trim, expected {expected}")
            }
            Self::Native(cause) => write!(f, "native cone trim query failed: {cause}"),
            Self::MissingRegion(region) => write!(
                f,
                "missing region {region:?}, expected each authored region and its fitted result"
            ),
            Self::InnerTrimOverlap {
                region,
                wire,
                depth_row,
            } => write!(
                f,
                "cone region={region:?} depth_row={depth_row:?} touches inner trim {wire}, expected disjoint footprint and trim enclosures"
            ),
            Self::Metric {
                region,
                depth_row,
                cause,
            } => write!(
                f,
                "cone footprint region={region:?} depth_row={depth_row:?}: {cause}"
            ),
            Self::Extrusion(cause) => cause.fmt(f),
        }
    }
}
impl std::error::Error for ConeFootprintFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Attachment(cause) => Some(cause.as_ref()),
            Self::Native(cause) => Some(cause.as_ref()),
            Self::Metric { cause, .. } => Some(cause.as_ref()),
            Self::Extrusion(cause) => Some(cause.as_ref()),
            _ => None,
        }
    }
}
