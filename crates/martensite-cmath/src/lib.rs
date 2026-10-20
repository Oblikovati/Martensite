//! Transcendental functions with portable rounding for real-valued results.
//!
//! Every transcendental function the kernel uses comes from this crate. Each one is
//! correctly rounded: it returns the exact mathematical result rounded once to the
//! nearest result in its destination format (ties to even). This gives unique
//! finite nonzero results across the supported release platforms (#201).
//! NaN sign and payload are not specified by correct rounding, and this adapter
//! does not canonicalize them. Domain-error NaNs are tested by classification;
//! signed zeros and infinities follow each function's special-value contract.
//!
//! The standard `f64` methods (`sin`, `exp`, `powf`, ...) lower to each platform's C
//! math library, whose last bit differs between platforms, and the compiler folds
//! constant arguments with its build host's library. `clippy.toml` therefore forbids
//! those methods in every crate; call the functions here instead. Exact IEEE
//! operations (`sqrt`, `mul_add`, `abs`, `floor`, ...) need no adapter.
//!
//! The functions are a thin layer over the pure-Rust `metallic` crate, pinned to an
//! exact version because its results are part of the kernel's recorded behavior.
//! `powi` uses a fixed repeated-squaring sequence with separately documented
//! intermediate rounding. IEEE `remainder`, which Rust lacks, is taken from the
//! platform libm; its finite valid results are exact. [`quotient`] gives invalid
//! divisions one NaN word on every target instead of the hardware default NaN.
//!
//! ```
//! assert_eq!(martensite_cmath::sin(0.0), 0.0);
//! assert_eq!(martensite_cmath::pow(2.0, 10.0), 1024.0);
//! ```

/// Returns the portable result of one adapter call.
///
/// Release and ordinary builds expand to the portable expression alone. The
/// test-only `libm-attribution` feature also evaluates the platform libm for the
/// same call, records any differing result and returns the selected one, so changed
/// test outputs can be attributed to individual elementary-function results (#201).
#[cfg(not(feature = "libm-attribution"))]
macro_rules! observed {
    ($name:ident($($arg:expr),*) => $portable:expr) => {
        $portable
    };
}

#[cfg(feature = "libm-attribution")]
macro_rules! observed {
    ($name:ident($($arg:expr),*) => $portable:expr) => {
        crate::attribution::select(
            stringify!($name),
            &[$(crate::attribution::Words::words(&$arg)),*],
            $portable,
            crate::attribution::platform::$name($($arg),*),
        )
    };
}

#[cfg(feature = "libm-attribution")]
mod attribution;
mod exponential;
mod hyperbolic;
mod integer_power;
mod invalid;
mod trigonometric;

pub use exponential::{cbrt, exp, exp2, expm1, hypot, log, log1p, log2, log10, pow, powf};
pub use hyperbolic::{acosh, asinh, atanh, cosh, sinh, tanh};
pub use integer_power::powi;
pub use invalid::{INVALID_OPERATION_NAN, quotient};
pub use trigonometric::{acos, asin, atan, atan2, cos, sin, sincos, tan};

#[allow(unsafe_code)]
mod ffi {
    // SAFETY: these are C99 libm signatures, linked from the platform C library that
    // std already links. Each function reads only its arguments, has no
    // preconditions and touches no memory, so each is declared `safe`. The
    // feature-gated entries exist only in test attribution builds, never in releases.
    unsafe extern "C" {
        pub safe fn remainder(numerator: f64, denominator: f64) -> f64;
        #[cfg(feature = "libm-attribution")]
        pub safe fn asinh(value: f64) -> f64;
        #[cfg(feature = "libm-attribution")]
        pub safe fn acosh(value: f64) -> f64;
        #[cfg(feature = "libm-attribution")]
        pub safe fn atanh(value: f64) -> f64;
    }
}

/// IEEE 754 remainder from the platform libm, the C `remainder(x, y)`:
/// `x - n*y` with `n` the integer nearest to `x/y` (ties to even).
///
/// A finite result for valid inputs is exactly representable. Invalid inputs
/// produce NaN, whose sign and payload are not part of this interface's contract.
///
/// ```
/// assert_eq!(martensite_cmath::remainder(7.0, 2.0), -1.0);
/// ```
pub fn remainder(numerator: f64, denominator: f64) -> f64 {
    ffi::remainder(numerator, denominator)
}
