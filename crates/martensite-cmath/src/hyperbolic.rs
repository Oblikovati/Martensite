//! Correctly rounded hyperbolic functions and their inverses.

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

/// Inverse hyperbolic cosine, correctly rounded; NaN below 1.
///
/// ```
/// assert_eq!(martensite_cmath::acosh(1.0), 0.0);
/// ```
pub fn acosh(x: f64) -> f64 {
    observed!(acosh(x) => metallic::acosh(x))
}

/// Inverse hyperbolic tangent, correctly rounded; infinite at `±1`, NaN beyond.
///
/// ```
/// assert_eq!(martensite_cmath::atanh(0.0), 0.0);
/// ```
pub fn atanh(x: f64) -> f64 {
    observed!(atanh(x) => metallic::atanh(x))
}
