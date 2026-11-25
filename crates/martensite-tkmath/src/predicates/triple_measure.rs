// SPDX-License-Identifier: GPL-2.0-only
//! Binary64 enclosure of an exact dyadic triple product.
use super::{NonFiniteCoordinate, exact_vector3, triple_product::exact_triple_product};
use crate::math_poly::extended::scale_double;
use num_bigint::{BigInt, BigUint, Sign};
use std::cmp::Ordering;

/// Return the exact determinant sign and bounds enclosing its absolute value.
/// A finite determinant outside the binary64 range can have an infinite upper bound.
/// Example: `enclose_scalar_triple_product_3d([1.,0.,0.], [0.,1.,0.], [0.,0.,2.])?`.
pub fn enclose_scalar_triple_product_3d(
    first: [f64; 3],
    second: [f64; 3],
    third: [f64; 3],
) -> Result<(Ordering, [f64; 2]), NonFiniteCoordinate> {
    let first = exact_vector3(first, 0)?;
    let second = exact_vector3(second, 1)?;
    let third = exact_vector3(third, 2)?;
    let determinant = exact_triple_product(&first, &second, &third);
    let sign = determinant.cmp(&BigInt::from(0));
    let bounds = magnitude_bounds(determinant.magnitude(), determinant.sign());
    Ok((sign, bounds))
}

fn magnitude_bounds(magnitude: &BigUint, sign: Sign) -> [f64; 2] {
    if sign == Sign::NoSign {
        return [0., 0.];
    }
    // The exact vectors are binary64 in units of 2^-1074. Keep the top 53 bits
    // of the determinant integer and enclose the discarded bits by one unit.
    let shift = magnitude.bits().saturating_sub(53) as usize;
    let top = (magnitude >> shift).iter_u64_digits().next().unwrap_or(0);
    let power = shift as i32 - 3 * 1074;
    let floor = scale_double(top as f64, power).next_down().max(0.);
    let ceil = scale_double((top + 1) as f64, power).next_up();
    [floor, ceil]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_sign_and_enclosure_survive_cancellation_and_subnormal_output() {
        let n = 4_503_599_627_370_496.;
        let (sign, [lo, hi]) =
            enclose_scalar_triple_product_3d([n, n - 1., 0.], [n + 1., n, 0.], [0., 0., -1.])
                .unwrap();
        assert_eq!(sign, Ordering::Less);
        assert!(lo <= 1. && hi >= 1. && hi - lo < 2e-15);

        let s = f64::from_bits(1);
        let oblique = enclose_scalar_triple_product_3d([s, 0., 0.], [0., s, 0.], [0., 0., s]);
        assert_eq!(oblique, Ok((Ordering::Greater, [0., s])));
        let zero = enclose_scalar_triple_product_3d([1., 0., 0.], [1., 0., 0.], [0., 0., s]);
        assert_eq!(zero, Ok((Ordering::Equal, [0., 0.])));
    }
}
