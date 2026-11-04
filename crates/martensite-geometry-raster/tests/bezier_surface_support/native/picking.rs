//! Pixel-unit pole picking retains the inherited U/V resume behavior.
use super::super::{
    fixtures,
    surface_api::{self, Surface},
};
#[test]
fn bezier_surface_pick_search_and_null_failure_are_exact() {
    let surface = Surface::new(fixtures::geometry(1));
    let mut viewer = surface_api::viewer(10.);
    let display = viewer.make_display(1);
    assert_hits(&surface, &display);
    assert_resume(&surface, &display);
    assert_negative_and_null(&surface, &display);
}

fn assert_hits(surface: &Surface, display: &martensite_raster_core::RasterDisplay<'_>) {
    assert_eq!(
        surface.find_pole((0., 80.), display, 0.1, (0, 0)).unwrap(),
        (2, 2)
    );
    assert_eq!(
        surface
            .find_pole((-50., -50.), display, 0.1, (0, 0))
            .unwrap(),
        (1, 1)
    );
}
fn assert_resume(surface: &Surface, display: &martensite_raster_core::RasterDisplay<'_>) {
    assert_eq!(
        surface.find_pole((0., 80.), display, 0.1, (2, 2)).unwrap(),
        (0, 0)
    );
    assert_eq!(
        surface
            .find_pole((999., 999.), display, 0.1, (0, 0))
            .unwrap(),
        (0, 0)
    );
}
fn assert_negative_and_null(
    surface: &Surface,
    display: &martensite_raster_core::RasterDisplay<'_>,
) {
    assert_eq!(
        surface.find_pole((0., 80.), display, -0.1, (0, 0)).unwrap(),
        (0, 0)
    );
    let error = Surface::new(Default::default())
        .find_pole((0., 0.), display, 1., (0, 0))
        .unwrap_err();
    assert_eq!(error.exception_type(), "Standard_NullObject");
    assert_eq!(error.message(), "null geometry in a drawable");
}
