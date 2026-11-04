//! Degree-one chords and exact inherited span traversal are pinned independently.
use super::super::{fixtures, spline_api::Spatial};
use super::pixels;
#[test]
fn ranged_splines_preserve_chord_and_span_endpoint_contracts() {
    let curve = Spatial::with_style(fixtures::space(0), &fixtures::look(), (8, 0.01, 0));
    let bytes = pixels::manual("spline-range-linear-chord", |d| {
        curve.draw_on_range(d, (0., 2.), 0, (false, false))
    });
    for x in 200..=240 {
        assert_eq!(pixels::color(&bytes, x, 200), [255, 255, 0]);
    }
    assert_eq!(pixels::color(&bytes, 220, 160), [0, 0, 0]);
    for (ordinal, range) in [(0.5, 1.5), (1.5, 0.5), (1., 1.)].into_iter().enumerate() {
        pixels::manual(&format!("spline-range-linear-{ordinal}"), |d| {
            curve.draw_on_range(d, range, 0, (false, true))
        });
    }
    range_matrix();
}
fn range_matrix() {
    for (index, range, label) in [
        (1, (0.2, 0.8), "one-span"),
        (1, (0.2, 1.8), "multi-span"),
        (1, (1.8, 0.2), "reversed"),
        (1, (1., 1.), "zero"),
        (2, (0.25, 0.75), "unclamped-one-span"),
        (2, (0.25, 2.75), "unclamped-cross"),
        (3, (0.2, 3.8), "periodic"),
    ] {
        let curve = Spatial::with_style(fixtures::space(index), &fixtures::look(), (8, 0.01, 0));
        pixels::manual(&format!("spline-range-{label}"), |d| {
            curve.draw_on_range(d, range, 0, (true, true))
        });
    }
}
#[test]
fn ranged_pole_neighborhoods_preserve_first_last_interior_and_bad_index() {
    let curve = Spatial::with_style(fixtures::space(1), &fixtures::look(), (8, 0.01, 0));
    for pole in [0, 1, 3, 5] {
        pixels::manual(&format!("spline-range-pole-{pole}"), |d| {
            curve.draw_on_range(d, (0.2, 1.8), pole, (true, false))
        });
    }
    let mut viewer = super::super::super::spline_api::viewer(true);
    let mut display = viewer.make_display(1);
    let error = curve
        .draw_on_range(&mut display, (0., 1.), -1, (true, true))
        .unwrap_err();
    assert_eq!(error.exception_type(), "Standard_OutOfRange");
}
