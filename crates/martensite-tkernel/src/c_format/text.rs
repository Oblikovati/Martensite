//! Character and string conversions (`%c %s %lc %ls %C %S`), `%m` and `%n`.
//!
//! In the C locale glibc accepts only 7-bit wide units; MSVC accepts 8-bit units.
//! An unencodable wide string fails; MSVC omits an unencodable single character (#337).
use super::{CArg, Length, Spec, Unencodable, errno_text::os_error_text, pad};

/// Renders one `c s C S` directive (narrow unless the `l` modifier or `C`/`S` asks wide).
pub(super) fn render(
    spec: &Spec,
    argument: Option<&CArg<'_>>,
    offset: usize,
    output: &mut Vec<u8>,
) -> Result<(), Unencodable> {
    let wide = spec.length == Length::Long || matches!(spec.conversion, b'C' | b'S');
    let body = match (spec.conversion, wide) {
        (b'c', false) => character(spec, argument),
        (b's', false) => text(spec, argument),
        (b'c' | b'C', true) => wide_character(spec, argument, offset)?,
        _ => wide_text(spec, argument, offset)?,
    };
    output.extend_from_slice(&body);
    Ok(())
}

fn character(spec: &Spec, argument: Option<&CArg<'_>>) -> Vec<u8> {
    let byte = match argument {
        Some(CArg::Int(value)) => *value as u8,
        _ => 0,
    };
    pad(spec, Vec::new(), vec![byte], false)
}

fn text(spec: &Spec, argument: Option<&CArg<'_>>) -> Vec<u8> {
    let bytes: &[u8] = match argument {
        Some(CArg::Text(bytes)) => bytes,
        _ => b"(null)",
    };
    let bytes = &bytes[..bytes
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(bytes.len())];
    shown(spec, bytes)
}

/// Pads `bytes` cut to the precision; `%s` and `%m` share it.
fn shown(spec: &Spec, bytes: &[u8]) -> Vec<u8> {
    let shown = spec
        .precision
        .map_or(bytes, |limit| &bytes[..limit.min(bytes.len())]);
    pad(spec, Vec::new(), shown.to_vec(), false)
}

fn wide_character(
    spec: &Spec,
    argument: Option<&CArg<'_>>,
    offset: usize,
) -> Result<Vec<u8>, Unencodable> {
    let unit = match argument {
        Some(CArg::WideChar(unit)) => *unit,
        _ => 0,
    };
    let byte = match c_locale_byte(unit, offset) {
        Ok(byte) => byte,
        Err(_) if cfg!(windows) => return Ok(Vec::new()),
        Err(error) => return Err(error),
    };
    Ok(pad(spec, Vec::new(), vec![byte], false))
}

/// `%ls`: units up to the first zero, at most `precision` bytes; units past the
/// precision are never converted, so they cannot fail.
fn wide_text(
    spec: &Spec,
    argument: Option<&CArg<'_>>,
    offset: usize,
) -> Result<Vec<u8>, Unencodable> {
    let units: &[u32] = match argument {
        Some(CArg::WideText(units)) => units,
        _ => return Ok(shown(spec, b"(null)")),
    };
    let length = units
        .iter()
        .position(|&unit| unit == 0)
        .unwrap_or(units.len());
    let limit = spec
        .precision
        .map_or(length, |precision| precision.min(length));
    let bytes = units[..limit]
        .iter()
        .map(|&unit| c_locale_byte(unit, offset))
        .collect::<Result<Vec<u8>, Unencodable>>()?;
    Ok(pad(spec, Vec::new(), bytes, false))
}

/// The native C-locale single-byte range differs between MSVC and Unix (#337).
fn c_locale_byte(unit: u32, offset: usize) -> Result<u8, Unencodable> {
    u8::try_from(unit)
        .ok()
        .filter(|byte| cfg!(windows) || byte.is_ascii())
        .ok_or(Unencodable { offset, unit })
}

#[cfg(all(test, windows))]
#[allow(unsafe_code)]
mod windows_tests {
    #[test]
    fn wide_units_match_the_native_msvc_c_locale() {
        let _locale = crate::standard::c_locale_sentry::CLocaleSentry::new();
        for format in [c"%lc", c"%3lc", c"%-3lc"] {
            for unit in [0u32, 0x41, 0x7f, 0x80, 0xe9, 0xff, 0x100, 0xd800] {
                let mut buffer = [0i8; 16];
                // SAFETY: buffer and terminated format match the promoted %lc argument.
                let length = unsafe {
                    libc::snprintf(buffer.as_mut_ptr(), buffer.len(), format.as_ptr(), unit)
                };
                assert!(length >= 0);
                let expected: Vec<u8> = buffer[..length as usize]
                    .iter()
                    .map(|byte| *byte as u8)
                    .collect();
                assert_eq!(
                    crate::c_format::c_sprintf(format.to_bytes(), &[super::CArg::WideChar(unit)]),
                    expected,
                    "unit {unit:#x}, format {format:?}"
                );
            }
        }
    }
}
/// `%m`: `strerror(errno)`, padded and cut like a `%s` of that text.
pub(super) fn errno_message(spec: &Spec, errno: i32) -> Vec<u8> {
    shown(spec, os_error_text(errno).as_bytes())
}

/// `%n`: stores `written` narrowed to the length modifier's type, as C converts it.
pub(super) fn store_count(spec: &Spec, argument: Option<&CArg<'_>>, written: usize) {
    let Some(CArg::Count(target)) = argument else {
        return;
    };
    let count = written as i64;
    let narrowed = match spec.length {
        Length::Char => i64::from(count as i8),
        Length::Short => i64::from(count as i16),
        Length::None => i64::from(count as i32),
        _ => count,
    };
    target.set(narrowed);
}
