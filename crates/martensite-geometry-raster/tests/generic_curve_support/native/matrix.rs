//! Finite/infinite lines, conics and piecewise curves in both dimensions and modes.
use super::super::{
    curve_api::{self, PlanarCurve, SpatialCurve},
    fixtures,
};
use super::pixels;
use std::rc::Rc;
#[test]
fn generic_curve_matrix_keeps_modes_and_endpoint_pixels() {
    for (index, record) in fixtures::RECORDS.iter().enumerate() {
        for mode in [0, 1] {
            capture_pair(index, record.label, mode);
        }
    }
}
fn capture_pair(index: usize, label: &str, mode: i32) {
    let spatial = Rc::new(SpatialCurve::with_look(
        fixtures::spatial(index),
        fixtures::red_look(),
        (30, 0.01, mode),
    ));
    let planar = Rc::new(PlanarCurve::with_look(
        fixtures::planar(index),
        fixtures::red_look(),
        30,
    ));
    curve_api::planar_controls(&planar).set_draw_mode(mode);
    let space = pixels::spatial(spatial, &format!("generic-spatial-{label}-mode{mode}"));
    let plane = pixels::planar(planar, &format!("generic-planar-{label}-mode{mode}"));
    if index == 0 {
        pixels::finite_line(&space);
        pixels::finite_line(&plane);
    }
}
#[test]
fn circle_curvature_caps_preserve_independent_radius_pixels() {
    for capped in [false, true] {
        curvature_pair(capped);
    }
}
fn curvature_pair(capped: bool) {
    let spatial = Rc::new(SpatialCurve::with_look(
        fixtures::spatial(2),
        fixtures::red_look(),
        (4, 0.01, 0),
    ));
    let planar = Rc::new(PlanarCurve::with_look(
        fixtures::planar(2),
        fixtures::red_look(),
        4,
    ));
    radii_pair(spatial.style(), planar.style(), capped);
    let space = pixels::spatial(
        spatial,
        &format!("generic-spatial-curvature-capped-{capped}"),
    );
    let plane = pixels::planar(planar, &format!("generic-planar-curvature-capped-{capped}"));
    assert_radii(&space, capped);
    assert_radii(&plane, capped);
}
fn radii_style(style: &martensite_geometry_raster::CurveStyle, capped: bool) {
    style.set_display_origin(false);
    style.show_curvature();
    style.set_radius_max(if capped { 1. } else { 1000. });
    style.set_radius_ratio(if capped { 0.5 } else { 0.2 });
}
fn assert_radii(bytes: &[u8], capped: bool) {
    let first = if capped { 245 } else { 240 };
    for x in first..=250 {
        assert!(pixels::red(bytes, x, 200));
    }
    assert!(!pixels::red(bytes, first - 1, 200));
}

fn radii_pair(
    spatial: &martensite_geometry_raster::CurveStyle,
    planar: &martensite_geometry_raster::CurveStyle,
    capped: bool,
) {
    radii_style(spatial, capped);
    radii_style(planar, capped);
}
