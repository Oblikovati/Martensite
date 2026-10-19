//! The owned exact-remainder contract uses N1570 7.12.10.2/F.10.7.2 numerical rules.
//! No errno, floating-environment, or NaN representation contract is imposed (#201).
use super::special_contract_support::{assert_binary_words, signed_nan_arguments};
use martensite_cmath::remainder;

#[test]
fn exact_remainder_preserves_numerator_zero_sign() {
    for numerator in [0.0f64, -0.0] {
        for divisor in [1.0, -1.0, f64::INFINITY, f64::NEG_INFINITY] {
            assert_binary_words("remainder", remainder, &[(numerator, divisor, numerator.to_bits())]);
        }
    }
    assert_binary_words("remainder", remainder, &[(4.0, 2.0, 0),
        (-4.0, 2.0, (-0.0f64).to_bits()), (4.0, -2.0, 0),
        (-4.0, -2.0, (-0.0f64).to_bits())]);
}

#[test]
fn infinite_divisor_returns_the_finite_numerator_unchanged() {
    for numerator in [0.0f64, -0.0, 1.0, -1.0, f64::MIN_POSITIVE, f64::MAX] {
        for divisor in [f64::INFINITY, f64::NEG_INFINITY] {
            assert_binary_words("remainder", remainder, &[(numerator, divisor, numerator.to_bits())]);
        }
    }
}

#[test]
fn invalid_remainder_inputs_require_only_nan_classification() {
    for numerator in [0.0, -0.0, 1.0, -1.0, f64::INFINITY, f64::NEG_INFINITY] {
        for zero in [0.0, -0.0] {
            assert!(remainder(numerator, zero).is_nan(), "remainder({numerator},{zero}): expected NaN");
        }
    }
    for infinity in [f64::INFINITY, f64::NEG_INFINITY] {
        for divisor in [1.0, -1.0, infinity] {
            assert!(remainder(infinity, divisor).is_nan(), "remainder({infinity},{divisor}): expected NaN");
        }
    }
}

#[test]
fn remainder_nan_arguments_have_no_payload_or_sign_expectation() {
    for nan in signed_nan_arguments() {
        for other in [0.0, -0.0, 1.0, -1.0, f64::INFINITY, f64::NEG_INFINITY, nan] {
            assert!(remainder(nan, other).is_nan(), "remainder({nan:?},{other:?}): expected NaN");
            assert!(remainder(other, nan).is_nan(), "remainder({other:?},{nan:?}): expected NaN");
        }
    }
}
