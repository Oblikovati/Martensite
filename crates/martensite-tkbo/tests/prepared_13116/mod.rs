//! Original OCC13116 two-stage Boolean operations and every non-view assertion.
#[path = "../common_operation/mod.rs"]
// These cases select prepared operations, not direct command entry points.
mod common_operation;
mod validation;
use common_operation::{prepared, restored_shape};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;

const OPERATIONS: [Operation; 4] = [
    Operation::Fuse,
    Operation::Common,
    Operation::Cut,
    Operation::Cut21,
];

const FIRST: &[u8] = include_bytes!(
    "../../../../external/martensite-test-data/dataset-7.9.0/brep/OCC13116_sh1.brep"
);
const SECOND: &[u8] = include_bytes!(
    "../../../../external/martensite-test-data/dataset-7.9.0/brep/OCC13116_sh2.brep"
);

/// Execute the source operation and checks; `checked_result(1)` replays the fuse case.
pub fn checked_result(case: usize) -> Shape {
    assert!(
        (1..=4).contains(&case),
        "OCC13116 case {case}: expected 1..=4"
    );
    let kind = OPERATIONS[case - 1];
    let first = restored_shape(FIRST, ShapeEnum::Solid);
    let second = restored_shape(SECOND, ShapeEnum::Solid);
    let result = prepared(&first, &second, kind);
    validation::check_result(&result, case);
    result
}
