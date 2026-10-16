//! `std::min`/`std::max` on doubles with their exact shared geometry semantics.
//!
//! `f64::min`/`f64::max` may return either zero for `(-0.0, 0.0)` and skip NaN;
//! the shared geometry templates return the first argument unless the comparison says
//! otherwise, which decides the bits of signed zeros and NaN results.

/// `std::max(a, b)`: `a < b ? b : a`; `std_max(-0.0, 0.0).is_sign_negative()`.
pub fn std_max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// `std::min(a, b)`: `b < a ? b : a`; `std_min(0.0, -0.0).is_sign_positive()`.
pub fn std_min(a: f64, b: f64) -> f64 {
    if b < a { b } else { a }
}
