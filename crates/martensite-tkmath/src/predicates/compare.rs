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
    let length_squared: BigInt = first
        .into_iter()
        .zip(second)
        .map(|(first, second)| {
            let delta = first - second;
            &delta * &delta
        })
        .sum();
    Ok(length_squared.cmp(&(&limit * &limit)))
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
