// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::{
    DyadicBound,
    certificate_arithmetic::{
        ExactRatio, bounded_dyadic::ArithmeticPolicy, bounded_failure::BoundedArithmeticFailure,
    },
};

// Exact rational sums avoid the production grid, term count and tail allowance.
fn rational_atan(divisor: u64) -> [ExactRatio; 2] {
    let mut sum = ExactRatio::from_finite(0.);
    let mut power = num_bigint::BigInt::from(divisor);
    for k in 0..120u64 {
        let term = ExactRatio::new(1.into(), &power * (2 * k + 1));
        sum = if k % 2 == 0 {
            sum.add(&term)
        } else {
            sum.subtract(&term)
        };
        power *= divisor * divisor;
    }
    let tail = ExactRatio::new(1.into(), power * 241u64);
    [sum.clone(), sum.add(&tail)]
}

fn rational_pi() -> [ExactRatio; 2] {
    let fifth = rational_atan(5);
    let other = rational_atan(239);
    let sixteen = ExactRatio::from_finite(16.);
    let four = ExactRatio::from_finite(4.);
    [
        fifth[0]
            .multiply(&sixteen)
            .subtract(&other[1].multiply(&four)),
        fifth[1]
            .multiply(&sixteen)
            .subtract(&other[0].multiply(&four)),
    ]
}

#[test]
fn machins_bound_contains_independent_exact_rational_series() {
    let expected = rational_pi();
    for fractional_bits in [8, 64, 128, 256] {
        let policy = ArithmeticPolicy {
            fractional_bits,
            ..Default::default()
        };
        let mut cancelled = || false;
        let mut arithmetic = Arithmetic::new(policy, &mut cancelled);
        let actual = pi(&mut arithmetic).unwrap();
        let exponent = -i32::from(fractional_bits);
        let lower = ExactRatio::from_dyadic(&DyadicBound::new(actual.lower, exponent));
        let upper = ExactRatio::from_dyadic(&DyadicBound::new(actual.upper, exponent));
        assert!(lower <= expected[0], "bits={fractional_bits}");
        assert!(upper >= expected[1], "bits={fractional_bits}");
    }
}

#[test]
fn pi_refuses_exhausted_arithmetic_without_partial_bound() {
    let policy = ArithmeticPolicy {
        max_operations: 0,
        ..Default::default()
    };
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(policy, &mut cancelled);
    assert_eq!(
        pi(&mut arithmetic),
        Err(BoundedArithmeticFailure::WorkLimit {
            completed: 0,
            limit: 0
        })
    );
}

#[test]
fn pi_observes_cancellation() {
    let mut cancelled = || true;
    let mut arithmetic = Arithmetic::new(Default::default(), &mut cancelled);
    assert_eq!(
        pi(&mut arithmetic),
        Err(BoundedArithmeticFailure::Cancelled)
    );
}

#[test]
fn fixed_grid_matches_machin_bounds_work_limits_and_cancellation() {
    for remaining in 0..=MACHIN_GRID_STEPS + 1 {
        assert_eq!(
            evaluate(pi, remaining, None),
            evaluate(machin_pi, remaining, None),
            "remaining={remaining}"
        );
        assert_eq!(
            evaluate(pi, remaining, Some(remaining)),
            evaluate(machin_pi, remaining, Some(remaining)),
            "cancelled at work limit={remaining}"
        );
        assert_eq!(
            evaluate(pi, MACHIN_GRID_STEPS + 1, Some(remaining)),
            evaluate(machin_pi, MACHIN_GRID_STEPS + 1, Some(remaining)),
            "cancelled={remaining}"
        );
    }
}

fn evaluate(
    operation: impl FnOnce(&mut Arithmetic<'_>) -> Checked<Interval>,
    remaining: u32,
    stop: Option<u32>,
) -> (Checked<Interval>, u32, u32) {
    let policy = ArithmeticPolicy {
        fractional_bits: MACHIN_GRID_BITS,
        max_operations: 1 + remaining,
    };
    let mut polls = 0;
    let mut cancelled = || {
        polls += 1;
        stop.is_some_and(|step| polls == 2 + step)
    };
    let mut arithmetic = Arithmetic::new(policy, &mut cancelled);
    arithmetic.certificate_step().unwrap();
    let result = operation(&mut arithmetic);
    (result, arithmetic.operations(), polls)
}
