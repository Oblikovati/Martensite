//! Owned assertion boundaries protect retained numeric comparisons.
mod support;
use support::{expect_double_eq, expect_near};

#[test]
fn four_ulp_equality_accepts_boundary_and_rejects_the_next_value() {
    let four = f64::from_bits(1_f64.to_bits() + 4);
    let five = f64::from_bits(1_f64.to_bits() + 5);
    expect_double_eq(1., four);
    assert!(std::panic::catch_unwind(|| expect_double_eq(1., five)).is_err());
}

#[test]
fn signed_zero_infinities_and_nan_keep_their_comparison_domains() {
    expect_double_eq(-0., 0.);
    expect_double_eq(f64::INFINITY, f64::INFINITY);
    expect_double_eq(f64::NEG_INFINITY, f64::NEG_INFINITY);
    assert!(std::panic::catch_unwind(|| expect_double_eq(f64::NAN, f64::NAN)).is_err());
    assert!(
        std::panic::catch_unwind(|| expect_double_eq(f64::NEG_INFINITY, f64::INFINITY)).is_err()
    );
}

#[test]
fn absolute_near_is_inclusive_and_rejects_the_next_value() {
    expect_near(0., 0.5, 0.5);
    let outside = f64::from_bits(0.5_f64.to_bits() + 1);
    assert!(std::panic::catch_unwind(|| expect_near(0., outside, 0.5)).is_err());
}
