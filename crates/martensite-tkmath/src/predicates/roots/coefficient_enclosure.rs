// SPDX-License-Identifier: GPL-2.0-only
//! Exact power-basis interval Horner evaluation shared by root certificates.
use super::{
    RootBracket, RootIsolationError,
    certificate_arithmetic::{ExactRatio, RatioInterval},
    error::poll,
};
use num_bigint::BigInt;
pub(super) fn power_enclosure(
    coefficients: &[BigInt],
    parameter: &RatioInterval,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<RatioInterval, RootIsolationError> {
    let mut value = RatioInterval::point(ExactRatio::from_finite(0.));
    for coefficient in coefficients.iter().rev() {
        poll(cancelled)?;
        let term = RatioInterval::point(ExactRatio::new(coefficient.clone(), BigInt::from(1)));
        value = value.multiply(parameter).add(&term);
    }
    Ok(value)
}
pub(super) fn parameter_interval(root: &RootBracket) -> RatioInterval {
    RatioInterval {
        lower: ExactRatio::from_dyadic(root.bounds().0),
        upper: ExactRatio::from_dyadic(root.bounds().1),
    }
}
