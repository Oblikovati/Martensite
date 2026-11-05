//! Original prepared face and edge intersections with exact source assertions.
#![allow(dead_code)] // Separate graphical and non-graphical hosts select their source cases.
#[path = "../common_operation/mod.rs"]
mod common_operation;
mod validation;
use common_operation::{intersections, prepared, restored_shape, section};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkg3d::top_abs::ShapeEnum;

const COMPOUND: &[u8] = include_bytes!(
    "../../../../external/martensite-test-data/dataset-7.9.0/brep/bug23998_m1s.brep"
);
const FACES: [&[u8]; 2] = [
    include_bytes!(
        "../../../../external/martensite-test-data/dataset-7.9.0/brep/bug24003_fz33.brep"
    ),
    include_bytes!(
        "../../../../external/martensite-test-data/dataset-7.9.0/brep/bug24003_fz47.brep"
    ),
];
const EDGES: [&[u8]; 2] = [
    include_bytes!("../../../../external/martensite-test-data/dataset-7.9.0/brep/bug24811_e1.brep"),
    include_bytes!("../../../../external/martensite-test-data/dataset-7.9.0/brep/bug24811_e2.brep"),
];

/// Replay the closed face section; `checked_section()` returns the checked view shape.
pub fn checked_section() -> Shape {
    let compound = restored_shape(COMPOUND, ShapeEnum::Compound);
    let children: Vec<Shape> = ShapeIterator::new(&compound, true, true).collect();
    assert_eq!(
        children.len(),
        2,
        "bug23998: original compound contains two faces"
    );
    let result = section(&children[0], &children[1]);
    validation::check_section(&result);
    result
}

/// Exercise the original exception regression; `checked_face_intersections()`.
pub fn checked_face_intersections() {
    let first = restored_shape(FACES[0], ShapeEnum::Face);
    let second = restored_shape(FACES[1], ShapeEnum::Face);
    intersections(&first, &second);
}

/// Replay the ordered edge cut; `checked_cut()` checks the original endpoint tolerances.
pub fn checked_cut() -> Shape {
    let first = restored_shape(EDGES[0], ShapeEnum::Edge);
    let second = restored_shape(EDGES[1], ShapeEnum::Edge);
    let result = prepared(&first, &second, Operation::Cut);
    validation::check_cut(&result);
    result
}
