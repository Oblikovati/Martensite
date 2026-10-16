//! `%La`: the hexadecimal spelling of a `long double` holding a binary64 value.
//!
//! glibc on x86 stores `long double` in the x87 80-bit format, whose 64-bit significand
//! keeps its leading bit, so the leading hex digit is 8..f: `%La` of 1 prints `0x8p-3`.
//! Binary64 subnormals are normal there (`5e-324` prints `0x8p-1077`).
use super::Spec;
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), not(windows)))]
use super::pad;

/// Formats a finite non-negative `magnitude` after the sign `prefix`.
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), not(windows)))]
pub(super) fn format(spec: &Spec, magnitude: f64, mut prefix: Vec<u8>) -> Vec<u8> {
    let (significand, exponent) = x87_parts(magnitude);
    let (lead, fraction, exponent) = rounded(significand, exponent, spec.precision);
    let mut digits = mantissa(lead, fraction, spec);
    digits.extend_from_slice(format!("p{exponent:+}").as_bytes());
    let upper = spec.conversion == b'A';
    prefix.extend_from_slice(if upper { b"0X" } else { b"0x" });
    if upper {
        digits.make_ascii_uppercase();
    }
    pad(spec, prefix, digits, true)
}

/// MSVC's `long double` is binary64, so `%La` is `%a`.
#[cfg(windows)]
pub(super) fn format(spec: &Spec, magnitude: f64, prefix: Vec<u8>) -> Vec<u8> {
    super::hexadecimal::format(spec, magnitude, prefix)
}

// TODO(platform): AArch64 and other non-x86 Unix targets store `long double` as IEEE
// binary128 (leading digit 1, 28 fraction digits); only the x86 glibc spelling is implemented.
// Until it is, the binary64 spelling is used there.
#[cfg(not(any(windows, target_arch = "x86", target_arch = "x86_64")))]
pub(super) fn format(spec: &Spec, magnitude: f64, prefix: Vec<u8>) -> Vec<u8> {
    super::hexadecimal::format(spec, magnitude, prefix)
}

/// The 64-bit x87 significand (leading bit 63 set) and the exponent of the leading
/// hex digit (`value = significand / 2^60 * 2^exponent`); zero is `(0, 0)`.
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), not(windows)))]
fn x87_parts(magnitude: f64) -> (u64, i32) {
    if magnitude == 0.0 {
        return (0, 0);
    }
    let bits = magnitude.to_bits();
    let biased = (bits >> 52) as i32;
    let fraction = bits & ((1_u64 << 52) - 1);
    let (stored, binary_exponent) = if biased == 0 {
        (fraction, -1074)
    } else {
        (fraction | (1_u64 << 52), biased - 1075)
    };
    let shift = stored.leading_zeros();
    // value = stored * 2^binary_exponent = (stored << shift) / 2^60 * 2^(binary_exponent - shift + 60)
    (stored << shift, binary_exponent - shift as i32 + 60)
}

/// Rounds the 15 fraction digits to `precision` (nearest, ties to even, as glibc); a
/// carry out of the leading digit makes it 1 and adds 4 to the exponent.
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), not(windows)))]
fn rounded(significand: u64, exponent: i32, precision: Option<usize>) -> (u64, u64, i32) {
    let digits = precision.unwrap_or(15).min(15);
    let discarded = 4 * (15 - digits) as u32;
    let wide = u128::from(significand);
    let retained = wide >> discarded;
    let remainder = wide & ((1_u128 << discarded) - 1);
    let half = if discarded == 0 {
        0
    } else {
        1_u128 << (discarded - 1)
    };
    let up = discarded != 0 && (remainder > half || (remainder == half && retained & 1 != 0));
    let value = retained + u128::from(up);
    let fraction_bits = 4 * digits as u32;
    if value >> fraction_bits >= 16 {
        return (1, 0, exponent + 4);
    }
    let fraction = (value & ((1_u128 << fraction_bits) - 1)) as u64;
    (
        (value >> fraction_bits) as u64,
        fraction << (60 - fraction_bits),
        exponent,
    )
}

/// The leading digit, the point and the fraction digits (trimmed when no precision).
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), not(windows)))]
fn mantissa(lead: u64, fraction: u64, spec: &Spec) -> Vec<u8> {
    let full = format!("{fraction:015x}");
    let count = spec
        .precision
        .unwrap_or_else(|| full.trim_end_matches('0').len());
    let mut digits = format!("{lead:x}").into_bytes();
    if count != 0 || spec.alternate {
        digits.push(b'.');
    }
    digits.extend_from_slice(&full.as_bytes()[..count.min(15)]);
    digits.resize(digits.len() + count.saturating_sub(15), b'0');
    digits
}

#[cfg(all(test, any(target_arch = "x86", target_arch = "x86_64"), not(windows)))]
mod tests {
    use crate::c_format::{CArg, c_sprintf};

    fn hex(format: &str, value: f64) -> String {
        String::from_utf8(c_sprintf(format.as_bytes(), &[CArg::LongDouble(value)])).unwrap()
    }

    #[test]
    fn x87_rounding_matches_glibc() {
        // glibc 2.39 x86-64 spellings of `(long double)` of each binary64 value.
        assert_eq!(hex("%La", 15.9), "0xf.e666666666668p+0");
        assert_eq!(hex("%.0La", 15.9), "0x1p+4");
        assert_eq!(hex("%.0La", 15.5), "0x1p+4");
        assert_eq!(hex("%.0La", 14.5), "0xep+0");
        assert_eq!(hex("%.1La", 1.9999), "0x1.0p+1");
        assert_eq!(hex("%#.0La", 15.5), "0x1.p+4");
        assert_eq!(hex("%.16La", 15.9), "0xf.e666666666668000p+0");
        assert_eq!(hex("%La", 0.1), "0xc.cccccccccccdp-7");
        assert_eq!(hex("%.2La", 1.0 / 3.0), "0xa.abp-5");
        assert_eq!(hex("%La", 5e-324), "0x8p-1077");
        assert_eq!(hex("%La", 1e-320), "0xf.dp-1067");
        assert_eq!(hex("%La", 0.0), "0x0p+0");
        assert_eq!(hex("%.3La", 0.0), "0x0.000p+0");
        assert_eq!(hex("%LA", 3.0), "0XCP-2");
    }
}
