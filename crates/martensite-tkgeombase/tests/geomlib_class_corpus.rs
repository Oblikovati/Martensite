//! Complete GeomLib class families with frozen inputs and reviewed retained observations.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/geomlib_class_corpus/inputs-00.txt"),
        include_str!("fixtures/geomlib_class_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/geomlib_class_corpus/inputs-01.txt"),
        include_str!("fixtures/geomlib_class_corpus/expected-01.txt"),
    ),
];
#[test]
fn fixture_bytes_and_all_rows_are_preserved() {
    assert_eq!(
        CHUNKS.iter().map(|(r, _)| r.lines().count()).sum::<usize>(),
        332
    );
    assert_eq!(
        CHUNKS.iter().map(|(_, r)| r.lines().count()).sum::<usize>(),
        332
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(r, _)| *r)),
        10811448573312533798
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, r)| *r)),
        210354488069086800
    );
}
#[test]
fn gl_check_bspline_complete_family() {
    super::replay_family(CHUNKS, "gl_check_bspline", 22);
}
#[test]
fn gl_cos_complete_family() {
    super::replay_family(CHUNKS, "gl_cos", 56);
}
#[test]
fn gl_denom_complete_family() {
    super::replay_family(CHUNKS, "gl_denom", 8);
}
#[test]
fn gl_interpolate_complete_family() {
    super::replay_family(CHUNKS, "gl_interpolate", 10);
}
#[test]
fn gl_is_planar_complete_family() {
    super::replay_family(CHUNKS, "gl_is_planar", 18);
}
#[test]
fn gl_tool_complete_family() {
    super::replay_family(CHUNKS, "gl_tool", 132);
}
#[test]
fn gl_deviation_complete_family() {
    super::replay_family(CHUNKS, "gl_deviation", 66);
}
#[test]
fn gl_make_curve_complete_family() {
    super::replay_family(CHUNKS, "gl_make_curve", 20);
}
#[path = "support/geomlib_class_controls.rs"]
mod independent_controls;
