// SPDX-License-Identifier: GPL-2.0-only
//! Exact intersection of an authored segment with an unbounded work plane.
use super::super::certificate_arithmetic::{
    ExactRatio,
    rounding::{nearest_binary64, outward_binary64, outward_sqrt},
};
use crate::predicates::NonFiniteCoordinate;
use std::{cmp::Ordering, fmt};

/// Finite model-space work plane with a nonzero, possibly unnormalized normal.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SegmentWorkPlane {
    origin: [f64; 3],
    normal: [f64; 3],
}

impl SegmentWorkPlane {
    /// Retain the original coefficients. Example: `SegmentWorkPlane::new([0.; 3], [1., 0., 0.])?`.
    pub fn new(origin: [f64; 3], normal: [f64; 3]) -> Result<Self, SegmentPlaneFailure> {
        check_finite(origin, 2)?;
        check_finite(normal, 3)?;
        if normal == [0.; 3] {
            return Err(SegmentPlaneFailure::ZeroNormal);
        }
        Ok(Self { origin, normal })
    }
}

/// Classification of a closed, directed segment against the work plane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SegmentPlaneIntersection {
    /// The segment never meets the plane.
    Disjoint,
    /// The directed start endpoint belongs to the plane.
    Start,
    /// The directed end endpoint belongs to the plane.
    End,
    /// The entire segment belongs to the plane.
    Overlap,
    /// A unique interior transverse intersection.
    Crossing(SegmentPlaneCut),
}

/// Exact-cut enclosure and position rounded from the source segment coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SegmentPlaneCut {
    /// Outward interval in the original segment's normalized parameter.
    pub parameter: [f64; 2],
    /// Outward coordinate intervals of the exact point on the original segment.
    pub coordinates: [[f64; 2]; 3],
    /// Nominal cut position, each coordinate rounded to nearest, ties to even.
    pub position: [f64; 3],
    /// Outward Euclidean distance from the nominal position to the exact cut.
    pub error_upper: f64,
}

/// Reason a segment or plane operation could not supply finite evidence.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum SegmentPlaneFailure {
    /// Nonfinite coefficient. Operands are start, end, plane origin, plane normal.
    Coordinate(NonFiniteCoordinate),
    /// A plane needs a nonzero normal.
    ZeroNormal,
    /// The directed segment endpoints coincide.
    CoincidentEndpoints,
    /// No finite binary64 enclosure was available.
    Unrepresentable,
}

impl fmt::Display for SegmentPlaneFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Coordinate(cause) => cause.fmt(f),
            Self::ZeroNormal => write!(f, "work plane normal is zero, expected a nonzero vector"),
            Self::CoincidentEndpoints => {
                write!(f, "segment endpoints coincide, expected a nonzero segment")
            }
            Self::Unrepresentable => write!(f, "expected a finite binary64 cut enclosure"),
        }
    }
}

impl std::error::Error for SegmentPlaneFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Coordinate(cause) => Some(cause),
            _ => None,
        }
    }
}

/// Classify using exact arithmetic on the original finite binary64 inputs.
/// Example: `intersect_segment_plane([[-1., 0., 0.], [1., 0., 0.]], plane)?`.
pub fn intersect_segment_plane(
    endpoints: [[f64; 3]; 2],
    plane: SegmentWorkPlane,
) -> Result<SegmentPlaneIntersection, SegmentPlaneFailure> {
    use SegmentPlaneIntersection::*;
    check_finite(endpoints[0], 0)?;
    check_finite(endpoints[1], 1)?;
    if endpoints[0] == endpoints[1] {
        return Err(SegmentPlaneFailure::CoincidentEndpoints);
    }
    let [start, end] = endpoints.map(|point| point.map(ExactRatio::from_finite));
    let origin = plane.origin.map(ExactRatio::from_finite);
    let normal = plane.normal.map(ExactRatio::from_finite);
    let [a, b] = [&start, &end].map(|point| signed_distance(point, &origin, &normal));
    match (a.sign(), b.sign()) {
        (Ordering::Equal, Ordering::Equal) => Ok(Overlap),
        (Ordering::Equal, _) => Ok(Start),
        (_, Ordering::Equal) => Ok(End),
        (left, right) if left == right => Ok(Disjoint),
        _ => cut(&start, &end, a.divide(&a.subtract(&b))).map(Crossing),
    }
}

fn check_finite(values: [f64; 3], point: usize) -> Result<(), SegmentPlaneFailure> {
    if let Some((axis, value)) = values
        .into_iter()
        .enumerate()
        .find(|(_, value)| !value.is_finite())
    {
        return Err(SegmentPlaneFailure::Coordinate(NonFiniteCoordinate {
            point,
            axis,
            value,
        }));
    }
    Ok(())
}

fn signed_distance(
    point: &[ExactRatio; 3],
    origin: &[ExactRatio; 3],
    normal: &[ExactRatio; 3],
) -> ExactRatio {
    (0..3).fold(ExactRatio::from_finite(0.), |sum, axis| {
        sum.add(&point[axis].subtract(&origin[axis]).multiply(&normal[axis]))
    })
}

fn cut(
    start: &[ExactRatio; 3],
    end: &[ExactRatio; 3],
    parameter: ExactRatio,
) -> Result<SegmentPlaneCut, SegmentPlaneFailure> {
    let mut position = [0.; 3];
    let mut coordinates = [[0.; 2]; 3];
    let mut squared = ExactRatio::from_finite(0.);
    for axis in 0..3 {
        let value = start[axis].add(&end[axis].subtract(&start[axis]).multiply(&parameter));
        coordinates[axis] = enclosure(&value)?;
        position[axis] = nearest_binary64(&value).ok_or(SegmentPlaneFailure::Unrepresentable)?;
        let residual = value.subtract(&ExactRatio::from_finite(position[axis]));
        squared = squared.add(&residual.multiply(&residual));
    }
    Ok(SegmentPlaneCut {
        parameter: enclosure(&parameter)?,
        coordinates,
        position,
        error_upper: radius_bound(&squared)?,
    })
}

fn enclosure(value: &ExactRatio) -> Result<[f64; 2], SegmentPlaneFailure> {
    outward_binary64(value).ok_or(SegmentPlaneFailure::Unrepresentable)
}

fn radius_bound(squared: &ExactRatio) -> Result<f64, SegmentPlaneFailure> {
    if squared.sign() == Ordering::Equal {
        return Ok(0.);
    }
    outward_sqrt(squared).ok_or(SegmentPlaneFailure::Unrepresentable)
}

#[cfg(test)]
mod tests;
