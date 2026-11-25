// SPDX-License-Identifier: GPL-2.0-only
use super::triple_product::{exact_cross, exact_triple_product};
use super::{NonFiniteCoordinate, exact_coordinate, exact_vector3};
use num_bigint::BigInt;
use std::cmp::Ordering;

/// Compare a triangle's oriented normal with the supplied direction, exactly.
///
/// A positive result means `(first − origin) × (second − origin)` points
/// toward the direction. The direction is operand 3. Example:
/// `compare_triangle_orientation_3d([0.; 3], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.])`.
pub fn compare_triangle_orientation_3d(
    origin: [f64; 3],
    first: [f64; 3],
    second: [f64; 3],
    direction: [f64; 3],
) -> Result<Ordering, NonFiniteCoordinate> {
    let Triangle { first, second } = Triangle::new(origin, first, second)?;
    let direction = exact_vector3(direction, 3)?;
    Ok(exact_triple_product(&first, &second, &direction).cmp(&BigInt::from(0)))
}

/// Compare the exact distance from `vertex` to the chord support with `|limit|`.
///
/// The caller must first certify distinct endpoints. For a zero chord both
/// cross-multiplied squares are zero. The limit is operand 3. Example:
/// `compare_triangle_altitude_3d([0.; 3], [2., 0., 0.], [1., 1., 0.], 1.)`.
pub fn compare_triangle_altitude_3d(
    start: [f64; 3],
    end: [f64; 3],
    vertex: [f64; 3],
    limit: f64,
) -> Result<Ordering, NonFiniteCoordinate> {
    let Triangle { first, second } = Triangle::new(start, end, vertex)?;
    let limit = exact_coordinate(limit, 3, 0)?;
    let cross = exact_cross(&first, &second);
    Ok(square_norm(&cross).cmp(&(&limit * &limit * square_norm(&first))))
}

/// Compare the projected dot product of two radii with zero.
///
/// `Less` means their projected dot product is negative. The plane normal
/// need not be unit length. A zero normal or radial projection yields `Equal`.
/// Example: `compare_triangle_projected_dot_3d([0.; 3], [1., 0., 3.], [-1., 0., 7.], [0., 0., 1.])`.
pub fn compare_triangle_projected_dot_3d(
    origin: [f64; 3],
    first: [f64; 3],
    second: [f64; 3],
    normal: [f64; 3],
) -> Result<Ordering, NonFiniteCoordinate> {
    let Triangle { first, second } = Triangle::new(origin, first, second)?;
    let normal = exact_vector3(normal, 3)?;
    let scaled_dot = square_norm(&normal) * dot3(&first, &second);
    Ok(scaled_dot.cmp(&(dot3(&normal, &first) * dot3(&normal, &second))))
}

struct Triangle {
    first: [BigInt; 3],
    second: [BigInt; 3],
}

impl Triangle {
    fn new(
        origin: [f64; 3],
        first: [f64; 3],
        second: [f64; 3],
    ) -> Result<Self, NonFiniteCoordinate> {
        let origin = exact_vector3(origin, 0)?;
        let first = exact_vector3(first, 1)?;
        let second = exact_vector3(second, 2)?;
        Ok(Self {
            first: std::array::from_fn(|axis| &first[axis] - &origin[axis]),
            second: std::array::from_fn(|axis| &second[axis] - &origin[axis]),
        })
    }
}

fn square_norm(vector: &[BigInt; 3]) -> BigInt {
    dot3(vector, vector)
}

fn dot3(first: &[BigInt; 3], second: &[BigInt; 3]) -> BigInt {
    first.iter().zip(second).map(|(a, b)| a * b).sum()
}

#[cfg(test)]
mod tests {
    use super::compare_triangle_projected_dot_3d as projected_dot;
    use super::{
        compare_triangle_altitude_3d as altitude, compare_triangle_orientation_3d as side,
    };
    use std::cmp::Ordering::{Equal, Greater, Less};

    #[test]
    fn exact_altitude_compares_each_side_of_a_binary64_boundary() {
        let a = [10., 11., 12.];
        let b = [12., 11., 12.];
        let c = [11., 14., 16.];
        assert_eq!(altitude(a, b, c, 5.), Ok(Equal));
        assert_eq!(altitude(a, b, c, 5_f64.next_down()), Ok(Greater));
        assert_eq!(altitude(a, b, c, 5_f64.next_up()), Ok(Less));
        assert_eq!(altitude(a, b, [9., 11., 12.], 1.), Ok(Less));
    }

    #[test]
    fn orientation_resolves_a_nonzero_determinant_with_rounded_cancellation() {
        let n = 4_503_599_627_370_496.;
        let a = [-4., -3., 2.];
        let b = [n - 4., n - 4., 2.];
        let c = [n - 3., n - 3., 2.];
        let expected = [0., 0., 1.];
        assert_eq!(side(a, b, c, expected), Ok(Greater));
        assert_eq!(side(a, c, b, expected), Ok(Less));
        assert_eq!(side(a, b, c, [1., 2., 0.]), Ok(Equal));
        assert_eq!(side(a, b, c, [1., f64::INFINITY, 0.]).unwrap_err().point, 3);
    }
    #[test]
    fn projected_dot_resolves_opposed_coincident_and_axial_ranges() {
        let z = [0., 0., 5.];
        let origin = [8., 3., 7.];
        let first = [9., 3., 11.];
        assert_eq!(projected_dot(origin, first, [7., 3., 14.], z), Ok(Less));
        assert_eq!(projected_dot(origin, first, [9., 3., -1.], z), Ok(Greater));
        assert_eq!(projected_dot(origin, first, [8., 3., 14.], z), Ok(Equal));
        assert_eq!(projected_dot(origin, first, [8., 4., 14.], z), Ok(Equal));
    }
}
