// SPDX-License-Identifier: GPL-2.0-only
//! Transfer the boundary isotopy to the normal image of the complete winding fill.
use super::*;
mod decoded;
pub use decoded::ConicalDecodedCapEvidence;

/// Bounded volume and spatial discrepancy of the exact cap reference over the complete depth interval.
#[derive(Debug)]
pub struct ConicalCapNormalEvidence {
    volume: [DyadicBound; 2],
    displacement: DyadicBound,
    radii: [DyadicBound; 2],
    spatial_limit: f64,
    options: ConeClearanceOptions,
    operations: u32,
}
impl ConicalCapNormalEvidence {
    /// Signed volume enclosure. Read it with `let endpoints = evidence.volume();`.
    pub fn volume(&self) -> &[DyadicBound; 2] {
        &self.volume
    }

    /// Symmetric filled-body distance bound in model units. Read it with `let d = evidence.filled_error();`.
    pub fn filled_error(&self) -> &DyadicBound {
        &self.displacement
    }

    /// Radius bounds over the interpolation and normal depths. Read it with `let r = evidence.radius_bounds();`.
    pub fn radius_bounds(&self) -> &[DyadicBound; 2] {
        &self.radii
    }

    /// Requested world spatial limit. Read it with `let limit = evidence.requested_spatial_limit();`.
    pub fn requested_spatial_limit(&self) -> f64 {
        self.spatial_limit
    }

    /// Precision and finite arithmetic allowance used. Read it with `let options = evidence.options();`.
    pub fn options(&self) -> ConeClearanceOptions {
        self.options
    }

    /// Total charged arithmetic for both measures. Read it with `let work = evidence.operations();`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
}

/// Refusal while measuring the certified cap reference.
#[derive(Debug)]
pub enum ConicalCapNormalError {
    /// Integration or meridian setup failed.
    Volume(ConicalCapVolumeError),
    /// Radius, spatial limit or its arithmetic proof failed.
    Spatial(ConicalReferenceError),
}
impl ConicalCapNormalError {
    /// Actual charged arithmetic before refusal. Read it with `let spent = error.operations();`.
    pub fn operations(&self) -> u32 {
        match self {
            Self::Volume(cause) => cause.operations,
            Self::Spatial(cause) => cause.operations,
        }
    }
}
impl std::fmt::Display for ConicalCapNormalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Volume(cause) => cause.fmt(f),
            Self::Spatial(cause) => cause.fmt(f),
        }
    }
}
impl std::error::Error for ConicalCapNormalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Volume(cause) => Some(cause),
            Self::Spatial(cause) => Some(cause),
        }
    }
}
impl From<ConicalCapVolumeError> for ConicalCapNormalError {
    fn from(cause: ConicalCapVolumeError) -> Self {
        Self::Volume(cause)
    }
}

impl ConicalCapHomotopyEvidence<'_> {
    /// Certify symmetric distance between the authored and cap winding fills over all depths,
    /// and enclose the cap's signed normal volume. The composing operation establishes the
    /// selected material and exclusion of filled periodic overlaps.
    /// Read it with `let reference = proof.certify_normal_reference(epsilon, options, || false)?;`.
    pub fn certify_normal_reference(
        &self,
        spatial_limit: f64,
        options: ConeClearanceOptions,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<ConicalCapNormalEvidence, ConicalCapNormalError> {
        admit_limit(spatial_limit)?;
        let mut arithmetic = volume_arithmetic(options, &mut cancelled)?;
        let measures = self.normal_measures(spatial_limit, &mut arithmetic)?;
        Ok(measures.evidence(spatial_limit, &arithmetic, arithmetic.operations()))
    }

    fn normal_measures(
        &self,
        spatial_limit: f64,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Result<NormalMeasures, ConicalCapNormalError> {
        let moments = self.moments(arithmetic)?;
        let meridian = Meridian::capture(self.cap, arithmetic)
            .map_err(|cause| error(cause, None, arithmetic))?;
        let [displacement, radii] =
            spatial(self.cap, &moments, &meridian, spatial_limit, arithmetic)
                .map_err(|cause| spatial_error(cause, arithmetic.operations()))?;
        let volume = integrate_volume(&meridian, &moments, arithmetic)
            .map_err(|cause| error(cause, None, arithmetic))?;
        Ok(NormalMeasures {
            volume,
            displacement,
            radii,
            meridian,
        })
    }
}

struct NormalMeasures {
    volume: Interval,
    displacement: Interval,
    radii: Interval,
    meridian: Meridian,
}
impl NormalMeasures {
    fn evidence(
        self,
        spatial_limit: f64,
        arithmetic: &Arithmetic<'_>,
        operations: u32,
    ) -> ConicalCapNormalEvidence {
        let policy = arithmetic.policy();
        let bits = policy.fractional_bits;
        ConicalCapNormalEvidence {
            volume: exact_enclosure(&self.volume, bits),
            displacement: DyadicBound::new(self.displacement.upper, -i32::from(bits)),
            radii: exact_enclosure(&self.radii, bits),
            spatial_limit,
            options: ConeClearanceOptions {
                fractional_bits: bits,
                max_operations: policy.max_operations,
            },
            operations,
        }
    }
}

fn admit_limit(limit: f64) -> Result<(), ConicalCapNormalError> {
    if limit.is_finite() && limit > 0. {
        return Ok(());
    }
    Err(spatial_error(
        ConicalReferenceFailure::Bound {
            field: "positive finite cap normal spatial allowance",
            actual: limit,
            limit: 0.,
        },
        0,
    ))
}

fn spatial_error(cause: ConicalReferenceFailure, operations: u32) -> ConicalCapNormalError {
    ConicalCapNormalError::Spatial(ConicalReferenceError {
        offender: None,
        cause,
        operations,
    })
}

type SpatialResult<T> = Result<T, ConicalReferenceFailure>;

fn spatial(
    cap: &CapBinding,
    moments: &Moments,
    meridian: &Meridian,
    spatial_limit: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> SpatialResult<[Interval; 2]> {
    let slant = import_bound(&cap.slant_motion, arithmetic)?;
    let radii = swept_radii(moments, meridian, &slant, arithmetic)?;
    let squared = import_bound(&cap.squared_world_motion, arithmetic)?;
    let bound = displacement_upper(squared.upper, arithmetic)?;
    reference::geometry::allocated(
        &bound,
        spatial_limit,
        1,
        "filled cap normal discrepancy",
        arithmetic,
    )?;
    Ok([bound, radii])
}

fn displacement_upper(
    upper: num_bigint::BigInt,
    arithmetic: &mut Arithmetic<'_>,
) -> SpatialResult<Interval> {
    use crate::predicates::roots::certificate_arithmetic::bounded_dyadic::PositiveSqrtFailure;
    if upper == 0.into() {
        return Ok(arithmetic.zero());
    }
    let squared = Interval {
        lower: upper.clone(),
        upper,
    };
    arithmetic
        .positive_sqrt(&squared)
        .map_err(|cause| match cause {
            PositiveSqrtFailure::Arithmetic(cause) => cause.into(),
            PositiveSqrtFailure::InvalidInterval { .. } => ConicalReferenceFailure::Binding {
                field: "ordered squared cap displacement bound",
            },
        })
}

fn import_bound(bound: &DyadicBound, arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
    crate::predicates::roots::metric_bounds::conical_profile::trim_bridge::bounds::captured_storage(
        bound, arithmetic,
    )
}

fn swept_radii(
    moments: &Moments,
    meridian: &Meridian,
    slant_motion: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> SpatialResult<Interval> {
    let slant_hull = cap_slant_hull(moments)?;
    let motion = Interval {
        lower: -&slant_motion.upper,
        upper: slant_motion.upper.clone(),
    };
    let swept_slants = arithmetic.add(&slant_hull, &motion)?;
    let radii = meridian_radius(meridian, &swept_slants, arithmetic)?;
    if radii.lower <= 0.into() {
        return Err(ConicalReferenceFailure::Bound {
            field: "positive radius over the filled cap interpolation",
            actual: arithmetic.upper_f64(radii.lower)?,
            limit: 0.,
        });
    }
    Ok(radii)
}

fn cap_slant_hull(moments: &Moments) -> SpatialResult<Interval> {
    let (Some(lower), Some(upper)) = (&moments.minimum_v, &moments.maximum_v) else {
        return Err(ConicalReferenceFailure::Binding {
            field: "nonempty cap slant hull",
        });
    };
    Ok(Interval {
        lower: lower.clone(),
        upper: upper.clone(),
    })
}

fn meridian_radius(
    meridian: &Meridian,
    slants: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let lower_depth = &meridian.depth_minimum;
    let depth_span = arithmetic.multiply(&meridian.thickness, &meridian.cosine)?;
    let upper_depth = arithmetic.add(lower_depth, &depth_span)?;
    let depth = Interval {
        lower: lower_depth.lower.clone(),
        upper: upper_depth.upper,
    };
    let slant = arithmetic.multiply(slants, &meridian.sine)?;
    let radius = arithmetic.add(&meridian.reference_radius, &slant)?;
    arithmetic.add(&radius, &depth)
}

#[cfg(test)]
mod tests;
