//! Correctly rounded hyperbolic functions and their inverses.

use crate::INVALID_OPERATION_NAN;

/// Hyperbolic sine, correctly rounded.
///
/// ```
/// assert_eq!(martensite_cmath::sinh(0.0), 0.0);
/// ```
pub fn sinh(x: f64) -> f64 {
    observed!(sinh(x) => metallic::sinh(x))
}

/// Hyperbolic cosine, correctly rounded.
///
/// ```
/// assert_eq!(martensite_cmath::cosh(0.0), 1.0);
/// ```
pub fn cosh(x: f64) -> f64 {
    observed!(cosh(x) => metallic::cosh(x))
}

/// Hyperbolic tangent, correctly rounded.
///
/// ```
/// assert_eq!(martensite_cmath::tanh(f64::INFINITY), 1.0);
/// ```
pub fn tanh(x: f64) -> f64 {
    observed!(tanh(x) => metallic::tanh(x))
}

/// Inverse hyperbolic sine, correctly rounded.
///
/// ```
/// assert_eq!(martensite_cmath::asinh(0.0), 0.0);
/// ```
pub fn asinh(x: f64) -> f64 {
    observed!(asinh(x) => metallic::asinh(x))
}

/// Inverse hyperbolic cosine, correctly rounded; [`INVALID_OPERATION_NAN`] below 1.
///
/// For non-NaN `x < 1` (including `-inf`) the domain error has the word
/// `0xfff8000000000000` on every target. NaN operands are classification-only
/// (#201, #203).
///
/// ```
/// assert_eq!(martensite_cmath::acosh(1.0), 0.0);
/// assert_eq!(martensite_cmath::acosh(0.25).to_bits(), 0xfff8_0000_0000_0000);
/// ```
pub fn acosh(x: f64) -> f64 {
    observed!(acosh(x) => guarded_acosh(x))
}

fn guarded_acosh(x: f64) -> f64 {
    if x < 1.0 {
        return INVALID_OPERATION_NAN;
    }
    metallic::acosh(x)
}

/// Inverse hyperbolic tangent, correctly rounded; infinite at `±1`,
/// [`INVALID_OPERATION_NAN`] beyond.
///
/// For non-NaN `abs(x) > 1` (including both infinities) the domain error has the
/// word `0xfff8000000000000` on every target. NaN operands are
/// classification-only (#201, #203).
///
/// ```
/// assert_eq!(martensite_cmath::atanh(0.0), 0.0);
/// assert_eq!(martensite_cmath::atanh(2.0).to_bits(), 0xfff8_0000_0000_0000);
/// ```
pub fn atanh(x: f64) -> f64 {
    observed!(atanh(x) => guarded_atanh(x))
}

fn guarded_atanh(x: f64) -> f64 {
    if x.abs() > 1.0 {
        return INVALID_OPERATION_NAN;
    }
    metallic::atanh(x)
}
