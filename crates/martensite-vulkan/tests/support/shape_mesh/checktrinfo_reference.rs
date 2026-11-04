//! Preserve the checker’s source-token dispatch before converting a finite numeric reference.
use super::{CountExpectation, Failure};

impl CountExpectation {
    /// Preserve boolean versus numeric spelling; `CountExpectation::from_checktrinfo_reference("0", 0., 1e-7)?`.
    pub fn from_checktrinfo_reference(
        reference: &str,
        tolerance_absolute: f64,
        tolerance_relative: f64,
    ) -> Result<Self, Failure> {
        if let Some(present) = source_boolean(reference) {
            return Ok(if present { Self::Present } else { Self::Any });
        }
        let value = source_number(reference).ok_or_else(|| invalid_reference(reference))?;
        Ok(Self::Near {
            value,
            tolerance_absolute,
            tolerance_relative,
        })
    }
}

fn source_boolean(reference: &str) -> Option<bool> {
    if reference.is_empty() {
        return None;
    }
    let lower = reference.to_ascii_lowercase();
    let words = [
        ("0", false),
        ("1", true),
        ("false", false),
        ("true", true),
        ("yes", true),
        ("no", false),
        ("on", true),
        ("off", false),
    ];
    let mut matches = words.iter().filter(|(word, _)| word.starts_with(&lower));
    let (_, value) = matches.next()?;
    matches.next().is_none().then_some(*value)
}

fn source_number(reference: &str) -> Option<f64> {
    let value = reference.trim();
    let integer = value.strip_prefix(['+', '-']).unwrap_or(value);
    if (integer.bytes().all(|b| b.is_ascii_digit()) || integer.starts_with(['0']))
        && let Some(number) = source_integer(value)
    {
        return Some(number);
    }
    let parsed: f64 = value.parse().ok()?;
    // Integral-looking literals with a leading zero are octal, never silently decimal.
    if integer.len() > 1 && integer.starts_with('0') && integer.bytes().all(|b| b.is_ascii_digit())
    {
        return None;
    }
    parsed.is_finite().then_some(parsed)
}

fn source_integer(reference: &str) -> Option<f64> {
    let negative = reference.starts_with('-');
    let digits = reference.strip_prefix(['+', '-']).unwrap_or(reference);
    let (radix, digits) = integer_digits(digits);
    let value = u128::from_str_radix(digits, radix).ok()? as f64;
    Some(if negative { -value } else { value })
}

fn integer_digits(reference: &str) -> (u32, &str) {
    for (prefix, radix) in [
        ("0x", 16),
        ("0X", 16),
        ("0o", 8),
        ("0O", 8),
        ("0b", 2),
        ("0B", 2),
    ] {
        if let Some(digits) = reference.strip_prefix(prefix) {
            return (radix, digits);
        }
    }
    if reference.len() > 1 && reference.starts_with('0') {
        return (8, &reference[1..]);
    }
    (10, reference)
}

fn invalid_reference(reference: &str) -> Failure {
    Failure::new(&format!(
        "checktrinfo reference={reference:?}; expected a nonempty boolean spelling or finite numeric literal"
    ))
}

#[cfg(test)]
#[path = "checktrinfo_reference_tests.rs"]
mod tests;
