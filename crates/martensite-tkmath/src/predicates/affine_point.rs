// SPDX-License-Identifier: GPL-2.0-only
//! Exact position comparison against supplied affine coefficients.
use super::{NonFiniteCoordinate, exact_coordinate, exact_vector3};
use num_bigint::BigInt;
use std::cmp::Ordering;

/// Compare the distance from `target` to `out_anchor + matrix × (source − in_anchor)` with `|limit|`.
/// The comparison uses exact supplied binary64 coefficients and coordinates.
/// Operand indices are source 0, target 1, input/output anchors 2/3, rows 4–6, and limit 7.
/// Example: `compare_affine_point_error_3d(source, target, anchors, matrix, 1e-8)`.
pub fn compare_affine_point_error_3d(
    source: [f64; 3],
    target: [f64; 3],
    [in_anchor, out_anchor]: [[f64; 3]; 2],
    matrix: [[f64; 3]; 3],
    limit: f64,
) -> Result<Ordering, NonFiniteCoordinate> {
    let source = exact_vector3(source, 0)?;
    let target = exact_vector3(target, 1)?;
    let input = exact_vector3(in_anchor, 2)?;
    let output = exact_vector3(out_anchor, 3)?;
    let local: [BigInt; 3] = std::array::from_fn(|axis| &source[axis] - &input[axis]);
    let squared_error = squared_error(matrix, &local, &target, &output)?;
    let limit = to_product_scale(exact_coordinate(limit, 7, 0)?);
    Ok(squared_error.cmp(&(&limit * &limit)))
}

fn squared_error(
    matrix: [[f64; 3]; 3],
    local: &[BigInt; 3],
    target: &[BigInt; 3],
    output: &[BigInt; 3],
) -> Result<BigInt, NonFiniteCoordinate> {
    let mut squared = BigInt::from(0);
    for (axis, row) in matrix.into_iter().enumerate() {
        let coefficients = exact_vector3(row, 4 + axis)?;
        let mapped: BigInt = coefficients.iter().zip(local).map(|(a, b)| a * b).sum();
        let error = to_product_scale(&target[axis] - &output[axis]) - mapped;
        squared += &error * &error;
    }
    Ok(squared)
}

fn to_product_scale(coordinate: BigInt) -> BigInt {
    // The dyadic decoder uses 2^-1074 units. A product has twice that exponent.
    coordinate << 1074
}

#[cfg(test)]
mod tests {
    use super::compare_affine_point_error_3d as compare;
    use std::cmp::Ordering::{Equal, Greater, Less};

    const IDENTITY: [[f64; 3]; 3] = [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]];

    #[test]
    fn compares_the_exact_unit_lost_when_a_rounded_translation_lands_on_a_midpoint() {
        let large = f64::from_bits(1076_u64 << 52);
        let anchors = [[0.; 3], [large, 0., 0.]];
        let first = [1., 0., 0.];
        let target = [large, 0., 0.];
        assert_eq!(compare(first, target, anchors, IDENTITY, 1.), Ok(Equal));
        assert_eq!(
            compare(first, target, anchors, IDENTITY, 1_f64.next_down()),
            Ok(Greater)
        );
        assert_eq!(
            compare(first, target, anchors, IDENTITY, 1_f64.next_up()),
            Ok(Less)
        );
    }

    #[test]
    fn exact_relative_rotation_and_input_errors_use_declared_indices() {
        let quarter = [[0., -1., 0.], [1., 0., 0.], [0., 0., 1.]];
        let anchor = [2., 1., -1.];
        let source = [4., 1., 1.];
        assert_eq!(
            compare(source, [2., 3., 1.], [anchor; 2], quarter, 0.),
            Ok(Equal)
        );
        let mut nonfinite = quarter;
        nonfinite[1][2] = f64::NAN;
        let error = compare(source, [2., 3., 1.], [anchor; 2], nonfinite, 0.).unwrap_err();
        assert_eq!((error.point, error.axis), (5, 2));
    }
}
