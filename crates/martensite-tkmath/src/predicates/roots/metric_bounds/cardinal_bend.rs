// SPDX-License-Identifier: GPL-2.0-only
//! Exact placement of a quarter-circle tangent to two coordinate-axis legs.
use super::super::certificate_arithmetic::{
    ExactRatio,
    rounding::{nearest_binary64, outward_binary64, outward_sqrt},
};
use crate::predicates::NonFiniteCoordinate;
use std::{cmp::Ordering, error::Error, fmt};

/// Stored tangency point and outward normalized source parameters in both directions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CardinalLegCut {
    /// Exact finite coordinate values for the new endpoint on its authored leg.
    pub tangent: [f64; 3],
    /// Bracket from the corner (0) toward the far endpoint (1).
    pub from_corner: [f64; 2],
    /// Bracket from the far endpoint (0) toward the corner (1).
    pub to_corner: [f64; 2],
}

/// Finite quarter-circle placement with no out-of-plane or unequal leg offsets.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CardinalBendPlacement {
    /// The two cuts in supplied leg order.
    pub legs: [CardinalLegCut; 2],
    /// Literal stored center.
    pub center: [f64; 3],
    /// Cardinal X and Y frame directions for positive traversal from the first cut to the second.
    pub radial_axes: [[f64; 3]; 2],
    /// Right-handed normal of the positive quarter turn.
    pub normal: [f64; 3],
    /// Exact common distance from center to both stored cuts.
    pub radius: f64,
    /// Certified upper Euclidean deviation of any stored placement point from its ideal point.
    pub position_error_upper: f64,
    /// Certified upper deviation of the exact stored radius from the requested radius.
    pub radius_error_upper: f64,
    /// Certified arc-length bound on the analytic end of the binary64 quarter parameter.
    pub terminal_error_upper: f64,
}

/// Why an exact cardinal quarter-circle placement was unavailable.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum CardinalBendFailure {
    /// Coordinate of the corner or far endpoint, indexed 0, 1 and 2 respectively.
    NonFinite(NonFiniteCoordinate),
    /// Expected a finite strictly positive radius.
    InvalidRadius(f64),
    /// This leg is coincident or varies along more than one world axis.
    NonCardinalLeg(usize),
    /// The two cardinal axes must differ.
    ParallelLegs,
    /// The radius must be strictly less than the indicated leg's exact length.
    RadiusReachesEnd(usize, f64),
    /// The rounded tangency point is the corner or the far endpoint of this leg.
    RoundedLegEnd(usize),
    /// The rounded cuts give unequal exact radial lengths.
    UnequalStoredRadii,
    /// An exact radius, coordinate, or certificate has no supported finite representation.
    Unrepresentable,
}

impl fmt::Display for CardinalBendFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite(cause) => cause.fmt(output),
            Self::InvalidRadius(radius) => {
                write!(output, "bend radius {radius} must be finite and positive")
            }
            Self::NonCardinalLeg(leg) => write!(
                output,
                "bend leg {leg} needs distinct endpoints on one world axis"
            ),
            Self::ParallelLegs => write!(output, "bend legs must use distinct world axes"),
            Self::RadiusReachesEnd(leg, radius) => write!(
                output,
                "bend radius {radius} must be strictly shorter than leg {leg}"
            ),
            Self::RoundedLegEnd(leg) => write!(
                output,
                "bend cut on leg {leg} needs a representable interior point"
            ),
            Self::UnequalStoredRadii => write!(
                output,
                "bend cut points must give equal exact radii after rounding"
            ),
            Self::Unrepresentable => write!(
                output,
                "bend placement requires finite binary64 geometry and evidence"
            ),
        }
    }
}

impl Error for CardinalBendFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NonFinite(cause) => Some(cause),
            _ => None,
        }
    }
}

/// The binary64 `FRAC_PI_2` undershoots the real quarter angle by less than 2^-52 radians.
/// The real quarter lies between 1.570796326794896619 and 1.570796326794896620.
pub const BINARY64_QUARTER_ANGLE_ERROR: f64 = f64::EPSILON;

/// Enclose the arc length from the stored binary64 quarter parameter to the real quarter.
/// Example: `cardinal_quarter_terminal_bound(5.0)?`.
pub fn cardinal_quarter_terminal_bound(radius: f64) -> Result<f64, CardinalBendFailure> {
    if !radius.is_finite() || radius <= 0. {
        return Err(CardinalBendFailure::InvalidRadius(radius));
    }
    let angle = ExactRatio::from_finite(BINARY64_QUARTER_ANGLE_ERROR);
    Ok(enclosure(&ExactRatio::from_finite(radius).multiply(&angle))?[1])
}

/// Place the actual common-radius quarter-circle, rounding each changed coordinate once.
/// Example: `cardinal_bend_placement([0.; 3], [[1., 0., 0.], [0., 1., 0.]], 0.25)?`.
pub fn cardinal_bend_placement(
    corner: [f64; 3],
    far: [[f64; 3]; 2],
    radius: f64,
) -> Result<CardinalBendPlacement, CardinalBendFailure> {
    if !radius.is_finite() || radius <= 0. {
        return Err(CardinalBendFailure::InvalidRadius(radius));
    }
    validate_points([corner, far[0], far[1]])?;
    let rays = [Ray::new(corner, far[0], 0)?, Ray::new(corner, far[1], 1)?];
    if rays[0].axis == rays[1].axis {
        return Err(CardinalBendFailure::ParallelLegs);
    }
    let requested = ExactRatio::from_finite(radius);
    let cuts = [
        rays[0].place(corner, &requested, radius, 0)?,
        rays[1].place(corner, &requested, radius, 1)?,
    ];
    assemble(corner, rays, cuts, &requested)
}

fn validate_points(points: [[f64; 3]; 3]) -> Result<(), CardinalBendFailure> {
    for (point, coordinates) in points.into_iter().enumerate() {
        for (axis, value) in coordinates.into_iter().enumerate() {
            if !value.is_finite() {
                return Err(CardinalBendFailure::NonFinite(NonFiniteCoordinate {
                    point,
                    axis,
                    value,
                }));
            }
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct Ray {
    axis: usize,
    sign: f64,
    far: f64,
}

struct LegPlacement {
    cut: CardinalLegCut,
    radius: ExactRatio,
    error: ExactRatio,
}

impl Ray {
    fn new(corner: [f64; 3], far: [f64; 3], leg: usize) -> Result<Self, CardinalBendFailure> {
        let mut axes = (0..3).filter(|&axis| corner[axis] != far[axis]);
        let axis = axes
            .next()
            .ok_or(CardinalBendFailure::NonCardinalLeg(leg))?;
        if axes.next().is_some() {
            return Err(CardinalBendFailure::NonCardinalLeg(leg));
        }
        let sign = if far[axis] < corner[axis] { -1. } else { 1. };
        Ok(Self {
            axis,
            sign,
            far: far[axis],
        })
    }

    fn place(
        self,
        mut tangent: [f64; 3],
        radius: &ExactRatio,
        requested: f64,
        leg: usize,
    ) -> Result<LegPlacement, CardinalBendFailure> {
        let original = tangent[self.axis];
        let origin = ExactRatio::from_finite(original);
        let length = absolute(ExactRatio::from_finite(self.far).subtract(&origin));
        if radius >= &length {
            return Err(CardinalBendFailure::RadiusReachesEnd(leg, requested));
        }
        let signed = if self.sign < 0. {
            radius.negated()
        } else {
            radius.clone()
        };
        let ideal = origin.add(&signed);
        let value = nearest_binary64(&ideal).ok_or(CardinalBendFailure::Unrepresentable)?;
        if value == original || value == self.far {
            return Err(CardinalBendFailure::RoundedLegEnd(leg));
        }
        tangent[self.axis] = value;
        let actual = ExactRatio::from_finite(value);
        let stored_radius = absolute(actual.subtract(&origin));
        let fraction = stored_radius.divide(&length);
        Ok(LegPlacement {
            cut: CardinalLegCut {
                tangent,
                from_corner: enclosure(&fraction)?,
                to_corner: enclosure(&ExactRatio::from_finite(1.).subtract(&fraction))?,
            },
            radius: stored_radius,
            error: actual.subtract(&ideal),
        })
    }

    fn inward(self) -> [f64; 3] {
        let mut direction = [0.; 3];
        direction[self.axis] = -self.sign;
        direction
    }
}

fn assemble(
    mut center: [f64; 3],
    [a, b]: [Ray; 2],
    [first, second]: [LegPlacement; 2],
    requested: &ExactRatio,
) -> Result<CardinalBendPlacement, CardinalBendFailure> {
    if first.radius != second.radius {
        return Err(CardinalBendFailure::UnequalStoredRadii);
    }
    let [radius, upper] = enclosure(&first.radius)?;
    if radius != upper {
        return Err(CardinalBendFailure::Unrepresentable);
    }
    center[a.axis] = first.cut.tangent[a.axis];
    center[b.axis] = second.cut.tangent[b.axis];
    let squared_error = first
        .error
        .multiply(&first.error)
        .add(&second.error.multiply(&second.error));
    let position_error_upper =
        outward_sqrt(&squared_error).ok_or(CardinalBendFailure::Unrepresentable)?;
    let radius_error_upper = enclosure(&absolute(first.radius.subtract(requested)))?[1];
    let terminal_error_upper = cardinal_quarter_terminal_bound(radius)?;
    let radial_axes = [b.inward(), a.inward()];
    let mut normal = [0.; 3];
    let handedness = if (b.axis + 1) % 3 == a.axis { 1. } else { -1. };
    normal[3 - a.axis - b.axis] = handedness * a.sign * b.sign;
    Ok(CardinalBendPlacement {
        legs: [first.cut, second.cut],
        center,
        radial_axes,
        normal,
        radius,
        position_error_upper,
        radius_error_upper,
        terminal_error_upper,
    })
}

fn absolute(value: ExactRatio) -> ExactRatio {
    if value.sign() == Ordering::Less {
        value.negated()
    } else {
        value
    }
}

fn enclosure(value: &ExactRatio) -> Result<[f64; 2], CardinalBendFailure> {
    outward_binary64(value).ok_or(CardinalBendFailure::Unrepresentable)
}

#[cfg(test)]
mod tests;
