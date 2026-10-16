//! Character classification through the active C character locale.
//!
//! C classification takes `char` and casts it to `unsigned char` before calling `<cctype>`,
//! so a negative `char` is classified as its byte value 128..255. This module takes that byte
//! (`u8`) directly, which keeps all 256 values independent of the platform's `char`
//! signedness. Unix builds use the host C library and follow its active `LC_CTYPE` locale.
//! Non-Unix builds use the ASCII fallback and have not been verified for non-C locales.

#[cfg(unix)]
#[path = "character/ctype_unix.rs"]
mod ctype;
#[cfg(not(unix))]
#[path = "character/ctype_fallback.rs"]
mod ctype;

/// C `isalpha`.
///
/// ```
/// assert!(martensite_tkernel::standard::character::is_alphabetic(b'A'));
/// ```
pub fn is_alphabetic(byte: u8) -> bool {
    ctype::is_alpha(byte)
}

/// C `isdigit`.
///
/// ```
/// assert!(martensite_tkernel::standard::character::is_digit(b'7'));
/// ```
pub fn is_digit(byte: u8) -> bool {
    ctype::is_digit(byte)
}

/// C `isxdigit`.
///
/// ```
/// assert!(martensite_tkernel::standard::character::is_xdigit(b'f'));
/// ```
pub fn is_xdigit(byte: u8) -> bool {
    ctype::is_xdigit(byte)
}

/// `IsAlphabetic || IsDigit`.
///
/// ```
/// assert!(martensite_tkernel::standard::character::is_alphanumeric(b'z'));
/// ```
pub fn is_alphanumeric(byte: u8) -> bool {
    is_alphabetic(byte) || is_digit(byte)
}

/// C `iscntrl` (0..=31 and 127).
///
/// ```
/// assert!(martensite_tkernel::standard::character::is_control(0x7F));
/// ```
pub fn is_control(byte: u8) -> bool {
    ctype::is_control(byte)
}

/// C `isgraph` (`!`..=`~`).
///
/// ```
/// assert!(!martensite_tkernel::standard::character::is_graphic(b' '));
/// ```
pub fn is_graphic(byte: u8) -> bool {
    ctype::is_graphic(byte)
}

/// C `islower`.
///
/// ```
/// assert!(martensite_tkernel::standard::character::is_lower_case(b'q'));
/// ```
pub fn is_lower_case(byte: u8) -> bool {
    ctype::is_lower(byte)
}

/// C `isprint` (space..=`~`).
///
/// ```
/// assert!(martensite_tkernel::standard::character::is_printable(b' '));
/// ```
pub fn is_printable(byte: u8) -> bool {
    ctype::is_print(byte)
}

/// `IsGraphic && !IsAlphanumeric`.
///
/// ```
/// assert!(martensite_tkernel::standard::character::is_punctuation(b'#'));
/// ```
pub fn is_punctuation(byte: u8) -> bool {
    is_graphic(byte) && !is_alphanumeric(byte)
}

/// C `isspace`, which unlike `u8::is_ascii_whitespace` includes `\v` (11).
///
/// ```
/// assert!(martensite_tkernel::standard::character::is_space(11));
/// ```
pub fn is_space(byte: u8) -> bool {
    ctype::is_space(byte)
}

/// C `isupper`.
///
/// ```
/// assert!(martensite_tkernel::standard::character::is_upper_case(b'Q'));
/// ```
pub fn is_upper_case(byte: u8) -> bool {
    ctype::is_upper(byte)
}

/// C `tolower`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::character::lower_case(b'A'), b'a');
/// ```
pub fn lower_case(byte: u8) -> u8 {
    ctype::to_lower(byte)
}

/// C `toupper`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::character::upper_case(b'a'), b'A');
/// ```
pub fn upper_case(byte: u8) -> u8 {
    ctype::to_upper(byte)
}

/// Value equality.
///
/// ```
/// assert!(martensite_tkernel::standard::character::is_equal(b'A', b'A'));
/// ```
pub const fn is_equal(first: u8, second: u8) -> bool {
    first == second
}
