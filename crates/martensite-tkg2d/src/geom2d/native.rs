//! Ordered floating extrema with first-argument retention.
//!
//! An unordered comparison or equal operands retains the first argument,
//! preserving its signed-zero or NaN bits.
/// Retain `a` unless `a < b`; `std_max(-0.0, 0.0).is_sign_negative()`.
pub fn std_max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// Retain `a` unless `b < a`; `std_min(0.0, -0.0).is_sign_positive()`.
pub fn std_min(a: f64, b: f64) -> f64 {
    if b < a { b } else { a }
}
