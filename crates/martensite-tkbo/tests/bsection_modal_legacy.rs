//! Native regression translations for compact section-algorithm defect cases.
#![allow(non_snake_case)]

use martensite_tkbo::bop_algo::BOPAlgoSection;
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis1, Direction3, Point3, Transform3};
use martensite_tkmath::toploc::Location;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeVertex,
};

fn vertex(point: Point3) -> Shape {
    BRepBuilderAPIMakeVertex::new(point)
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape()
}

fn count(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut map = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, kind, &mut map).unwrap();
    map.extent()
}

fn section(first: &Shape, second: &Shape) -> Shape {
    let arguments: List<Shape> = [first.clone(), second.clone()].into_iter().collect();
    let mut operation = BOPAlgoSection::new();
    operation.set_arguments(&arguments);
    operation.perform();
    assert!(!operation.options().has_errors());
    operation.shape().clone()
}

fn assert_point_section(result: &Shape) {
    assert_eq!(count(result, ShapeEnum::Vertex), 1);
    assert_eq!(count(result, ShapeEnum::Edge), 0);
    assert_eq!(count(result, ShapeEnum::Wire), 0);
    assert_eq!(count(result, ShapeEnum::Face), 0);
    assert_eq!(count(result, ShapeEnum::Shell), 0);
    assert_eq!(count(result, ShapeEnum::Solid), 0);
    assert_eq!(result.shape_type().unwrap(), ShapeEnum::Compound);
}

// Migrated from tests/bugs/modalg_5/bug25354_39.
#[test]
fn BSection__ModalgBug_25354_39_CoincidentVertices() {
    assert_point_section(&section(
        &vertex(Point3::default()),
        &vertex(Point3::default()),
    ));
}

fn box_shape() -> Shape {
    BRepPrimAPIMakeBox::from_sizes(10., 10., 10.)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

fn assert_vertex_box_section(point: Point3, expected_vertices: usize) {
    let result = section(&vertex(point), &box_shape());
    assert_eq!(count(&result, ShapeEnum::Vertex), expected_vertices);
    assert_eq!(count(&result, ShapeEnum::Edge), 0);
    assert_eq!(count(&result, ShapeEnum::Wire), 0);
    assert_eq!(count(&result, ShapeEnum::Face), 0);
    assert_eq!(result.shape_type().unwrap(), ShapeEnum::Compound);
}

// Migrated from tests/bugs/modalg_5/bug25354_45.
#[test]
fn BSection__ModalgBug_25354_45_BoxCorner() {
    assert_vertex_box_section(Point3::default(), 1);
}

// Migrated from tests/bugs/modalg_5/bug25354_46.
#[test]
fn BSection__ModalgBug_25354_46_BoxEdge() {
    assert_vertex_box_section(Point3::new(4., 0., 0.), 1);
}

// Migrated from tests/bugs/modalg_5/bug25354_47.
#[test]
fn BSection__ModalgBug_25354_47_BoxFace() {
    assert_vertex_box_section(Point3::new(4., 3., 0.), 1);
}

// Migrated from tests/bugs/modalg_5/bug25354_48.
#[test]
fn BSection__ModalgBug_25354_48_BoxInteriorIsEmpty() {
    assert_vertex_box_section(Point3::new(4., 3., 2.), 0);
}

fn translated_box(origin: Point3) -> Shape {
    BRepPrimAPIMakeBox::at_point(origin, 10., 10., 10.)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

fn assert_box_section(origin: Point3, vertices: usize, edges: usize) {
    let result = section(&box_shape(), &translated_box(origin));
    assert_eq!(count(&result, ShapeEnum::Vertex), vertices);
    assert_eq!(count(&result, ShapeEnum::Edge), edges);
    assert_eq!(count(&result, ShapeEnum::Wire), 0);
    assert_eq!(count(&result, ShapeEnum::Face), 0);
    assert_eq!(count(&result, ShapeEnum::Shell), 0);
    assert_eq!(count(&result, ShapeEnum::Solid), 0);
    assert_eq!(result.shape_type().unwrap(), ShapeEnum::Compound);
}

// Migrated from tests/bugs/modalg_5/bug25354_32.
#[test]
fn BSection__ModalgBug_25354_32_BoxCorner() {
    assert_box_section(Point3::new(-10., -10., -10.), 1, 0);
}

// Migrated from tests/bugs/modalg_5/bug25354_35.
#[test]
fn BSection__ModalgBug_25354_35_BoxEdge() {
    assert_box_section(Point3::new(-10., -10., -4.), 2, 1);
}

// Migrated from tests/bugs/modalg_5/bug25354_37.
#[test]
fn BSection__ModalgBug_25354_37_BoxFaceBoundary() {
    assert_box_section(Point3::new(10., 3., 4.), 4, 4);
}

// Migrated from tests/bugs/modalg_5/bug25354_38.
#[test]
fn BSection__ModalgBug_25354_38_OverlappingBoxes() {
    assert_box_section(Point3::new(5., 3., 4.), 6, 6);
}

fn rotate(shape: &mut Shape, pivot: Point3, axis: Direction3, degrees: f64) {
    let mut transform = Transform3::default();
    transform
        .set_rotation(Axis1::new(pivot, axis), degrees.to_radians())
        .unwrap();
    shape.move_by(&Location::new(transform), false).unwrap();
}

fn assert_rotated_box_section(
    origin: Point3,
    rotations: &[(Point3, Direction3, f64)],
    vertices: usize,
    edges: usize,
) {
    let mut second = translated_box(origin);
    for &(pivot, axis, degrees) in rotations {
        rotate(&mut second, pivot, axis, degrees);
    }
    let result = section(&box_shape(), &second);
    assert_eq!(count(&result, ShapeEnum::Vertex), vertices);
    assert_eq!(count(&result, ShapeEnum::Edge), edges);
    assert_eq!(count(&result, ShapeEnum::Wire), 0);
    assert_eq!(count(&result, ShapeEnum::Face), 0);
    assert_eq!(result.shape_type().unwrap(), ShapeEnum::Compound);
}

// Migrated from tests/bugs/modalg_5/bug25354_33.
#[test]
fn BSection__ModalgBug_25354_33_RotatedCorner() {
    assert_rotated_box_section(
        Point3::new(5., -10., -10.),
        &[(
            Point3::new(5., 0., 0.),
            Direction3::new(0., 0., 1.).unwrap(),
            -45.,
        )],
        1,
        0,
    );
}

// Migrated from tests/bugs/modalg_5/bug25354_34.
#[test]
fn BSection__ModalgBug_25354_34_DoublyRotatedCorner() {
    assert_rotated_box_section(
        Point3::new(5., -10., -4.),
        &[
            (
                Point3::new(5., 0., 0.),
                Direction3::new(0., 0., 1.).unwrap(),
                -45.,
            ),
            (
                Point3::new(5., 0., 0.),
                Direction3::new(1., 0., 0.).unwrap(),
                45.,
            ),
        ],
        1,
        0,
    );
}

// Migrated from tests/bugs/modalg_5/bug25354_36.
#[test]
fn BSection__ModalgBug_25354_36_RotatedEdge() {
    assert_rotated_box_section(
        Point3::new(5., -10., -4.),
        &[(
            Point3::new(5., 0., 0.),
            Direction3::new(0., 0., 1.).unwrap(),
            -45.,
        )],
        2,
        1,
    );
}

fn edge(first: Point3, second: Point3) -> Shape {
    BRepBuilderAPIMakeEdge::from_points(first, second)
        .unwrap()
        .edge()
        .unwrap()
        .into_shape()
}

// Migrated from tests/bugs/modalg_5/bug25354_40.
#[test]
fn BSection__ModalgBug_25354_40_VertexOnEdge() {
    assert_point_section(&section(
        &vertex(Point3::new(4., 0., 0.)),
        &edge(Point3::default(), Point3::new(10., 0., 0.)),
    ));
}

// Migrated from tests/bugs/modalg_5/bug25354_41.
#[test]
fn BSection__ModalgBug_25354_41_EdgeEndpoint() {
    assert_point_section(&section(
        &vertex(Point3::default()),
        &edge(Point3::default(), Point3::new(10., 0., 0.)),
    ));
}
