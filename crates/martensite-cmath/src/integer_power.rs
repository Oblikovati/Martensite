//! Integer powers by repeated squaring.

/// `x^n` by a fixed binary repeated-squaring sequence.
///
/// The result is not correctly rounded: each multiplication rounds. It is
/// Negative exponents take the reciprocal after computing the positive power.
/// Intermediate overflow can therefore produce zero even when the mathematical
/// result is a representable subnormal. This sequence can differ from compiler
/// constant folding of `f64::powi`; it is an explicit numerical contract (#201).
///
/// ```
/// assert_eq!(martensite_cmath::powi(3.0, 4), 81.0);
/// assert_eq!(martensite_cmath::powi(2.0, -2), 0.25);
/// ```
pub fn powi(x: f64, n: i32) -> f64 {
    let mut square = x;
    let mut remaining = n.unsigned_abs();
    let mut product = 1.0;
    loop {
        if remaining & 1 != 0 {
            product *= square;
        }
        remaining >>= 1;
        if remaining == 0 {
            break;
        }
        square *= square;
    }
    if n < 0 { 1.0 / product } else { product }
}
