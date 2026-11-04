use super::*;

type RelationOperandBuilder = fn(&[Shape]) -> Relation;
use martensite_tkg3d::geom::GeomCircle;
use martensite_tkmath::gp::Axis2;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex};

fn authored_circle(radius: f64) -> Shape {
    let axis = Axis2::new(
        Point3::new(3., -2., 7.),
        Direction3::new(0., 0., 1.).unwrap(),
    )
    .unwrap();
    let curve = Handle::new(GeomCircle::new(axis, radius).unwrap()).cast();
    BRepBuilderAPIMakeEdge::from_curve(&curve)
        .unwrap()
        .edge()
        .unwrap()
        .as_ref()
        .clone()
}

fn authored_line(a: [f64; 3], b: [f64; 3]) -> Shape {
    let [x, y, z] = a;
    let first = BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z))
        .unwrap()
        .vertex()
        .unwrap();
    let [x, y, z] = b;
    let second = BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z))
        .unwrap()
        .vertex()
        .unwrap();
    BRepBuilderAPIMakeEdge::from_vertices(&first, &second)
        .unwrap()
        .edge()
        .unwrap()
        .as_ref()
        .clone()
}

const REJECTED_EMPTY_RELATION_OPERANDS: [(RelationOperandBuilder, &str); 7] = [
    (concentric, "2 circular edges"),
    (equal_distance, "4 shapes (two parallel pairs)"),
    (equal_radius, "1 or 2 circular edges"),
    (fix, "1 edge"),
    (identic, "2 shapes"),
    (offset, "2 parallel faces"),
    (
        |shapes| pair_relation(RelationKind::Parallel, shapes),
        "2 edges or 2 faces",
    ),
];

#[test]
fn relation_command_empty_inputs_report_the_actual_count_and_expected_operands() {
    for (build, expected) in REJECTED_EMPTY_RELATION_OPERANDS {
        let error = build(&[]).err().unwrap().to_string();
        assert!(error.contains("0 selected shapes"), "{error}");
        assert!(error.contains(expected), "{error}");
    }
    let error = symmetric(&[], origin()).err().unwrap().to_string();
    assert!(
        error.contains("0 selected shapes") && error.contains("an edge and 2 edges or 2 vertices")
    );
}

#[test]
fn relation_command_concentric_circles_keep_exact_shape_owners_and_plane_height() {
    let shapes = [authored_circle(2.5), authored_circle(4.5)];
    let object = concentric(&shapes).unwrap();
    let relation = object.cast::<dyn PrsDimRelationSource>();
    assert!(relation.first_shape().is_same(&shapes[0]));
    assert!(relation.second_shape().is_same(&shapes[1]));
    assert_eq!(
        relation
            .plane()
            .get()
            .unwrap()
            .pln()
            .location()
            .z()
            .to_bits(),
        7_f64.to_bits()
    );
}

#[test]
fn relation_command_one_circle_equal_radius_reuses_the_same_shape_owner() {
    let shapes = [authored_circle(2.5)];
    let object = equal_radius(&shapes).unwrap();
    let relation = object.cast::<dyn PrsDimRelationSource>();
    assert!(relation.first_shape().is_same(&shapes[0]));
    assert!(relation.second_shape().is_same(&shapes[0]));
}

#[test]
fn relation_command_nonparallel_edges_are_refused_without_a_plane_fallback() {
    let a = authored_line([0., 0., 0.], [2., 0., 0.]);
    let b = authored_line([0., 0., 0.], [0., 2., 0.]);
    let error = require_parallel(&a, &b).err().unwrap().to_string();
    assert!(error.contains("non parallel edges; expected parallel edges"));
}

#[test]
fn relation_command_parallel_edges_build_the_authored_xy_plane() {
    let shapes = [
        authored_line([0., 0., 0.], [2., 0., 0.]),
        authored_line([0., 2., 0.], [2., 2., 0.]),
    ];
    let object = pair_relation(RelationKind::Parallel, &shapes).unwrap();
    let relation = object.cast::<dyn PrsDimRelationSource>();
    let plane = relation.plane().get().unwrap().pln();
    assert_eq!(plane.location().z().to_bits(), 0_f64.to_bits());
    assert_eq!(plane.axis().direction().z().to_bits(), 1_f64.to_bits());
}

#[test]
fn relation_command_point_helpers_preserve_the_authored_geometry_and_five_unit_offset() {
    let point = Point3::new(-7., 0.5, 2.);
    let vertex = BRepBuilderAPIMakeVertex::new(point)
        .unwrap()
        .vertex()
        .unwrap();
    assert_eq!(vertex_point(vertex.as_ref()).unwrap(), point);
    assert_eq!(offset5(point), Point3::new(-2., 5.5, 7.));
    let edge = authored_line([0., 1., 2.], [2., 1., 2.]);
    assert_eq!(
        first_vertex_point(&topods::edge(&edge).unwrap()).unwrap(),
        Point3::new(0., 1., 2.)
    );
}

#[test]
fn relation_command_wrong_shape_type_identifies_the_actual_vertex() {
    let vertex = BRepBuilderAPIMakeVertex::new(origin())
        .unwrap()
        .vertex()
        .unwrap();
    let error = require_type(&[vertex.as_ref().clone()], ShapeEnum::Edge)
        .err()
        .unwrap()
        .to_string();
    assert!(error.contains("Vertex") && error.contains("expected Edge shapes"));
}
