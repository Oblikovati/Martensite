//! Original sweep regression requests, serialized observations and immutable byte locks.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/approx_sweep_corpus/inputs-00.txt"),
        include_str!("fixtures/approx_sweep_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/approx_sweep_corpus/inputs-01.txt"),
        include_str!("fixtures/approx_sweep_corpus/expected-01.txt"),
    ),
    (
        include_str!("fixtures/approx_sweep_corpus/inputs-02.txt"),
        include_str!("fixtures/approx_sweep_corpus/expected-02.txt"),
    ),
];
#[test]
fn fixture_bytes_and_all_rows_are_preserved() {
    assert_eq!(
        CHUNKS.iter().map(|(r, _)| r.lines().count()).sum::<usize>(),
        219
    );
    assert_eq!(
        CHUNKS.iter().map(|(_, r)| r.lines().count()).sum::<usize>(),
        219
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(r, _)| *r)),
        1648509502155294402
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, r)| *r)),
        8646830641522471383
    );
}
#[test]
fn axs_sweep_complete_family() {
    super::replay_family(CHUNKS, "axs_sweep", 219);
}
