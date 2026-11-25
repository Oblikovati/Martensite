// SPDX-License-Identifier: GPL-2.0-only
//! Explicit midpoint refinement tightens the same integral and supremum bounds.
use super::*;
use crate::predicates::roots::certificate_arithmetic::bernstein_curve::bisect;
#[cfg(test)]
mod tests;
pub(super) fn metrics(
    primitive: &[Interval],
    old_mean: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<(Interval, Interval)> {
    let [left, right] = bisect(primitive, arithmetic)?;
    let left_mean = absolute_coefficient_mean(&left, arithmetic)?;
    let right_mean = absolute_coefficient_mean(&right, arithmetic)?;
    let sum = arithmetic.add(&left_mean, &right_mean)?;
    let mean = arithmetic.fraction(&sum, 1, 2)?;
    let magnitude = magnitude_interval(&left)
        .upper
        .max(magnitude_interval(&right).upper);
    let magnitude = magnitude.min(magnitude_interval(primitive).upper);
    Ok((
        upper_bound(mean.upper.min(old_mean.upper.clone())),
        upper_bound(magnitude),
    ))
}
fn upper_bound(upper: num_bigint::BigInt) -> Interval {
    Interval {
        lower: 0.into(),
        upper,
    }
}
