// SPDX-License-Identifier: GPL-2.0-only
//! Shared outward Machin bound for argument reduction and cylindrical periods.
use super::{
    bounded_dyadic::{Arithmetic, Interval},
    bounded_failure::Checked,
};

fn atan_inverse(
    divisor: u64,
    bits_per_power: u16,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let terms = arithmetic.fractional_bits().div_ceil(bits_per_power);
    let one = arithmetic.scalar(1.)?;
    let mut power = arithmetic.fraction(&one, 1, divisor)?;
    let mut result = arithmetic.zero();
    for k in 0..u64::from(terms) {
        let term = arithmetic.fraction(&power, 1, 2 * k + 1)?;
        result = if k % 2 == 0 {
            arithmetic.add(&result, &term)?
        } else {
            arithmetic.subtract(&result, &term)?
        };
        power = arithmetic.fraction(&power, 1, divisor * divisor)?;
    }
    alternating_tail(result, arithmetic)
}

fn alternating_tail(result: Interval, arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
    let tail = arithmetic.quantum();
    arithmetic.add(
        &result,
        &Interval {
            lower: -tail.upper.clone(),
            upper: tail.upper,
        },
    )
}

pub(in crate::predicates::roots) fn pi(arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
    // Machin's identity; the alternating tails are bounded by one dyadic quantum.
    let fifth = atan_inverse(5, 4, arithmetic)?;
    let other = atan_inverse(239, 14, arithmetic)?;
    let first = arithmetic.fraction(&fifth, 16, 1)?;
    let last = arithmetic.fraction(&other, 4, 1)?;
    arithmetic.subtract(&first, &last)
}

#[cfg(test)]
#[path = "circular_constants_tests.rs"]
mod tests;
