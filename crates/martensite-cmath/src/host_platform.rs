//! The platform C math library entry points the kernel called before #201.
//!
//! Compiled only with the `host-libm` or the `libm-attribution` feature. `host-libm`
//! returns these results from every adapter function (behavior version 1 replay);
//! `libm-attribution` compares them with the portable results. Neither feature is
//! enabled by a release build or a Martensite CI gate.

pub(crate) use entry_points::*;

#[allow(clippy::disallowed_methods)] // These are the platform calls being replayed.
mod entry_points {
    macro_rules! platform_methods {
        ($($name:ident => $method:ident),* $(,)?) => {$(
            pub(crate) fn $name(x: f64) -> f64 {
                x.$method()
            }
        )*};
    }

    platform_methods! {
        sin => sin, cos => cos, tan => tan, asin => asin, acos => acos, atan => atan,
        sinh => sinh, cosh => cosh, tanh => tanh, exp => exp, exp2 => exp2,
        expm1 => exp_m1, log => ln, log1p => ln_1p, log2 => log2, log10 => log10,
    }

    pub(crate) use super::glibc_cbrt::cbrt;
    pub(crate) use crate::ffi::{acosh, asinh, atanh};

    pub(crate) fn sincos(x: f64) -> (f64, f64) {
        x.sin_cos()
    }

    pub(crate) fn atan2(y: f64, x: f64) -> f64 {
        y.atan2(x)
    }

    pub(crate) fn pow(x: f64, y: f64) -> f64 {
        x.powf(y)
    }

    pub(crate) fn powf(x: f32, y: f32) -> f32 {
        x.powf(y)
    }

    pub(crate) fn powi(x: f64, n: i32) -> f64 {
        x.powi(n)
    }

    pub(crate) fn hypot(x: f64, y: f64) -> f64 {
        x.hypot(y)
    }
}

/// The pre-#201 kernel's cube root, removed from `martensite-tkmath` in #201.
///
/// It reproduced glibc 2.39 `cbrt` (`sysdeps/ieee754/dbl-64/s_cbrt.c`) in Rust because
/// an `extern "C" cbrt` resolves to the correctly rounded compiler-builtins symbol,
/// not to glibc. Kept here unchanged so the platform replay returns the old bits.
#[allow(clippy::excessive_precision)] // The literals are glibc's source text.
mod glibc_cbrt {
    const CBRT2: f64 = 1.2599210498948731648;
    const SQR_CBRT2: f64 = 1.5874010519681994748;
    const FACTOR: [f64; 5] = [1.0 / SQR_CBRT2, 1.0 / CBRT2, 1.0, CBRT2, SQR_CBRT2];

    fn frexp(x: f64) -> (f64, i32) {
        let bits: u64 = x.to_bits();
        let field: i32 = ((bits >> 52) & 0x7ff) as i32;
        if x == 0.0 || field == 0x7ff {
            return (x, 0);
        }
        if field == 0 {
            let (mantissa, exponent): (f64, i32) = frexp(x * f64::from_bits((1023 + 54) << 52));
            return (mantissa, exponent - 54);
        }
        let mantissa: f64 = f64::from_bits((bits & !(0x7ff << 52)) | (1022 << 52));
        (mantissa, field - 1022)
    }

    fn unsigned_cbrt(x: f64) -> f64 {
        let (xm, xe): (f64, i32) = frexp(x.abs());
        if xe == 0 && (x.is_nan() || x.is_infinite() || x == 0.0) {
            return x + x;
        }
        let u: f64 = 0.354895765043919860
            + ((1.50819193781584896
                + ((-2.11499494167371287
                    + ((2.44693122563534430
                        + ((-1.83469277483613086
                            + (0.784932344976639262 - 0.145263899385486377 * xm) * xm)
                            * xm))
                        * xm))
                    * xm))
                * xm);
        let t2: f64 = u * u * u;
        let ym: f64 = u * (t2 + 2.0 * xm) / (2.0 * t2 + xm) * FACTOR[(2 + xe % 3) as usize];
        let scale: f64 = f64::from_bits(((xe / 3 + 1023) as u64) << 52);
        (if x > 0.0 { ym } else { -ym }) * scale
    }

    /// The old `math_utils::cube_root` sign dispatch around the glibc kernel.
    pub(crate) fn cbrt(value: f64) -> f64 {
        if value >= 0.0 {
            unsigned_cbrt(value)
        } else {
            -unsigned_cbrt(-value)
        }
    }
}
