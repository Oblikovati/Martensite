//! Complete static geometry requests retain their input and retained byte streams.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/geomlib_static_corpus/inputs-00.txt"),
        include_str!("fixtures/geomlib_static_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/geomlib_static_corpus/inputs-01.txt"),
        include_str!("fixtures/geomlib_static_corpus/expected-01.txt"),
    ),
    (
        include_str!("fixtures/geomlib_static_corpus/inputs-02.txt"),
        include_str!("fixtures/geomlib_static_corpus/expected-02.txt"),
    ),
];
#[test]
fn fixture_bytes_and_all_rows_are_preserved() {
    assert_eq!(
        CHUNKS.iter().map(|(r, _)| r.lines().count()).sum::<usize>(),
        704
    );
    assert_eq!(
        CHUNKS.iter().map(|(_, r)| r.lines().count()).sum::<usize>(),
        704
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(r, _)| *r)),
        4008229675160113554
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, r)| *r)),
        11642113300266243956
    );
}
#[test]
fn gl_to3d_complete_family() {
    super::replay_family(CHUNKS, "gl_to3d", 30);
}
#[test]
fn gl_gtransform_complete_family() {
    super::replay_family(CHUNKS, "gl_gtransform", 105);
}
#[test]
fn gl_same_range_complete_family() {
    super::replay_family(CHUNKS, "gl_same_range", 112);
}
#[test]
fn gl_build_curve3d_complete_family() {
    super::replay_family(CHUNKS, "gl_build_curve3d", 33);
}
#[test]
fn gl_adjust_complete_family() {
    super::replay_family(CHUNKS, "gl_adjust", 18);
}
#[test]
fn gl_extend_curve_complete_family() {
    super::replay_family(CHUNKS, "gl_extend_curve", 81);
}
#[test]
fn gl_extend_surf_complete_family() {
    super::replay_family(CHUNKS, "gl_extend_surf", 144);
}
#[test]
fn gl_eval_max_complete_family() {
    super::replay_family(CHUNKS, "gl_eval_max", 17);
}
#[test]
fn gl_cancel_denom_complete_family() {
    super::replay_family(CHUNKS, "gl_cancel_denom", 8);
}
#[test]
fn gl_norm_estim_complete_family() {
    super::replay_family(CHUNKS, "gl_norm_estim", 24);
}
#[test]
fn gl_is_closed_complete_family() {
    super::replay_family(CHUNKS, "gl_is_closed", 64);
}
#[test]
fn gl_iso_line_complete_family() {
    super::replay_family(CHUNKS, "gl_iso_line", 68);
}
#[path = "support/geomlib_static_controls.rs"]
mod independent_controls;
