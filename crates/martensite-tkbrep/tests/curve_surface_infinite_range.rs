//! Infinite range endpoints skip evaluation and retain explicitly stored UVs.
use super::curve_on_surface_order::failing_curve::FailingParameterCurve;
use martensite_tkbrep::brep::{
    BRepCurveOnClosedSurface, BRepCurveOnSurface, BRepCurveRepresentation, BRepGCurve,
};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkmath::{
    geom_abs::Shape,
    gp::{Axis2d, Point2},
    toploc::Location,
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

fn line() -> Handle<dyn Geom2dCurve> {
    Handle::new(Geom2dLine::new(Axis2d::default())).cast()
}

fn seam(first: &Handle<dyn Geom2dCurve>) -> BRepCurveOnClosedSurface {
    BRepCurveOnClosedSurface::new(
        first,
        &line(),
        &Handle::default(),
        &Location::default(),
        Shape::C0,
    )
    .unwrap()
}

#[test]
fn absent_pcurve_with_both_infinite_endpoints_preserves_surface_uvs() {
    let value = BRepCurveOnSurface::new(&line(), &Handle::default(), &Location::default()).unwrap();
    let uv = (Point2::new(2., 3.), Point2::new(5., 7.));
    value.set_uv_points(uv.0, uv.1);
    value.set_pcurve(&Handle::default()).unwrap();
    value
        .set_range(-precision::infinite(), precision::infinite())
        .unwrap();
    assert_eq!(value.uv_points(), uv);
    assert_eq!(
        value.range(),
        (-precision::infinite(), precision::infinite())
    );
}

#[test]
fn absent_seam_pcurves_with_infinite_endpoints_preserve_both_uv_pairs() {
    let value = seam(&line());
    let first = (Point2::new(2., 3.), Point2::new(5., 7.));
    let second = (Point2::new(11., 13.), Point2::new(17., 19.));
    value.set_uv_points(first.0, first.1);
    value.set_uv_points2(second.0, second.1);
    value.set_pcurve(&Handle::default()).unwrap();
    value.set_pcurve2(&Handle::default()).unwrap();
    value
        .set_range(-precision::infinite(), precision::infinite())
        .unwrap();
    assert_eq!(value.uv_points(), first);
    assert_eq!(value.uv_points2(), second);
}

#[test]
fn finite_endpoint_still_rejects_absent_pcurve_after_storing_new_range() {
    let value = BRepCurveOnSurface::new(&line(), &Handle::default(), &Location::default()).unwrap();
    let uv = (Point2::new(2., 3.), Point2::new(5., 7.));
    value.set_uv_points(uv.0, uv.1);
    value.set_pcurve(&Handle::default()).unwrap();
    let error = value.set_range(-precision::infinite(), 4.).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_NullObject");
    assert_eq!(value.range(), (-precision::infinite(), 4.));
    assert_eq!(value.uv_points(), uv);
}

#[test]
fn seam_updates_second_uv_pair_before_propagating_first_curve_failure() {
    let evaluations = Arc::new(AtomicUsize::new(0));
    let first_curve: Handle<dyn Geom2dCurve> =
        Handle::new(FailingParameterCurve::new(evaluations.clone())).cast();
    let value = seam(&first_curve);
    let first = (Point2::new(2., 3.), Point2::new(5., 7.));
    value.set_uv_points(first.0, first.1);
    let error = value.set_range(4., 8.).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_RangeError");
    assert_eq!(evaluations.load(Ordering::SeqCst), 1);
    assert_eq!(value.range(), (4., 8.));
    assert_eq!(value.uv_points(), first);
    assert_eq!(
        value.uv_points2(),
        (Point2::new(4., 0.), Point2::new(8., 0.))
    );
}
