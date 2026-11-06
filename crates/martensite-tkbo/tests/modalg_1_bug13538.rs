//! Native replay of the OCC13538 shell Boolean regression.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{CreateBox, PerformCommon, Translate, ValidateResult};
use martensite_tkbrep::{
    topexp::{self, Explorer},
    topods::Shape,
    toptools::IndexedMapOfShape,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;

#[allow(clippy::iter_nth_zero)] // The explorer has its own `next`, which advances without returning.
fn first_shell(shape: &Shape) -> Shape {
    Explorer::new(shape, ShapeEnum::Shell, ShapeEnum::Shape)
        .unwrap()
        .nth(0)
        .expect("expected box shell")
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
fn Modalg1__Bug13538() {
    let first = CreateBox(Point3::new(0., 0., 0.), 100., 100., 100.);
    let second = Translate(
        &CreateBox(Point3::new(0., 0., 0.), 100., 100., 100.),
        0.,
        0.,
        100.,
    );
    let result = PerformCommon(&first_shell(&first), &first_shell(&second));
    ValidateResult(&result, 10000., -1., false);
    expect_counts(
        &result,
        &[
            (ShapeEnum::Vertex, 4),
            (ShapeEnum::Edge, 4),
            (ShapeEnum::Wire, 1),
            (ShapeEnum::Face, 1),
            (ShapeEnum::Shell, 1),
            (ShapeEnum::Solid, 0),
            (ShapeEnum::CompSolid, 0),
            (ShapeEnum::Compound, 1),
            (ShapeEnum::Shape, 12),
        ],
    );
}
