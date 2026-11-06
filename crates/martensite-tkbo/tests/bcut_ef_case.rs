//! Original E/F fixture restore order and direct Cut defaults.
#[path = "boolean_unconditional_case.rs"]
pub(super) mod boolean_unconditional_case;
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;
macro_rules! fixture {
    ($name:literal, $kind:ident) => {
        (
            include_bytes!(concat!(
                "../../../external/martensite-test-data/dataset-7.9.0/brep/",
                $name
            ))
            .as_slice(),
            ShapeEnum::$kind,
        )
    };
}
/// Restore and cut the original E4 pair; `checked_e4()`.
pub fn checked_e4() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [
            fixture!("CTO900_pro6066a.rle", Compound),
            fixture!("CTO900_pro6066b.rle", Solid),
        ],
        Operation::Cut,
        false,
        0.264921,
    )
}
/// Restore and cut the original E6 pair; `checked_e6()`.
pub fn checked_e6() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [
            fixture!("CTO900_ger60043-part.rle", Compound),
            fixture!("CTO900_ger60043-tool.rle", Solid),
        ],
        Operation::Cut,
        false,
        17135.1,
    )
}
/// Restore and cut the original E8 pair; `checked_e8()`.
pub fn checked_e8() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [
            fixture!("CTO900_jap60038-part.rle", Solid),
            fixture!("jap60038-tool.rle", Solid),
        ],
        Operation::Cut,
        false,
        173510.0,
    )
}
/// Restore and cut the original E9 pair; `checked_e9()`.
pub fn checked_e9() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [
            fixture!("CTO900_pro6066-part.rle", Compound),
            fixture!("CTO900_pro6066-tool.rle", Solid),
        ],
        Operation::Cut,
        false,
        570249.0,
    )
}
/// Restore and cut the original F1 pair; `checked_f1()`.
pub fn checked_f1() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [
            fixture!("cts17785-part.rle", Solid),
            fixture!("cts17785-tool.rle", Solid),
        ],
        Operation::Cut,
        false,
        56064.6,
    )
}
/// Restore and cut the original F3 pair; `checked_f3()`.
pub fn checked_f3() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [
            fixture!("CTO900_cts18140-part.rle", Compound),
            fixture!("CTO900_cts18140-tool.rle", Solid),
        ],
        Operation::Cut,
        false,
        92062.6,
    )
}
