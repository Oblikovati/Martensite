//! Special values follow C99 Annex F; the IEEE remainder is exact.
use martensite_cmath::{
    acos, acosh, asin, asinh, atan2, atanh, cos, exp, hypot, log, pow, remainder, sin, tanh,
};

#[test]
fn special_values_follow_c99() {
    assert!(asinh(-0.0).is_sign_negative() && asinh(f64::INFINITY) == f64::INFINITY);
    assert!(acosh(0.5).is_nan() && acosh(f64::INFINITY) == f64::INFINITY);
    assert_eq!(atanh(1.0), f64::INFINITY);
    assert!(atanh(2.0).is_nan() && asinh(f64::NAN).is_nan());
    assert!(sin(-0.0).is_sign_negative() && sin(f64::INFINITY).is_nan());
    assert!(cos(f64::NEG_INFINITY).is_nan());
    assert!(asin(1.5).is_nan() && acos(-1.5).is_nan());
    assert_eq!(exp(f64::NEG_INFINITY), 0.0);
    assert_eq!(log(0.0), f64::NEG_INFINITY);
    assert!(log(-1.0).is_nan());
    assert_eq!(tanh(-0.0).to_bits(), (-0.0f64).to_bits());
    assert_eq!(atan2(-0.0, 1.0).to_bits(), (-0.0f64).to_bits());
    assert_eq!(atan2(0.0, -1.0), std::f64::consts::PI);
    assert_eq!(hypot(f64::INFINITY, f64::NAN), f64::INFINITY);
    assert_eq!(pow(f64::NAN, 0.0), 1.0);
    assert_eq!(pow(1.0, f64::NAN), 1.0);
    assert!(pow(-8.0, 1.0 / 3.0).is_nan());
    assert_eq!(pow(-2.0, 3.0), -8.0);
}

#[test]
fn values_are_inverse_hyperbolic_functions() {
    for x in [1.0e-300, 1.0e-9, 0.5, 2.0, 30.0, 1.0e300] {
        assert!((martensite_cmath::sinh(asinh(x)) - x).abs() <= 1.0e-12 * x);
    }
    assert!((martensite_cmath::cosh(acosh(2.0)) - 2.0).abs() <= 1.0e-15);
    assert!((tanh(atanh(0.5)) - 0.5).abs() <= 1.0e-15);
}

#[test]
fn remainder_rounds_the_quotient_to_nearest_even() {
    assert_eq!(remainder(5.0, 2.0), 1.0);
    assert_eq!(remainder(7.0, 2.0), -1.0);
    assert_eq!(remainder(-0.0, 1.0).to_bits(), (-0.0f64).to_bits());
    assert!(remainder(1.0, 0.0).is_nan() && remainder(f64::INFINITY, 1.0).is_nan());
    assert_eq!(
        remainder(2.0 * std::f64::consts::PI, std::f64::consts::PI),
        0.0
    );
}
