//! Independent controls and exact null-geometry errors apply to both implementations.
use super::{
    curve_api::{self, CurveDrawableContract, PlanarCurve, SpatialCurve},
    fixtures,
};
use martensite_raster_core::RasterDisplay;
use martensite_tkernel::standard::{Failure, Handle};
#[test]
fn null_geometry_keeps_the_exact_failure_before_sampling() {
    for mode in [0, 1] {
        let spatial =
            SpatialCurve::with_look(Handle::default(), fixtures::red_look(), (30, 0.01, mode));
        let planar = PlanarCurve::with_look(Handle::default(), fixtures::red_look(), 30);
        curve_api::planar_controls(&planar).set_draw_mode(mode);
        assert_null(
            CurveDrawableContract::draw_on(&spatial, &mut RasterDisplay::batch()).unwrap_err(),
        );
        assert_null(
            CurveDrawableContract::draw_on(&planar, &mut RasterDisplay::batch()).unwrap_err(),
        );
    }
}
fn assert_null(error: Failure) {
    assert_eq!(error.exception_type(), "Standard_NullObject");
    assert_eq!(error.message(), "null geometry in a drawable");
}
#[test]
fn controls_and_geometry_accessors_retain_instance_identity() {
    let geometry = fixtures::spatial(2);
    assert_circle_fixture(&geometry);
    let first = SpatialCurve::with_look(geometry.clone(), fixtures::red_look(), (30, 0.01, 0));
    let second = SpatialCurve::with_look(geometry.clone(), fixtures::red_look(), (16, 0.125, 1));
    curve_api::spatial_controls(&first).set_discretisation(7);
    first.style().set_display_origin(false);
    assert_eq!(curve_api::spatial_controls(&first).discretisation(), 7);
    assert_eq!(curve_api::spatial_controls(&second).discretisation(), 16);
    assert!(!first.style().display_origin() && second.style().display_origin());
    assert!(std::ptr::eq(
        first.curve().get().unwrap(),
        geometry.get().unwrap()
    ));
    assert!(CurveDrawableContract::is_3d(&first));
    planar_identity();
}
fn planar_identity() {
    let curve = PlanarCurve::with_look(fixtures::planar(2), fixtures::red_look(), 30);
    let replacement = fixtures::planar(0);
    let controls = curve_api::planar_controls(&curve) as *const _;
    let style = curve.style() as *const _;
    curve.set_curve(replacement.clone());
    assert!(std::ptr::eq(
        curve.curve().get().unwrap(),
        replacement.get().unwrap()
    ));
    assert_eq!(curve_api::planar_controls(&curve) as *const _, controls);
    assert_eq!(curve.style() as *const _, style);
    assert!(!CurveDrawableContract::is_3d(&curve));
}

fn assert_circle_fixture(geometry: &Handle<dyn martensite_tkg3d::geom::GeomCurve>) {
    assert_eq!(fixtures::RECORDS[2].label, "circle");
    assert_eq!(
        geometry.get().unwrap().d0(0.).unwrap(),
        martensite_tkmath::gp::Point3::new(5., 0., 0.)
    );
}
