//! Original A1–A3 inputs and expected properties.
#[path = "boolean_unconditional_case.rs"]
mod boolean_unconditional_case;
pub use boolean_unconditional_case::{check_end, check_view_result};
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;
const FIXTURES: [[&[u8]; 2]; 3] = [
    [
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/CIN900_cts20hlk.rle"
        ),
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/CIN900_cts20hll.rle"
        ),
    ],
    [
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/fra61083a.rle"),
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/CIN900_fra61hlt.rle"
        ),
    ],
    [
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/CIN900_cts21hlo.rle"
        ),
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/CIN900_cts21hlp.rle"
        ),
    ],
];
/// Run source fusion and checkprops before its view; `checked_result(1)`.
pub fn checked_result(case: usize) -> Shape {
    assert!((1..=3).contains(&case), "case {case}: expected 1..=3");
    let first_kind = if case == 3 {
        ShapeEnum::Compound
    } else {
        ShapeEnum::Solid
    };
    boolean_unconditional_case::checked_fuse(
        [
            (FIXTURES[case - 1][0], first_kind),
            (FIXTURES[case - 1][1], ShapeEnum::Solid),
        ],
        [43552.5, 309.521, 164274.][case - 1],
    )
}
