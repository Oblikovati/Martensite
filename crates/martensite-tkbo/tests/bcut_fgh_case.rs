//! Original F/G/H fixture restore order and direct Cut defaults.
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
/// Restore and cut the original F8 pair; `checked_f8()`.
pub fn checked_f8() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [
            fixture!("CTO900_pro8951a.rle", Solid),
            fixture!("pro8951d.rle", Solid),
        ],
        Operation::Cut,
        false,
        298872.0,
    )
}
/// Restore and cut the original F9 pair; `checked_f9()`.
pub fn checked_f9() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [
            fixture!("CTO900_fra60656-part.rle", Compound),
            fixture!("CTO900_fra60656-tool.rle", Solid),
        ],
        Operation::Cut,
        false,
        1.21042e+06,
    )
}
/// Restore and cut the original G4 pair; `checked_g4()`.
pub fn checked_g4() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [
            fixture!("CTO900_fra50089-part.rle", Solid),
            fixture!("fra50089-tool.rle", Solid),
        ],
        Operation::Cut,
        false,
        193593.0,
    )
}
/// Restore and cut the original G5 pair; `checked_g5()`.
pub fn checked_g5() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [
            fixture!("CTO900_pro8320-part.rle", Compound),
            fixture!("pro8320-tool.rle", Solid),
        ],
        Operation::Cut,
        false,
        13770.0,
    )
}
/// Restore and cut the original G9 pair; `checked_g9()`.
pub fn checked_g9() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [
            fixture!("cts21128c.rle", Solid),
            fixture!("cts21128d.rle", Solid),
        ],
        Operation::Cut,
        false,
        2814.01,
    )
}
/// Restore and cut the original H2 pair; `checked_h2()`.
pub fn checked_h2() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [
            fixture!("CTO900_cts20212-part.rle", Compound),
            fixture!("CTO900_cts20212-tool.rle", Solid),
        ],
        Operation::Cut,
        false,
        3038.87,
    )
}
/// Restore and cut the original H3 pair; `checked_h3()`.
pub fn checked_h3() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [
            fixture!("CTO900_cts20374-part.rle", Solid),
            fixture!("cts20374-tool.rle", Solid),
        ],
        Operation::Cut,
        false,
        472107.0,
    )
}
/// Restore and cut the original H4 pair; `checked_h4()`.
pub fn checked_h4() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [
            fixture!("CTO900_pro12953-part.rle", Compound),
            fixture!("CTO900_pro12953-tool.rle", Solid),
        ],
        Operation::Cut,
        false,
        78.7758,
    )
}
/// Restore and cut the original H5 pair; `checked_h5()`.
pub fn checked_h5() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [
            fixture!("CTO900_pro15323c.rle", Compound),
            fixture!("pro15323d.rle", Solid),
        ],
        Operation::Cut,
        false,
        43877.8,
    )
}
/// Restore and cut the original H9 pair; `checked_h9()`.
pub fn checked_h9() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [
            fixture!("CTO900_ger50084c.rle", Solid),
            fixture!("CTO900_ger50084d.rle", Solid),
        ],
        Operation::Cut,
        false,
        1290.02,
    )
}
