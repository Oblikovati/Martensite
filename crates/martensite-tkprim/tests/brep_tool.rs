//! Tests of `BRep_Tool`.
//! Production primitive/edge builders preserve the original setup and done guards.
#![allow(non_snake_case)]
mod support;
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomBezierCurve, GeomCircle, GeomCurve, GeomPlane, GeomSurface};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{self, Axis2, Axis3, Point3};
use martensite_tkmath::toploc::Location;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape};
use support::expect_double_eq;

fn gp_Pnt(x: f64, y: f64, z: f64) -> Point3 {
    Point3::new(x, y, z)
}

/// Shape access triggers the lazy build before the original completion assertion.
fn box_shape() -> Shape {
    let mut maker = BRepPrimAPIMakeBox::from_sizes(10.0, 20.0, 30.0).unwrap();
    let shape = maker.shape().unwrap().clone();
    assert!(maker.is_done());
    shape
}

/// The first sub-shape of `kind` found by `TopExp_Explorer`.
fn first_of(shape: &Shape, kind: ShapeEnum) -> Shape {
    let anExp = Explorer::new(shape, kind, ShapeEnum::Shape).unwrap();
    assert!(anExp.more());
    anExp.current().clone()
}

/// `new Geom_Circle(gp_Ax2(Origin, DZ), radius)`.
fn circle(radius: f64) -> Handle<dyn GeomCurve> {
    let anAx2 = Axis2::new(gp::origin(), gp::dz()).unwrap();
    Handle::new(GeomCircle::new(anAx2, radius).unwrap()).into_cast()
}

/// `new Geom_Plane(gp::XOY())`.
fn xoy_plane() -> Handle<dyn GeomSurface> {
    Handle::new(GeomPlane::new(Axis3::from_axis2(gp::xoy()))).into_cast()
}

#[test]
fn BRep_Tool_Test__Pnt_FromVertex() {
    let aBox = box_shape();

    let aVertex = topods::vertex(&first_of(&aBox, ShapeEnum::Vertex)).unwrap();
    let aPnt = BRepTool::pnt(&aVertex).unwrap();

    // The point coordinates should be within the box bounds [0,10] x [0,20] x [0,30]
    assert!(aPnt.x() >= -precision::confusion());
    assert!(aPnt.x() <= 10.0 + precision::confusion());
    assert!(aPnt.y() >= -precision::confusion());
    assert!(aPnt.y() <= 20.0 + precision::confusion());
    assert!(aPnt.z() >= -precision::confusion());
    assert!(aPnt.z() <= 30.0 + precision::confusion());
}

#[test]
fn BRep_Tool_Test__Tolerance_Vertex() {
    let aBox = box_shape();

    let aVertex = topods::vertex(&first_of(&aBox, ShapeEnum::Vertex)).unwrap();
    let aTol = BRepTool::vertex_tolerance(&aVertex).unwrap();
    assert!(aTol >= 0.0);
}

#[test]
fn BRep_Tool_Test__Tolerance_Edge() {
    let aBox = box_shape();

    let anEdge = topods::edge(&first_of(&aBox, ShapeEnum::Edge)).unwrap();
    let aTol = BRepTool::edge_tolerance(&anEdge).unwrap();
    assert!(aTol >= 0.0);
}

#[test]
fn BRep_Tool_Test__Tolerance_Face() {
    let aBox = box_shape();

    let aFace = topods::face(&first_of(&aBox, ShapeEnum::Face)).unwrap();
    let aTol = BRepTool::face_tolerance(&aFace).unwrap();
    assert!(aTol >= 0.0);
}

#[test]
fn BRep_Tool_Test__Curve_FromEdge() {
    let aBox = box_shape();

    let anEdge = topods::edge(&first_of(&aBox, ShapeEnum::Edge)).unwrap();
    let (aCurve, aFirst, aLast) = BRepTool::curve(&anEdge).unwrap();

    assert!(
        !aCurve.is_null(),
        "Curve from a box edge should not be null"
    );
    assert!(
        aFirst < aLast,
        "First parameter should be less than last parameter"
    );
}

#[test]
fn BRep_Tool_Test__Surface_FromFace() {
    let aBox = box_shape();

    let aFace = topods::face(&first_of(&aBox, ShapeEnum::Face)).unwrap();
    let aSurface = BRepTool::surface(&aFace).unwrap();

    assert!(
        !aSurface.is_null(),
        "Surface from a box face should not be null"
    );
}

#[test]
fn BRep_Tool_Test__IsClosed_CircleEdge() {
    let aCircle = circle(5.0);
    let anEdgeMaker = BRepBuilderAPIMakeEdge::from_curve(&aCircle).unwrap();
    assert!(anEdgeMaker.is_done());
    let anEdge = anEdgeMaker.edge().unwrap();

    assert!(
        BRepTool::is_closed(&anEdge).unwrap(),
        "A full circle edge should be closed"
    );
}

#[test]
fn BRep_Tool_Test__IsClosed_LineEdge() {
    let anEdgeMaker =
        BRepBuilderAPIMakeEdge::from_points(gp_Pnt(0.0, 0.0, 0.0), gp_Pnt(10.0, 0.0, 0.0)).unwrap();
    assert!(anEdgeMaker.is_done());
    let anEdge = anEdgeMaker.edge().unwrap();

    assert!(
        !BRepTool::is_closed(&anEdge).unwrap(),
        "A line segment edge should not be closed"
    );
}

#[test]
fn BRep_Tool_Test__Degenerated() {
    let aBox = box_shape();

    let anEdge = topods::edge(&first_of(&aBox, ShapeEnum::Edge)).unwrap();
    assert!(
        !BRepTool::degenerated(&anEdge).unwrap(),
        "Box edges should not be degenerated"
    );
}

#[test]
fn BRep_Tool_Test__CurveOnPlane_RejectsEdgeRangeOutsideBoundedCurve() {
    let aPolesBuffer = [
        gp_Pnt(0.0, 0.0, 0.0),
        gp_Pnt(0.5, 0.25, 0.0),
        gp_Pnt(1.0, 0.0, 0.0),
    ];
    let aPoles = Array1::from_shared(&aPolesBuffer, 1);
    let aCurve: Handle<dyn GeomCurve> =
        Handle::new(GeomBezierCurve::new(&aPoles).unwrap()).into_cast();

    let anEdgeMaker = BRepBuilderAPIMakeEdge::from_curve(&aCurve).unwrap();
    assert!(anEdgeMaker.is_done());
    let anEdge = anEdgeMaker.edge().unwrap();

    const THE_EDGE_FIRST: f64 = -0.000002673149646;
    const THE_EDGE_LAST: f64 = 0.999997326850354;
    BRepBuilder::default()
        .range(&anEdge, THE_EDGE_FIRST, THE_EDGE_LAST, true)
        .unwrap();

    let aPlane = xoy_plane();
    let aResult = BRepTool::curve_on_surface(&anEdge, &aPlane, &Location::default());
    assert!(aResult.is_ok(), "CurveOnSurface should not throw");
    let aResult = aResult.unwrap();

    assert!(aResult.pcurve.is_null());
    assert!(!aResult.is_stored);
    expect_double_eq(aResult.first, THE_EDGE_FIRST);
    expect_double_eq(aResult.last, THE_EDGE_LAST);
}

#[test]
fn BRep_Tool_Test__CurveOnPlane_ProjectsShiftedPeriodicRange() {
    let aCircle = circle(1.0);
    let anEdgeMaker = BRepBuilderAPIMakeEdge::from_curve(&aCircle).unwrap();
    assert!(anEdgeMaker.is_done());
    let anEdge = anEdgeMaker.edge().unwrap();

    let aPeriod = aCircle.get().unwrap().period().unwrap();
    let aRangeFirst = aPeriod;
    let aRangeLast = 2.0 * aPeriod;
    BRepBuilder::default()
        .range(&anEdge, aRangeFirst, aRangeLast, true)
        .unwrap();

    let aPlane = xoy_plane();
    let aResult = BRepTool::curve_on_plane(&anEdge, &aPlane, &Location::default());
    assert!(aResult.is_ok(), "CurveOnPlane should not throw");
    let (aPCurve, aFirst, aLast) = aResult.unwrap();

    assert!(!aPCurve.is_null());
    expect_double_eq(aFirst, aRangeFirst);
    expect_double_eq(aLast, aRangeLast);
}

#[test]
fn BRep_Tool_Test__CurveOnPlane_ProjectsPeriodicEdgeBuiltFromEqualParameters() {
    let aCircle = circle(1.0);
    let anEdgeMaker = BRepBuilderAPIMakeEdge::from_curve_params(&aCircle, 0.0, 0.0).unwrap();
    assert!(anEdgeMaker.is_done());
    let anEdge = anEdgeMaker.edge().unwrap();

    let (aEdgeFirst, aEdgeLast) = BRepTool::range(&anEdge).unwrap();
    let aPeriod = aCircle.get().unwrap().period().unwrap();
    assert!(((aEdgeLast - aEdgeFirst) - aPeriod).abs() <= precision::p_confusion());

    let aPlane = xoy_plane();
    let aResult = BRepTool::curve_on_plane(&anEdge, &aPlane, &Location::default());
    assert!(aResult.is_ok(), "CurveOnPlane should not throw");
    let (aPCurve, aFirst, aLast) = aResult.unwrap();

    assert!(!aPCurve.is_null());
    expect_double_eq(aFirst, aEdgeFirst);
    expect_double_eq(aLast, aEdgeLast);
}

#[test]
fn BRep_Tool_Test__CurveOnPlane_RejectsEqualPeriodicRange() {
    let aCircle = circle(1.0);
    let anEdgeMaker = BRepBuilderAPIMakeEdge::from_curve(&aCircle).unwrap();
    assert!(anEdgeMaker.is_done());
    let anEdge = anEdgeMaker.edge().unwrap();
    BRepBuilder::default()
        .range(&anEdge, 0.0, 0.0, true)
        .unwrap();

    let aPlane = xoy_plane();
    let aResult = BRepTool::curve_on_plane(&anEdge, &aPlane, &Location::default());
    assert!(aResult.is_ok(), "CurveOnPlane should not throw");
    let (aPCurve, aFirst, aLast) = aResult.unwrap();

    assert!(aPCurve.is_null());
    expect_double_eq(aFirst, 0.0);
    expect_double_eq(aLast, 0.0);
}
