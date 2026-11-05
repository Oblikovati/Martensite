//! Original B1–B3 face fixtures and expected area assertions.
#[path = "boolean_unconditional_case.rs"]
mod boolean_unconditional_case;
pub use boolean_unconditional_case::{check_end, check_view_result};
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;
const FIXTURES: [&[u8]; 4] = [
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/f2"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/f3"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/f4"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/f5"),
];
/// Preserve source operand order and unconditional fusion; `checked_result(1)`.
pub fn checked_result(case: usize) -> Shape {
    assert!((1..=3).contains(&case), "case {case}: expected 1..=3");
    let [first, second] = [[0, 1], [1, 2], [1, 3]][case - 1];
    boolean_unconditional_case::checked_fuse(
        [
            (FIXTURES[first], ShapeEnum::Face),
            (FIXTURES[second], ShapeEnum::Face),
        ],
        [200.713, 392.699, 157.08][case - 1],
    )
}
