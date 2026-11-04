//! Both dimensions/modes, all specialized flags, live controls and geometry.
use super::super::{
    fixtures,
    spline_api::{self, DrawContract, Planar, Spatial},
};
use super::pixels;
use std::rc::Rc;
#[test]
fn spline_normal_matrix_keeps_modes_and_specialized_knots() {
    for (index, label) in fixtures::LABELS.iter().enumerate() {
        for mode in [0, 1] {
            matrix_pair(index, label, mode);
        }
    }
}
fn matrix_pair(index: usize, label: &str, mode: i32) {
    let space = Rc::new(Spatial::with_style(
        fixtures::space(index),
        &fixtures::look(),
        (30, 0.01, mode),
    ));
    let plane = Rc::new(Planar::with_style(
        fixtures::plane(index),
        &fixtures::look(),
        30,
    ));
    spline_api::set_plane_mode(&plane, mode);
    let a = pixels::spatial(space, &format!("spline-space-{label}-mode{mode}"));
    let b = pixels::planar(plane, &format!("spline-plane-{label}-mode{mode}"));
    assert_matrix_pixels(&a, &b, index);
}
fn assert_matrix_pixels(a: &[u8], b: &[u8], index: usize) {
    if index == 0 {
        assert!(matches!(
            pixels::color(a, 210, 180),
            [255, 255, 0] | [255, 0, 0]
        ));
        assert!(matches!(
            pixels::color(b, 210, 180),
            [255, 255, 0] | [255, 0, 0]
        ));
    }
    if index == 3 {
        assert_eq!(pixels::color(a, 190, 240), [255, 0, 0]);
        assert_eq!(pixels::color(b, 190, 240), [255, 0, 0]);
    }
}
#[test]
fn spline_explicit_flags_keep_open_periodic_polygon_and_all_knots() {
    for index in [2, 3] {
        for (poles, knots) in [(false, false), (false, true), (true, false), (true, true)] {
            capture_explicit(index, poles, knots);
        }
    }
}
fn capture_explicit(index: usize, poles: bool, knots: bool) {
    let curve = Spatial::with_style(fixtures::space(index), &fixtures::look(), (30, 0.01, 0));
    let label = format!(
        "spline-with-{}-poles{poles}-knots{knots}",
        fixtures::LABELS[index]
    );
    let bytes = pixels::manual(&label, |display| curve.draw_on_with(display, poles, knots));
    if index == 3 {
        assert_ne!(pixels::color(&bytes, 190, 240), [255, 0, 0]);
    }
}
#[test]
fn spline_retained_controls_and_geometry_mutations_remain_live() {
    let space = Rc::new(Spatial::with_style(
        fixtures::space(1),
        &fixtures::look(),
        (30, 0.01, 0),
    ));
    let plane = Rc::new(Planar::with_style(
        fixtures::plane(1),
        &fixtures::look(),
        30,
    ));
    capture_pair(&space, &plane, "before");
    configure(space.curve_part().style(), space.poles(), space.knots());
    configure(plane.curve_part().style(), plane.poles(), plane.knots());
    capture_pair(&space, &plane, "controls");
    mutate(&space, &plane);
    capture_pair(&space, &plane, "geometry");
}
fn capture_pair(space: &Rc<Spatial>, plane: &Rc<Planar>, stage: &str) {
    pixels::spatial(space.clone(), &format!("spline-space-mutate-{stage}"));
    pixels::planar(plane.clone(), &format!("spline-plane-mutate-{stage}"));
}
fn configure(
    style: &martensite_geometry_raster::CurveStyle,
    poles: &martensite_geometry_raster::PolesStyle,
    knots: &martensite_geometry_raster::KnotsStyle,
) {
    style.set_display_origin(false);
    style.show_curvature();
    style.set_radius_ratio(0.2);
    style.set_radius_max(1.);
    poles.clear();
    knots.set_shape(martensite_raster_core::MarkerShape::Plus);
}
fn mutate(space: &Spatial, plane: &Planar) {
    use martensite_tkg2d::geom2d::Geom2dBSplineCurve;
    use martensite_tkg3d::geom::GeomBSplineCurve;
    use martensite_tkmath::gp::{Point2, Point3};
    let handle = space.curve_part().curve().cast::<GeomBSplineCurve>();
    handle
        .get()
        .unwrap()
        .set_pole(2, Point3::new(1., 4., 0.))
        .unwrap();
    let handle = plane.curve_part().curve().cast::<Geom2dBSplineCurve>();
    handle
        .get()
        .unwrap()
        .set_pole(2, Point2::new(1., 4.))
        .unwrap();
}
#[test]
fn spline_hidden_poles_and_knots_still_validate_null_geometry_first() {
    let curve = Spatial::with_style(Default::default(), &fixtures::look(), (30, 0.01, 0));
    let mut viewer = spline_api::viewer(true);
    let mut display = viewer.make_display(1);
    assert_null(curve.draw_on_with(&mut display, false, false).unwrap_err());
    assert_null(
        curve
            .draw_on_range(&mut display, (0., 1.), 0, (false, false))
            .unwrap_err(),
    );
    let plane = Planar::with_style(Default::default(), &fixtures::look(), 30);
    plane.poles().clear();
    plane.knots().clear();
    assert_null(plane.draw_on(&mut display).unwrap_err());
}
fn assert_null(error: martensite_tkernel::standard::Failure) {
    assert_eq!(error.exception_type(), "Standard_NullObject");
    assert_eq!(error.message(), "null geometry in a drawable");
}
