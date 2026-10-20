//! Correctly rounded circular functions and their inverses, in radians.

/// Sine of `x` radians, correctly rounded.
///
/// ```
/// assert_eq!(martensite_cmath::sin(std::f64::consts::FRAC_PI_2), 1.0);
/// ```
pub fn sin(x: f64) -> f64 {
    observed!(sin(x) => metallic::sin(x))
}

/// Cosine of `x` radians, correctly rounded.
///
/// ```
/// assert_eq!(martensite_cmath::cos(0.0), 1.0);
/// ```
pub fn cos(x: f64) -> f64 {
    observed!(cos(x) => metallic::cos(x))
}

/// Sine and cosine of `x` radians as `(sin, cos)`, each correctly rounded.
///
/// ```
/// assert_eq!(martensite_cmath::sincos(0.0), (0.0, 1.0));
/// ```
pub fn sincos(x: f64) -> (f64, f64) {
    observed!(sincos(x) => metallic::sincos(x))
}

/// Tangent of `x` radians, correctly rounded.
///
/// ```
/// assert_eq!(martensite_cmath::tan(0.0), 0.0);
/// ```
pub fn tan(x: f64) -> f64 {
    observed!(tan(x) => metallic::tan(x))
}

/// Arcsine in `[-pi/2, pi/2]`, correctly rounded; NaN outside `[-1, 1]`.
///
/// ```
/// assert_eq!(martensite_cmath::asin(1.0), std::f64::consts::FRAC_PI_2);
/// ```
pub fn asin(x: f64) -> f64 {
    observed!(asin(x) => metallic::asin(x))
}

/// Arccosine in `[0, pi]`, correctly rounded; NaN outside `[-1, 1]`.
///
/// ```
/// assert_eq!(martensite_cmath::acos(-1.0), std::f64::consts::PI);
/// ```
pub fn acos(x: f64) -> f64 {
    observed!(acos(x) => metallic::acos(x))
}

/// Arctangent in `[-pi/2, pi/2]`, correctly rounded.
///
/// ```
/// assert_eq!(martensite_cmath::atan(1.0), std::f64::consts::FRAC_PI_4);
/// ```
pub fn atan(x: f64) -> f64 {
    observed!(atan(x) => metallic::atan(x))
}

/// Angle of the point `(x, y)` in `[-pi, pi]`, the C `atan2(y, x)`, correctly
/// rounded. The receiver of the standard `y.atan2(x)` is the first argument.
///
/// ```
/// assert_eq!(martensite_cmath::atan2(1.0, 0.0), std::f64::consts::FRAC_PI_2);
/// ```
pub fn atan2(y: f64, x: f64) -> f64 {
    observed!(atan2(y, x) => metallic::atan2(y, x))
}
