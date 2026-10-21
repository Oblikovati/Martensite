//! Tests of `IntCurveSurface_ThePolygonOfHInter`.
#![allow(non_snake_case)]
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomCurve, GeomLine};
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkgeomalgo::int_curve_surface::{HCurve, ThePolygonOfHInter};
use martensite_tkmath::gp::{Axis1, Direction3, Point3};

#[test]
fn IntCurveSurface_ThePolygonOfHInter_Test__ClosedFlagReflectsStoredState() {
    let a_line: Handle<dyn GeomCurve> = Handle::new(GeomLine::new(Axis1::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(1.0, 0.0, 0.0).unwrap(),
    )))
    .cast();
    let an_adaptor: HCurve = Handle::new(GeomAdaptorCurve::with_curve(&a_line).unwrap()).cast();
    let mut a_polygon = ThePolygonOfHInter::new(&an_adaptor, 6).unwrap();

    assert!(!a_polygon.closed());
    a_polygon.set_closed(true);
    assert!(a_polygon.closed());
}
