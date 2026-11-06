//! Original F/G fixture pairs and independent regression declarations.
#[path = "boolean_unconditional_case.rs"]
pub(super) mod boolean_unconditional_case;
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;
/// Restore the original F3 pair and assert its area; `checked_f3()`.
pub fn checked_f3() -> Shape {
    boolean_unconditional_case::checked_fuse(
        [
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO900_pro6110a.rle"
                ),
                ShapeEnum::Compound,
            ),
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO900_pro6110b.rle"
                ),
                ShapeEnum::Solid,
            ),
        ],
        1.7431e+06,
    )
}
/// Restore the original F4 pair and assert its area; `checked_f4()`.
pub fn checked_f4() -> Shape {
    boolean_unconditional_case::checked_fuse(
        [
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO900_pro10494b.rle"
                ),
                ShapeEnum::Solid,
            ),
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO900_pro10494c.rle"
                ),
                ShapeEnum::Solid,
            ),
        ],
        152689.0,
    )
}
/// Restore the original F9 pair and assert its area; `checked_f9()`.
pub fn checked_f9() -> Shape {
    boolean_unconditional_case::checked_fuse(
        [
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO900_cts20124a.rle"
                ),
                ShapeEnum::Solid,
            ),
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO900_cts20124b.rle"
                ),
                ShapeEnum::Solid,
            ),
        ],
        49957.7,
    )
}
/// Restore the original G3 pair and assert its area; `checked_g3()`.
pub fn checked_g3() -> Shape {
    // G3 intentionally repeats F9; its separate test retains the original declaration.
    checked_f9()
}
/// Restore the original G4 pair and assert its area; `checked_g4()`.
pub fn checked_g4() -> Shape {
    boolean_unconditional_case::checked_fuse(
        [
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO901_cts20171_base.rle"
                ),
                ShapeEnum::Compound,
            ),
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO901_cts20171_outil.rle"
                ),
                ShapeEnum::Solid,
            ),
        ],
        47886.5,
    )
}
/// Restore the original G5 pair and assert its area; `checked_g5()`.
pub fn checked_g5() -> Shape {
    boolean_unconditional_case::checked_fuse(
        [
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO901_cts20204_base.rle"
                ),
                ShapeEnum::Compound,
            ),
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO901_cts20204_outil.rle"
                ),
                ShapeEnum::Solid,
            ),
        ],
        541480.0,
    )
}
/// Restore the original G6 pair and assert its area; `checked_g6()`.
pub fn checked_g6() -> Shape {
    boolean_unconditional_case::checked_fuse(
        [
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO901_cts20205-base.rle"
                ),
                ShapeEnum::Compound,
            ),
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO901_cts20205-outil.rle"
                ),
                ShapeEnum::Solid,
            ),
        ],
        542196.0,
    )
}
/// Restore the original G7 pair and assert its area; `checked_g7()`.
pub fn checked_g7() -> Shape {
    boolean_unconditional_case::checked_fuse(
        [
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO901_cts20216_base.rle"
                ),
                ShapeEnum::Compound,
            ),
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO901_cts20216_outil.rle"
                ),
                ShapeEnum::Solid,
            ),
        ],
        2005.99,
    )
}
/// Restore the original G8 pair and assert its area; `checked_g8()`.
pub fn checked_g8() -> Shape {
    boolean_unconditional_case::checked_fuse(
        [
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO901_cts20222_base.rle"
                ),
                ShapeEnum::Compound,
            ),
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO901_cts20222_outil.rle"
                ),
                ShapeEnum::Solid,
            ),
        ],
        3342.49,
    )
}
/// Restore the original G9 pair and assert its area; `checked_g9()`.
pub fn checked_g9() -> Shape {
    boolean_unconditional_case::checked_fuse(
        [
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO902_cts20452a.rle"
                ),
                ShapeEnum::Solid,
            ),
            (
                include_bytes!(
                    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO902_cts20452b.rle"
                ),
                ShapeEnum::Solid,
            ),
        ],
        538530.0,
    )
}
