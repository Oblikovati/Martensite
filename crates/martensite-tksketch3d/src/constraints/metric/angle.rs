// SPDX-License-Identifier: GPL-2.0-only
//! Outward angular checks on raw line displacement intervals.

use super::{Interval, LineGeometry, Point3, angular_bound, cross, dot, scaled_direction};
use crate::constraints::LineAngle;

type Displacement = [Interval; 3];

pub(in crate::constraints) fn nonsingular(lines: [LineGeometry; 2], tolerance: f64) -> bool {
    nonsingular_coordinates(lines.map(|line| line.points), tolerance)
}

pub(in crate::constraints) fn nonsingular_coordinates(
    points: [[Point3; 2]; 2],
    tolerance: f64,
) -> bool {
    directions(points)
        .and_then(|[a, b]| {
            let area = cross(a, b)?;
            Some(dot(area, area)?.lo > angular_bound(a, b, tolerance)?.hi)
        })
        .unwrap_or(false)
}

pub(super) fn matches(lines: [LineGeometry; 2], target: LineAngle, tolerance: f64) -> Option<bool> {
    let [a, b] = directions(lines.map(|line| line.points))?;
    let cross_vector = cross(a, b)?;
    let cross_squared = dot(cross_vector, cross_vector)?;
    let limit_squared = angular_bound(a, b, tolerance)?;
    if cross_squared.lo <= limit_squared.hi {
        return Some(false);
    }
    let projection = dot(a, b)?;
    let cross_norm = square_root(cross_squared)?;
    let (sine, cosine) = target.sin_cos();
    let sine = Interval::rounded(sine, sine)?;
    let cosine = Interval::rounded(cosine, cosine)?;
    // The common positive norm product cancels from both angle-difference identities.
    let cosine_numerator = projection.mul(cosine)?.add(cross_norm.mul(sine)?)?;
    let sine_numerator = cross_norm.mul(cosine)?.sub(projection.mul(sine)?)?;
    Some(cosine_numerator.lo > 0. && sine_numerator.mul(sine_numerator)?.hi <= limit_squared.lo)
}

fn directions([a, b]: [[Point3; 2]; 2]) -> Option<[Displacement; 2]> {
    Some([scaled_direction(a)?, scaled_direction(b)?])
}

fn square_root(value: Interval) -> Option<Interval> {
    Interval::rounded(value.lo.max(0.).sqrt(), value.hi.sqrt())
}
