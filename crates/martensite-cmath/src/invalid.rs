//! A target-independent NaN for invalid arithmetic.
//!
//! Hardware generates a "default NaN" for invalid operations such as `0/0` or
//! `inf/inf`, and its sign differs by target: x86-64 sets the sign bit and Arm
//! clears it. Kernel results whose text or words are compared exactly (a NaN
//! direction read from text, an expression evaluated at `0/0`) therefore produce
//! this NaN explicitly. Its word is the x86-64 default NaN, so outputs recorded on
//! x86-64 stay unchanged (#201).

/// The NaN the kernel returns for an invalid operation on non-NaN operands: quiet,
/// sign bit set, zero payload (`0xfff8000000000000`) on every target.
///
/// ```
/// assert_eq!(martensite_cmath::INVALID_OPERATION_NAN.to_bits(), 0xfff8_0000_0000_0000);
/// ```
pub const INVALID_OPERATION_NAN: f64 = f64::from_bits(0xfff8_0000_0000_0000);

/// `numerator / denominator`, with [`INVALID_OPERATION_NAN`] when the quotient is
/// invalid (`0/0`, `inf/inf`). A NaN operand propagates unchanged, as in hardware.
///
/// ```
/// assert_eq!(martensite_cmath::quotient(1.0, 4.0), 0.25);
/// assert_eq!(martensite_cmath::quotient(0.0, 0.0).to_bits(), 0xfff8_0000_0000_0000);
/// ```
pub fn quotient(numerator: f64, denominator: f64) -> f64 {
    let value = numerator / denominator;
    if value.is_nan() && !numerator.is_nan() && !denominator.is_nan() {
        return INVALID_OPERATION_NAN;
    }
    value
}
