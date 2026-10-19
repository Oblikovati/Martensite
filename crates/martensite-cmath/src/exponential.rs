//! Correctly rounded exponentials, logarithms, powers, roots and `hypot`.

/// `e^x`, correctly rounded.
///
/// ```
/// assert_eq!(martensite_cmath::exp(0.0), 1.0);
/// ```
pub fn exp(x: f64) -> f64 {
    metallic::exp(x)
}

/// `2^x`, correctly rounded.
///
/// ```
/// assert_eq!(martensite_cmath::exp2(10.0), 1024.0);
/// ```
pub fn exp2(x: f64) -> f64 {
    metallic::exp2(x)
}

/// `e^x - 1`, correctly rounded, accurate for `x` near zero.
///
/// ```
/// assert_eq!(martensite_cmath::expm1(0.0), 0.0);
/// ```
pub fn expm1(x: f64) -> f64 {
    metallic::expm1(x)
}

/// Natural logarithm, correctly rounded; the standard `f64::ln`.
///
/// ```
/// assert_eq!(martensite_cmath::log(1.0), 0.0);
/// ```
pub fn log(x: f64) -> f64 {
    metallic::log(x)
}

/// `ln(1 + x)`, correctly rounded, accurate for `x` near zero; the standard
/// `f64::ln_1p`.
///
/// ```
/// assert_eq!(martensite_cmath::log1p(0.0), 0.0);
/// ```
pub fn log1p(x: f64) -> f64 {
    metallic::log1p(x)
}

/// Base-2 logarithm, correctly rounded.
///
/// ```
/// assert_eq!(martensite_cmath::log2(8.0), 3.0);
/// ```
pub fn log2(x: f64) -> f64 {
    metallic::log2(x)
}

/// Base-10 logarithm, correctly rounded.
///
/// ```
/// assert_eq!(martensite_cmath::log10(1000.0), 3.0);
/// ```
pub fn log10(x: f64) -> f64 {
    metallic::log10(x)
}

/// `x^y`, correctly rounded, with the C99 special cases; the standard `f64::powf`.
///
/// ```
/// assert_eq!(martensite_cmath::pow(9.0, 0.5), 3.0);
/// ```
pub fn pow(x: f64, y: f64) -> f64 {
    metallic::pow(x, y)
}

/// `x^y` in single precision, correctly rounded; the standard `f32::powf`.
///
/// ```
/// assert_eq!(martensite_cmath::powf(4.0, 0.5), 2.0);
/// ```
pub fn powf(x: f32, y: f32) -> f32 {
    // The pinned dependency's NaN-exponent path reaches an unchecked integer
    // conversion. Preserve the power identities before guarding that path (#202).
    if y == 0.0 || x == 1.0 {
        return 1.0;
    }
    if x.is_nan() || y.is_nan() {
        return f32::NAN;
    }
    metallic::powf(x, y)
}

/// Cube root, correctly rounded.
///
/// ```
/// assert_eq!(martensite_cmath::cbrt(-27.0), -3.0);
/// ```
pub fn cbrt(x: f64) -> f64 {
    metallic::cbrt(x)
}

/// `sqrt(x^2 + y^2)` without intermediate overflow or underflow, correctly rounded.
///
/// ```
/// assert_eq!(martensite_cmath::hypot(3.0, 4.0), 5.0);
/// ```
pub fn hypot(x: f64, y: f64) -> f64 {
    metallic::hypot(x, y)
}
