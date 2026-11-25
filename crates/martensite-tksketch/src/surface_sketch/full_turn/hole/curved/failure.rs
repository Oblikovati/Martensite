// SPDX-License-Identifier: GPL-2.0-only
//! Report the original mathematical cause from periodic aperture fitting.
use super::*;
use std::{error::Error, fmt};

/// Failure to publish a complete curved periodic band with its authored aperture history.
#[derive(Debug)]
#[non_exhaustive]
pub enum CurvedBandFailure {
    /// The band, cut, history, or ordered geometric proof was refused.
    Surface(SurfaceSketchFailure),
    /// The polynomial cutter fit or the certified native export was refused.
    Wrapped {
        /// Stock band supplied by the caller.
        band: SurfaceRegionId,
        /// Selected authored aperture identities in region order.
        apertures: Vec<SurfaceRegionId>,
        /// Original failed wrapped operation, including any known offending entities.
        cause: Box<WrappedNormalFailure>,
    },
    /// The rational cutter proof or fit was refused.
    Rational {
        /// Stock band supplied by the caller.
        band: SurfaceRegionId,
        /// Selected authored aperture identities in region order.
        apertures: Vec<SurfaceRegionId>,
        /// Original rational source, work-limit or fitting cause.
        cause: Box<RationalWrappedFailure>,
    },
}

impl CurvedBandFailure {
    /// Identity of the requested stock band. Example: `assert_eq!(failure.band(), band.region)`.
    pub fn band(&self) -> SurfaceRegionId {
        match self {
            Self::Surface(cause) => cause.region,
            Self::Wrapped { band, .. } | Self::Rational { band, .. } => *band,
        }
    }

    /// Periodic operation boundary. The enclosed fitter retains its more specific stage.
    /// Example: `assert_eq!(failure.stage(), SurfaceStage::Construction)`.
    pub fn stage(&self) -> SurfaceStage {
        match self {
            Self::Surface(cause) => cause.stage,
            Self::Wrapped { cause, .. }
                if cause.operation() == WrappedOperation::DetachedExport =>
            {
                SurfaceStage::Encoding
            }
            Self::Wrapped { .. } | Self::Rational { .. } => SurfaceStage::Construction,
        }
    }

    pub(super) fn wrapped(plan: &AperturePlan, cause: WrappedNormalFailure) -> Self {
        Self::Wrapped {
            band: plan.band.region,
            apertures: aperture_ids(plan),
            cause: Box::new(cause),
        }
    }

    pub(super) fn rational(plan: &AperturePlan, cause: RationalWrappedFailure) -> Self {
        Self::Rational {
            band: plan.band.region,
            apertures: aperture_ids(plan),
            cause: Box::new(cause),
        }
    }
}

fn aperture_ids(plan: &AperturePlan) -> Vec<SurfaceRegionId> {
    plan.contours
        .iter()
        .map(|aperture| aperture.region)
        .collect()
}

impl From<SurfaceSketchFailure> for CurvedBandFailure {
    fn from(cause: SurfaceSketchFailure) -> Self {
        Self::Surface(cause)
    }
}

impl fmt::Display for CurvedBandFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (apertures, cause): (&[SurfaceRegionId], &dyn Error) = match self {
            Self::Wrapped {
                apertures, cause, ..
            } => (apertures, cause.as_ref()),
            Self::Rational {
                apertures, cause, ..
            } => (apertures, cause.as_ref()),
            Self::Surface(cause) => return cause.fmt(f),
        };
        write!(
            f,
            "periodic band {:?} apertures {apertures:?} at {:?}: {cause}",
            self.band(),
            self.stage()
        )
    }
}

impl Error for CurvedBandFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Surface(cause) => Some(cause),
            Self::Wrapped { cause, .. } => Some(cause.as_ref()),
            Self::Rational { cause, .. } => Some(cause.as_ref()),
        }
    }
}
