//! Edge analysis regression cases with geometric fixture builders.
#![allow(non_snake_case)]
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Edge};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomCircle, GeomCurve};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis2, Point3, dz};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tkshhealing::shape_analysis::ShapeAnalysisEdge;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape};

fn legacy_analysis_segment() -> Edge {
    let maker = BRepBuilderAPIMakeEdge::from_points(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(10.0, 0.0, 0.0),
    )
    .unwrap();
    assert!(maker.is_done());
    maker.edge().unwrap()
}

#[test]
fn ShapeAnalysis_EdgeTest__HasCurve3d() {
    let edge = legacy_analysis_segment();
    let analyzer = ShapeAnalysisEdge::new();
    assert!(analyzer.has_curve_3d(&edge).unwrap());
}

#[test]
fn ShapeAnalysis_EdgeTest__FirstVertex_LastVertex() {
    let edge = legacy_analysis_segment();
    let analyzer = ShapeAnalysisEdge::new();
    let first = analyzer.first_vertex(&edge);
    let last = analyzer.last_vertex(&edge);
    assert!(!first.is_null());
    assert!(!last.is_null());
}

#[test]
fn ShapeAnalysis_EdgeTest__IsClosed_OpenEdge() {
    let edge = legacy_analysis_segment();
    let analyzer = ShapeAnalysisEdge::new();
    assert!(!analyzer.is_closed_3d(&edge).unwrap());
}

#[test]
fn ShapeAnalysis_EdgeTest__IsClosed_ClosedEdge() {
    let frame = Axis2::new(Point3::new(0.0, 0.0, 0.0), dz()).unwrap();
    let circle: Handle<dyn GeomCurve> = Handle::new(GeomCircle::new(frame, 5.0).unwrap()).cast();
    let maker = BRepBuilderAPIMakeEdge::from_curve(&circle).unwrap();
    assert!(maker.is_done());
    let edge = maker.edge().unwrap();
    let analyzer = ShapeAnalysisEdge::new();
    assert!(analyzer.is_closed_3d(&edge).unwrap());
}

#[test]
fn ShapeAnalysis_EdgeTest__IsSeam_NonSeam() {
    let mut maker = BRepPrimAPIMakeBox::from_sizes(10.0, 10.0, 10.0).unwrap();
    let shape = maker.shape().unwrap().clone();
    assert!(maker.is_done());
    let faces = Explorer::new(&shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    assert!(faces.more());
    let face = topods::face(faces.current()).unwrap();
    let edges = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    assert!(edges.more());
    let edge = topods::edge(edges.current()).unwrap();
    let analyzer = ShapeAnalysisEdge::new();
    assert!(!analyzer.is_seam_on_face(&edge, &face).unwrap());
}
