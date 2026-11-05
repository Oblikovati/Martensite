//! Original B4–B9 fixture order and unchanged area expectations.
#[path = "boolean_unconditional_case.rs"]
pub(super) mod boolean_unconditional_case;
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;
const FIXTURES: [&[u8]; 7] = [
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/f1"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/f41"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/so1"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/so2"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/so3"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/so4"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/so5"),
];
/// Restore and fuse the original pair; `checked_result(4)`.
pub fn checked_result(case: usize) -> Shape {
    assert!((4..=9).contains(&case), "case {case}: expected 4..=9");
    let index = case - 4;
    let [first, second] = [[0, 1], [2, 5], [2, 3], [3, 4], [5, 3], [6, 3]][index];
    let kind = if case == 4 {
        ShapeEnum::Face
    } else {
        ShapeEnum::Solid
    };
    boolean_unconditional_case::checked_fuse(
        [(FIXTURES[first], kind), (FIXTURES[second], kind)],
        [628.319, 942.478, 942.478, 458.149, 743.615, 430.878][index],
    )
}
