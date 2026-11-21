//! `%d`/`%g` construction and the `strtol`/`Strtod` based numeric queries.
use super::AsciiString;
use super::raise::no_number;
use crate::standard::{Failure, character, strtod};
mod real;

impl AsciiString {
    /// Decimal `%d`; `AsciiString::from_integer(-12).as_bytes() == b"-12"`.
    pub fn from_integer(value: i32) -> Self {
        Self::from_raw(itoa::Buffer::new().format(value).as_bytes())
    }
    /// C's default `%g` (six significant digits, `inf`,
    /// `-nan`, `-0`); `AsciiString::from_real(3.3).as_bytes() == b"3.3"`.
    pub fn from_real(value: f64) -> Self {
        real::format_general(value)
    }
    /// `strtol` base 10 of the C string, the `long` result truncated to
    /// `int`; `Standard_NumericError` without digits; `AsciiString::new("42x").integer_value()? == 42`.
    pub fn integer_value(&self) -> Result<i32, Failure> {
        let (value, end) = integer_prefix(self.as_bytes());
        if end == 0 {
            return Err(no_number("IntegerValue", self.as_bytes()));
        }
        // `(int)strtol(...)` keeps the low 32 bits of the clamped 64-bit `long`.
        Ok(value as i32)
    }
    /// An integer prefix and no `'.'` anywhere after it, even past an
    /// embedded NUL; `AsciiString::new("12e3").is_integer_value()`.
    pub fn is_integer_value(&self) -> bool {
        let (_, end) = integer_prefix(self.as_bytes());
        end != 0 && !self.as_bytes()[end..].contains(&b'.')
    }
    /// `Strtod` of the C string; `Standard_NumericError` without a number;
    /// `AsciiString::new("1.5e2x").real_value()? == 150.0`.
    pub fn real_value(&self) -> Result<f64, Failure> {
        let (value, end) = self.numeric_prefix();
        if end == 0 {
            return Err(no_number("RealValue", self.as_bytes()));
        }
        Ok(value)
    }
    /// A numeric prefix, or with `full` a number followed by NUL; an empty
    /// string passes the full check; `AsciiString::new("1.5").is_real_value(true)`.
    pub fn is_real_value(&self, full: bool) -> bool {
        let (_, end) = self.numeric_prefix();
        if full {
            end == self.len() || self.as_bytes().get(end) == Some(&0)
        } else {
            end != 0
        }
    }
    fn numeric_prefix(&self) -> (f64, usize) {
        // A non-ASCII byte cannot extend a C-locale number; retain its byte offset.
        let end = self
            .as_bytes()
            .iter()
            .position(|byte| !byte.is_ascii())
            .unwrap_or(self.len());
        let ascii = std::str::from_utf8(&self.as_bytes()[..end]).unwrap_or_default();
        strtod(ascii)
    }
}

/// C `strtol(text, &end, 10)` in the C locale: the value clamped to the 64-bit `long` range
/// and the end offset, 0 when no digit was read.
fn integer_prefix(bytes: &[u8]) -> (i64, usize) {
    let start = bytes
        .iter()
        .position(|&byte| !character::is_space(byte))
        .unwrap_or(bytes.len());
    let negative = bytes.get(start) == Some(&b'-');
    let digits = start + usize::from(matches!(bytes.get(start), Some(b'+' | b'-')));
    let count = bytes[digits..]
        .iter()
        .take_while(|byte| byte.is_ascii_digit())
        .count();
    if count == 0 {
        return (0, 0);
    }
    let magnitude = decimal_magnitude(&bytes[digits..digits + count]);
    (clamp_long(magnitude, negative), digits + count)
}

/// The value of ASCII digits, saturating at `u64::MAX`; any overflow clamps to `long` anyway.
fn decimal_magnitude(digits: &[u8]) -> u64 {
    digits.iter().fold(0, |value: u64, byte| {
        value
            .saturating_mul(10)
            .saturating_add(u64::from(byte - b'0'))
    })
}

/// `LONG_MIN`/`LONG_MAX` saturation of a signed magnitude, as `strtol` reports overflow.
fn clamp_long(magnitude: u64, negative: bool) -> i64 {
    if negative {
        (-(magnitude.min(1_u64 << 63) as i128)) as i64
    } else {
        magnitude.min(i64::MAX as u64) as i64
    }
}
