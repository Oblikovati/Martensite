//! Integer powers by repeated squaring.

/// `x^n` by binary repeated squaring, the same product sequence the compiler runtime
/// and its constant-exponent expansion use for the standard `f64::powi`.
///
/// The result is not correctly rounded: each multiplication rounds. It is
/// still identical on every platform because only IEEE multiplication and division
/// are involved. The standard method is forbidden all the same, because the compiler
/// folds `powi` of constant arguments with its build host's `pow` (#201).
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
