//! Owned numerical special-value contract (#201), independently specified in
//! C N1570 Annex F.10.1–F.10.4; no floating-environment or NaN payload promise.
use super::special_contract_support::{assert_unary_nan_class, assert_unary_words, signed_nan_arguments};
use martensite_cmath as math;

type UnaryFunction = fn(f64) -> f64;
const NEGATIVE_ZERO: u64 = 0x8000_0000_0000_0000;
const POSITIVE_INFINITY: u64 = 0x7ff0_0000_0000_0000;
const NEGATIVE_INFINITY: u64 = 0xfff0_0000_0000_0000;

#[test]
fn odd_functions_preserve_both_zero_signs() {
    let operations: [(&str, UnaryFunction); 10] = [
        ("sin", math::sin), ("tan", math::tan), ("asin", math::asin),
        ("atan", math::atan), ("sinh", math::sinh), ("asinh", math::asinh),
        ("tanh", math::tanh), ("atanh", math::atanh), ("cbrt", math::cbrt),
        ("expm1", math::expm1),
    ];
    for (name, operation) in operations {
        assert_unary_words(name, operation, &[(0, 0), (NEGATIVE_ZERO, NEGATIVE_ZERO)]);
    }
    assert_unary_words("log1p", math::log1p, &[(0, 0), (NEGATIVE_ZERO, NEGATIVE_ZERO)]);
}

#[test]
fn even_functions_and_exponentials_map_zero_to_one() {
    let operations: [(&str, UnaryFunction); 4] = [
        ("cos", math::cos), ("cosh", math::cosh), ("exp", math::exp), ("exp2", math::exp2),
    ];
    for (name, operation) in operations {
        assert_unary_words(name, operation, &[(0, 0x3ff0_0000_0000_0000),
            (NEGATIVE_ZERO, 0x3ff0_0000_0000_0000)]);
    }
    assert_unary_words("acos", math::acos, &[(0, 0x3ff9_21fb_5444_2d18),
        (NEGATIVE_ZERO, 0x3ff9_21fb_5444_2d18)]);
}

#[test]
fn hyperbolic_and_root_infinities_have_specified_signs() {
    for (name, operation) in [("sinh", math::sinh as UnaryFunction),
        ("asinh", math::asinh), ("cbrt", math::cbrt)] {
        assert_unary_words(name, operation, &[(POSITIVE_INFINITY, POSITIVE_INFINITY),
            (NEGATIVE_INFINITY, NEGATIVE_INFINITY)]);
    }
    assert_unary_words("cosh", math::cosh, &[(POSITIVE_INFINITY, POSITIVE_INFINITY),
        (NEGATIVE_INFINITY, POSITIVE_INFINITY)]);
    assert_unary_words("tanh", math::tanh, &[(POSITIVE_INFINITY, 0x3ff0_0000_0000_0000),
        (NEGATIVE_INFINITY, 0xbff0_0000_0000_0000)]);
    assert_unary_words("acosh", math::acosh, &[(POSITIVE_INFINITY, POSITIVE_INFINITY)]);
}

#[test]
fn exponential_infinities_and_inverse_endpoints_are_specified() {
    for (name, operation) in [("exp", math::exp as UnaryFunction), ("exp2", math::exp2)] {
        assert_unary_words(name, operation, &[(POSITIVE_INFINITY, POSITIVE_INFINITY),
            (NEGATIVE_INFINITY, 0)]);
    }
    assert_unary_words("expm1", math::expm1, &[(POSITIVE_INFINITY, POSITIVE_INFINITY),
        (NEGATIVE_INFINITY, 0xbff0_0000_0000_0000)]);
    assert_unary_words("acosh", math::acosh, &[(0x3ff0_0000_0000_0000, 0)]);
    assert_unary_words("atanh", math::atanh, &[(0x3ff0_0000_0000_0000, POSITIVE_INFINITY),
        (0xbff0_0000_0000_0000, NEGATIVE_INFINITY)]);
}

#[test]
fn logarithm_poles_and_identity_have_specified_words() {
    for (name, operation) in [("log", math::log as UnaryFunction),
        ("log2", math::log2), ("log10", math::log10)] {
        assert_unary_words(name, operation, &[(0, NEGATIVE_INFINITY),
            (NEGATIVE_ZERO, NEGATIVE_INFINITY), (0x3ff0_0000_0000_0000, 0),
            (POSITIVE_INFINITY, POSITIVE_INFINITY)]);
        assert_unary_nan_class(name, operation, &[-1.0, f64::NEG_INFINITY]);
    }
    assert_unary_words("log1p", math::log1p, &[(0xbff0_0000_0000_0000, NEGATIVE_INFINITY),
        (POSITIVE_INFINITY, POSITIVE_INFINITY)]);
    assert_unary_nan_class("log1p", math::log1p, &[-2.0, f64::NEG_INFINITY]);
}

#[test]
fn unary_domain_errors_are_nan_without_payload_requirements() {
    for (name, operation) in [("sin", math::sin as UnaryFunction),
        ("cos", math::cos), ("tan", math::tan)] {
        assert_unary_nan_class(name, operation, &[f64::INFINITY, f64::NEG_INFINITY]);
    }
    for (name, operation) in [("asin", math::asin as UnaryFunction),
        ("acos", math::acos), ("atanh", math::atanh)] {
        assert_unary_nan_class(name, operation, &[-2.0, 2.0, f64::INFINITY, f64::NEG_INFINITY]);
    }
    assert_unary_nan_class("acosh", math::acosh, &[0.0, -0.0, 0.5, -1.0, f64::NEG_INFINITY]);
}

#[test]
fn all_unary_functions_classify_signed_quiet_and_signaling_nans() {
    let operations: [(&str, UnaryFunction); 20] = [
        ("acos", math::acos), ("acosh", math::acosh), ("asin", math::asin),
        ("asinh", math::asinh), ("atan", math::atan), ("atanh", math::atanh),
        ("cbrt", math::cbrt), ("cos", math::cos), ("cosh", math::cosh),
        ("exp", math::exp), ("exp2", math::exp2), ("expm1", math::expm1),
        ("log", math::log), ("log1p", math::log1p), ("log2", math::log2),
        ("log10", math::log10), ("sin", math::sin), ("sinh", math::sinh),
        ("tan", math::tan), ("tanh", math::tanh),
    ];
    for (name, operation) in operations {
        assert_unary_nan_class(name, operation, &signed_nan_arguments());
    }
}
