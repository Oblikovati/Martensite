//! Source-edge endpoint materialization distinguishes lower and upper unbounded signs.
use martensite_tkbo::bop_ds::DataStructure;
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topods::{Edge, Vertex},
};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg3d::geom::{GeomCurve, GeomLine};
use martensite_tkmath::gp::{Axis1, Direction3, Point3};

fn line_edge(first: f64, last: f64) -> Edge {
    let line: Handle<dyn GeomCurve> =
        Handle::new(GeomLine::new(Axis1::new(Point3::default(), Direction3::X))).cast();
    let builder = BRepBuilder::default();
    let mut edge = Edge::default();
    builder
        .make_edge_with_curve(&mut edge, &line, 1e-7)
        .unwrap();
    builder.range(&edge, first, last, true).unwrap();
    edge
}

fn assert_single_synthetic_endpoint(first: f64, last: f64, expected: f64) {
    let edge = line_edge(first, last).into_shape();
    let mut structure = DataStructure::default();
    structure.set_arguments(std::slice::from_ref(&edge));
    let failure = structure.init(1e-7).unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_Failure");
    assert_eq!(failure.message(), "GeomBndLib_Line::Box - bad parameter");
    let edge_info = structure.shape_info(structure.index(&edge)).unwrap();
    assert_eq!(edge_info.sub_shapes().len(), 1);
    assert_eq!(structure.nb_source_shapes(), 1);
    assert_eq!(structure.nb_shapes(), 2);
    let endpoint = structure.shape_info(edge_info.sub_shapes()[0]).unwrap();
    assert_eq!(endpoint.flag(), 1);
    let point = BRepTool::pnt(&Vertex::unchecked(endpoint.shape().clone())).unwrap();
    assert_eq!(point.x(), expected);
}

#[test]
fn positive_threshold_at_start_does_not_create_a_lower_infinite_vertex() {
    let threshold = 0.5 * precision::infinite();
    assert_single_synthetic_endpoint(threshold, 1.5 * threshold, 1.5 * threshold);
}

#[test]
fn negative_threshold_at_end_does_not_create_an_upper_infinite_vertex() {
    let threshold = 0.5 * precision::infinite();
    assert_single_synthetic_endpoint(-1.5 * threshold, -threshold, -1.5 * threshold);
}
