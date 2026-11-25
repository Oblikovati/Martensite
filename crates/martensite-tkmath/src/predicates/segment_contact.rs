// SPDX-License-Identifier: GPL-2.0-only
//! Exact incidence of closed spatial segments from their original endpoints.
use super::{NonFiniteCoordinate, exact_vector3};
use num_bigint::{BigInt, Sign};

/// Topological incidence of two closed 3D line segments.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SegmentContact3d {
    /// The closed segments have empty intersection.
    Disjoint,
    /// The closed segments have exactly one point in common.
    Point,
    /// The collinear segments share a positive-length interval.
    Overlap,
}

/// Classifies the closed segments exactly over the supplied finite binary64 endpoints.
/// Zero-length segments represent points. Segment and endpoint order do not affect the result.
///
/// ```
/// use martensite_tkmath::predicates::{SegmentContact3d, exact_segment_contact_3d};
/// let first = [[0., 0., 0.], [2., 0., 0.]];
/// let second = [[2., -1., 0.], [2., 1., 0.]];
/// assert_eq!(exact_segment_contact_3d(first, second), Ok(SegmentContact3d::Point));
/// ```
pub fn exact_segment_contact_3d(
    first: [[f64; 3]; 2],
    second: [[f64; 3]; 2],
) -> Result<SegmentContact3d, NonFiniteCoordinate> {
    finite_endpoints(first, second)?;
    if (0..3).any(|axis| interval_contact(first, second, axis) == SegmentContact3d::Disjoint) {
        return Ok(SegmentContact3d::Disjoint);
    }
    let [a, b, c, d] = [
        exact_vector3(first[0], 0)?,
        exact_vector3(first[1], 1)?,
        exact_vector3(second[0], 2)?,
        exact_vector3(second[1], 3)?,
    ];
    Ok(classify(
        first,
        second,
        difference(&b, &a),
        difference(&d, &c),
        difference(&c, &a),
    ))
}

fn finite_endpoints(
    first: [[f64; 3]; 2],
    second: [[f64; 3]; 2],
) -> Result<(), NonFiniteCoordinate> {
    for (point, coordinates) in first.into_iter().chain(second).enumerate() {
        if let Some(axis) = coordinates.iter().position(|value| !value.is_finite()) {
            return Err(NonFiniteCoordinate {
                point,
                axis,
                value: coordinates[axis],
            });
        }
    }
    Ok(())
}

fn interval_contact(first: [[f64; 3]; 2], second: [[f64; 3]; 2], axis: usize) -> SegmentContact3d {
    let [a, b] = first.map(|point| point[axis]);
    let [c, d] = second.map(|point| point[axis]);
    let low = a.min(b).max(c.min(d));
    let high = a.max(b).min(c.max(d));
    if low > high {
        return SegmentContact3d::Disjoint;
    }
    if low == high {
        return SegmentContact3d::Point;
    }
    SegmentContact3d::Overlap
}

fn classify(
    first: [[f64; 3]; 2],
    second: [[f64; 3]; 2],
    r: [BigInt; 3],
    s: [BigInt; 3],
    delta: [BigInt; 3],
) -> SegmentContact3d {
    match exact_support_parameters(&r, &s, &delta) {
        SupportParameters::Transverse {
            numerators: [t, u],
            denominator,
        } => {
            if unit_interval(&t, &denominator) && unit_interval(&u, &denominator) {
                return SegmentContact3d::Point;
            }
            SegmentContact3d::Disjoint
        }
        SupportParameters::Collinear(Some(axis)) => interval_contact(first, second, axis),
        SupportParameters::Collinear(None) => SegmentContact3d::Point,
        SupportParameters::Parallel | SupportParameters::Skew => SegmentContact3d::Disjoint,
    }
}

pub(super) enum SupportParameters {
    Parallel,
    Skew,
    Collinear(Option<usize>),
    Transverse {
        numerators: [BigInt; 2],
        denominator: BigInt,
    },
}

pub(super) fn exact_support_parameters(
    first: &[BigInt; 3],
    second: &[BigInt; 3],
    delta: &[BigInt; 3],
) -> SupportParameters {
    let normal = cross(first, second);
    if let Some(axis) = nonzero_axis(&normal) {
        return transverse_contact(first, second, delta, normal, axis);
    }
    let direction = if nonzero_axis(first).is_some() {
        first
    } else {
        second
    };
    if nonzero_axis(&cross(delta, direction)).is_some() {
        return SupportParameters::Parallel;
    }
    SupportParameters::Collinear(nonzero_axis(direction))
}

fn transverse_contact(
    first: &[BigInt; 3],
    second: &[BigInt; 3],
    delta: &[BigInt; 3],
    normal: [BigInt; 3],
    axis: usize,
) -> SupportParameters {
    let plane: BigInt = (0..3).map(|i| &delta[i] * &normal[i]).sum();
    if plane.sign() != Sign::NoSign {
        return SupportParameters::Skew;
    }
    // The nonzero normal component selects an exact two-dimensional solve.
    let [x, y, z] = normal;
    let denominator = match axis {
        0 => x,
        1 => y,
        _ => z,
    };
    SupportParameters::Transverse {
        numerators: [
            cross_component(delta, second, axis),
            cross_component(delta, first, axis),
        ],
        denominator,
    }
}

fn unit_interval(numerator: &BigInt, denominator: &BigInt) -> bool {
    (numerator.sign() == Sign::NoSign || numerator.sign() == denominator.sign())
        && numerator.magnitude() <= denominator.magnitude()
}

fn nonzero_axis(vector: &[BigInt; 3]) -> Option<usize> {
    vector
        .iter()
        .position(|component| component.sign() != Sign::NoSign)
}

pub(super) fn difference(first: &[BigInt; 3], second: &[BigInt; 3]) -> [BigInt; 3] {
    std::array::from_fn(|i| &first[i] - &second[i])
}

fn cross(first: &[BigInt; 3], second: &[BigInt; 3]) -> [BigInt; 3] {
    std::array::from_fn(|axis| cross_component(first, second, axis))
}

fn cross_component(first: &[BigInt; 3], second: &[BigInt; 3], axis: usize) -> BigInt {
    let a = (axis + 1) % 3;
    let b = (axis + 2) % 3;
    &first[a] * &second[b] - &first[b] * &second[a]
}

#[cfg(test)]
#[path = "segment_contact/tests.rs"]
mod tests;
