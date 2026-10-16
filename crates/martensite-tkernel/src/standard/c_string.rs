//! Plain C string helpers: equality, `atof` and the C-string hasher policy.
//!
//! A C string is a byte slice read up to its first NUL (or its end), so embedded NULs
//! end the string exactly as `strlen`/`strcmp` do. `Strtod` is [`crate::standard::strtod`].
//!
//! The formatting wrappers implement `printf`/`fprintf`/`sprintf`/`vsprintf`, which on Linux
//! forward to glibc `vprintf`/`vfprintf`/`vsprintf` in the C locale. C varargs and
//! `va_list` become a slice of typed [`FormatArg`] values; `FILE*` becomes any
//! [`Write`] sink (process stdout for `Printf`). Every directive with a defined meaning
//! is supported: numbered (`%2$d`, `*3$`) or sequential arguments, all C conversions and
//! length modifiers, glibc's `%m`, `%b`, `%C`, `%S`, `q`, `Z`, `wN`, the `'` and `I` flags,
//! wide characters, `long double` and `%n`. Directives C leaves undefined, and arguments
//! of the wrong type, return `InvalidInput` before any byte is written; a wide character
//! with no C-locale form returns `InvalidData` (C `EILSEQ`), and a count above `INT_MAX`
//! returns `InvalidData` (C `EOVERFLOW`), both before writing. Vec buffer variants
//! replace storage; caller-slice variants preserve bytes after the final NUL.

use super::hashing::{HashWord, fnv_hash, hash_bytes};
pub use crate::c_format::FormatArg;
use crate::c_format::{FormatFault, checked_sprintf};
use crate::ncollection::HashPolicy;
use std::io::{self, Write};

/// The bytes `strlen` would count: everything before the first NUL.
pub(crate) fn c_str_prefix(text: &[u8]) -> &[u8] {
    let length: usize = text
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(text.len());
    &text[..length]
}

/// `strcmp(a, b) == 0`, comparing up to the first NUL.
///
/// ```
/// assert!(martensite_tkernel::standard::c_string::is_equal(b"abc\0x", b"abc"));
/// ```
pub fn is_equal(first: &[u8], second: &[u8]) -> bool {
    c_str_prefix(first) == c_str_prefix(second)
}

/// `strtod` of `text`: the C-locale numeric prefix (0 if none).
///
/// ```
/// assert_eq!(martensite_tkernel::standard::c_string::atof(" 2.5e1xyz"), 25.0);
/// ```
pub fn atof(text: &str) -> f64 {
    super::strtod(text).0
}

/// `atof` for arbitrary C-string bytes; `atof_bytes(b"1.25\xff") == 1.25`.
pub fn atof_bytes(text: &[u8]) -> f64 {
    super::strtod_bytes(text).0
}

/// Formats to the process standard output and returns the byte count;
/// `printf(b"%s=%d\n", &[FormatArg::Text(b"edges"), FormatArg::Int(12)])`.
/// A failing stdout returns its error, as C `vprintf` returns -1.
pub fn printf(format: &[u8], args: &[FormatArg<'_>]) -> io::Result<usize> {
    write_format(&mut io::stdout().lock(), format, args)
}

/// `Printf` with typed arguments and an injected writer standing for stdout;
/// `printf_to(&mut out, b"%d", &[FormatArg::Int(4)])`. Returns the bytes written and
/// propagates sink errors.
pub fn printf_to(
    output: &mut impl Write,
    format: &[u8],
    args: &[FormatArg<'_>],
) -> io::Result<usize> {
    write_format(output, format, args)
}

/// `Fprintf` with an injected writer instead of a process-owned `FILE*`;
/// `fprintf_to(&mut file, b"%s", &[FormatArg::Text(b"shape")])`.
/// The whole text is formatted first and handed to the sink in one `write_all`, so a
/// sink error is the C stream error (-1); returns the bytes written.
pub fn fprintf_to(
    output: &mut impl Write,
    format: &[u8],
    args: &[FormatArg<'_>],
) -> io::Result<usize> {
    write_format(output, format, args)
}

/// `Sprintf` into caller-owned growable storage, including the terminating NUL;
/// `sprintf_into(&mut buffer, b"%04d", &[FormatArg::Int(7)])` returns 4.
/// The `Vec` is replaced; use [`sprintf_into_slice`] to keep a fixed buffer's tail bytes.
pub fn sprintf_into(
    output: &mut Vec<u8>,
    format: &[u8],
    args: &[FormatArg<'_>],
) -> io::Result<usize> {
    let rendered = render_format(format, args)?;
    let count = rendered.len();
    output.clear();
    output.extend_from_slice(&rendered);
    output.push(0);
    Ok(count)
}

/// `sprintf` into a fixed caller slice; `sprintf_into_slice(&mut b, b"%s", &[FormatArg::Text(b"edge")])`.
/// The slice must fit the bytes plus final NUL; remaining bytes are left untouched.
pub fn sprintf_into_slice(
    output: &mut [u8],
    format: &[u8],
    args: &[FormatArg<'_>],
) -> io::Result<usize> {
    let rendered = render_format(format, args)?;
    write_formatted_slice(output, &rendered)
}

/// Typed `vsprintf` counterpart; `vsprintf_into(&mut b, b"%s", &[FormatArg::Text(b"edge")])` writes `edge\0`.
/// The `va_list` is the typed argument slice; the `Vec` is replaced.
pub fn vsprintf_into(
    output: &mut Vec<u8>,
    format: &[u8],
    args: &[FormatArg<'_>],
) -> io::Result<usize> {
    sprintf_into(output, format, args)
}

/// Typed `vsprintf` into a fixed caller slice; `vsprintf_into_slice(&mut b, b"%s", &[FormatArg::Text(b"edge")])`.
/// `va_list` is replaced by `FormatArg` values; short slices return an error without mutation.
pub fn vsprintf_into_slice(
    output: &mut [u8],
    format: &[u8],
    args: &[FormatArg<'_>],
) -> io::Result<usize> {
    sprintf_into_slice(output, format, args)
}

fn write_format(
    output: &mut impl Write,
    format: &[u8],
    args: &[FormatArg<'_>],
) -> io::Result<usize> {
    let rendered = render_format(format, args)?;
    output.write_all(&rendered)?;
    Ok(rendered.len())
}

fn write_formatted_slice(output: &mut [u8], rendered: &[u8]) -> io::Result<usize> {
    let required = required_slice_capacity(rendered.len())?;
    ensure_slice_capacity(output.len(), required)?;
    write_slice_contents(output, rendered);
    Ok(rendered.len())
}

fn required_slice_capacity(rendered_length: usize) -> io::Result<usize> {
    rendered_length
        .checked_add(1)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "formatted length overflow"))
}

fn ensure_slice_capacity(actual: usize, required: usize) -> io::Result<()> {
    if actual >= required {
        return Ok(());
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("formatted result needs {required} bytes; caller buffer has {actual}"),
    ))
}

fn write_slice_contents(output: &mut [u8], rendered: &[u8]) {
    output[..rendered.len()].copy_from_slice(rendered);
    output[rendered.len()] = 0;
}

fn render_format(format: &[u8], args: &[FormatArg<'_>]) -> io::Result<Vec<u8>> {
    let rendered = checked_sprintf(c_str_prefix(format), args).map_err(format_fault)?;
    c_int_count(rendered.len())?;
    Ok(rendered)
}

/// The C `int` the C formatting functions return: a count above `INT_MAX` fails with
/// `EOVERFLOW` (C11 7.21.6.1p14 return value); `c_int_count(3)` is `Ok(3)`.
pub(crate) fn c_int_count(count: usize) -> io::Result<i32> {
    i32::try_from(count).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "formatted length {count} exceeds INT_MAX {} (EOVERFLOW)",
                i32::MAX
            ),
        )
    })
}

fn format_fault(fault: FormatFault) -> io::Error {
    match fault {
        FormatFault::Directive(error) => format_error(error),
        FormatFault::Unencodable(error) => io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "wide character {:#x} of the directive at byte {} has no C-locale multibyte form (EILSEQ); expected a unit below 0x80",
                error.unit, error.offset
            ),
        ),
    }
}

fn format_error(error: crate::c_format::FormatError) -> io::Error {
    let conversion = if error.conversion == 0 {
        "end-of-format".to_owned()
    } else {
        format!("'{}'", char::from(error.conversion))
    };
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!(
            "format conversion {conversion} at byte {}: expected {}",
            error.offset, error.expected
        ),
    )
}

/// The hash/equality policy for C-string keys.
///
/// Strings shorter than four bytes use FNV-1a over their bytes (so the empty string
/// hashes to the FNV offset basis); longer ones use the Murmur byte hash.
#[derive(Clone, Copy, Debug, Default)]
pub struct CStringHasher;

impl CStringHasher {
    /// The hash of the C string in `text`.
    ///
    /// ```
    /// use martensite_tkernel::standard::c_string::CStringHasher;
    /// assert_eq!(CStringHasher.hash_code(b"abc\0tail"), CStringHasher.hash_code(b"abc"));
    /// ```
    pub fn hash_code(&self, text: &[u8]) -> usize {
        let text: &[u8] = c_str_prefix(text);
        if text.len() < 4 {
            return fnv_hash::hash_combine(text, text.len(), usize::FNV_SEED);
        }
        hash_bytes(text)
    }

    /// C-string equality: `strcmp(a, b) == 0`.
    ///
    /// ```
    /// use martensite_tkernel::standard::c_string::CStringHasher;
    /// assert!(CStringHasher.is_equal(b"abc", b"abc\0"));
    /// ```
    pub fn is_equal(&self, first: &[u8], second: &[u8]) -> bool {
        is_equal(first, second)
    }
}

impl<K: AsRef<[u8]>> HashPolicy<K> for CStringHasher {
    fn hash(&self, key: &K) -> usize {
        self.hash_code(key.as_ref())
    }
    fn equal(&self, first: &K, second: &K) -> bool {
        self.is_equal(first.as_ref(), second.as_ref())
    }
}

#[cfg(test)]
mod count_tests {
    #[test]
    fn counts_above_int_max_fail_with_eoverflow() {
        assert_eq!(super::c_int_count(0).unwrap(), 0);
        assert_eq!(super::c_int_count(i32::MAX as usize).unwrap(), i32::MAX);
        let error = super::c_int_count(i32::MAX as usize + 1).unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
        assert!(error.to_string().contains("2147483648"));
        assert!(error.to_string().contains("EOVERFLOW"));
    }
}
