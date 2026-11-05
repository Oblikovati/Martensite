//! Native replay of the OCC24187 Boolean common regression.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{CreateBox, PerformCommon};
use martensite_tkbrep::{
    brep::BRepBuilder,
    topexp,
    topods::{Compound, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Axis2, Circle3, Direction3, Point3};
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeEdge, brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp,
};

fn circle_edge(height: f64) -> Shape {
    let axis = Axis2::new(Point3::new(-3., 5., height), Direction3::Z).unwrap();
    BRepBuilderAPIMakeEdge::from_circ(Circle3::new(axis, 5.).unwrap())
        .unwrap()
        .edge()
        .unwrap()
        .into_shape()
}

fn compound(shapes: &[Shape]) -> Shape {
    let builder = BRepBuilder::default();
    let mut result = Compound::default();
    builder.make_compound(&mut result);
    for shape in shapes {
        builder.add(&mut result, shape).unwrap();
    }
    result.into_shape()
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

#[test]
fn Modalg5__Bug24187() {
    let box_shape = CreateBox(Point3::new(0., 0., 0.), 10., 10., 10.);
    let edges = compound(&[circle_edge(2.), circle_edge(8.)]);
    let result = PerformCommon(&box_shape, &edges);
    let mut properties = GProps::default();
    BRepGProp::linear_properties(&result, &mut properties, false, false).unwrap();
    assert!((properties.mass() - 18.5459).abs() / 18.5459 <= 0.01);
    assert!(
        BRepCheckAnalyzer::new(&result, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    expect_counts(
        &result,
        &[
            (ShapeEnum::Vertex, 6),
            (ShapeEnum::Edge, 4),
            (ShapeEnum::Wire, 0),
            (ShapeEnum::Face, 0),
            (ShapeEnum::Shell, 0),
            (ShapeEnum::Solid, 0),
            (ShapeEnum::CompSolid, 0),
            (ShapeEnum::Compound, 1),
            (ShapeEnum::Shape, 11),
        ],
    );
}
