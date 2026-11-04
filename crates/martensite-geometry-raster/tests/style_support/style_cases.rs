//! Mutable styles preserve their exact values and copies own independent Cells.
use super::style_api::{CurveLook, CurveStyle, KnotsStyle, PolesStyle};
use martensite_raster_core::{MarkerShape, PaletteColor, PaletteKind};
fn red() -> PaletteColor {
    PaletteColor::new(PaletteKind::Rouge)
}
fn blue() -> PaletteColor {
    PaletteColor::new(PaletteKind::Bleu)
}
#[test]
fn curve_defaults_and_scalar_mutations_are_retained_per_instance() {
    let look = CurveLook::new(red());
    assert!(look.display_origin && !look.display_curvature);
    assert_eq!((look.radius_max, look.radius_ratio), (1000., 0.1));
    let style = CurveStyle::new(look);
    let other = CurveStyle::new(look);
    style.set_color(blue());
    style.set_display_origin(false);
    style.show_curvature();
    assert_curve_instances(&style, &other);
    style.clear_curvature();
    assert!(!style.curvature_shown());
    assert_scalar_styles(&style);
}
fn assert_scalar_styles(style: &CurveStyle) {
    for value in [0., -1., f64::INFINITY, f64::NAN] {
        style.set_radius_max(value);
        style.set_radius_ratio(value);
        assert_eq!(style.radius_max().to_bits(), value.to_bits());
        assert_eq!(style.radius_ratio().to_bits(), value.to_bits());
    }
}
#[test]
fn pole_style_copies_do_not_share_mutations() {
    let poles = PolesStyle::default_shown();
    let copied = poles.clone();
    assert!(poles.shown());
    assert_eq!(poles.color(), red());
    poles.clear();
    poles.set_color(blue());
    assert!(!poles.shown());
    assert_eq!(poles.color(), blue());
    assert!(copied.shown());
    assert_eq!(copied.color(), red());
    poles.show();
    assert!(poles.shown());
}
#[test]
fn knot_style_copies_keep_marker_size_and_independent_flags() {
    let knots = KnotsStyle::default_shown();
    let copied = knots.clone();
    assert_eq!((knots.shape(), knots.size()), (MarkerShape::Losange, 5));
    assert_eq!(knots.color(), PaletteColor::new(PaletteKind::Violet));
    knots.clear();
    knots.set_color(blue());
    knots.set_shape(MarkerShape::Square);
    assert_eq!(
        (knots.shown(), knots.shape(), knots.color()),
        (false, MarkerShape::Square, blue())
    );
    assert_eq!(
        (copied.shown(), copied.shape()),
        (true, MarkerShape::Losange)
    );
    knots.show();
    assert!(knots.shown());
}

fn assert_curve_instances(style: &CurveStyle, other: &CurveStyle) {
    assert_eq!(
        (
            style.color(),
            style.display_origin(),
            style.curvature_shown()
        ),
        (blue(), false, true)
    );
    assert_eq!(
        (
            other.color(),
            other.display_origin(),
            other.curvature_shown()
        ),
        (red(), true, false)
    );
}
