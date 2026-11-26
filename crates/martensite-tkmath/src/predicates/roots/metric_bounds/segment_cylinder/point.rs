// SPDX-License-Identifier: GPL-2.0-only
//! Exact native axial trim and certified world discrepancy of a periodic point lift.
mod evaluation;

use super::{CylinderWallPlace, ExactVector, Ratio, SegmentCylinderWall, exact_vector};
use crate::predicates::{
    NonFiniteCoordinate,
    roots::{
        certificate_arithmetic::rounding::nearest_binary64,
        metric_bounds::{BoundedArithmeticFailure, CylindricalProfileFailure},
    },
};
use std::{
    error::Error,
    f64::consts::{PI, TAU},
    fmt,
};

/// Continuous unwrapped original cylindrical chart anchored at one admitted UV.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CylinderPointChart {
    /// Angular radians and axial parameter at the anchor.
    pub uv: [f64; 2],
    /// Nominal world point rounded from the certified lift.
    pub world: [f64; 3],
    /// Nominal radial offset from the placed cylinder axis to the point.
    pub radial: [f64; 3],
    /// Nominal angular and axial tangents in the original parameter units.
    pub tangents: [[f64; 3]; 2],
}

/// Certified report for a retained XYZ and its actual unwrapped original chart lift.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CylinderPointQuery {
    /// Exact axial relation of the retained world point to both rim planes.
    pub world_place: CylinderWallPlace,
    /// Exact axial relation of the accepted solver UV to both rim parameters.
    pub uv_place: CylinderWallPlace,
    /// Outward coordinate intervals of the ideal lifted world point.
    pub lifted: [[f64; 2]; 3],
    /// Certified Euclidean distance between retained XYZ and the ideal UV lift.
    pub distance_bound: f64,
}

/// Why the original cylindrical chart or point evidence could not be admitted.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum CylinderPointError {
    /// A query point or UV coordinate is not finite.
    Coordinate(NonFiniteCoordinate),
    /// The point has no radial direction in the exact captured chart.
    Axis,
    /// Point qualification requires a full wall without an interior aperture.
    UnsupportedHole,
    /// The angular hint or proposed motion has no uniquely admitted nearby lift.
    AmbiguousLift {
        /// Prior or requested reference angle.
        prior: f64,
        /// Candidate angle.
        next: f64,
    },
    /// Nonnegative finite model distance is required.
    DistanceLimit(f64),
    /// Exact stored-lift distance could not be certified under the requested model limit.
    Distance {
        /// Computed outward upper distance if finite.
        bound: Option<f64>,
        /// Original requested model limit.
        limit: f64,
    },
    /// No finite binary64 nominal chart or interval can be materialized.
    Unrepresentable,
    /// Outward phi or world-distance arithmetic was incomplete.
    Arithmetic(Box<BoundedArithmeticFailure>),
    /// Certified trigonometric reduction failed.
    Trigonometry(Box<CylindricalProfileFailure>),
    /// Point query cancelled before publication.
    Cancelled,
}

impl fmt::Display for CylinderPointError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Coordinate(cause) => cause.fmt(output),
            Self::Axis => {
                output.write_str("the cylindrical point needs a nonzero radial direction")
            }
            Self::UnsupportedHole => output.write_str(
                "the cylindrical point chart requires a wall without an interior aperture",
            ),
            Self::AmbiguousLift { prior, next } => write!(
                output,
                "angles {prior} and {next} require a unique lift within a half turn"
            ),
            Self::DistanceLimit(limit) => write!(
                output,
                "distance limit {limit} must be finite and nonnegative"
            ),
            Self::Distance { bound, limit } => write!(
                output,
                "world lift distance upper bound {bound:?} exceeds or cannot establish {limit}"
            ),
            Self::Unrepresentable => {
                output.write_str("the cylindrical point chart has no finite binary64 enclosure")
            }
            Self::Arithmetic(cause) => {
                write!(output, "bounded cylindrical point arithmetic: {cause:?}")
            }
            Self::Trigonometry(cause) => cause.fmt(output),
            Self::Cancelled => output.write_str("cylindrical point was cancelled"),
        }
    }
}
impl Error for CylinderPointError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Coordinate(cause) => Some(cause),
            Self::Trigonometry(cause) => Some(cause.as_ref()),
            _ => None,
        }
    }
}
impl From<NonFiniteCoordinate> for CylinderPointError {
    fn from(cause: NonFiniteCoordinate) -> Self {
        Self::Coordinate(cause)
    }
}
impl From<BoundedArithmeticFailure> for CylinderPointError {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        match cause {
            BoundedArithmeticFailure::Cancelled => Self::Cancelled,
            cause => Self::Arithmetic(Box::new(cause)),
        }
    }
}
impl From<CylindricalProfileFailure> for CylinderPointError {
    fn from(cause: CylindricalProfileFailure) -> Self {
        match cause {
            CylindricalProfileFailure::Arithmetic(cause) => (*cause).into(),
            cause => Self::Trigonometry(Box::new(cause)),
        }
    }
}

type Checked<T> = Result<T, CylinderPointError>;

impl SegmentCylinderWall {
    /// Estimate original UV on the sheet near the given angular hint; certify the lift separately.
    /// Example: `wall.point_seed(position, previous_u)?`.
    pub fn point_seed(&self, point: [f64; 3], near: f64) -> Checked<[f64; 2]> {
        let [x, y, z] = self.point_local(&point)?;
        let scale = absolute(&x).max(absolute(&y));
        let direction = [rounded(&x.divide(&scale))?, rounded(&y.divide(&scale))?];
        let principal = martensite_cmath::atan2(direction[1], direction[0]);
        let turns = ((near - principal) / TAU).round();
        let unwrapped = turns.mul_add(TAU, principal);
        require_nearby_lift(near, unwrapped)?;
        Ok([unwrapped, rounded(&z)?])
    }

    /// Read the exact finite wall location of a retained world point.
    /// Example: `wall.point_place(position)?`.
    pub fn point_place(&self, world: [f64; 3]) -> Checked<CylinderWallPlace> {
        Ok(axial_place(&self.point_local(&world)?[2], &self.axial))
    }

    /// Determine exact antipodality of two world directions in the original chart.
    /// Example: `wall.point_antipodes(previous, cursor)?`.
    pub fn point_antipodes(&self, a: [f64; 3], b: [f64; 3]) -> Checked<bool> {
        let [ax, ay, _] = self.point_local(&a)?;
        let [bx, by, _] = self.point_local(&b)?;
        let cross = ax.multiply(&by).subtract(&ay.multiply(&bx));
        let dot = ax.multiply(&bx).add(&ay.multiply(&by));
        Ok(cross.sign().is_eq() && dot.sign().is_lt())
    }

    /// Certify actual retained XYZ against the unwrapped ideal UV lift and both rims.
    /// Example: `wall.point_qualify(uv, position, 1e-8, || false)?`.
    pub fn point_qualify(
        &self,
        uv: [f64; 2],
        point: [f64; 3],
        limit: f64,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Checked<CylinderPointQuery> {
        poll(&mut cancelled)?;
        if !limit.is_finite() || limit < 0. {
            return Err(CylinderPointError::DistanceLimit(limit));
        }
        let world = exact_vector(point, 0)?;
        let world_place = self.point_place(point)?;
        let [angle, axial, _] = exact_vector([uv[0], uv[1], 0.], 1)?;
        let uv_place = axial_place(&axial, &self.axial);
        let (lifted, distance_bound) =
            evaluation::qualify(self, &angle, &axial, &world, limit, &mut cancelled)?;
        poll(&mut cancelled)?;
        Ok(CylinderPointQuery {
            world_place,
            uv_place,
            lifted,
            distance_bound,
        })
    }

    /// Read an anchored local solver chart from the same exact placed coefficients.
    /// Example: `wall.point_chart(uv, || false)?`.
    pub fn point_chart(
        &self,
        uv: [f64; 2],
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Checked<CylinderPointChart> {
        self.require_point_trim()?;
        let [angle, axial, _] = exact_vector([uv[0], uv[1], 0.], 1)?;
        let chart = evaluation::chart(self, uv, &angle, &axial, &mut cancelled)?;
        poll(&mut cancelled)?;
        Ok(chart)
    }

    fn point_local(&self, world: &[f64; 3]) -> Checked<ExactVector> {
        self.require_point_trim()?;
        let local = self.pullback(&exact_vector(*world, 0)?);
        if local[0].sign().is_eq() && local[1].sign().is_eq() {
            return Err(CylinderPointError::Axis);
        }
        Ok(local)
    }

    fn require_point_trim(&self) -> Checked<()> {
        if self.hole.is_some() {
            return Err(CylinderPointError::UnsupportedHole);
        }
        Ok(())
    }
}

/// Require an exact binary64 angular difference below a conservative half-turn threshold.
/// Example: `require_nearby_lift(old_u, solved_u)?`.
pub fn require_nearby_lift(prior: f64, next: f64) -> Checked<()> {
    let error = CylinderPointError::AmbiguousLift { prior, next };
    if !prior.is_finite() || !next.is_finite() {
        return Err(error);
    }
    let delta = Ratio::from_finite(next).subtract(&Ratio::from_finite(prior));
    if absolute(&delta) >= Ratio::from_finite(PI.next_down()) {
        return Err(error);
    }
    Ok(())
}

fn axial_place(value: &Ratio, [first, last]: &[Ratio; 2]) -> CylinderWallPlace {
    use CylinderWallPlace::*;
    match (value.cmp(first), value.cmp(last)) {
        (std::cmp::Ordering::Less, _) => Below,
        (std::cmp::Ordering::Equal, _) => FirstRim,
        (_, std::cmp::Ordering::Greater) => Above,
        (_, std::cmp::Ordering::Equal) => LastRim,
        _ => Interior,
    }
}

fn rounded(value: &Ratio) -> Checked<f64> {
    nearest_binary64(value).ok_or(CylinderPointError::Unrepresentable)
}

fn absolute(value: &Ratio) -> Ratio {
    if value.sign().is_lt() {
        value.negated()
    } else {
        value.clone()
    }
}

fn poll(cancelled: &mut impl FnMut() -> bool) -> Checked<()> {
    if cancelled() {
        return Err(CylinderPointError::Cancelled);
    }
    Ok(())
}
