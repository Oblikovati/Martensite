//! `frexp` for the solver scale factors.

/// Splits a finite nonzero `x` into a signed mantissa with magnitude in [0.5, 1)
/// and a power of two, like `frexp`;
/// zero, infinity and NaN come back unchanged with exponent 0 (glibc's convention).
pub(crate) fn frexp(x: f64) -> (f64, i32) {
    let bits: u64 = x.to_bits();
    let field: i32 = ((bits >> 52) & 0x7ff) as i32;
    if x == 0.0 || field == 0x7ff {
        return (x, 0);
    }
    if field == 0 {
        // Subnormal: scale into the normal range first (2^54 is exact).
        let (mantissa, exponent): (f64, i32) = frexp(x * f64::from_bits((1023 + 54) << 52));
        return (mantissa, exponent - 54);
    }
    let mantissa: f64 = f64::from_bits((bits & !(0x7ff << 52)) | (1022 << 52));
    (mantissa, field - 1022)
}

#[cfg(test)]
#[path = "frexp_tests.rs"]
mod tests;
