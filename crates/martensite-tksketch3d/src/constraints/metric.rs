// SPDX-License-Identifier: GPL-2.0-only
//! Independent physical admission from the accepted endpoint coordinates.
use super::{ConstraintPolicy, ParallelDirection, Relation};
use crate::{LineGeometry, Snapshot};
use martensite_tkmath::{gp::Point3, predicates::compare_squared_length_3d};
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
        Relation::Length(line, target) => snapshot.line(line).is_some_and(|line| {
            length_band(line.points, target.value(), policy.length_tolerance.value())
        }),
        Relation::Parallel([first, second], direction) => {
            let (Some(a), Some(b)) = (snapshot.line(first), snapshot.line(second)) else {
                return false;
            };
            parallel(a, b, direction, policy.parallel_sine_tolerance).unwrap_or(false)
        }
    }
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
    a: LineGeometry,
    b: LineGeometry,
    direction: ParallelDirection,
    tolerance: f64,
) -> Option<bool> {
    let a = scaled_direction(a.points)?;
    let b = scaled_direction(b.points)?;
    let sign = dot(a, b)?;
    let correct_branch = match direction {
        ParallelDirection::Same => sign.lo > 0.,
        ParallelDirection::Opposite => sign.hi < 0.,
    };
    if !correct_branch {
        return Some(false);
    }
    let cross = cross(a, b)?;
    let lhs = dot(cross, cross)?;
    let sine = Interval::point(tolerance);
    let rhs = sine.mul(sine)?.mul(dot(a, a)?)?.mul(dot(b, b)?)?;
    Some(lhs.hi <= rhs.lo)
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
    let sum = left + right;
    if !sum.is_finite() {
        return Interval {
            lo: sum.next_down(),
            hi: sum.next_up(),
        };
    }
    // The error-free sum preserves representable closed endpoints exactly.
    let right_part = sum - left;
    let remainder = (left - (sum - right_part)) + (right - right_part);
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
