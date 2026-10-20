//! Angle endpoints and quadrants follow the owned #201 contract and N1570 F.10.1.
use super::special_contract_support::{
    assert_binary_words, assert_unary_words, signed_nan_arguments,
};
use martensite_cmath as math;

const PI: u64 = 0x4009_21fb_5444_2d18;
const HALF_PI: u64 = 0x3ff9_21fb_5444_2d18;
const QUARTER_PI: u64 = 0x3fe9_21fb_5444_2d18;
const THREE_QUARTER_PI: u64 = 0x4002_d97c_7f33_21d2;
const SIGN: u64 = 0x8000_0000_0000_0000;

#[test]
fn circular_inverse_endpoints_use_the_rounded_angle_constants() {
    assert_unary_words(
        "asin",
        math::asin,
        &[
            (1.0f64.to_bits(), HALF_PI),
            ((-1.0f64).to_bits(), HALF_PI | SIGN),
        ],
    );
    assert_unary_words(
        "acos",
        math::acos,
        &[(1.0f64.to_bits(), 0), ((-1.0f64).to_bits(), PI)],
    );
    assert_unary_words(
        "atan",
        math::atan,
        &[
            (f64::INFINITY.to_bits(), HALF_PI),
            (f64::NEG_INFINITY.to_bits(), HALF_PI | SIGN),
        ],
    );
}

#[test]
fn atan2_distinguishes_all_zero_quadrants_and_zero_axes() {
    assert_binary_words(
        "atan2",
        math::atan2,
        &[
            (0.0, 0.0, 0),
            (-0.0, 0.0, SIGN),
            (0.0, -0.0, PI),
            (-0.0, -0.0, PI | SIGN),
            (0.0, 1.0, 0),
            (-0.0, 1.0, SIGN),
            (0.0, -1.0, PI),
            (-0.0, -1.0, PI | SIGN),
            (1.0, 0.0, HALF_PI),
            (1.0, -0.0, HALF_PI),
            (-1.0, 0.0, HALF_PI | SIGN),
            (-1.0, -0.0, HALF_PI | SIGN),
        ],
    );
}

#[test]
fn atan2_infinite_quadrants_have_exact_specified_words() {
    let infinity = f64::INFINITY;
    assert_binary_words(
        "atan2",
        math::atan2,
        &[
            (infinity, infinity, QUARTER_PI),
            (-infinity, infinity, QUARTER_PI | SIGN),
            (infinity, -infinity, THREE_QUARTER_PI),
            (-infinity, -infinity, THREE_QUARTER_PI | SIGN),
            (infinity, 1.0, HALF_PI),
            (-infinity, 1.0, HALF_PI | SIGN),
            (1.0, infinity, 0),
            (-1.0, infinity, SIGN),
            (1.0, -infinity, PI),
            (-1.0, -infinity, PI | SIGN),
        ],
    );
}

#[test]
fn atan2_nan_arguments_require_only_nan_classification() {
    for nan in signed_nan_arguments() {
        for other in [0.0, -0.0, 1.0, -1.0, f64::INFINITY, f64::NEG_INFINITY, nan] {
            assert!(
                math::atan2(nan, other).is_nan(),
                "atan2({nan:?},{other:?}): expected NaN"
            );
            assert!(
                math::atan2(other, nan).is_nan(),
                "atan2({other:?},{nan:?}): expected NaN"
            );
        }
    }
}

#[test]
fn paired_sine_cosine_preserve_zero_sign_and_classify_nonfinite_inputs() {
    for zero in [0.0f64, -0.0] {
        let (sine, cosine) = math::sincos(zero);
        assert_eq!(sine.to_bits(), zero.to_bits());
        assert_eq!(cosine.to_bits(), 1.0f64.to_bits());
    }
    for argument in [f64::INFINITY, f64::NEG_INFINITY]
        .into_iter()
        .chain(signed_nan_arguments())
    {
        let (sine, cosine) = math::sincos(argument);
        assert!(
            sine.is_nan() && cosine.is_nan(),
            "sincos({argument:?}): expected two NaNs"
        );
    }
}
