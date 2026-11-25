// SPDX-License-Identifier: GPL-2.0-only
//! Validate incidence independently of the solver's moving frame.
use super::{Interval, certified_branch, cross, dot, parallel, scaled_direction, sum_bounds};
use crate::constraints::incidence::{SharedEnds, shared_ends};
use crate::constraints::{ConstraintPolicy, ParallelDirection};
use crate::{LineGeometry, LineId, PointId, Snapshot};
use martensite_tkmath::{gp::Point3, predicates::compare_triangle_altitude_3d};
use std::cmp::Ordering;

pub(in crate::constraints) fn line_branch(
    lines: [LineGeometry; 2],
    direction: ParallelDirection,
) -> bool {
    if let SharedEnds::Both(intrinsic) = shared_ends(lines) {
        return intrinsic == direction;
    }
    raw_branch(lines, direction).unwrap_or(false)
}

fn raw_branch([a, b]: [LineGeometry; 2], direction: ParallelDirection) -> Option<bool> {
    certified_branch(
        scaled_direction(a.points)?,
        scaled_direction(b.points)?,
        direction,
    )
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

fn parallel_lines([a, b]: [LineGeometry; 2], direction: ParallelDirection, sine: f64) -> bool {
    let (Some(a), Some(b)) = (scaled_direction(a.points), scaled_direction(b.points)) else {
        return false;
    };
    parallel(a, b, direction, sine).unwrap_or(false)
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
    if observed.hi <= allowed.lo {
        Some(true)
    } else if observed.lo > allowed.hi {
        Some(false)
    } else {
        None
    }
}
