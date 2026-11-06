//! Generic rendering includes bounded, infinite and curved geometry in both modes.
use super::super::{
    geometry, state,
    surface_api::{self, Surface},
};
use super::pixels;
use std::rc::Rc;
#[test]
fn surface_matrix_preserves_bounds_isos_and_orientation_mark() {
    for index in 0..6 {
        for mode in [0, 1] {
            capture_case(index, mode);
        }
    }
}
fn capture_case(index: usize, mode: i32) {
    let surface = configured_surface(index, mode);
    let zoom = if [0, 2, 3].contains(&index) {
        0.25
    } else {
        10.
    };
    let label = format!("surface-{index}-mode{mode}");
    let on = pixels::normal(surface.clone(), zoom, &format!("{label}-isos-on"));
    let off = pixels::manual(&surface, zoom, false, &format!("{label}-isos-off"));
    if index == 1 {
        assert_plane_pixels(&on, &off);
    }
}
fn configured_surface(index: usize, mode: i32) -> Rc<Surface> {
    Rc::new(Surface::with_look(
        geometry::fixture(index),
        state::look(),
        (30, 0.01, mode),
    ))
}
fn assert_plane_pixels(on: &[u8], off: &[u8]) {
    assert_eq!(pixels::color(on, 200, 200), [0, 0, 255]);
    assert_eq!(pixels::color(off, 200, 200), [0, 0, 0]);
    assert_eq!(pixels::color(on, 150, 200), [255, 255, 0]);
    assert_eq!(pixels::color(off, 150, 200), [255, 255, 0]);
    assert_eq!(pixels::color(on, 160, 245), [255, 255, 0]);
    assert_eq!(pixels::color(off, 160, 245), [255, 255, 0]);
}
#[test]
fn surface_controls_and_geometry_mutations_preserve_actual_cells() {
    let surface = Rc::new(Surface::with_look(
        geometry::fixture(4),
        state::look(),
        (30, 0.01, 0),
    ));
    pixels::normal(surface.clone(), 10., "surface-mutation-before");
    configure_controls(&surface);
    pixels::normal(surface.clone(), 10., "surface-mutation-controls");
    mutate_surface(&surface);
    pixels::normal(surface.clone(), 10., "surface-mutation-geometry");
    surface.clear_isos();
    pixels::normal(surface.clone(), 10., "surface-mutation-clear");
    surface.show_isos(2, 3);
    pixels::normal(surface, 10., "surface-mutation-restore");
}
fn configure_controls(surface: &Surface) {
    surface.show_isos(-2, -3);
    surface.set_bounds_color(martensite_raster_core::PaletteColor::new(
        martensite_raster_core::PaletteKind::Vert,
    ));
    surface.set_isos_color(martensite_raster_core::PaletteColor::new(
        martensite_raster_core::PaletteKind::Rouge,
    ));
    surface_api::controls(surface).set_discretisation(17);
    surface_api::controls(surface).set_draw_mode(1);
}
fn mutate_surface(surface: &Surface) {
    let geometry = surface
        .surface()
        .cast::<martensite_tkg3d::geom::GeomBezierSurface>();
    geometry
        .get()
        .unwrap()
        .set_pole(2, 2, martensite_tkmath::gp::Point3::new(8., 5., 3.))
        .unwrap();
}
#[test]
fn null_surface_is_a_typed_failure_even_without_isos() {
    let surface = Surface::new(Default::default());
    let mut viewer = surface_api::viewer(10.);
    let mut display = viewer.make_display(1);
    for isos in [false, true] {
        let error = surface.draw_on_with_isos(&mut display, isos).unwrap_err();
        assert_eq!(error.exception_type(), "Standard_NullObject");
        assert_eq!(error.message(), "null geometry in a drawable");
    }
}
