//! Native regression translations for face-level General Fuse cases.
#![allow(non_snake_case)]

use martensite_tkbo::bop_algo::BOPAlgoBuilder;
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeCone, BRepPrimAPIMakeCylinder};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape,
    BRepBuilderAPIMakeVertex,
};

fn first_face(shape: &Shape) -> Shape {
    let mut explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    Iterator::next(&mut explorer).expect("primitive must have a face")
}

fn unique_count(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut shapes = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, kind, &mut shapes).unwrap();
    shapes.extent()
}

// Migrated from tests/bugs/modalg_8/bug33615.  The partition has to avoid
// manufacturing vertices while combining the selected primitive faces.
#[test]
fn BOPAlgoBuilder__ModalgBug_33615_FacePartitionKeepsExpectedTopology() {
    let axis = Axis2::new(
        Point3::new(-5., 0., 4.),
        Direction3::new(-1., 0., 0.).unwrap(),
    )
    .unwrap();
    let cone = BRepPrimAPIMakeCone::with_axes(axis, 3., 1., 10.)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let cylinder = BRepPrimAPIMakeCylinder::new(10., 20.)
        .shape()
        .unwrap()
        .clone();
    let arguments: List<Shape> = [first_face(&cone), first_face(&cylinder)]
        .into_iter()
        .collect();
    let mut builder = BOPAlgoBuilder::new();
    builder.set_arguments(&arguments);
    builder.perform();
    assert!(!builder.options().has_errors());
    for (kind, expected) in [
        (ShapeEnum::Vertex, 5),
        (ShapeEnum::Edge, 8),
        (ShapeEnum::Wire, 5),
        (ShapeEnum::Face, 4),
    ] {
        assert_eq!(unique_count(builder.shape(), kind), expected, "{kind:?}");
    }
}

fn planar_profile() -> Shape {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for point in [
        Point3::new(0., 0., 0.),
        Point3::new(0., 0., 1.),
        Point3::new(1., 0., 1.),
        Point3::new(1., 0., 0.),
    ] {
        polygon.add_point(point).unwrap();
    }
    polygon.close().unwrap();
    BRepBuilderAPIMakeFace::from_wire(&polygon.wire().unwrap(), true)
        .unwrap()
        .face()
        .unwrap()
        .into_shape()
}

// Migrated from tests/bugs/modalg_5/bug24060.  Combining a face with a vertex
// must retain the profile vertices without introducing duplicate topology.
#[test]
fn BOPAlgoBuilder__ModalgBug_24060_FaceVertexFuseKeepsFourVertices() {
    let face = planar_profile();
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::new(0., 0., 1.))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape();
    let arguments: List<Shape> = [face, vertex].into_iter().collect();
    let mut builder = BOPAlgoBuilder::new();
    builder.set_arguments(&arguments);
    builder.perform();
    assert!(!builder.options().has_errors());
    assert_eq!(unique_count(builder.shape(), ShapeEnum::Vertex), 4);
}
