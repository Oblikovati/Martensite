//! Failed initialization retains the edge assignment and previous 3D evaluator.
use super::*;
use crate::brep::BRepBuilder;
use martensite_tkg3d::geom::{GeomCurve, GeomLine};
use martensite_tkmath::gp::{Axis1, Direction3};

fn initialized_line() -> BRepAdaptorCurve {
    let line: Handle<dyn GeomCurve> =
        Handle::new(GeomLine::new(Axis1::new(Point3::default(), Direction3::X))).cast();
    let mut edge = Edge::default();
    let builder = BRepBuilder::default();
    builder
        .make_edge_with_curve(&mut edge, &line, 1e-7)
        .unwrap();
    builder.range(&edge, 0., 2., true).unwrap();
    BRepAdaptorCurve::with_edge(&edge).unwrap()
}

#[test]
fn geometryless_edge_initialization_preserves_exact_failure_and_partial_state() {
    let adaptor = initialized_line();
    let mut empty = Edge::default();
    BRepBuilder::default().make_edge(&mut empty).unwrap();
    let failure = adaptor.initialize(&empty).unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_NullObject");
    assert_eq!(failure.message(), "BRepAdaptor_Curve::No geometry");
    assert!(adaptor.edge().is_same(&empty));
    assert_eq!(adaptor.d0(1.).unwrap(), Point3::new(1., 0., 0.));
    let construction = BRepAdaptorCurve::with_edge(&empty).err().unwrap();
    assert_eq!(construction.exception_type(), "Standard_NullObject");
    assert_eq!(construction.message(), "BRepAdaptor_Curve::No geometry");
}
