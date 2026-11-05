//! Parse the actual six-significant-digit general stream representation of a mass.
use martensite_tkernel::storage::format_general;

/// Preserve source checkprops text rounding, including ties and tiny magnitudes.
///
/// Example: `draw_mass(123456.5)` returns `123456.0`.
pub(crate) fn draw_mass(actual: f64) -> f64 {
    let text = format_general(actual, 6);
    text.parse().unwrap_or_else(|error| {
        panic!("checkprops mass {actual:?} rendered as {text:?}: expected numeric text: {error}")
    })
}
