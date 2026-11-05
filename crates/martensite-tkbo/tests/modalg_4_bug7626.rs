//! Native replay of OCC7626 Boolean regression cases.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{
    CreateBox, CreateSphere, PerformCommon, PerformCut, RotateAroundAxis, ValidateResult,
};
use martensite_tkbrep::{topexp, topods::Shape, toptools::IndexedMapOfShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;

fn rotated_box() -> Shape {
    let mut shape = CreateBox(Point3::new(10., 0., 0.), 20., 20., 20.);
    RotateAroundAxis(&mut shape, Point3::new(10., 0., 0.), [0., 0., 1.], 45.);
    shape
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
fn Modalg4__Bug7626_1() {
    let sphere = CreateSphere(Point3::new(0., 0., 0.), 10.);
    let result = PerformCut(&sphere, &rotated_box());
    ValidateResult(&result, 1271.7, -1., false);
    expect_counts(
        &result,
        &[
            (ShapeEnum::Vertex, 4),
            (ShapeEnum::Edge, 7),
            (ShapeEnum::Wire, 4),
            (ShapeEnum::Face, 3),
            (ShapeEnum::Shell, 1),
            (ShapeEnum::Solid, 1),
            (ShapeEnum::CompSolid, 0),
            (ShapeEnum::Compound, 1),
            (ShapeEnum::Shape, 21),
        ],
    );
}

#[test]
fn Modalg4__Bug7626_2() {
    let sphere = CreateSphere(Point3::new(0., 0., 0.), 10.);
    let result = PerformCommon(&sphere, &rotated_box());
    ValidateResult(&result, 199.095, -1., false);
    expect_counts(
        &result,
        &[
            (ShapeEnum::Vertex, 2),
            (ShapeEnum::Edge, 3),
            (ShapeEnum::Wire, 3),
            (ShapeEnum::Face, 3),
            (ShapeEnum::Shell, 1),
            (ShapeEnum::Solid, 1),
            (ShapeEnum::CompSolid, 0),
            (ShapeEnum::Compound, 1),
            (ShapeEnum::Shape, 14),
        ],
    );
}
