//! Native replay of the OCC25715 Boolean cut regression.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{CreateCylinder, PerformCut, RotateAroundAxis, Translate};
use martensite_tkbrep::{topexp, topods::Shape, toptools::IndexedMapOfShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;

fn transformed_cylinders() -> (Shape, Shape) {
    let mut first = CreateCylinder(0.069, 0.6);
    RotateAroundAxis(&mut first, Point3::new(0., 0., 0.), [0., 0., 1.], 90.);
    let mut second = CreateCylinder(0.024, 0.4);
    RotateAroundAxis(&mut second, Point3::new(0., 0., 0.), [0., 1., 0.], 90.);
    RotateAroundAxis(&mut second, Point3::new(0., 0., 0.), [1., 0., 0.], -45.);
    let second = Translate(&second, -0.2, 0., 0.48);
    (first, second)
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
fn Modalg5__Bug25715_1() {
    let (first, second) = transformed_cylinders();
    let result = PerformCut(&first, &second);
    expect_counts(
        &result,
        &[
            (ShapeEnum::Vertex, 4),
            (ShapeEnum::Edge, 6),
            (ShapeEnum::Wire, 6),
            (ShapeEnum::Face, 4),
            (ShapeEnum::Shell, 1),
            (ShapeEnum::Solid, 1),
            (ShapeEnum::CompSolid, 0),
            (ShapeEnum::Compound, 1),
            (ShapeEnum::Shape, 23),
        ],
    );
}
