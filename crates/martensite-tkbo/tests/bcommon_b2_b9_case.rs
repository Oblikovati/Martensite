//! Original solid operand order and direct Common defaults.
#[path = "boolean_unconditional_case.rs"]
pub(super) mod boolean_unconditional_case;
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;
macro_rules! fixture {
    ($file:literal) => {
        (
            include_bytes!(concat!(
                "../../../external/martensite-test-data/dataset-7.9.0/brep/",
                $file
            ))
            .as_slice(),
            ShapeEnum::Solid,
        )
    };
}
/// Preserve original B2 restore order and Common; `checked_b2()`.
pub fn checked_b2() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [fixture!("so1"), fixture!("so4")],
        Operation::Common,
        false,
        549.779,
    )
}
/// Preserve original B3 restore order and Common; `checked_b3()`.
pub fn checked_b3() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [fixture!("so1"), fixture!("so2")],
        Operation::Common,
        false,
        392.699,
    )
}
/// Preserve original B4 restore order and Common; `checked_b4()`.
pub fn checked_b4() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [fixture!("so2"), fixture!("so3")],
        Operation::Common,
        false,
        327.249,
    )
}
/// Preserve original B5 restore order and Common; `checked_b5()`.
pub fn checked_b5() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [fixture!("so4"), fixture!("so2")],
        Operation::Common,
        false,
        198.863,
    )
}
/// Preserve original B6 restore order and Common; `checked_b6()`.
pub fn checked_b6() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [fixture!("so5"), fixture!("so2")],
        Operation::Common,
        false,
        160.684,
    )
}
/// Preserve original B7 restore order and Common; `checked_b7()`.
pub fn checked_b7() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [fixture!("so3"), fixture!("so7")],
        Operation::Common,
        false,
        340.339,
    )
}
/// Preserve original B8 restore order and Common; `checked_b8()`.
pub fn checked_b8() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [fixture!("so2"), fixture!("so7")],
        Operation::Common,
        false,
        319.364,
    )
}
/// Preserve original B9 restore order and Common; `checked_b9()`.
pub fn checked_b9() -> Shape {
    boolean_unconditional_case::checked_fixture_operation(
        [fixture!("so2"), fixture!("so6")],
        Operation::Common,
        false,
        315.548,
    )
}
