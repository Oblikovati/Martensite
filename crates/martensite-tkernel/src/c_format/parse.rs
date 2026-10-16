//! Directive parsing: `%[n$][flags][width][.precision][length]conversion`, as glibc reads it.
use super::{Length, Spec};

/// Parses the directive after its `%` at `index`; returns the spec and the index after it.
///
/// A missing conversion (the format ends inside the directive) leaves `conversion` 0.
pub(crate) fn parse_spec(format: &[u8], index: usize) -> (Spec, usize) {
    let mut spec = Spec::default();
    let (position, mut index) = positional(format, index);
    spec.position = position;
    index = parse_flags(format, index, &mut spec);
    index = parse_width(format, index, &mut spec);
    if format.get(index) == Some(&b'.') {
        index = parse_precision(format, index + 1, &mut spec);
    }
    (spec.length, index) = length_modifier(format, index);
    spec.conversion = format.get(index).copied().unwrap_or(0);
    (spec, (index + 1).min(format.len()))
}

/// `n$`: a 1-based argument position (POSIX numbered arguments); none when absent.
fn positional(format: &[u8], index: usize) -> (Option<usize>, usize) {
    let (value, end) = digits(format, index);
    if end > index && format.get(end) == Some(&b'$') {
        return (Some(value), end + 1);
    }
    (None, index)
}

fn parse_flags(format: &[u8], mut index: usize, spec: &mut Spec) -> usize {
    while let Some(&flag) = format.get(index) {
        match flag {
            b'-' => spec.left = true,
            b'+' => spec.plus = true,
            b' ' => spec.space = true,
            b'#' => spec.alternate = true,
            b'0' => spec.zero = true,
            // Grouping and locale digits: no-ops in the C locale the wrappers run in.
            b'\'' | b'I' => spec.locale_flag = true,
            _ => break,
        }
        index += 1;
    }
    index
}

fn parse_width(format: &[u8], index: usize, spec: &mut Spec) -> usize {
    if format.get(index) == Some(&b'*') {
        spec.dynamic_width = true;
        let (position, end) = positional(format, index + 1);
        spec.width_position = position;
        return end;
    }
    let (width, end) = digits(format, index);
    spec.width = width;
    end
}

fn parse_precision(format: &[u8], index: usize, spec: &mut Spec) -> usize {
    if format.get(index) == Some(&b'*') {
        spec.dynamic_precision = true;
        let (position, end) = positional(format, index + 1);
        spec.precision_position = position;
        return end;
    }
    let (precision, end) = digits(format, index);
    spec.precision = Some(precision);
    end
}

pub(super) fn digits(format: &[u8], mut index: usize) -> (usize, usize) {
    let mut value: usize = 0;
    while let Some(digit) = format.get(index).filter(|byte| byte.is_ascii_digit()) {
        value = value
            .saturating_mul(10)
            .saturating_add(usize::from(digit - b'0'));
        index += 1;
    }
    (value, index)
}

/// The length modifier, with glibc's aliases: `q` is `ll`, `Z` is `z`, and C23 `wN`/`wfN`
/// name the exact and fast integer widths of the Linux LP64 ABI.
fn length_modifier(format: &[u8], index: usize) -> (Length, usize) {
    match (format.get(index), format.get(index + 1)) {
        (Some(b'h'), Some(b'h')) => (Length::Char, index + 2),
        (Some(b'h'), _) => (Length::Short, index + 1),
        (Some(b'l'), Some(b'l')) | (Some(b'q'), _) => (
            Length::LongLong,
            index + 1 + usize::from(format[index] == b'l'),
        ),
        (Some(b'l'), _) => (Length::Long, index + 1),
        (Some(b'z' | b'Z'), _) => (Length::Size, index + 1),
        (Some(b't'), _) => (Length::PtrDiff, index + 1),
        (Some(b'j'), _) => (Length::IntMax, index + 1),
        (Some(b'L'), _) => (Length::LongDouble, index + 1),
        (Some(b'w'), _) => bit_width(format, index + 1),
        _ => (Length::None, index),
    }
}

/// C23 `wN` (exact `intN_t`) and `wfN` (`int_fastN_t`; glibc makes the 16- to 64-bit
/// fast types `long`). Any other width is invalid.
fn bit_width(format: &[u8], index: usize) -> (Length, usize) {
    let fast = format.get(index) == Some(&b'f');
    let start = index + usize::from(fast);
    let (bits, end) = digits(format, start);
    let length = match (bits, fast, end > start) {
        (8, _, true) => Length::Char,
        (16, false, true) => Length::Short,
        (32, false, true) => Length::None,
        (64, false, true) | (16 | 32 | 64, true, true) => Length::Long,
        _ => Length::Invalid,
    };
    (length, end)
}
