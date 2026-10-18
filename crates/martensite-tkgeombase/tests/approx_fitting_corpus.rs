//! Complete original Approx multiline and analytic fitting corpus.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/approx_fitting_corpus/inputs-00.txt"),
        include_str!("fixtures/approx_fitting_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/approx_fitting_corpus/inputs-01.txt"),
        include_str!("fixtures/approx_fitting_corpus/expected-01.txt"),
    ),
    (
        include_str!("fixtures/approx_fitting_corpus/inputs-02.txt"),
        include_str!("fixtures/approx_fitting_corpus/expected-02.txt"),
    ),
    (
        include_str!("fixtures/approx_fitting_corpus/inputs-03.txt"),
        include_str!("fixtures/approx_fitting_corpus/expected-03.txt"),
    ),
    (
        include_str!("fixtures/approx_fitting_corpus/inputs-04.txt"),
        include_str!("fixtures/approx_fitting_corpus/expected-04.txt"),
    ),
    (
        include_str!("fixtures/approx_fitting_corpus/inputs-05.txt"),
        include_str!("fixtures/approx_fitting_corpus/expected-05.txt"),
    ),
    (
        include_str!("fixtures/approx_fitting_corpus/inputs-06.txt"),
        include_str!("fixtures/approx_fitting_corpus/expected-06.txt"),
    ),
];
#[test]
fn fixture_bytes_and_all_rows_are_preserved() {
    assert_eq!(
        CHUNKS.iter().map(|(r, _)| r.lines().count()).sum::<usize>(),
        1324
    );
    assert_eq!(
        CHUNKS.iter().map(|(_, r)| r.lines().count()).sum::<usize>(),
        1324
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(r, _)| *r)),
        15354305192795642763
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, r)| *r)),
        12369838560633273088
    );
}
#[test]
fn ax_compute_complete_family() {
    super::replay_family(CHUNKS, "ax_compute", 741);
}
#[test]
fn ax_bspl_complete_family() {
    super::replay_family(CHUNKS, "ax_bspl", 340);
}
#[test]
fn ax_fit_complete_family() {
    super::replay_family(CHUNKS, "ax_fit", 120);
}
#[test]
fn ax_mcurves_complete_family() {
    super::replay_family(CHUNKS, "ax_mcurves", 123);
}
#[path = "support/approx_fitting_controls.rs"]
mod independent_controls;
