// SPDX-License-Identifier: GPL-2.0-only
//! Validate incidence independently of the solver's moving frame.
use super::{Interval, angular_bound, cross, dot, scaled_direction, sum_bounds};
use crate::constraints::incidence::{SharedEnds, shared_ends};
use crate::constraints::{ConstraintPolicy, ParallelDirection};
use crate::{LineGeometry, LineId, PointId, Snapshot};
use martensite_tkmath::{
    gp::Point3,
    predicates::{compare_directed_line_sine_3d, compare_triangle_altitude_3d},
};
use std::cmp::Ordering;

pub(in crate::constraints) fn line_branch(
    lines: [LineGeometry; 2],
    direction: ParallelDirection,
) -> bool {
    if let SharedEnds::Both(intrinsic) = shared_ends(lines) {
        return intrinsic == direction;
    }
    let [a, b] = lines;
    let filtered = scaled_direction(a.points)
        .zip(scaled_direction(b.points))
        .and_then(|(a, b)| branch_filter(a, b));
    filtered.or_else(|| exact_angular(lines, 0.).map(|(branch, _)| branch))
        == Some(required_branch(direction))
}

fn branch_filter(a: [Interval; 3], b: [Interval; 3]) -> Option<Ordering> {
    let product = dot(a, b)?;
    if product.lo > 0. {
        Some(Ordering::Greater)
    } else if product.hi < 0. {
        Some(Ordering::Less)
    } else {
        None
    }
}

fn required_branch(direction: ParallelDirection) -> Ordering {
    match direction {
        ParallelDirection::Same => Ordering::Greater,
        ParallelDirection::Opposite => Ordering::Less,
    }
}

pub(super) fn colinear(
    lines: [LineGeometry; 2],
    direction: ParallelDirection,
    policy: ConstraintPolicy,
) -> bool {
    let shared = shared_ends(lines);
    if let SharedEnds::Both(intrinsic) = shared {
        return intrinsic == direction;
    }
    let [a, b] = lines;
    parallel_lines(lines, direction, policy.parallel_sine_tolerance)
        && (!matches!(shared, SharedEnds::None)
            || altitude(a.points, b.points[0], policy.length_tolerance.value()))
}

fn parallel_lines(lines: [LineGeometry; 2], direction: ParallelDirection, sine: f64) -> bool {
    let [a, b] = lines;
    let branch = required_branch(direction);
    let filtered = scaled_direction(a.points)
        .zip(scaled_direction(b.points))
        .and_then(|(a, b)| angular_filter(a, b, branch, sine));
    filtered.unwrap_or_else(|| {
        exact_angular(lines, sine)
            .is_some_and(|(actual, limit)| actual == branch && limit != Ordering::Greater)
    })
}

fn angular_filter(a: [Interval; 3], b: [Interval; 3], branch: Ordering, sine: f64) -> Option<bool> {
    if branch_filter(a, b)? != branch {
        return Some(false);
    }
    let cross = cross(a, b)?;
    let observed = dot(cross, cross)?;
    interval_at_most(observed, angular_bound(a, b, sine)?)
}

fn exact_angular(lines: [LineGeometry; 2], sine: f64) -> Option<(Ordering, Ordering)> {
    let [a, b] = lines.map(|line| line.points.map(|point| *point.coordinates()));
    compare_directed_line_sine_3d(a, b, sine).ok()
}

pub(super) fn point_on_line(
    snapshot: &Snapshot,
    point: PointId,
    line: LineId,
    policy: ConstraintPolicy,
) -> bool {
    let (Some(point), Some(line)) = (snapshot.point(point), snapshot.line(line)) else {
        return false;
    };
    altitude(line.points, point.position, policy.length_tolerance.value())
}

fn altitude(line: [Point3; 2], point: Point3, limit: f64) -> bool {
    altitude_filter(line, point, limit).unwrap_or_else(|| {
        let [a, b] = line.map(|point| *point.coordinates());
        matches!(
            compare_triangle_altitude_3d(a, b, *point.coordinates(), limit),
            Ok(Ordering::Less | Ordering::Equal)
        )
    })
}

fn altitude_filter(line: [Point3; 2], point: Point3, limit: f64) -> Option<bool> {
    let direction = scaled_direction(line)?;
    let a = line[0].coordinates();
    let b = point.coordinates();
    let offset = std::array::from_fn(|i| sum_bounds(b[i], -a[i]));
    let transverse = cross(direction, offset)?;
    let observed = dot(transverse, transverse)?;
    let limit = Interval::point(limit);
    let allowed = limit.mul(limit)?.mul(dot(direction, direction)?)?;
    interval_at_most(observed, allowed)
}

fn interval_at_most(observed: Interval, allowed: Interval) -> Option<bool> {
    if observed.hi <= allowed.lo {
        Some(true)
    } else if observed.lo > allowed.hi {
        Some(false)
    } else {
        None
    }
}
