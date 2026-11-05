//! Source checkprops surface-area acceptance shared by Boolean case replays.
#[path = "boolean_property_assertion.rs"]
mod boolean_property_assertion;
use martensite_tkbrep::topods::Shape;
/// Preserve source epsilon, nonzero and relative checks; `check_area(&shape, 21.154)`.
pub fn check_area(shape: &Shape, expected: f64) {
    // checkprops parses the default six-significant-digit sprops stream.
    boolean_property_assertion::check_property(shape, expected, false);
}
