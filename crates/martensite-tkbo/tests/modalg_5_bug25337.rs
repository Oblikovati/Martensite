//! Native replays of the OCC25337 coplanar circular-face Boolean regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{PerformCommon, PerformFuse, ValidateResult};
use martensite_tkbrep::{topexp, topods::Shape, toptools::IndexedMapOfShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis2, Circle3, Direction3, Point3};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeWire,
};

fn circular_face(center_x: f64) -> Shape {
    let axis = Axis2::new(Point3::new(center_x, 0., 0.), Direction3::Z).unwrap();
    let edge = BRepBuilderAPIMakeEdge::from_circ(Circle3::new(axis, 10.).unwrap())
        .unwrap()
        .edge()
        .unwrap();
    let wire = BRepBuilderAPIMakeWire::from_edge(&edge)
        .unwrap()
        .wire()
        .unwrap();
    BRepBuilderAPIMakeFace::from_wire(&wire, false)
        .unwrap()
        .face()
        .unwrap()
        .into_shape()
}

fn expect_counts(shape: &Shape, counts: &[(ShapeEnum, usize)]) {
    for &(kind, expected) in counts {
        let mut found = IndexedMapOfShape::default();
        if kind == ShapeEnum::Shape {
            topexp::map_shapes(shape, &mut found, true, true);
        } else {
            topexp::map_shapes_of_type(shape, kind, &mut found).unwrap();
        }
        assert_eq!(found.extent(), expected, "{kind:?}");
    }
}

fn faces() -> (Shape, Shape) {
    (circular_face(-50.), circular_face(-35.))
}

#[test]
fn Modalg5__Bug25337_1() {
    let (first, second) = faces();
    let result = PerformFuse(&first, &second);
    ValidateResult(&result, 582.987, -1., false);
    expect_counts(
        &result,
        &[
            (ShapeEnum::Vertex, 4),
            (ShapeEnum::Edge, 6),
            (ShapeEnum::Wire, 3),
            (ShapeEnum::Face, 3),
            (ShapeEnum::Shell, 0),
            (ShapeEnum::Solid, 0),
            (ShapeEnum::CompSolid, 0),
            (ShapeEnum::Compound, 1),
            (ShapeEnum::Shape, 17),
        ],
    );
}

#[test]
fn Modalg5__Bug25337_2() {
    let (first, second) = faces();
    let result = PerformCommon(&first, &second);
    ValidateResult(&result, 45.3312, -1., false);
    expect_counts(
        &result,
        &[
            (ShapeEnum::Vertex, 3),
            (ShapeEnum::Edge, 3),
            (ShapeEnum::Wire, 1),
            (ShapeEnum::Face, 1),
            (ShapeEnum::Shell, 0),
            (ShapeEnum::Solid, 0),
            (ShapeEnum::CompSolid, 0),
            (ShapeEnum::Compound, 1),
            (ShapeEnum::Shape, 9),
        ],
    );
}
