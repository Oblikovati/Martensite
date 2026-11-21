//! The color legacy suite, including its sample tables.
// Keep all reference numeric literals intact.
#![allow(non_snake_case, clippy::excessive_precision)]
use super::{Color, ColorSpace, NamedColor};
fn rgb(r: f64, g: f64, b: f64) -> Color {
    Color::new(r, g, b, ColorSpace::Rgb).unwrap()
}
fn near(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() < tolerance,
        "{actual} vs {expected}, tolerance {tolerance}"
    );
}
fn components(color: Color, expected: [f64; 3], tolerance: f64) {
    for (a, b) in [color.red(), color.green(), color.blue()]
        .into_iter()
        .zip(expected)
    {
        near(a, b, tolerance);
    }
}
include!("../../tests/quantity_color/reference_samples.rs");
include!("../../tests/quantity_color/basic.rs");
include!("../../tests/quantity_color/conversions.rs");
