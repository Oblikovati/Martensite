//! Nonperiodic nets are drawn before the shared generic surface engine.
use super::super::{
    fixtures,
    surface_api::{self, Surface},
};
use super::pixels;
use martensite_geometry_raster::PolesStyle;
use martensite_raster_core::{PaletteColor as Color, PaletteKind as Kind};
use std::rc::Rc;
#[test]
fn bezier_surface_modes_and_pole_flags_preserve_independent_control_pixels() {
    for index in [0, 1] {
        for mode in [0, 1] {
            capture_pair(index, mode);
        }
    }
    let surface = Rc::new(Surface::new(fixtures::geometry(1)));
    let bytes = pixels::capture(surface, "bezier-surface-default");
    assert_eq!(pixels::color(&bytes, 200, 120), [255, 0, 0]);
}
fn capture_pair(index: usize, mode: i32) {
    for shown in [false, true] {
        let poles = PolesStyle::new(shown, Color::new(Kind::Rouge));
        let surface = Rc::new(Surface::with_style(
            fixtures::geometry(index),
            fixtures::look(),
            poles,
            (30, 0.01, mode),
        ));
        let bytes = pixels::capture(
            surface,
            &format!("bezier-surface-{index}-mode{mode}-poles{shown}"),
        );
        if index == 1 {
            assert_center_pole(&bytes, shown);
        }
    }
}
fn assert_center_pole(bytes: &[u8], shown: bool) {
    // The quadratic surface stays at y<=5 while its middle control pole has y=8.
    let pixel = pixels::color(bytes, 200, 120);
    if shown {
        assert_eq!(pixel, [255, 0, 0]);
    } else {
        assert_eq!(pixel, [0, 0, 0]);
    }
}
#[test]
fn bezier_surface_style_and_geometry_changes_remain_live() {
    let surface = Rc::new(Surface::new(fixtures::geometry(1)));
    pixels::capture(surface.clone(), "bezier-surface-mutate-before");
    surface.poles().set_color(Color::new(Kind::Vert));
    surface.surface_part().show_isos(2, 3);
    surface_api::controls(&surface).set_discretisation(17);
    surface_api::controls(&surface).set_draw_mode(1);
    pixels::capture(surface.clone(), "bezier-surface-mutate-controls");
    mutate(&surface);
    let bytes = pixels::capture(surface.clone(), "bezier-surface-mutate-geometry");
    assert_eq!(pixels::color(&bytes, 220, 120), [0, 255, 0]);
    surface.poles().clear();
    pixels::capture(surface.clone(), "bezier-surface-mutate-hide");
    surface.poles().show();
    pixels::capture(surface.clone(), "bezier-surface-mutate-show");
    surface.surface_part().clear_isos();
    pixels::capture(surface, "bezier-surface-mutate-clear-isos");
}
fn mutate(surface: &Surface) {
    let handle = surface
        .surface_part()
        .surface()
        .cast::<martensite_tkg3d::geom::GeomBezierSurface>();
    handle
        .get()
        .unwrap()
        .set_pole(2, 2, martensite_tkmath::gp::Point3::new(2., 8., 4.))
        .unwrap();
}
#[test]
fn bezier_surface_failed_display_keeps_membership_with_poles_visible_or_hidden() {
    for shown in [false, true] {
        failed_display(shown);
    }
}
fn failed_display(shown: bool) {
    let surface = Rc::new(Surface::with_style(
        Default::default(),
        fixtures::look(),
        PolesStyle::new(shown, Color::new(Kind::Rouge)),
        (30, 0.01, 0),
    ));
    let handle = surface_api::handle(surface);
    let mut viewer = surface_api::viewer(10.);
    let error = viewer.display(&handle).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_NullObject");
    assert_eq!(error.message(), "null geometry in a drawable");
    assert!(Rc::ptr_eq(&viewer.drawables()[0], &handle));
    viewer.delete_view(1);
}
