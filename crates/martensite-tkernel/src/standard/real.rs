//! Real-number limits, conversions and scalar helpers.
//!
//! The deprecated one-line wrappers of `<cmath>` live in the private `elementary` module and are re-exported
//! here, so `standard::real::acos` names `ACos` as the header does.

mod elementary;
pub use elementary::*;

/// The smallest positive normal double, `DBL_MIN`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::real_small(), f64::MIN_POSITIVE);
/// ```
pub const fn real_small() -> f64 {
    f64::MIN_POSITIVE
}

/// `|a - b| < real_small()`, so NaN is never equal.
///
/// ```
/// assert!(martensite_tkernel::standard::real::is_equal(1.0, 1.0));
/// ```
pub fn is_equal(first: f64, second: f64) -> bool {
    (first - second).abs() < real_small()
}

/// Decimal digits of precision, `DBL_DIG`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::real_digits(), 15);
/// ```
pub const fn real_digits() -> i32 {
    f64::DIGITS as i32
}

/// The gap between 1.0 and the next double, `DBL_EPSILON`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::real_epsilon(), f64::EPSILON);
/// ```
pub const fn real_epsilon() -> f64 {
    f64::EPSILON
}

/// The most negative finite double, `-DBL_MAX`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::real_first(), f64::MIN);
/// ```
pub const fn real_first() -> f64 {
    -f64::MAX
}

/// The minimum decimal exponent of a normal double, `DBL_MIN_10_EXP`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::real_first10_exp(), -307);
/// ```
pub const fn real_first10_exp() -> i32 {
    f64::MIN_10_EXP
}

/// The largest finite double, `DBL_MAX`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::real_last(), f64::MAX);
/// ```
pub const fn real_last() -> f64 {
    f64::MAX
}

/// The maximum decimal exponent of a double, `DBL_MAX_10_EXP`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::real_last10_exp(), 308);
/// ```
pub const fn real_last10_exp() -> i32 {
    f64::MAX_10_EXP
}

/// Significand bits including the hidden bit, `DBL_MANT_DIG`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::real_mantissa(), 53);
/// ```
pub const fn real_mantissa() -> i32 {
    f64::MANTISSA_DIGITS as i32
}

/// The floating-point radix, `FLT_RADIX`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::real_radix(), 2);
/// ```
pub const fn real_radix() -> i32 {
    f64::RADIX as i32
}

/// The size of a double in bits, `CHAR_BIT * sizeof(double)`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::real_size(), 64);
/// ```
pub const fn real_size() -> i32 {
    (u8::BITS as usize * size_of::<f64>()) as i32
}

/// The exact conversion of a 32-bit integer.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::int_to_real(-3), -3.0);
/// ```
pub const fn int_to_real(value: i32) -> f64 {
    value as f64
}

/// The gap to the next double away from zero.
///
/// Non-negative values (including `-0.0`) step toward `real_last()`, the rest toward
/// `real_first()`, so `DBL_MAX` yields zero, the infinities yield `-inf` and NaN stays NaN.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::epsilon(1.0), f64::EPSILON);
/// ```
pub fn epsilon(value: f64) -> f64 {
    if value >= 0.0 {
        next_after(value, real_last()) - value
    } else {
        value - next_after(value, real_first())
    }
}

/// `|trunc(x) - x|`, the non-negative fractional part (NaN for infinities).
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::real_part(-2.5), 0.5);
/// ```
pub fn real_part(value: f64) -> f64 {
    (value.trunc() - value).abs()
}

/// Truncates toward zero, clamping out-of-range values to the int limits.
///
/// NaN passes both clamps and has no defined `int` conversion;
/// x86-64 `cvttsd2si` yields `INT_MIN`, and that value is returned instead of Rust's 0.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::real_to_int(3.0e10), i32::MAX);
/// ```
pub fn real_to_int(value: f64) -> i32 {
    if value < f64::from(i32::MIN) {
        return i32::MIN;
    }
    if value > f64::from(i32::MAX) {
        return i32::MAX;
    }
    if value.is_nan() {
        return i32::MIN;
    }
    value as i32
}

/// Rounds to float, clamping values beyond `FLT_MAX` (NaN stays NaN).
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::real_to_short_real(1.0e300), f32::MAX);
/// ```
pub fn real_to_short_real(value: f64) -> f32 {
    if value < -f64::from(f32::MAX) {
        return -f32::MAX;
    }
    if value > f64::from(f32::MAX) {
        return f32::MAX;
    }
    value as f32
}

/// `x * x`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::square(-3.0), 9.0);
/// ```
pub const fn square(value: f64) -> f64 {
    value * value
}

/// The arc-cosine approximation polynomial divided by `sqrt(2x)`.
#[allow(clippy::excessive_precision)]
fn acos_approx_polynomial(value: f64) -> f64 {
    // Operation order and literals are fixed, so results are bit-identical.
    (-0.000007239283986332
        + value
            * (2.000291665285952400
                + value
                    * (0.163910606547823220
                        + value
                            * (0.047654245891495528
                                - value * (0.005516443930088506 + 0.015098965761299077 * value)))))
        / (2.0 * value).sqrt()
}

/// An approximate arc cosine, about one degree of error near 0.
///
/// Inputs within `real_small()` of -1 or +1 return 0 (0, not PI, at -1, on purpose).
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::acos_approx(1.0), 0.0);
/// ```
pub fn acos_approx(value: f64) -> f64 {
    if value < 0.0 {
        let shifted: f64 = 1.0 + value;
        return if shifted < real_small() {
            0.0
        } else {
            std::f64::consts::PI - acos_approx_polynomial(shifted)
        };
    }
    let shifted: f64 = 1.0 - value;
    if shifted < real_small() {
        0.0
    } else {
        acos_approx_polynomial(shifted)
    }
}
