//! Native replay of the OCC12507 edge Boolean regression.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{CreateCylinder, PerformFuse, Translate};
use martensite_tkbrep::{
    topexp::{self, Explorer},
    topods::Shape,
    toptools::IndexedMapOfShape,
};
use martensite_tkg3d::top_abs::ShapeEnum;

fn third_edge(shape: &Shape) -> Shape {
    Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)
        .unwrap()
        .nth(2)
        .expect("expected third cylinder edge")
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
fn Modalg1__Bug12507() {
    let edge = third_edge(&CreateCylinder(3., 3.));
    let translated = Translate(&edge, 0., 6., 0.);
    let result = PerformFuse(&edge, &translated);
    expect_counts(
        &result,
        &[
            (ShapeEnum::Vertex, 3),
            (ShapeEnum::Edge, 4),
            (ShapeEnum::Wire, 0),
            (ShapeEnum::Face, 0),
            (ShapeEnum::Shell, 0),
            (ShapeEnum::Solid, 0),
            (ShapeEnum::CompSolid, 0),
            (ShapeEnum::Compound, 1),
            (ShapeEnum::Shape, 8),
        ],
    );
}
