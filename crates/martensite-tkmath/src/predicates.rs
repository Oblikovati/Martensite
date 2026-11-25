// SPDX-License-Identifier: GPL-2.0-only
//! Exact signs of supplied binary64 coordinates, separate from metric tolerance.
//! This extension does not change the metric decisions.

mod affine_point;
/// Exact injectivity certificates for supported bilinear patches.
pub mod bilinear_patch;
mod compare;
mod line_trace;
mod plane_sign;
/// Certified polynomial roots for extension topology.
pub mod roots;
mod segment_contact;
mod triangle;
mod triple_product;
pub use affine_point::compare_affine_point_error_3d;
pub use compare::{
    compare_line_projections_2d, compare_squared_distances_2d, compare_squared_length_3d,
};
pub use line_trace::{
    affine_line_trace_bound, placed_affine_line_trace_bound, placed_point_distance_upper_bound,
    point_distance_upper_bound,
};
pub use plane_sign::{compare_plane_direction_3d, compare_tetrahedron_orientation_3d};
pub use segment_contact::{SegmentContact3d, exact_segment_contact_3d};
pub use triangle::{
    compare_triangle_altitude_3d, compare_triangle_orientation_3d,
    compare_triangle_projected_dot_3d, point_on_anchored_direction_3d,
};
pub use triple_product::compare_scalar_triple_product_3d;

use num_bigint::{BigInt, Sign};
use std::fmt;

/// Exact determinant sign. Zero means exact collinearity, not metric proximity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OrientationSign {
    /// The points turn clockwise.
    Negative,
    /// The points are collinear (including coincident points).
    Zero,
    /// The points turn counterclockwise.
    Positive,
}

/// A coordinate outside the finite binary64 domain, in input traversal order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NonFiniteCoordinate {
    /// Zero-based supplied point or vector index (a, b, c for orientation).
    pub point: usize,
    /// Zero-based coordinate index (x, y, and z for three-dimensional inputs).
    pub axis: usize,
    /// Rejected coordinate, including its NaN or infinity payload.
    pub value: f64,
}

impl fmt::Display for NonFiniteCoordinate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "point {} axis {} is {}; expected a finite binary64 coordinate",
            self.point, self.axis, self.value
        )
    }
}

impl std::error::Error for NonFiniteCoordinate {}

/// Return the exact sign of the oriented triangle a, b, c.
///
/// No floating arithmetic, epsilon, or coordinate perturbation is used. This
/// classifies the supplied coordinates; it does not certify their construction.
/// ```
/// use martensite_tkmath::predicates::{orient2d, OrientationSign};
/// assert_eq!(orient2d([0., 0.], [1., 0.], [0., 1.]), Ok(OrientationSign::Positive));
/// ```
pub fn orient2d(
    a: [f64; 2],
    b: [f64; 2],
    c: [f64; 2],
) -> Result<OrientationSign, NonFiniteCoordinate> {
    let [ax, ay] = exact_point(a, 0)?;
    let [bx, by] = exact_point(b, 1)?;
    let [cx, cy] = exact_point(c, 2)?;
    let determinant = exact_determinant(&[bx - &ax, by - &ay], &[cx - ax, cy - ay]);
    Ok(match determinant.sign() {
        Sign::Minus => OrientationSign::Negative,
        Sign::NoSign => OrientationSign::Zero,
        Sign::Plus => OrientationSign::Positive,
    })
}

fn exact_dot(left: &[BigInt; 2], right: &[BigInt; 2]) -> BigInt {
    &left[0] * &right[0] + &left[1] * &right[1]
}

fn exact_determinant(left: &[BigInt; 2], right: &[BigInt; 2]) -> BigInt {
    &left[0] * &right[1] - &left[1] * &right[0]
}

fn exact_point(point: [f64; 2], index: usize) -> Result<[BigInt; 2], NonFiniteCoordinate> {
    Ok([
        exact_coordinate(point[0], index, 0)?,
        exact_coordinate(point[1], index, 1)?,
    ])
}

fn exact_vector3(vector: [f64; 3], index: usize) -> Result<[BigInt; 3], NonFiniteCoordinate> {
    Ok([
        exact_coordinate(vector[0], index, 0)?,
        exact_coordinate(vector[1], index, 1)?,
        exact_coordinate(vector[2], index, 2)?,
    ])
}

fn exact_coordinate(value: f64, point: usize, axis: usize) -> Result<BigInt, NonFiniteCoordinate> {
    let parts = binary64_parts(value, point, axis)?;
    let integer = BigInt::from(parts.significand) << parts.shift;
    Ok(if parts.negative { -integer } else { integer })
}

struct Binary64Parts {
    significand: u64,
    shift: usize,
    negative: bool,
}

fn binary64_parts(
    value: f64,
    point: usize,
    axis: usize,
) -> Result<Binary64Parts, NonFiniteCoordinate> {
    if !value.is_finite() {
        return Err(NonFiniteCoordinate { point, axis, value });
    }
    let bits = value.to_bits();
    let exponent = ((bits >> 52) & 0x7ff) as usize;
    let fraction = bits & ((1_u64 << 52) - 1);
    let significand = fraction | if exponent == 0 { 0 } else { 1_u64 << 52 };
    // Every finite binary64 is an integer multiple of 2^-1074. The common
    // positive scale cancels from the sign; subnormals need no rounded arithmetic.
    Ok(Binary64Parts {
        significand,
        shift: exponent.saturating_sub(1),
        negative: bits >> 63 != 0,
    })
}
