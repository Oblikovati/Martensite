//! Embedded English healing messages.
//!
//! The message catalog preserves its resource bytes, without a terminating NUL.
//! `ShapeExtend::init` loads these messages into the catalog.
/// `SHMessage_SHAPE_us`: the English message texts, the bytes of the literal without its
/// terminating NUL (`sizeof(SHMessage_SHAPE_us) - 1` in this implementation).
///
/// ```
/// use martensite_tkshhealing::sh_message::SHAPE_US;
/// let text = std::str::from_utf8(SHAPE_US).unwrap();
/// assert!(text.contains(".ShapeFix.FixSmallSolid.MSG0\n"));
/// ```
pub const SHAPE_US: &[u8] = include_bytes!("SHAPE.us");

/// `SHMessage_SHAPE_fr`: the French message texts. The file ships with the package but is
/// not compiled in this implementation; no Shape Healing member reads it.
///
/// ```
/// use martensite_tkshhealing::sh_message::SHAPE_FR;
/// assert!(SHAPE_FR.starts_with(b"! Message file for Shape Healing"));
/// ```
pub const SHAPE_FR: &[u8] = include_bytes!("SHAPE.fr");
