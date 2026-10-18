//! Complete classic point/curve families retain every constructor and reused-state request.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/classic_point_curve_corpus/inputs-00.txt"),
        include_str!("fixtures/classic_point_curve_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/classic_point_curve_corpus/inputs-01.txt"),
        include_str!("fixtures/classic_point_curve_corpus/expected-01.txt"),
    ),
    (
        include_str!("fixtures/classic_point_curve_corpus/inputs-02.txt"),
        include_str!("fixtures/classic_point_curve_corpus/expected-02.txt"),
    ),
    (
        include_str!("fixtures/classic_point_curve_corpus/inputs-03.txt"),
        include_str!("fixtures/classic_point_curve_corpus/expected-03.txt"),
    ),
];
#[test]
fn fixture_byte_hashes_and_all_rows_are_preserved() {
    assert_eq!(
        CHUNKS.iter().map(|(r, _)| r.lines().count()).sum::<usize>(),
        1148
    );
    assert_eq!(
        CHUNKS.iter().map(|(_, a)| a.lines().count()).sum::<usize>(),
        1148
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(r, _)| *r)),
        7021838840433962061
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, a)| *a)),
        4593067153454659782
    );
}

#[test]
fn ex_pc_complete_family() {
    super::replay_family(CHUNKS, "ex_pc", 158);
}

#[test]
fn ex_pc2d_complete_family() {
    super::replay_family(CHUNKS, "ex_pc2d", 192);
}

#[test]
fn ex_locpc_complete_family() {
    super::replay_family(CHUNKS, "ex_locpc", 71);
}

#[test]
fn ex_locpc_owned_complete_family() {
    super::replay_family(CHUNKS, "ex_locpc_owned", 71);
}

#[test]
fn ex_locpc2d_complete_family() {
    super::replay_family(CHUNKS, "ex_locpc2d", 83);
}

#[test]
fn ex_locpc2d_owned_complete_family() {
    super::replay_family(CHUNKS, "ex_locpc2d_owned", 83);
}

#[test]
fn ex_genpc_complete_family() {
    super::replay_family(CHUNKS, "ex_genpc", 42);
}

#[test]
fn ex_genpc2d_complete_family() {
    super::replay_family(CHUNKS, "ex_genpc2d", 49);
}

#[test]
fn ex_pcf_complete_family() {
    super::replay_family(CHUNKS, "ex_pcf", 71);
}

#[test]
fn ex_pcf2d_complete_family() {
    super::replay_family(CHUNKS, "ex_pcf2d", 83);
}

#[test]
fn ex_cloc_complete_family() {
    super::replay_family(CHUNKS, "ex_cloc", 42);
}

#[test]
fn ex_cloc2d_complete_family() {
    super::replay_family(CHUNKS, "ex_cloc2d", 49);
}

#[test]
fn ex_ctool_complete_family() {
    super::replay_family(CHUNKS, "ex_ctool", 71);
}

#[test]
fn ex_ctool2d_complete_family() {
    super::replay_family(CHUNKS, "ex_ctool2d", 83);
}

#[path = "support/classic_point_curve_controls.rs"]
mod independent_controls;
