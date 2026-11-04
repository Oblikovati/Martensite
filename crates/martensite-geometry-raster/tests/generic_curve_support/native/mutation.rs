//! The next repaint observes changed controls, live geometry and planar replacement.
use super::super::{
    curve_api::{self, CurveDrawableContract, PlanarCurve, SpatialCurve},
    fixtures,
};
use super::pixels;
use martensite_geometry_raster::CurveStyle;
use martensite_raster_core::{PaletteColor, PaletteKind};
use martensite_tkg2d::geom2d::Geom2dCircle;
use martensite_tkg3d::geom::GeomCircle;
use std::rc::Rc;
#[test]
fn mutable_controls_redraw_the_same_retained_curve() {
    spatial_controls();
    planar_controls();
}
fn changed_style(style: &CurveStyle) {
    style.set_color(PaletteColor::new(PaletteKind::Bleu));
    style.set_display_origin(false);
    style.show_curvature();
    style.set_radius_max(2.);
    style.set_radius_ratio(0.5);
}
fn spatial_controls() {
    let curve = Rc::new(SpatialCurve::with_look(
        fixtures::spatial(5),
        fixtures::red_look(),
        (30, 0.01, 0),
    ));
    let mut viewer = curve_api::viewer(true);
    viewer
        .display(&curve_api::spatial_handle(curve.clone()))
        .unwrap();
    let before = pixels::capture(&mut viewer, "generic-spatial-settings-before");
    curve_api::spatial_controls(&curve).set_discretisation(7);
    curve_api::spatial_controls(&curve).set_deflection(0.2);
    curve_api::spatial_controls(&curve).set_draw_mode(1);
    changed_style(curve.style());
    let after = pixels::capture(&mut viewer, "generic-spatial-settings-after");
    assert_control_pixels(&before, &after);
    assert!(curve.visible());
    viewer.delete_view(1);
}
fn planar_controls() {
    let curve = Rc::new(PlanarCurve::with_look(
        fixtures::planar(5),
        fixtures::red_look(),
        30,
    ));
    let mut viewer = curve_api::viewer(false);
    viewer
        .display(&curve_api::planar_handle(curve.clone()))
        .unwrap();
    let before = pixels::capture(&mut viewer, "generic-planar-settings-before");
    curve_api::planar_controls(&curve).set_discretisation(7);
    curve_api::planar_controls(&curve).set_deflection(0.2);
    curve_api::planar_controls(&curve).set_draw_mode(1);
    changed_style(curve.style());
    let after = pixels::capture(&mut viewer, "generic-planar-settings-after");
    assert_control_pixels(&before, &after);
    assert!(curve.visible());
    viewer.delete_view(1);
}
fn assert_blue(bytes: &[u8]) {
    assert!(bytes[15..].as_chunks::<3>().0.contains(&[0, 0, 255]));
}
#[test]
fn shared_geometry_mutations_are_visible_on_the_next_repaint() {
    spatial_geometry();
    planar_geometry();
}
fn spatial_geometry() {
    let geometry = fixtures::spatial(2);
    let curve = Rc::new(SpatialCurve::with_look(
        geometry.clone(),
        fixtures::red_look(),
        (30, 0.01, 0),
    ));
    let mut viewer = curve_api::viewer(true);
    viewer.display(&curve_api::spatial_handle(curve)).unwrap();
    let before = pixels::capture(&mut viewer, "generic-spatial-geometry-before");
    geometry
        .cast::<GeomCircle>()
        .get()
        .unwrap()
        .set_radius(7.)
        .unwrap();
    let after = pixels::capture(&mut viewer, "generic-spatial-geometry-after");
    assert_geometry_pixels(&before, &after);
    viewer.delete_view(1);
}
fn planar_geometry() {
    let geometry = fixtures::planar(2);
    let curve = Rc::new(PlanarCurve::with_look(
        geometry.clone(),
        fixtures::red_look(),
        30,
    ));
    let mut viewer = curve_api::viewer(false);
    viewer.display(&curve_api::planar_handle(curve)).unwrap();
    let before = pixels::capture(&mut viewer, "generic-planar-geometry-before");
    geometry
        .cast::<Geom2dCircle>()
        .get()
        .unwrap()
        .set_radius(7.)
        .unwrap();
    let after = pixels::capture(&mut viewer, "generic-planar-geometry-after");
    assert_geometry_pixels(&before, &after);
    viewer.delete_view(1);
}
#[test]
fn planar_geometry_replacement_keeps_controls_and_redraws_the_new_curve() {
    let curve = Rc::new(PlanarCurve::with_look(
        fixtures::planar(2),
        fixtures::red_look(),
        30,
    ));
    let mut viewer = curve_api::viewer(false);
    viewer
        .display(&curve_api::planar_handle(curve.clone()))
        .unwrap();
    let before = pixels::capture(&mut viewer, "generic-planar-replacement-before");
    let controls = curve_api::planar_controls(&curve) as *const _;
    curve.set_curve(fixtures::planar(0));
    let after = pixels::capture(&mut viewer, "generic-planar-replacement-after");
    assert_ne!(before, after);
    pixels::finite_line(&after);
    assert_eq!(curve_api::planar_controls(&curve) as *const _, controls);
    viewer.delete_view(1);
}

fn assert_control_pixels(before: &[u8], after: &[u8]) {
    assert_ne!(before, after);
    assert_blue(after);
}
fn assert_geometry_pixels(before: &[u8], after: &[u8]) {
    assert_ne!(before, after);
    assert!(pixels::red(before, 250, 200));
    assert!(pixels::red(after, 270, 200));
}
