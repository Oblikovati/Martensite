//! Original modalg_5 face-section operations and assertions.
#[path = "../boolean_direct_section.rs"]
mod boolean_direct_section;
#[path = "../common_operation/mod.rs"]
mod common_operation;
mod validation;
use common_operation::restored_shape;
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;

const FIXTURES: [(&[u8], &[u8]); 3] = [
    (
        include_bytes!(
            "../../../../external/martensite-test-data/dataset-7.9.0/brep/bug23985_b1.brep"
        ),
        include_bytes!(
            "../../../../external/martensite-test-data/dataset-7.9.0/brep/bug23985_b2.brep"
        ),
    ),
    (
        include_bytes!(
            "../../../../external/martensite-test-data/dataset-7.9.0/brep/bug24496_face1.brep"
        ),
        include_bytes!(
            "../../../../external/martensite-test-data/dataset-7.9.0/brep/bug24496_face2.brep"
        ),
    ),
    (
        include_bytes!(
            "../../../../external/martensite-test-data/dataset-7.9.0/brep/bug24519_cylinder.brep"
        ),
        include_bytes!(
            "../../../../external/martensite-test-data/dataset-7.9.0/brep/bug24519_plane.brep"
        ),
    ),
];

fn inputs(case: usize) -> (&'static [u8], &'static [u8]) {
    let index = match case {
        23985 => 0,
        24496 => 1,
        24519 => 2,
        _ => panic!("face section case {case}: expected 23985, 24496 or 24519"),
    };
    FIXTURES[index]
}

/// Replay a face-section regression; `checked_result(24496)` checks both intersections.
pub fn checked_result(case: usize) -> Shape {
    let (first, second) = inputs(case);
    let first = restored_shape(first, ShapeEnum::Face);
    let second = restored_shape(second, ShapeEnum::Face);
    let result = boolean_direct_section::direct_section(&first, &second);
    validation::check_result(&result, case);
    result
}
