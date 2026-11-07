// SPDX-License-Identifier: GPL-2.0-only
use super::{NonFiniteCoordinate, exact_coordinate};
use num_bigint::BigInt;
use std::cmp::Ordering;

/// Exact sign of half the squared-distance second derivative for supplied coordinates.
/// The caller checks the stationary residual and conditioning separately from this sign.
/// ```
/// use martensite_tkmath::predicates::distance_second_variation_sign;
/// assert_eq!(distance_second_variation_sign([0.;3], [1.,0.,0.], [0.,1.,0.], [-1.,0.,0.])?, std::cmp::Ordering::Equal);
/// # Ok::<(), martensite_tkmath::predicates::NonFiniteCoordinate>(())
/// ```
pub fn distance_second_variation_sign(
    query: [f64; 3],
    position: [f64; 3],
    tangent: [f64; 3],
    second: [f64; 3],
) -> Result<Ordering, NonFiniteCoordinate> {
    let mut variation = BigInt::from(0);
    for axis in 0..3 {
        let q = exact_coordinate(query[axis], 0, axis)?;
        let p = exact_coordinate(position[axis], 1, axis)?;
        let d1 = exact_coordinate(tangent[axis], 2, axis)?;
        let d2 = exact_coordinate(second[axis], 3, axis)?;
        variation += &d1 * &d1 + (p - q) * d2;
    }
    Ok(variation.cmp(&BigInt::from(0)))
}
