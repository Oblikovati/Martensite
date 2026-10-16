//! The deprecated real-number wrappers of `<cmath>` (acos, cosh, next_after, max, ...).
//!
//! They are deprecated but public API. Each calls the same C library
//! function as the header: Rust's float methods reach the platform libm for the
//! transcendental functions, and the inverse hyperbolic functions, which Rust computes
//! with its own formulas, go through `martensite_cmath`. `Max`/`Min` keep the
//! `std::max`/`std::min` comparison, not IEEE `fmax`/`fmin`.

/// `std::acos`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::acos(1.0), 0.0);
/// ```
pub fn acos(value: f64) -> f64 {
    value.acos()
}

/// `std::asin`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::asin(0.0), 0.0);
/// ```
pub fn asin(value: f64) -> f64 {
    value.asin()
}

/// `std::atan`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::atan(0.0), 0.0);
/// ```
pub fn atan(value: f64) -> f64 {
    value.atan()
}

/// `std::atan2(y, x)`; the first argument is named `x` on purpose.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::atan2(0.0, 1.0), 0.0);
/// ```
pub fn atan2(first: f64, second: f64) -> f64 {
    first.atan2(second)
}

/// The C library `atanh` (NaN outside [-1, 1], infinities at the ends).
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::atanh(0.0), 0.0);
/// ```
pub fn atanh(value: f64) -> f64 {
    martensite_cmath::atanh(value)
}

/// The C library `acosh` (NaN below 1).
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::acosh(1.0), 0.0);
/// ```
pub fn acosh(value: f64) -> f64 {
    martensite_cmath::acosh(value)
}

/// The C library `asinh`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::asinh(0.0), 0.0);
/// ```
pub fn asinh(value: f64) -> f64 {
    martensite_cmath::asinh(value)
}

/// `std::cosh`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::cosh(0.0), 1.0);
/// ```
pub fn cosh(value: f64) -> f64 {
    value.cosh()
}

/// `std::sinh`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::sinh(0.0), 0.0);
/// ```
pub fn sinh(value: f64) -> f64 {
    value.sinh()
}

/// `std::tanh`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::tanh(0.0), 0.0);
/// ```
pub fn tanh(value: f64) -> f64 {
    value.tanh()
}

/// `std::cos`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::cos(0.0), 1.0);
/// ```
pub fn cos(value: f64) -> f64 {
    value.cos()
}

/// `std::sin`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::sin(0.0), 0.0);
/// ```
pub fn sin(value: f64) -> f64 {
    value.sin()
}

/// `std::tan`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::tan(0.0), 0.0);
/// ```
pub fn tan(value: f64) -> f64 {
    value.tan()
}

/// `std::exp`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::exp(0.0), 1.0);
/// ```
pub fn exp(value: f64) -> f64 {
    value.exp()
}

/// The natural logarithm `std::log`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::log(1.0), 0.0);
/// ```
pub fn log(value: f64) -> f64 {
    value.ln()
}

/// `std::log10`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::log10(100.0), 2.0);
/// ```
pub fn log10(value: f64) -> f64 {
    value.log10()
}

/// `std::pow`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::pow(2.0, 10.0), 1024.0);
/// ```
pub fn pow(value: f64, power: f64) -> f64 {
    value.powf(power)
}

/// The correctly rounded `std::sqrt`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::sqrt(4.0), 2.0);
/// ```
pub fn sqrt(value: f64) -> f64 {
    value.sqrt()
}

/// The neighbour of `value` toward `direction`, as C `nextafter`.
///
/// Equal arguments return `direction` (so `next_after(0.0, -0.0)` is `-0.0`), and a NaN
/// argument returns the sum of both, which propagates the NaN as the C library does.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::next_after(1.0, 2.0), 1.0 + f64::EPSILON);
/// ```
pub fn next_after(value: f64, direction: f64) -> f64 {
    if value.is_nan() || direction.is_nan() {
        // glibc returns `x + y`, compiled with `y` as the SSE destination operand: when both
        // are NaN the payload of `direction` wins (nextafter(NaN(0x123), -NaN) = -NaN).
        return direction + value;
    }
    if value == direction {
        return direction;
    }
    if direction > value {
        value.next_up()
    } else {
        value.next_down()
    }
}

/// `std::copysign`, the magnitude of the first with the sign of the second.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::sign(2.0, -0.0), -2.0);
/// ```
pub fn sign(magnitude: f64, sign_source: f64) -> f64 {
    magnitude.copysign(sign_source)
}

/// `std::abs`, which clears the sign bit (also of `-0.0` and NaN).
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::abs(-2.0), 2.0);
/// ```
pub fn abs(value: f64) -> f64 {
    value.abs()
}

/// `std::ceil`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::ceiling(1.2), 2.0);
/// ```
pub fn ceiling(value: f64) -> f64 {
    value.ceil()
}

/// `std::floor`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::floor(-1.2), -2.0);
/// ```
pub fn floor(value: f64) -> f64 {
    value.floor()
}

/// `std::trunc`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::integer_part(-1.7), -1.0);
/// ```
pub fn integer_part(value: f64) -> f64 {
    value.trunc()
}

/// `std::round`, halfway cases away from zero.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::real::round(-2.5), -3.0);
/// ```
pub fn round(value: f64) -> f64 {
    value.round()
}

/// `Max(double, double)`: `std::max`, i.e. `first < second ? second : first`.
///
/// Unlike `f64::max`, a NaN in either position is kept when the comparison selects it,
/// and `max(-0.0, 0.0)` returns the first argument, `-0.0`.
///
/// ```
/// assert!(martensite_tkernel::standard::real::max(f64::NAN, 1.0).is_nan());
/// ```
pub fn max(first: f64, second: f64) -> f64 {
    if first < second { second } else { first }
}

/// `Min(double, double)`: `std::min`, i.e. `second < first ? second : first`.
///
/// ```
/// assert!(martensite_tkernel::standard::real::min(f64::NAN, 1.0).is_nan());
/// ```
pub fn min(first: f64, second: f64) -> f64 {
    if second < first { second } else { first }
}
