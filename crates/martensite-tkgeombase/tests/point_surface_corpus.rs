//! Complete original point/surface and swept-surface requests execute in-process.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/point_surface_corpus/inputs-00.txt"),
        include_str!("fixtures/point_surface_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/point_surface_corpus/inputs-01.txt"),
        include_str!("fixtures/point_surface_corpus/expected-01.txt"),
    ),
];
#[test]
fn fixture_bytes_and_all_rows_are_preserved() {
    assert_eq!(
        CHUNKS.iter().map(|(r, _)| r.lines().count()).sum::<usize>(),
        345
    );
    assert_eq!(
        CHUNKS.iter().map(|(_, r)| r.lines().count()).sum::<usize>(),
        345
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(r, _)| *r)),
        4124626241745330208
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, r)| *r)),
        2311706633719641325
    );
}
#[test]
fn ex_fpsn_complete_family() {
    super::replay_family(CHUNKS, "ex_fpsn", 30);
}
#[test]
fn ex_fpsd_complete_family() {
    super::replay_family(CHUNKS, "ex_fpsd", 30);
}
#[test]
fn ex_glps_complete_family() {
    super::replay_family(CHUNKS, "ex_glps", 30);
}
#[test]
fn ex_genps_complete_family() {
    super::replay_family(CHUNKS, "ex_genps", 75);
}
#[test]
fn ex_ps_complete_family() {
    super::replay_family(CHUNKS, "ex_ps", 129);
}
#[test]
fn ex_pexts_complete_family() {
    super::replay_family(CHUNKS, "ex_pexts", 26);
}
#[test]
fn ex_prevs_complete_family() {
    super::replay_family(CHUNKS, "ex_prevs", 25);
}
#[path = "support/point_surface_controls.rs"]
mod independent_controls;
