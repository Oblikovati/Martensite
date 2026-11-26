// SPDX-License-Identifier: GPL-2.0-only
use super::{NonFiniteCoordinate, exact_coordinate, exact_dot, exact_point, exact_vector3};
use num_bigint::BigInt;
use std::cmp::Ordering;
/// Compare exact squared distances from `origin` to `first` and `second`.
/// No rounded arithmetic or tolerance participates. Coordinate indices are 0, 1, 2 respectively.
/// ```
/// use martensite_tkmath::predicates::compare_squared_distances_2d;
/// assert_eq!(compare_squared_distances_2d([0.,0.], [1.,0.], [2.,0.])?, std::cmp::Ordering::Less);
/// # Ok::<(), martensite_tkmath::predicates::NonFiniteCoordinate>(())
/// ```
pub fn compare_squared_distances_2d(
    origin: [f64; 2],
    first: [f64; 2],
    second: [f64; 2],
) -> Result<Ordering, NonFiniteCoordinate> {
    let origin = exact_point(origin, 0)?;
    let first = exact_point(first, 1)?;
    let second = exact_point(second, 2)?;
    Ok(squared_separation(&origin, &first).cmp(&squared_separation(&origin, &second)))
}
/// Compare the exact squared distance between two finite 3D points with the squared limit.
/// Operands are indexed first point 0, second point 1, and limit 2. The limit's sign
/// has no effect on its square. The comparison does not estimate the length.
/// Example: `compare_squared_length_3d([0.; 3], [3., 4., 12.], 13.)`.
pub fn compare_squared_length_3d(
    first: [f64; 3],
    second: [f64; 3],
    length_limit: f64,
) -> Result<Ordering, NonFiniteCoordinate> {
    let first = exact_vector3(first, 0)?;
    let second = exact_vector3(second, 1)?;
    let limit = exact_coordinate(length_limit, 2, 0)?;
    Ok(squared_separation_3d(first, second).cmp(&(&limit * &limit)))
}

/// Compare the absolute difference between two exact 3D segment lengths with `abs(limit)`.
/// The endpoints occupy operands 0–3 in segment order. The limit is operand 4, axis 0.
/// Example: `compare_length_difference_3d([[0.; 3], [3., 4., 0.]], [[0.; 3], [2., 0., 0.]], 3.)`.
pub fn compare_length_difference_3d(
    first: [[f64; 3]; 2],
    second: [[f64; 3]; 2],
    limit: f64,
) -> Result<Ordering, NonFiniteCoordinate> {
    let a = squared_separation_3d(exact_vector3(first[0], 0)?, exact_vector3(first[1], 1)?);
    let b = squared_separation_3d(exact_vector3(second[0], 2)?, exact_vector3(second[1], 3)?);
    let limit = exact_coordinate(limit, 4, 0)?;
    Ok(compare_radical_gap(a, b, &limit * &limit))
}

pub(super) fn compare_radical_gap(a: BigInt, b: BigInt, limit_sq: BigInt) -> Ordering {
    let (longer, shorter) = if a >= b { (a, b) } else { (b, a) };
    // sqrt(longer) <= sqrt(shorter) + limit reduces to one exact sign, then a square.
    let remainder = longer - &shorter - &limit_sq;
    if remainder < BigInt::from(0) {
        return Ordering::Less;
    }
    (&remainder * &remainder).cmp(&(4 * shorter * limit_sq))
}

fn squared_separation_3d(first: [BigInt; 3], second: [BigInt; 3]) -> BigInt {
    first
        .into_iter()
        .zip(second)
        .map(|(first, second)| {
            let delta = first - second;
            &delta * &delta
        })
        .sum()
}
/// Compare projections of `first` and `second` along the directed `start`→`end` support.
/// Indices are start=0, end=1, first=2, second=3. A zero direction returns equality;
/// this is an ordering of supplied points, not validation of a constructible line.
/// ```
/// use martensite_tkmath::predicates::compare_line_projections_2d;
/// assert_eq!(compare_line_projections_2d([0.,0.], [1.,0.], [3.,8.], [2.,9.])?, std::cmp::Ordering::Greater);
/// # Ok::<(), martensite_tkmath::predicates::NonFiniteCoordinate>(())
/// ```
pub fn compare_line_projections_2d(
    start: [f64; 2],
    end: [f64; 2],
    first: [f64; 2],
    second: [f64; 2],
) -> Result<Ordering, NonFiniteCoordinate> {
    let start = exact_point(start, 0)?;
    let end = exact_point(end, 1)?;
    let first = exact_point(first, 2)?;
    let second = exact_point(second, 3)?;
    let direction = difference(&end, &start);
    let displacement = difference(&first, &second);
    let product = exact_dot(&direction, &displacement);
    Ok(product.cmp(&BigInt::from(0)))
}
fn squared_separation(first: &[BigInt; 2], second: &[BigInt; 2]) -> BigInt {
    let displacement = difference(first, second);
    exact_dot(&displacement, &displacement)
}
fn difference(first: &[BigInt; 2], second: &[BigInt; 2]) -> [BigInt; 2] {
    [&first[0] - &second[0], &first[1] - &second[1]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn difference_uses_the_exact_closed_length_threshold_and_zero_cases() {
        let five = [[0.; 3], [3., 4., 0.]];
        let two = [[10., 0., 0.], [12., 0., 0.]];
        for (limit, expected) in [
            (3.0_f64.next_down(), Ordering::Greater),
            (3., Ordering::Equal),
            (-3., Ordering::Equal),
            (3.0_f64.next_up(), Ordering::Less),
        ] {
            assert_eq!(compare_length_difference_3d(five, two, limit), Ok(expected));
            assert_eq!(compare_length_difference_3d(two, five, limit), Ok(expected));
        }
        assert_eq!(
            compare_length_difference_3d(five, [[0.; 3]; 2], 5.),
            Ok(Ordering::Equal)
        );
        assert_eq!(
            compare_length_difference_3d(five, five, -0.),
            Ok(Ordering::Equal)
        );
        assert_eq!(
            compare_length_difference_3d(two, five, 0.),
            Ok(Ordering::Greater)
        );
    }

    #[test]
    fn difference_handles_binary64_extremes_and_an_unrounded_transverse_increment() {
        let extent = f64::MAX;
        let x = [[-extent, 0., 0.], [extent, 0., 0.]];
        let y = [[0., extent, 0.], [0., -extent, 0.]];
        assert_eq!(compare_length_difference_3d(x, y, 0.), Ok(Ordering::Equal));
        assert_eq!(
            compare_length_difference_3d(x, y, f64::from_bits(1)),
            Ok(Ordering::Less)
        );
        let extended = [[0.; 3], [1., f64::EPSILON, 0.]];
        let axial = [[0.; 3], [1., 0., 0.]];
        assert_eq!(
            compare_length_difference_3d(extended, axial, 0.),
            Ok(Ordering::Greater)
        );
        assert_eq!(
            compare_length_difference_3d(extended, axial, f64::EPSILON),
            Ok(Ordering::Less)
        );
    }

    #[test]
    fn difference_reports_original_operand_indices_before_ordering_lengths() {
        for operand in 0..4 {
            let mut points = [[0.; 3]; 4];
            points[operand][1] = f64::INFINITY;
            let error =
                compare_length_difference_3d([points[0], points[1]], [points[2], points[3]], 0.)
                    .unwrap_err();
            assert_eq!(
                (error.point, error.axis, error.value),
                (operand, 1, f64::INFINITY)
            );
        }
        let error = compare_length_difference_3d([[0.; 3]; 2], [[0.; 3]; 2], f64::NAN).unwrap_err();
        assert_eq!((error.point, error.axis), (4, 0));
        assert!(error.value.is_nan());
    }
}
