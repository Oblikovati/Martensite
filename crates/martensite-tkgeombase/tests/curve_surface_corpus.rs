//! Complete final Extrema request families preserve source objectives, state and failures.
const CHUNKS: &[(&str, &str)] = &[(
    include_str!("fixtures/curve_surface_corpus/inputs-00.txt"),
    include_str!("fixtures/curve_surface_corpus/expected-00.txt"),
)];
#[test]
fn fixture_bytes_and_all_rows_are_preserved() {
    assert_eq!(CHUNKS[0].0.lines().count(), 273);
    assert_eq!(CHUNKS[0].1.lines().count(), 273);
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(r, _)| *r)),
        13628839893898992081
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, r)| *r)),
        11659167731844292303
    );
}
#[test]
fn ex_fecs_complete_family() {
    super::replay_family(CHUNKS, "ex_fecs", 26);
}
#[test]
fn ex_gofcs_complete_family() {
    super::replay_family(CHUNKS, "ex_gofcs", 26);
}
#[test]
fn ex_gofconics_complete_family() {
    super::replay_family(CHUNKS, "ex_gofconics", 23);
}
#[test]
fn ex_gofcquad_complete_family() {
    super::replay_family(CHUNKS, "ex_gofcquad", 15);
}
#[test]
fn ex_genecs_complete_family() {
    super::replay_family(CHUNKS, "ex_genecs", 28);
}
#[test]
fn ex_cs_complete_family() {
    super::replay_family(CHUNKS, "ex_cs", 92);
}
#[test]
fn ex_fess_complete_family() {
    super::replay_family(CHUNKS, "ex_fess", 10);
}
#[test]
fn ex_genss_complete_family() {
    super::replay_family(CHUNKS, "ex_genss", 15);
}
#[test]
fn ex_ss_complete_family() {
    super::replay_family(CHUNKS, "ex_ss", 38);
}
#[path = "support/curve_surface_controls.rs"]
mod independent_controls;
