//! C's `%g` at default precision with the six digits rounded by Rust's formatter.
use crate::tcollection::ascii_string::{AsciiBytes, AsciiString};
use std::io::Write;

pub(super) fn format_general(value: f64) -> AsciiString {
    let mut terminated = AsciiBytes::new();
    if value.is_sign_negative() {
        terminated.push(b'-');
    }
    if value.is_finite() {
        write_finite(&mut terminated, value.abs());
    } else {
        terminated.extend_from_slice(if value.is_nan() { b"nan" } else { b"inf" });
    }
    terminated.push(0);
    AsciiString { terminated }
}

fn write_finite(bytes: &mut AsciiBytes, magnitude: f64) {
    // The positive scientific spelling occupies at most twelve bytes, including
    // an exponent sign and three exponent digits.
    let mut rounded = [0_u8; 16];
    let mut unused = rounded.as_mut_slice();
    let _ = write!(&mut unused, "{magnitude:.5e}");
    let unused_length = unused.len();
    let mut digits = [rounded[0]; 6];
    digits[1..].copy_from_slice(&rounded[2..7]);
    let exponent = decimal_exponent(&rounded[8..rounded.len() - unused_length]);
    let last = digits.iter().rposition(|&byte| byte != b'0').unwrap_or(0);
    write_rounded(bytes, &digits, last + 1, exponent);
}

fn decimal_exponent(text: &[u8]) -> i32 {
    let (digits, sign) = text
        .strip_prefix(b"-")
        .map_or((text, 1), |digits| (digits, -1));
    sign * digits
        .iter()
        .fold(0, |value, &digit| 10 * value + i32::from(digit - b'0'))
}

fn write_rounded(bytes: &mut AsciiBytes, digits: &[u8; 6], significant: usize, exponent: i32) {
    // %g selects the layout from the rounded exponent and suppresses fraction zeros.
    if !(-4..6).contains(&exponent) {
        write_mantissa(bytes, digits, 1, significant);
        write_exponent(bytes, exponent);
    } else if exponent < 0 {
        bytes.extend_from_slice(b"0.");
        bytes.extend(std::iter::repeat_n(b'0', (-exponent - 1) as usize));
        bytes.extend_from_slice(&digits[..significant]);
    } else {
        write_mantissa(bytes, digits, exponent as usize + 1, significant);
    }
}

fn write_mantissa(bytes: &mut AsciiBytes, digits: &[u8; 6], integer: usize, significant: usize) {
    bytes.extend_from_slice(&digits[..integer]);
    if significant > integer {
        bytes.push(b'.');
        bytes.extend_from_slice(&digits[integer..significant]);
    }
}

fn write_exponent(bytes: &mut AsciiBytes, exponent: i32) {
    bytes.extend_from_slice(if exponent < 0 { b"e-" } else { b"e+" });
    let mut buffer = itoa::Buffer::new();
    let digits = buffer.format(exponent.unsigned_abs()).as_bytes();
    if digits.len() == 1 {
        bytes.push(b'0');
    }
    bytes.extend_from_slice(digits);
}
