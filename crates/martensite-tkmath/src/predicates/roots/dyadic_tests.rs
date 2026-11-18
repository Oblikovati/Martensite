// SPDX-License-Identifier: GPL-2.0-only
use super::*;

fn assert_projection(bound: &DyadicBound, expected: [f64; 2]) {
    let actual = bound.binary64_bounds().unwrap();
    assert_eq!(actual.map(f64::to_bits), expected.map(f64::to_bits));
    assert!(bound.compare_f64(actual[0]).unwrap().is_ge());
    assert!(bound.compare_f64(actual[1]).unwrap().is_le());
}

#[test]
fn exact_binary64_values_keep_identical_projection_endpoints() {
    for value in [0., f64::from_bits(1), f64::MIN_POSITIVE, 0.5, 1., f64::MAX] {
        assert_projection(&DyadicBound::from_f64(value).unwrap(), [value; 2]);
        if value == 0. {
            continue;
        }
        assert_projection(&DyadicBound::from_f64(-value).unwrap(), [-value; 2]);
    }
}

#[test]
fn midpoint_projects_to_adjacent_binary64_values_on_both_sides_of_zero() {
    let significand: BigInt = (BigInt::from(1) << 53) + 1;
    let midpoint = DyadicBound::new(significand.clone(), -53);
    let next = f64::from_bits(1f64.to_bits() + 1);
    assert_projection(&midpoint, [1., next]);
    assert_projection(&DyadicBound::new(-significand, -53), [-next, -1.]);
}

#[test]
fn subnormal_projection_stays_outward_without_clamping_to_zero() {
    let tiny = f64::from_bits(1);
    assert_projection(&DyadicBound::new(3.into(), -1076), [0., tiny]);
    assert_projection(&DyadicBound::new((-3).into(), -1076), [-tiny, -0.]);
}

#[test]
fn finite_projection_refuses_values_beyond_either_binary64_extreme() {
    for sign in [-1, 1] {
        let bound = DyadicBound::new(BigInt::from(sign), 1024);
        assert!(bound.binary64_bounds().is_none());
    }
    let maximum = DyadicBound::from_f64(f64::MAX).unwrap();
    let just_beyond = maximum.midpoint(&DyadicBound::new(1.into(), 1024));
    assert!(just_beyond.binary64_bounds().is_none());
}

#[test]
fn dyadic_parameter_scale_preserves_exact_sign_and_zero() {
    for (input, power, expected) in [(0., 64, 0.), (-0.5, 3, -4.), (0.25, 2, 1.)] {
        let scaled = DyadicBound::from_f64(input).unwrap().scale_pow2(power);
        assert_eq!(scaled.compare_f64(expected).unwrap(), Ordering::Equal);
    }
}
