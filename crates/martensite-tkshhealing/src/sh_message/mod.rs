//! Embedded English healing messages.
//!
//! The message catalog preserves its resource bytes, without a terminating NUL.
//! `ShapeExtend::init` loads these messages into the catalog.
/// English message resource bytes, embedded without a terminating NUL.
///
/// ```
/// use martensite_tkshhealing::sh_message::SHAPE_US;
/// let text = std::str::from_utf8(SHAPE_US).unwrap();
/// assert!(text.contains(".ShapeFix.FixSmallSolid.MSG0\n"));
/// ```
pub const SHAPE_US: &[u8] = include_bytes!("SHAPE.us");

/// French message resource bytes, embedded for callers that select this catalog.
/// Automatic catalog initialization loads the English resource instead.
///
/// ```
/// use martensite_tkshhealing::sh_message::SHAPE_FR;
/// assert!(SHAPE_FR.starts_with(b"! Message file for Shape Healing"));
/// ```
pub const SHAPE_FR: &[u8] = include_bytes!("SHAPE.fr");
