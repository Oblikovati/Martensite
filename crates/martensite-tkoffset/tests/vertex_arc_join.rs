//! Differential checks for `BRepOffsetOffset::init(vertex, edges, offset)`.
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topexp::Explorer,
    topods::{Edge, Shape, Vertex},
};
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::{geom::GeomSphericalSurface, top_abs::ShapeEnum};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::Point3;
use martensite_tkoffset::brep_offset::{BRepOffsetOffset, BRepOffsetStatus, PipeOptions};
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape},
    brep_check::BRepCheckAnalyzer,
    brep_gprop::BRepGProp,
};

#[test]
fn vertex_arc_join_builds_reference_spherical_face_from_three_edges() {
    assert_reference_positive_frame(&joined_face(2.0));
    assert_reference_incomplete_face(&joined_face(2.0));
}

#[test]
fn negative_vertex_join_reverses_face_and_uses_absolute_radius() {
    let face = joined_face(-2.0);
    let surface = BRepTool::surface(&face)
        .unwrap()
        .cast::<GeomSphericalSurface>();
    assert_eq!(surface.get().unwrap().sphere().unwrap().radius(), 2.0);
    assert_eq!(
        face.orientation(),
        martensite_tkg3d::top_abs::Orientation::Reversed
    );
}

#[test]
fn offset_object_vertex_mode_retains_reference_state_and_face() {
    let mut vertex = Vertex::default();
    BRepBuilder::default()
        .make_vertex_at(&mut vertex, Point3::default(), 1.0e-7)
        .unwrap();
    let edges = [
        edge(Point3::new(3.0, 0.0, 0.0)),
        edge(Point3::new(0.0, 3.0, 0.0)),
        edge(Point3::new(0.0, 0.0, 3.0)),
    ];
    let offset =
        BRepOffsetOffset::from_vertex(&vertex, &edge_list(&edges), 2.0, PipeOptions::default())
            .unwrap();
    assert_eq!(offset.status(), BRepOffsetStatus::Good);
    assert!(offset.initial_shape().is_same(&vertex.clone().into_shape()));
    assert!(offset.generated(&vertex.clone().into_shape()).is_null());
    assert_reference_positive_frame(offset.face());
    assert_reference_incomplete_face(offset.face());
}

fn joined_face(offset: f64) -> martensite_tkbrep::topods::Face {
    let mut vertex = Vertex::default();
    BRepBuilder::default()
        .make_vertex_at(&mut vertex, Point3::default(), 1.0e-7)
        .unwrap();
    let edges = [
        edge(Point3::new(3.0, 0.0, 0.0)),
        edge(Point3::new(0.0, 3.0, 0.0)),
        edge(Point3::new(0.0, 0.0, 3.0)),
    ];
    BRepOffsetOffset::from_vertex(&vertex, &edge_list(&edges), offset, PipeOptions::default())
        .unwrap()
        .face()
        .clone()
}

fn edge_list(edges: &[Edge]) -> List<Shape> {
    let mut list = List::default();
    for e in edges {
        list.append(e.clone().into_shape());
    }
    list
}

fn assert_reference_positive_frame(face: &martensite_tkbrep::topods::Face) {
    let surface = BRepTool::surface(face)
        .unwrap()
        .cast::<GeomSphericalSurface>();
    let sphere = surface.get().unwrap().sphere().unwrap();
    assert_eq!(sphere.radius(), 2.0);
    assert!(
        sphere
            .position()
            .location()
            .square_distance(&Point3::default())
            < 1.0e-24
    );
    assert_near(
        sphere.position().direction().x(),
        -std::f64::consts::FRAC_1_SQRT_2,
    );
    assert_near(
        sphere.position().direction().z(),
        std::f64::consts::FRAC_1_SQRT_2,
    );
    assert_near(sphere.position().x_direction().x(), -1.0 / 3.0_f64.sqrt());
}

fn assert_reference_incomplete_face(face: &martensite_tkbrep::topods::Face) {
    let shape = Shape::from(face.clone());
    assert_eq!(count(&shape, ShapeEnum::Edge), 3);
    assert_eq!(count(&shape, ShapeEnum::Vertex), 6);
    assert_eq!(face.nb_children(), 1);
    assert_eq!(
        face.orientation(),
        martensite_tkg3d::top_abs::Orientation::Forward
    );
    assert!(
        !BRepCheckAnalyzer::new(&shape, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    let mut props = GProps::new();
    BRepGProp::surface_properties(&shape, &mut props, false, false).unwrap();
    assert_eq!(props.mass(), 0.0);
}

fn assert_near(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1.0e-6,
        "{actual} != {expected}"
    );
}

fn count(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).unwrap();
    let mut result = 0;
    while explorer.more() {
        result += 1;
        explorer.next().unwrap();
    }
    result
}

fn edge(end: Point3) -> Edge {
    let shape = BRepBuilderAPIMakeEdge::from_points(Point3::default(), end)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    Edge::unchecked(shape)
}
