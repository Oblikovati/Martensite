// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::DyadicBound;
use crate::predicates::roots::certificate_arithmetic::ExactRatio;
#[test]
fn numerator_cutoff_rounds_down_and_charges_all_three_interval_operations() {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: 8,
            max_operations: 100,
        },
        &mut never,
    );
    let weight = arithmetic.scalar(0.3).unwrap();
    let before = arithmetic.operations();
    let cutoff = numerator_cutoff(0.1, &weight, &mut arithmetic).unwrap();
    assert_eq!(arithmetic.operations() - before, 3);
    assert_eq!(cutoff.lower, cutoff.upper);
    let actual = DyadicBound::new(cutoff.upper, -8);
    let expected = ExactRatio::from_finite(0.1)
        .multiply(&ExactRatio::from_finite(0.3))
        .divide(&ExactRatio::from_finite(4.));
    assert!(ExactRatio::from_dyadic(&actual) <= expected);
}
