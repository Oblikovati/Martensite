//! CPU controls show the elliptical reference accepts faithful roundings and refuses wrong images.
use super::elliptical_reference::{
    EllipticalGradientReference, FAITHFUL_ROUNDING_LEVELS, ReferenceGrade, assert_faithful,
    srgb_encode,
};

const COLORS: [[f32; 4]; 2] = [[0.125, 0.25, 0.75, 1.], [0.75, 0.5, 0.125, 1.]];
const RECORDS_64: &str = include_str!("fixtures/elliptical/64x64.txt");

fn reference_64() -> EllipticalGradientReference {
    EllipticalGradientReference::from_native_records(RECORDS_64, COLORS)
}

/// Build a capture from per-channel levels; `capture(|levels| levels.map(f64::round))`.
fn capture(levels: impl Fn([f64; 3]) -> [f64; 3]) -> Vec<u8> {
    let reference = reference_64();
    let mut rgba = Vec::with_capacity(64 * 64 * 4);
    for index in 0..64 * 64 {
        let exact = reference.exact_levels(index % 64, index / 64, 64);
        rgba.extend(levels(exact).map(|level| level as u8));
        rgba.push(255);
    }
    rgba
}

fn grade(rgba: &[u8]) -> ReferenceGrade {
    reference_64().grade(rgba, &[1.; 64 * 64], [64, 64])
}

#[test]
fn srgb_transfer_matches_its_closed_form_endpoints_and_joins_continuously() {
    assert_eq!(srgb_encode(0.), 0.);
    assert!((srgb_encode(1.) - 1.).abs() < 1e-15);
    let knee = 0.003_130_8;
    let linear_side = 12.92 * knee;
    let power_side = 1.055 * martensite_cmath::pow(knee, 1. / 2.4) - 0.055;
    assert!((linear_side - power_side).abs() < 1e-7);
}

#[test]
fn single_pixel_fan_center_is_exactly_the_center_color() {
    let records = include_str!("fixtures/elliptical/1x1.txt");
    let reference = EllipticalGradientReference::from_native_records(records, COLORS);
    let expected = [0, 1, 2].map(|k| 255. * srgb_encode(f64::from(COLORS[0][k])));
    assert_eq!(reference.exact_levels(0, 0, 1), expected);
}

#[test]
fn nearest_and_directed_roundings_are_both_faithful() {
    for round in [f64::round, f64::floor, f64::ceil] {
        let actual = grade(&capture(|levels| levels.map(round)));
        assert!(actual.max_levels < FAITHFUL_ROUNDING_LEVELS, "{actual:?}");
        assert_faithful(actual, "synthetic-rounding");
    }
}

#[test]
fn one_level_past_a_faithful_rounding_is_refused() {
    // floor + 1 is faithful only where the exact value is not an integer; ceil + 1 never is.
    let actual = grade(&capture(|levels| levels.map(|level| level.ceil() + 1.)));
    assert!(actual.max_levels >= FAITHFUL_ROUNDING_LEVELS, "{actual:?}");
    let refused = std::panic::catch_unwind(|| assert_faithful(actual, "synthetic-ceil-plus-one"));
    assert!(refused.is_err());
}

#[test]
fn interpolating_encoded_colors_instead_of_linear_colors_is_refused() {
    // Native ellipse colors are raw linear RGB; an sRGB-space blend is a different image.
    let encoded = COLORS.map(|color| [0, 1, 2].map(|k| 255. * srgb_encode(f64::from(color[k]))));
    let rgba = capture(|exact| {
        let weight = rim_weight_from_blue(exact);
        [0, 1, 2].map(|k| ((1. - weight) * encoded[0][k] + weight * encoded[1][k]).round())
    });
    let actual = grade(&rgba);
    assert!(actual.max_levels > 4., "{actual:?}");
}

fn rim_weight_from_blue(exact: [f64; 3]) -> f64 {
    // Invert the linear blue channel: 0.75 at the center, 0.125 on the rim.
    let linear = srgb_decode(exact[2] / 255.);
    (0.75 - linear) / (0.75 - 0.125)
}

fn srgb_decode(encoded: f64) -> f64 {
    if encoded <= 0.040_45 {
        return encoded / 12.92;
    }
    martensite_cmath::pow((encoded + 0.055) / 1.055, 2.4)
}

#[test]
#[should_panic(expected = "expected inside the fan")]
fn pixel_centers_beyond_the_rim_are_not_extrapolated() {
    reference_64().exact_levels(200, 0, 64);
}
