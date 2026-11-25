// SPDX-License-Identifier: GPL-2.0-only
//! Independent physical admission from the accepted endpoint coordinates.
pub(super) mod angle;
mod incidence;
pub(super) use incidence::line_branch;

use super::{AxisDirection, ConstraintPolicy, ParallelDirection, Relation};
use crate::{LineGeometry, LineId, Snapshot};
use martensite_tkmath::{
    gp::Point3,
    predicates::{compare_length_difference_3d, compare_squared_length_3d},
};
use std::cmp::Ordering;

pub(super) fn feasible(snapshot: &Snapshot, relation: Relation, policy: ConstraintPolicy) -> bool {
    match relation {
        Relation::FixPoint(point, anchor) => snapshot
            .point(point)
            .is_some_and(|point| same_bits(point.position, anchor)),
        Relation::FixLine(line, anchors) => snapshot.line(line).is_some_and(|line| {
            same_bits(line.points[0], anchors[0]) && same_bits(line.points[1], anchors[1])
        }),
        Relation::Coincident([first, second]) => {
            let (Some(a), Some(b)) = (snapshot.point(first), snapshot.point(second)) else {
                return false;
            };
            matches!(
                compare(a.position, b.position, policy.length_tolerance.value()),
                Some(Ordering::Less | Ordering::Equal)
            )
        }
        Relation::PointOnLine(point, line) => {
            incidence::point_on_line(snapshot, point, line, policy)
        }
        Relation::Colinear(lines, direction) => {
            pair(snapshot, lines).is_some_and(|lines| incidence::colinear(lines, direction, policy))
        }
        Relation::Length(line, target) => snapshot.line(line).is_some_and(|line| {
            length_band(line.points, target.value(), policy.length_tolerance.value())
        }),
        Relation::Parallel(lines, direction) => pair(snapshot, lines)
            .and_then(|[a, b]| {
                parallel(
                    scaled_direction(a.points)?,
                    scaled_direction(b.points)?,
                    direction,
                    policy.parallel_sine_tolerance,
                )
            })
            .unwrap_or(false),
        Relation::ParallelToAxis(line, axis, direction) => snapshot
            .line(line)
            .and_then(|line| {
                let axes = std::array::from_fn(|i| Interval::point(f64::from(i == axis.index())));
                let branch = match direction {
                    AxisDirection::Positive => ParallelDirection::Same,
                    AxisDirection::Negative => ParallelDirection::Opposite,
                };
                parallel(
                    scaled_direction(line.points)?,
                    axes,
                    branch,
                    policy.parallel_sine_tolerance,
                )
            })
            .unwrap_or(false),
        Relation::ParallelToPlane(line, normal) => snapshot
            .line(line)
            .and_then(|line| {
                let normal = scaled_vector(*normal.coordinates())?;
                perpendicular(
                    scaled_direction(line.points)?,
                    normal,
                    policy.parallel_sine_tolerance,
                )
            })
            .unwrap_or(false),
        Relation::Perpendicular(lines) => pair(snapshot, lines)
            .and_then(|[a, b]| {
                perpendicular(
                    scaled_direction(a.points)?,
                    scaled_direction(b.points)?,
                    policy.parallel_sine_tolerance,
                )
            })
            .unwrap_or(false),
        Relation::EqualLength(lines) => pair(snapshot, lines).is_some_and(|[a, b]| {
            let endpoints = |line: LineGeometry| line.points.map(|point| *point.coordinates());
            matches!(
                compare_length_difference_3d(
                    endpoints(a),
                    endpoints(b),
                    policy.length_tolerance.value()
                ),
                Ok(Ordering::Less | Ordering::Equal)
            )
        }),
        Relation::Angle(lines, target) => pair(snapshot, lines)
            .and_then(|lines| angle::matches(lines, target, policy.parallel_sine_tolerance))
            .unwrap_or(false),
    }
}

fn pair(snapshot: &Snapshot, [first, second]: [LineId; 2]) -> Option<[LineGeometry; 2]> {
    Some([snapshot.line(first)?, snapshot.line(second)?])
}

fn same_bits(left: Point3, right: Point3) -> bool {
    left.coordinates().map(f64::to_bits) == right.coordinates().map(f64::to_bits)
}

fn compare(a: Point3, b: Point3, limit: f64) -> Option<Ordering> {
    compare_squared_length_3d(*a.coordinates(), *b.coordinates(), limit).ok()
}

fn length_band([a, b]: [Point3; 2], target: f64, tolerance: f64) -> bool {
    if compare(a, b, target) == Some(Ordering::Equal) {
        return true;
    }
    // Compare against inward rounded endpoints so the binary64 addition cannot widen the band.
    let lower = sum_bounds(target, -tolerance).hi.max(0.);
    let upper = sum_bounds(target, tolerance).lo;
    matches!(
        compare(a, b, lower),
        Some(Ordering::Greater | Ordering::Equal)
    ) && matches!(compare(a, b, upper), Some(Ordering::Less | Ordering::Equal))
}

fn parallel(
    a: [Interval; 3],
    b: [Interval; 3],
    direction: ParallelDirection,
    tolerance: f64,
) -> Option<bool> {
    if !certified_branch(a, b, direction)? {
        return Some(false);
    }
    let cross = cross(a, b)?;
    let lhs = dot(cross, cross)?;
    within_angular_limit(a, b, lhs, tolerance)
}

fn certified_branch(
    a: [Interval; 3],
    b: [Interval; 3],
    direction: ParallelDirection,
) -> Option<bool> {
    let sign = dot(a, b)?;
    Some(match direction {
        ParallelDirection::Same => sign.lo > 0.,
        ParallelDirection::Opposite => sign.hi < 0.,
    })
}

fn perpendicular(a: [Interval; 3], b: [Interval; 3], tolerance: f64) -> Option<bool> {
    let projection = dot(a, b)?;
    within_angular_limit(a, b, projection.mul(projection)?, tolerance)
}

fn within_angular_limit(
    a: [Interval; 3],
    b: [Interval; 3],
    squared_numerator: Interval,
    tolerance: f64,
) -> Option<bool> {
    Some(squared_numerator.hi <= angular_bound(a, b, tolerance)?.lo)
}

fn angular_bound(a: [Interval; 3], b: [Interval; 3], tolerance: f64) -> Option<Interval> {
    let sine = Interval::point(tolerance);
    sine.mul(sine)?.mul(dot(a, a)?)?.mul(dot(b, b)?)
}

fn scaled_vector(values: [f64; 3]) -> Option<[Interval; 3]> {
    let scale = values.into_iter().map(f64::abs).fold(0., f64::max);
    if scale <= 0. || !scale.is_finite() {
        return None;
    }
    let value = |i: usize| Interval::rounded(values[i] / scale, values[i] / scale);
    Some([value(0)?, value(1)?, value(2)?])
}

fn scaled_direction([a, b]: [Point3; 2]) -> Option<[Interval; 3]> {
    let a = *a.coordinates();
    let b = *b.coordinates();
    let diff: [f64; 3] = std::array::from_fn(|i| b[i] - a[i]);
    let scale = diff.into_iter().map(f64::abs).fold(0., f64::max);
    if scale <= 0. || !scale.is_finite() {
        return None;
    }
    let bound = |i: usize| {
        let difference = sum_bounds(b[i], -a[i]);
        Interval::rounded(difference.lo / scale, difference.hi / scale)
    };
    Some([bound(0)?, bound(1)?, bound(2)?])
}

fn sum_bounds(left: f64, right: f64) -> Interval {
    let (large, small) = if left.abs() >= right.abs() {
        (left, right)
    } else {
        (right, left)
    };
    let sum = large + small;
    if !sum.is_finite() {
        return Interval {
            lo: sum.next_down(),
            hi: sum.next_up(),
        };
    }
    // FastTwoSum's magnitude order keeps the subtraction finite near binary64 extremes.
    let remainder = small - (sum - large);
    Interval {
        lo: if remainder < 0. { sum.next_down() } else { sum },
        hi: if remainder > 0. { sum.next_up() } else { sum },
    }
}

#[derive(Clone, Copy)]
struct Interval {
    lo: f64,
    hi: f64,
}
impl Interval {
    fn point(value: f64) -> Self {
        Self {
            lo: value,
            hi: value,
        }
    }
    fn rounded(lo: f64, hi: f64) -> Option<Self> {
        let (lo, hi) = (lo.next_down(), hi.next_up());
        (lo.is_finite() && hi.is_finite()).then_some(Self { lo, hi })
    }
    fn add(self, other: Self) -> Option<Self> {
        Self::rounded(self.lo + other.lo, self.hi + other.hi)
    }
    fn sub(self, other: Self) -> Option<Self> {
        Self::rounded(self.lo - other.hi, self.hi - other.lo)
    }
    fn mul(self, other: Self) -> Option<Self> {
        let products = [
            self.lo * other.lo,
            self.lo * other.hi,
            self.hi * other.lo,
            self.hi * other.hi,
        ];
        if products.iter().any(|p| !p.is_finite()) {
            return None;
        }
        Self::rounded(
            products.into_iter().fold(f64::INFINITY, f64::min),
            products.into_iter().fold(f64::NEG_INFINITY, f64::max),
        )
    }
}

fn dot(a: [Interval; 3], b: [Interval; 3]) -> Option<Interval> {
    a[0].mul(b[0])?.add(a[1].mul(b[1])?)?.add(a[2].mul(b[2])?)
}

fn cross(a: [Interval; 3], b: [Interval; 3]) -> Option<[Interval; 3]> {
    let component = |i: usize, j: usize| a[i].mul(b[j])?.sub(a[j].mul(b[i])?);
    Some([component(1, 2)?, component(2, 0)?, component(0, 1)?])
}

#[cfg(test)]
mod tests {
    use super::sum_bounds;

    #[test]
    fn finite_near_max_sum_keeps_both_neighbors_of_the_exact_midpoint() {
        let half_ulp = f64::from_bits((970_u64 + 1023) << 52);
        let large = f64::MAX;
        let small = -3. * half_ulp;
        let high = large.next_down();
        let low = high.next_down();
        for (left, right) in [(small, large), (large, small)] {
            let positive = sum_bounds(left, right);
            assert_eq!([positive.lo, positive.hi], [low, high]);
            let negative = sum_bounds(-left, -right);
            assert_eq!([negative.lo, negative.hi], [-high, -low]);
        }
    }
}
