// SPDX-License-Identifier: GPL-2.0-only
//! Exact placed spherical chart and bounded full-period non-pole trim evidence.
mod point;
mod trace;
pub use point::{
    SphericalBandPlace, SphericalPointChart, SphericalPointQuery, require_spherical_lift,
};

use super::{
    BoundedArithmeticFailure, CylindricalProfileFailure,
    cylindrical_profile::trigonometric,
    placed_plane::{ExactPlacement, ExactVector, exact_point, exact_vector, inverse_columns},
};
use crate::{
    gp::{Axis3, Circle3, Sphere3},
    predicates::{
        NonFiniteCoordinate,
        roots::certificate_arithmetic::{
            ExactRatio as R,
            bounded_dyadic::{Arithmetic, ArithmeticPolicy, Interval},
        },
    },
};
use std::{error::Error, fmt};

const POLICY: ArithmeticPolicy = ArithmeticPolicy {
    fractional_bits: 160,
    max_operations: 50_000,
};
type Checked<T> = Result<T, SphericalBandError>;
type EVec = [Interval; 3];

/// Actual copied spherical support and native rim, seam and shared vertex values.
#[derive(Clone, Copy, Debug)]
pub struct SphericalBandTraceInput {
    /// Original local spherical support.
    pub sphere: Sphere3,
    /// Original effective affine placement in model coordinates.
    pub placement: [[f64; 4]; 3],
    /// Literal closed latitude limits in the selected native pcurves.
    pub latitude: [f64; 2],
    /// Actual full circular rims, in increasing latitude order.
    pub rims: [Circle3; 2],
    /// Actual spatial circle shared by both meridian coedges.
    pub meridian: Circle3,
    /// Native meridian and pcurve parameter range.
    pub meridian_range: [f64; 2],
    /// Original affine pcurve relation `latitude = meridian_parameter + offset`.
    pub meridian_offset: f64,
    /// Original vertices shared with the rims, in increasing latitude order.
    pub anchors: [[f64; 3]; 2],
}

/// Frozen full-period spherical chart and certified retained trim evidence.
#[derive(Clone, Debug)]
pub struct SphericalBand {
    origin: ExactVector,
    axes: [ExactVector; 3],
    inverse: [ExactVector; 3],
    radius: R,
    latitude: [f64; 2],
    trace: f64,
    sine_rims: [[R; 2]; 2],
    relative_area_lower: f64,
}

/// Why the bounded spherical band or point could not be certified.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum SphericalBandError {
    /// A required coordinate is nonfinite.
    Coordinate(NonFiniteCoordinate),
    /// The original frame does not define an invertible checked chart.
    Frame,
    /// The spherical support or a retained circle requires positive finite radius.
    Radius(f64),
    /// The actual band requires a finite positive span strictly inside both poles.
    Latitude([f64; 2]),
    /// Actual seam parameter and latitude intervals do not bind exactly.
    Meridian,
    /// No finite longitude or regular tangent exists at a pole or the center.
    Pole,
    /// The longitude hint or step has no uniquely admitted nearby lift.
    Lift {
        /// Prior or requested angular reference.
        prior: f64,
        /// Proposed angular coordinate.
        next: f64,
    },
    /// The actual world-ray latitude could not be compared to a retained rim.
    TrimUnresolved,
    /// A finite nominal chart component could not be established.
    Unrepresentable,
    /// The actual world-lift discrepancy could not be admitted under the requested cap.
    Distance {
        /// Certified outward model-space upper bound, when representable.
        bound: Option<f64>,
        /// Original requested inclusive maximum.
        limit: f64,
    },
    /// A model cap requires a finite nonnegative value.
    Limit(f64),
    /// The actual trim error exceeds the original model cap.
    Trace {
        /// Certified retained L1 trace bound in model units.
        bound: f64,
        /// Requested unchanged maximum.
        limit: f64,
    },
    /// Bounded outward arithmetic did not complete.
    Arithmetic(Box<BoundedArithmeticFailure>),
    /// The certified trig reduction did not complete.
    Trigonometry(Box<CylindricalProfileFailure>),
    /// Cooperative cancellation prevented publication.
    Cancelled,
}

impl fmt::Display for SphericalBandError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Coordinate(cause) => cause.fmt(out),
            Self::Frame => out.write_str("spherical band needs an invertible checked frame"),
            Self::Radius(r) => write!(out, "spherical band radius {r} must be finite and positive"),
            Self::Latitude(band) => write!(
                out,
                "spherical band {band:?} must increase strictly inside both poles"
            ),
            Self::Meridian => out
                .write_str("the original meridian parameter must map exactly onto the band limits"),
            Self::Pole => out.write_str("spherical point requires a non-pole regular chart"),
            Self::Lift { prior, next } => write!(
                out,
                "angles {prior} and {next} require a unique lift within a half turn"
            ),
            Self::TrimUnresolved => out.write_str(
                "the certified world latitude could not be ordered against the actual rim",
            ),
            Self::Unrepresentable => {
                out.write_str("a spherical point needs finite nominal or outward components")
            }
            Self::Distance { bound, limit } => write!(
                out,
                "world lift upper bound {bound:?} exceeds or cannot establish {limit}"
            ),
            Self::Limit(limit) => write!(
                out,
                "spherical band limit {limit} must be finite and nonnegative"
            ),
            Self::Trace { bound, limit } => {
                write!(out, "spherical band trim bound {bound} exceeds {limit}")
            }
            Self::Arithmetic(cause) => write!(out, "bounded spherical band arithmetic: {cause:?}"),
            Self::Trigonometry(cause) => cause.fmt(out),
            Self::Cancelled => out.write_str("spherical band query was cancelled"),
        }
    }
}
impl Error for SphericalBandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Coordinate(cause) => Some(cause),
            Self::Trigonometry(cause) => Some(cause.as_ref()),
            _ => None,
        }
    }
}
impl From<NonFiniteCoordinate> for SphericalBandError {
    fn from(cause: NonFiniteCoordinate) -> Self {
        Self::Coordinate(cause)
    }
}
impl From<BoundedArithmeticFailure> for SphericalBandError {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        match cause {
            BoundedArithmeticFailure::Cancelled => Self::Cancelled,
            cause => Self::Arithmetic(Box::new(cause)),
        }
    }
}
impl From<CylindricalProfileFailure> for SphericalBandError {
    fn from(cause: CylindricalProfileFailure) -> Self {
        match cause {
            CylindricalProfileFailure::Arithmetic(cause) => (*cause).into(),
            cause => Self::Trigonometry(Box::new(cause)),
        }
    }
}

impl SphericalBand {
    /// Certify the actual captured rim and seam geometry against an original full-period chart.
    /// The caller proves the literal pcurve incidences and shared topology before this step.
    /// Example: `SphericalBand::capture(input, 1e-8, || false)?`.
    pub fn capture(
        input: SphericalBandTraceInput,
        limit: f64,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Checked<Self> {
        poll(&mut cancelled)?;
        if !limit.is_finite() || limit < 0. {
            return Err(SphericalBandError::Limit(limit));
        }
        let placement = ExactPlacement::capture(input.placement)?;
        let mut band = Self::new(input.sphere, &placement, input.latitude)?;
        let mut arithmetic = Arithmetic::new(POLICY, &mut cancelled);
        band.trace = trace::prove(&band, input, &placement, &mut arithmetic)?;
        if band.trace > limit {
            return Err(SphericalBandError::Trace {
                bound: band.trace,
                limit,
            });
        }
        let (rims, area) = point::admit(&band, &mut arithmetic)?;
        band.sine_rims = rims;
        band.relative_area_lower = area;
        poll(&mut cancelled)?;
        Ok(band)
    }

    fn new(sphere: Sphere3, place: &ExactPlacement, latitude: [f64; 2]) -> Checked<Self> {
        let frame = sphere.position();
        Axis3::from_stored_coefficients(
            frame.location(),
            *frame.direction().coordinates(),
            *frame.x_direction().coordinates(),
            *frame.y_direction().coordinates(),
        )
        .map_err(|_| SphericalBandError::Frame)?;
        let directions = [frame.x_direction(), frame.y_direction(), frame.direction()];
        let axes = [
            place.vector(exact_vector(*directions[0].coordinates(), 1)?),
            place.vector(exact_vector(*directions[1].coordinates(), 2)?),
            place.vector(exact_vector(*directions[2].coordinates(), 3)?),
        ];
        let inverse = inverse_columns(&axes).ok_or(SphericalBandError::Frame)?;
        let radius = positive_radius(sphere.radius())?;
        let origin = place.point(exact_point(frame.location(), 0)?);
        Ok(Self {
            origin,
            axes,
            inverse,
            radius,
            latitude,
            trace: 0.,
            sine_rims: std::array::from_fn(|_| std::array::from_fn(|_| R::from_finite(0.))),
            relative_area_lower: 0.,
        })
    }

    /// Read the original stored latitude bounds. Example: `band.latitude()`.
    pub fn latitude(&self) -> [f64; 2] {
        self.latitude
    }

    /// Read the achievable original rim/seam L1 trace bound. Example: `band.trace_bound()`.
    pub fn trace_bound(&self) -> f64 {
        self.trace
    }
}

fn poll(cancelled: &mut impl FnMut() -> bool) -> Checked<()> {
    if cancelled() {
        return Err(SphericalBandError::Cancelled);
    }
    Ok(())
}

fn positive_radius(value: f64) -> Checked<R> {
    if !value.is_finite() || value <= 0. {
        return Err(SphericalBandError::Radius(value));
    }
    Ok(R::from_finite(value))
}

fn scalar_trig(angle: f64, arithmetic: &mut Arithmetic<'_>) -> Checked<[Interval; 2]> {
    let value = arithmetic.scalar(angle)?;
    Ok(trigonometric::scalar(&value, arithmetic)?)
}
