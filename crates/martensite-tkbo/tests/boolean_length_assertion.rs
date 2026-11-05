//! Source checkprops length acceptance with the original command text precision.
#[path = "boolean_property_assertion.rs"]
mod boolean_property_assertion;
use martensite_tkbrep::topods::Shape;

/// Preserve nonzero and relative checks on printed length; `check_length(&shape, 53.1714)`.
pub(super) fn check_length(shape: &Shape, expected: f64) {
    // Preserve this entry point for standalone hosts selecting only the original length check.
    boolean_property_assertion::check_property(shape, expected, true);
}
