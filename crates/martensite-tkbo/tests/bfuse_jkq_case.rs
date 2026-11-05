//! Original J/K/Q operand order and independent source declarations.
#[path = "boolean_unconditional_case.rs"]
pub(super) mod boolean_unconditional_case;
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;
/// Restore and check the original J4 pair; `checked_j4()`.
pub fn checked_j4() -> Shape {
    boolean_unconditional_case::checked_fuse(
        [
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/cts20549a.rle"
                ),
                ShapeEnum::Solid,
            ),
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts20549b.rle"
                ),
                ShapeEnum::Solid,
            ),
        ],
        22561.3,
    )
}
/// Restore and check the original J7 pair; `checked_j7()`.
pub fn checked_j7() -> Shape {
    // J7 intentionally repeats J4 while retaining its independent declaration.
    checked_j4()
}
/// Restore and check the original J8 pair; `checked_j8()`.
pub fn checked_j8() -> Shape {
    boolean_unconditional_case::checked_fuse(
        [
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/cts20751a.rle"
                ),
                ShapeEnum::Solid,
            ),
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts20751b.rle"
                ),
                ShapeEnum::Compound,
            ),
        ],
        52140.6,
    )
}
/// Restore and check the original J9 pair; `checked_j9()`.
pub fn checked_j9() -> Shape {
    boolean_unconditional_case::checked_fuse(
        [
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts20769a.rle"
                ),
                ShapeEnum::Compound,
            ),
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/cts20769a.brep"
                ),
                ShapeEnum::Solid,
            ),
        ],
        92385.1,
    )
}
/// Restore and check the original K1 pair; `checked_k1()`.
pub fn checked_k1() -> Shape {
    boolean_unconditional_case::checked_fuse(
        [
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts20370-part.rle"
                ),
                ShapeEnum::Solid,
            ),
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/cts20370-tool.rle"
                ),
                ShapeEnum::Solid,
            ),
        ],
        261265.0,
    )
}
/// Restore and check the original K2 pair; `checked_k2()`.
pub fn checked_k2() -> Shape {
    boolean_unconditional_case::checked_fuse(
        [
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts20578a.rle"
                ),
                ShapeEnum::Compound,
            ),
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts20578b.rle"
                ),
                ShapeEnum::Solid,
            ),
        ],
        759758.0,
    )
}
/// Restore and check the original K5 pair; `checked_k5()`.
pub fn checked_k5() -> Shape {
    boolean_unconditional_case::checked_fuse(
        [
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_buc60116a.rle"
                ),
                ShapeEnum::Solid,
            ),
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/buc60116b.rle"
                ),
                ShapeEnum::Solid,
            ),
        ],
        224496.0,
    )
}
/// Restore and check the original K6 pair; `checked_k6()`.
pub fn checked_k6() -> Shape {
    boolean_unconditional_case::checked_fuse(
        [
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/cts20973a.rle"
                ),
                ShapeEnum::Solid,
            ),
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/cts20973b.rle"
                ),
                ShapeEnum::Solid,
            ),
        ],
        324489.0,
    )
}
/// Restore and check the original Q6 pair; `checked_q6()`.
pub fn checked_q6() -> Shape {
    boolean_unconditional_case::checked_fuse(
        [
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/cts20769a.brep"
                ),
                ShapeEnum::Solid,
            ),
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_cts20769a.rle"
                ),
                ShapeEnum::Compound,
            ),
        ],
        92385.1,
    )
}
