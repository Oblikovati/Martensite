//! Full projection adaptor and approximation requests retain all original observations.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/projlib_curves_corpus/inputs-00.txt"),
        include_str!("fixtures/projlib_curves_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/projlib_curves_corpus/inputs-01.txt"),
        include_str!("fixtures/projlib_curves_corpus/expected-01.txt"),
    ),
    (
        include_str!("fixtures/projlib_curves_corpus/inputs-02.txt"),
        include_str!("fixtures/projlib_curves_corpus/expected-02.txt"),
    ),
    (
        include_str!("fixtures/projlib_curves_corpus/inputs-03.txt"),
        include_str!("fixtures/projlib_curves_corpus/expected-03.txt"),
    ),
    (
        include_str!("fixtures/projlib_curves_corpus/inputs-04.txt"),
        include_str!("fixtures/projlib_curves_corpus/expected-04.txt"),
    ),
];
#[test]
fn fixture_bytes_and_all_rows_are_preserved() {
    assert_eq!(
        CHUNKS.iter().map(|(r, _)| r.lines().count()).sum::<usize>(),
        625
    );
    assert_eq!(
        CHUNKS.iter().map(|(_, r)| r.lines().count()).sum::<usize>(),
        625
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(r, _)| *r)),
        16034883146323193756
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, r)| *r)),
        7596732264780189353
    );
}
#[test]
fn pl_on_plane_complete_family() {
    super::replay_family(CHUNKS, "pl_on_plane", 209);
}
#[test]
fn pl_approx_complete_family() {
    super::replay_family(CHUNKS, "pl_approx", 93);
}
#[test]
fn pl_projected_complete_family() {
    super::replay_family(CHUNKS, "pl_projected", 130);
}
#[test]
fn pl_polar_complete_family() {
    super::replay_family(CHUNKS, "pl_polar", 164);
}
#[test]
fn pl_comp_complete_family() {
    super::replay_family(CHUNKS, "pl_comp", 29);
}
#[path = "support/projlib_curves_controls.rs"]
mod independent_controls;
