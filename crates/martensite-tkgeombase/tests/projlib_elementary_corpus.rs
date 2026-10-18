//! Complete elementary projection families retain their original states and failures.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/projlib_elementary_corpus/inputs-00.txt"),
        include_str!("fixtures/projlib_elementary_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/projlib_elementary_corpus/inputs-01.txt"),
        include_str!("fixtures/projlib_elementary_corpus/expected-01.txt"),
    ),
    (
        include_str!("fixtures/projlib_elementary_corpus/inputs-02.txt"),
        include_str!("fixtures/projlib_elementary_corpus/expected-02.txt"),
    ),
    (
        include_str!("fixtures/projlib_elementary_corpus/inputs-03.txt"),
        include_str!("fixtures/projlib_elementary_corpus/expected-03.txt"),
    ),
    (
        include_str!("fixtures/projlib_elementary_corpus/inputs-04.txt"),
        include_str!("fixtures/projlib_elementary_corpus/expected-04.txt"),
    ),
    (
        include_str!("fixtures/projlib_elementary_corpus/inputs-05.txt"),
        include_str!("fixtures/projlib_elementary_corpus/expected-05.txt"),
    ),
];
#[test]
fn fixture_bytes_and_all_rows_are_preserved() {
    assert_eq!(
        CHUNKS.iter().map(|(r, _)| r.lines().count()).sum::<usize>(),
        1692
    );
    assert_eq!(
        CHUNKS.iter().map(|(_, r)| r.lines().count()).sum::<usize>(),
        1692
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(r, _)| *r)),
        17231690374916642748
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, r)| *r)),
        2515699925415089858
    );
}
#[test]
fn pl_elem_complete_family() {
    super::replay_family(CHUNKS, "pl_elem", 803);
}
#[test]
fn pl_static_complete_family() {
    super::replay_family(CHUNKS, "pl_static", 637);
}
#[test]
fn pl_prjfunc_complete_family() {
    super::replay_family(CHUNKS, "pl_prjfunc", 60);
}
#[test]
fn pl_prjresolve_complete_family() {
    super::replay_family(CHUNKS, "pl_prjresolve", 192);
}
#[path = "support/projlib_elementary_controls.rs"]
mod independent_controls;
