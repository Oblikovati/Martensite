//! Conversions between `char` and the UTF-16 unit `char16_t`.
//!
//! These are byte-level conversions, not Unicode transcoding: a `char` is zero-extended
//! through `unsigned char`, and a UTF-16 unit keeps only its low byte.

/// Zero-extends the byte, `(char16_t)((unsigned char)c & 0xff)`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::ext_character::to_ext_character(0xE9), 0x00E9);
/// ```
pub const fn to_ext_character(byte: u8) -> u16 {
    byte as u16
}

/// Keeps the low byte, `(char)(unsigned char)(c & 0xff)`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::ext_character::to_character(0x20AC), 0xAC);
/// ```
pub const fn to_character(unit: u16) -> u8 {
    (unit & 0x00ff) as u8
}

/// True when the high byte is zero, so 128..=255 count as "ASCII".
///
/// ```
/// assert!(martensite_tkernel::standard::ext_character::is_an_ascii(0x00FF));
/// ```
pub const fn is_an_ascii(unit: u16) -> bool {
    unit & 0xff00 == 0
}

/// Value equality.
///
/// ```
/// assert!(martensite_tkernel::standard::ext_character::is_equal(0x20AC, 0x20AC));
/// ```
pub const fn is_equal(first: u16, second: u16) -> bool {
    first == second
}
