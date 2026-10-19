//! Powers and distances follow numerical special cases in N1570 F.10.4 (#201).
//! NaN identities have precedence; only classification is required otherwise (#202).
use super::special_contract_support::{assert_binary_words, assert_power_words, signed_nan_arguments};
use martensite_cmath as math;

#[test]
fn power_identities_precede_nan_propagation() {
    for nan in signed_nan_arguments() {
        for zero in [0.0, -0.0] {
            assert_eq!(math::pow(nan, zero).to_bits(), 1.0f64.to_bits());
        }
        assert_eq!(math::pow(1.0, nan).to_bits(), 1.0f64.to_bits());
        for other in [0.0, -0.0, 2.0, -2.0, f64::INFINITY, f64::NEG_INFINITY, nan] {
            assert!(math::pow(other, nan).is_nan(), "pow({other:?},{nan:?}): expected NaN");
        }
        for exponent in [1.0, -1.0, f64::INFINITY, f64::NEG_INFINITY, nan] {
            assert!(math::pow(nan, exponent).is_nan(), "pow({nan:?},{exponent:?}): expected NaN");
        }
    }
}

#[test]
fn powers_of_signed_zero_follow_exponent_parity_and_sign() {
    for zero in [0.0f64, -0.0] {
        let sign = zero.to_bits();
        let signed_infinity = f64::INFINITY.to_bits() | sign;
        assert_power_words(&[
            (zero, 0.0, 1.0f64.to_bits()), (zero, -0.0, 1.0f64.to_bits()),
            (zero, 3.0, sign), (zero, 2.0, 0), (zero, 0.5, 0),
            (zero, -3.0, signed_infinity), (zero, -2.0, f64::INFINITY.to_bits()),
            (zero, -0.5, f64::INFINITY.to_bits()), (zero, f64::INFINITY, 0),
            (zero, f64::NEG_INFINITY, f64::INFINITY.to_bits()),
        ]);
    }
}

#[test]
fn powers_of_infinity_follow_exponent_parity_and_sign() {
    let infinity = f64::INFINITY;
    assert_power_words(&[
        (infinity, 0.0, 1.0f64.to_bits()), (infinity, -0.0, 1.0f64.to_bits()),
        (-infinity, 0.0, 1.0f64.to_bits()), (-infinity, -0.0, 1.0f64.to_bits()),
        (infinity, 3.0, infinity.to_bits()), (infinity, -3.0, 0),
        (-infinity, 3.0, (-infinity).to_bits()), (-infinity, 2.0, infinity.to_bits()),
        (-infinity, 0.5, infinity.to_bits()), (-infinity, -3.0, (-0.0f64).to_bits()),
        (-infinity, -2.0, 0), (-infinity, -0.5, 0),
    ]);
}

#[test]
fn infinite_exponents_compare_base_magnitude_to_one() {
    for infinity in [f64::INFINITY, f64::NEG_INFINITY] {
        assert_power_words(&[(-1.0, infinity, 1.0f64.to_bits()),
            (1.0, infinity, 1.0f64.to_bits())]);
    }
    for base in [0.5, -0.5] {
        assert_power_words(&[(base, f64::INFINITY, 0),
            (base, f64::NEG_INFINITY, f64::INFINITY.to_bits())]);
    }
    for base in [2.0, -2.0] {
        assert_power_words(&[(base, f64::INFINITY, f64::INFINITY.to_bits()),
            (base, f64::NEG_INFINITY, 0)]);
    }
}

#[test]
fn negative_finite_bases_require_integral_finite_exponents() {
    for base in [-0.5, -2.0, -8.0] {
        for exponent in [0.5, -0.5, 1.0 / 3.0, 1.5] {
            assert!(math::pow(base, exponent).is_nan(), "pow({base},{exponent}): expected NaN");
            assert!(math::powf(base as f32, exponent as f32).is_nan(),
                "powf({base},{exponent}): expected NaN");
        }
    }
    assert_power_words(&[(-2.0, 3.0, (-8.0f64).to_bits()),
        (-2.0, -3.0, (-0.125f64).to_bits()), (-2.0, 2.0, 4.0f64.to_bits())]);
}

#[test]
fn hypot_zero_is_positive_and_infinity_takes_precedence_over_nan() {
    for zero in [0.0, -0.0] {
        assert_binary_words("hypot", math::hypot, &[(zero, 0.0, 0), (zero, -0.0, 0),
            (zero, -3.0, 3.0f64.to_bits()), (-3.0, zero, 3.0f64.to_bits())]);
    }
    for infinity in [f64::INFINITY, f64::NEG_INFINITY] {
        for other in signed_nan_arguments().into_iter().chain([0.0, -0.0, 1.0, infinity]) {
            assert_binary_words("hypot", math::hypot, &[(infinity, other, f64::INFINITY.to_bits()),
                (other, infinity, f64::INFINITY.to_bits())]);
        }
    }
}

#[test]
fn hypot_without_infinite_operand_propagates_nan_class() {
    for nan in signed_nan_arguments() {
        for other in [0.0, -0.0, 1.0, -1.0, nan] {
            assert!(math::hypot(nan, other).is_nan(), "hypot({nan:?},{other:?}): expected NaN");
            assert!(math::hypot(other, nan).is_nan(), "hypot({other:?},{nan:?}): expected NaN");
        }
    }
}
