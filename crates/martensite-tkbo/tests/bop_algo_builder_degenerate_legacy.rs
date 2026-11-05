//! Native translations for degenerate General Fuse regressions.
#![allow(non_snake_case)]

use martensite_tkbo::bop_algo::BOPAlgoBuilder;
use martensite_tkbrep::{topexp, topods::Shape, toptools::IndexedMapOfShape};
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape,
    BRepBuilderAPIMakeVertex,
};

fn line(first: Point3, second: Point3) -> Shape {
    BRepBuilderAPIMakePolygon::from_two_points(first, second)
        .unwrap()
        .wire()
        .unwrap()
        .into_shape()
}

fn count(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut map = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, kind, &mut map).unwrap();
    map.extent()
}

// Migrated from tests/bugs/modalg_6/bug28775.
#[test]
fn BOPAlgoBuilder__ModalgBug_28775_CrossingPolylinesSplitIntoFourEdges() {
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::new(1., 0., 0.))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape();
    let arguments: List<Shape> = [
        vertex,
        line(Point3::default(), Point3::new(2., 0., 0.)),
        line(Point3::new(1., 0., -1.), Point3::new(1., 0., 1.)),
    ]
    .into_iter()
    .collect();
    let mut builder = BOPAlgoBuilder::new();
    builder.set_arguments(&arguments);
    builder.set_non_destructive(true);
    builder.perform();
    assert!(!builder.options().has_errors());
    assert_eq!(count(builder.shape(), ShapeEnum::Edge), 4);
}

// Migrated from tests/bugs/modalg_6/bug26420_1.
#[test]
fn BOPAlgoBuilder__ModalgBug_26420_ClosedPolylineRemainsClosed() {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for point in [
        Point3::new(0., 4., 0.),
        Point3::new(7., 4., 0.),
        Point3::new(7., 6., 0.),
        Point3::new(0., 6., 0.),
    ] {
        polygon.add_point(point).unwrap();
    }
    polygon.close().unwrap();
    let first = BRepBuilderAPIMakeVertex::new(Point3::new(5., 4., 0.))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape();
    let second = BRepBuilderAPIMakeVertex::new(Point3::new(4., 6., 0.))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape();
    let arguments: List<Shape> = [first, second, polygon.wire().unwrap().into_shape()]
        .into_iter()
        .collect();
    let mut builder = BOPAlgoBuilder::new();
    builder.set_arguments(&arguments);
    builder.perform();
    assert!(!builder.options().has_errors());
    let mut wires = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(builder.shape(), ShapeEnum::Wire, &mut wires).unwrap();
    assert!(wires.iter().all(|wire| wire.closed().unwrap()));
}

// Migrated from tests/bugs/modalg_6/bug26420_2.
#[test]
fn BOPAlgoBuilder__ModalgBug_26420_ClosedFaceRemainsClosed() {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for point in [
        Point3::new(0., 4., 0.),
        Point3::new(7., 4., 0.),
        Point3::new(7., 6., 0.),
        Point3::new(0., 6., 0.),
    ] {
        polygon.add_point(point).unwrap();
    }
    polygon.close().unwrap();
    let face = BRepBuilderAPIMakeFace::from_wire(&polygon.wire().unwrap(), true)
        .unwrap()
        .face()
        .unwrap()
        .into_shape();
    let a = BRepBuilderAPIMakeVertex::new(Point3::new(5., 4., 0.))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape();
    let b = BRepBuilderAPIMakeVertex::new(Point3::new(4., 6., 0.))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape();
    let mut builder = BOPAlgoBuilder::new();
    builder.set_arguments(&[a, b, face].into_iter().collect());
    builder.perform();
    assert!(!builder.options().has_errors());
    let mut wires = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(builder.shape(), ShapeEnum::Wire, &mut wires).unwrap();
    assert!(wires.iter().all(|wire| wire.closed().unwrap()));
}

// Migrated from tests/bugs/modalg_6/bug26420_3.
#[test]
fn BOPAlgoBuilder__ModalgBug_26420_ClosedShellRemainsClosed() {
    let box_shape = BRepPrimAPIMakeBox::at_point(Point3::default(), 5., 5., 5.)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let mut explorer =
        topexp::Explorer::new(&box_shape, ShapeEnum::Shell, ShapeEnum::Shape).unwrap();
    let shell = Iterator::next(&mut explorer).unwrap();
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for point in [
        Point3::new(-1., -1., 2.),
        Point3::new(-1., 6., 2.),
        Point3::new(6., 6., 2.),
        Point3::new(6., -1., 2.),
    ] {
        polygon.add_point(point).unwrap();
    }
    polygon.close().unwrap();
    let face = BRepBuilderAPIMakeFace::from_wire(&polygon.wire().unwrap(), true)
        .unwrap()
        .face()
        .unwrap()
        .into_shape();
    let mut builder = BOPAlgoBuilder::new();
    builder.set_arguments(&[face, shell].into_iter().collect());
    builder.perform();
    assert!(!builder.options().has_errors());
    let mut shells = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(builder.shape(), ShapeEnum::Shell, &mut shells).unwrap();
    assert!(shells.iter().all(|shell| shell.closed().unwrap()));
}
