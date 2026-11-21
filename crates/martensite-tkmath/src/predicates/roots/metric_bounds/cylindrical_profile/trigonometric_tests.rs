// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::certificate_arithmetic::ExactRatio;

// Independent direct rational Taylor evaluation: no interval argument reduction.
fn direct(argument: f64, axis: usize) -> (ExactRatio, ExactRatio) {
    let x = ExactRatio::from_finite(argument);
    let square = x.multiply(&x);
    let mut term = ExactRatio::from_finite(if axis == 0 { 1. } else { argument });
    let mut value = term.clone();
    for k in 1..=64 {
        let divisor = ExactRatio::from_finite(((2 * k - 1 + axis) * (2 * k + axis)) as f64);
        term = term.multiply(&square).divide(&divisor).negated();
        value = value.add(&term);
    }
    let mut error = ExactRatio::from_finite(1.);
    let amplitude = ExactRatio::from_finite(argument.abs());
    for k in 1..=129 {
        error = error
            .multiply(&amplitude)
            .divide(&ExactRatio::from_finite(k as f64));
    }
    (value.subtract(&error), value.add(&error))
}

#[test]
fn scalar_enclosures_contain_independent_direct_taylor_bounds() {
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(Default::default(), &mut cancelled);
    for argument in [-12.5, -0.5, 0., 0.5, 12.5] {
        let captured = arithmetic.scalar(argument).unwrap();
        let bounded = scalar(&captured, &mut arithmetic).unwrap();
        for (axis, interval) in bounded.iter().enumerate() {
            compare_direct(argument, axis, interval);
        }
    }
}

fn compare_direct(argument: f64, axis: usize, interval: &Interval) {
    let (lower, upper) = direct(argument, axis);
    let actual_lower = ExactRatio::from_dyadic(&DyadicBound::new(interval.lower.clone(), -128));
    let actual_upper = ExactRatio::from_dyadic(&DyadicBound::new(interval.upper.clone(), -128));
    assert!(
        actual_lower <= lower,
        "lower: argument={argument}, axis={axis}"
    );
    assert!(
        actual_upper >= upper,
        "upper: argument={argument}, axis={axis}"
    );
}
