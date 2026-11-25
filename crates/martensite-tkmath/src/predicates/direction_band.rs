// SPDX-License-Identifier: GPL-2.0-only
use super::{
    NonFiniteCoordinate, exact_coordinate, exact_vector3, triangle::dot3,
    triple_product::exact_cross,
};
use num_bigint::BigInt;
use std::cmp::Ordering;

/// Compare the endpoint displacements' dot product with zero and their squared angle sine
/// with the squared limit. Endpoints are indexed 0–3 in line order, and the limit is operand 4.
/// The caller must admit both lines as nonzero. A zero displacement produces two equalities.
/// Example: `compare_directed_line_sine_3d([[0.; 3], [1., 1., 0.]], [[0.; 3], [1., 2., 1.]], 0.5)`.
pub fn compare_directed_line_sine_3d(
    first: [[f64; 3]; 2],
    second: [[f64; 3]; 2],
    limit: f64,
) -> Result<(Ordering, Ordering), NonFiniteCoordinate> {
    let first = displacement(first, 0)?;
    let second = displacement(second, 2)?;
    let limit = exact_coordinate(limit, 4, 0)?;
    let cross = exact_cross(&first, &second);
    // Inputs are integers in units of 2^-1074. The extra squared limit on the
    // right contributes two more powers of that scale.
    const SQUARED_LIMIT_SHIFT: usize = 2 * 1074;
    let observed = dot3(&cross, &cross) << SQUARED_LIMIT_SHIFT;
    let allowed = &limit * &limit * dot3(&first, &first) * dot3(&second, &second);
    Ok((
        dot3(&first, &second).cmp(&BigInt::from(0)),
        observed.cmp(&allowed),
    ))
}

fn displacement(
    [start, end]: [[f64; 3]; 2],
    operand: usize,
) -> Result<[BigInt; 3], NonFiniteCoordinate> {
    let start = exact_vector3(start, operand)?;
    let end = exact_vector3(end, operand + 1)?;
    Ok(std::array::from_fn(|i| &end[i] - &start[i]))
}

#[cfg(test)]
mod tests {
    use super::compare_directed_line_sine_3d as compare;
    use std::cmp::Ordering::{Equal, Greater, Less};

    #[test]
    fn half_sine_is_exact_on_both_directed_branches_and_between_binary64_neighbors() {
        let a = [[0.; 3], [1., 1., 0.]];
        let b = [[0.; 3], [1., 2., 1.]];
        let reverse = [b[1], b[0]];
        for (limit, expected) in [
            (0.5_f64.next_down(), Greater),
            (0.5, Equal),
            (0.5_f64.next_up(), Less),
        ] {
            assert_eq!(compare(a, b, limit), Ok((Greater, expected)));
            assert_eq!(compare(a, reverse, limit), Ok((Less, expected)));
        }
    }

    #[test]
    fn underflowing_squared_limits_do_not_hide_a_parallel_hit_or_nearby_miss() {
        let a = [[0.; 3], [1., 0., 0.]];
        let b = [[0.; 3], [2., 0., 0.]];
        for limit in [f64::from_bits(1), f64::MIN_POSITIVE] {
            assert_eq!(compare(a, b, limit), Ok((Greater, Less)));
            assert_eq!(compare(a, [b[1], b[0]], limit), Ok((Less, Less)));
            assert_eq!(
                compare(a, [[0.; 3], [1., 2. * limit, 0.]], limit),
                Ok((Greater, Greater))
            );
        }
    }

    #[test]
    fn exact_branch_separates_cancelling_products_and_reports_nonfinite_input_order() {
        let n = 4_503_599_627_370_496.;
        let a = [[0.; 3], [n, n + 1., 0.]];
        let b = [[0.; 3], [n, -(n - 1.), 0.]];
        assert_eq!(compare(a, b, 0.5), Ok((Greater, Greater)));
        assert_eq!(compare(a, [a[0]; 2], 0.5), Ok((Equal, Equal)));
        for operand in 0..4 {
            let mut points = [a[0], a[1], b[0], b[1]];
            points[operand][1] = f64::INFINITY;
            let error = compare([points[0], points[1]], [points[2], points[3]], 0.5).unwrap_err();
            assert_eq!((error.point, error.axis), (operand, 1));
        }
        let error = compare(a, b, f64::NAN).unwrap_err();
        assert_eq!((error.point, error.axis), (4, 0));
    }
}
