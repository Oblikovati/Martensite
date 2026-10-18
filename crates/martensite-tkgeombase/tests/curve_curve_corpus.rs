//! Complete curve/curve requests retain defaults, reused state and source-defined objective quirks.
const CHUNKS: &[(&str, &str)] = &[(
    include_str!("fixtures/curve_curve_corpus/inputs-00.txt"),
    include_str!("fixtures/curve_curve_corpus/expected-00.txt"),
)];
#[test]
fn fixture_hashes_and_all_rows_are_preserved() {
    assert_eq!(CHUNKS[0].0.lines().count(), 110);
    assert_eq!(CHUNKS[0].1.lines().count(), 110);
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(r, _)| *r)),
        2009407977930174291
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, r)| *r)),
        9146267533082118241
    );
}

#[test]
fn ex_cc_complete_family() {
    super::replay_family(CHUNKS, "ex_cc", 31);
}

#[test]
fn ex_cc2d_complete_family() {
    super::replay_family(CHUNKS, "ex_cc2d", 26);
}

#[test]
fn ex_ecc_complete_family() {
    super::replay_family(CHUNKS, "ex_ecc", 10);
}

#[test]
fn ex_ecc2d_complete_family() {
    super::replay_family(CHUNKS, "ex_ecc2d", 8);
}

#[test]
fn ex_locecc_complete_family() {
    super::replay_family(CHUNKS, "ex_locecc", 11);
}

#[test]
fn ex_fcc_complete_family() {
    super::replay_family(CHUNKS, "ex_fcc", 9);
}

#[test]
fn ex_gofcc_complete_family() {
    super::replay_family(CHUNKS, "ex_gofcc", 8);
}

#[test]
fn ex_gofcc2d_complete_family() {
    super::replay_family(CHUNKS, "ex_gofcc2d", 7);
}

#[path = "support/curve_curve_controls.rs"]
mod independent_controls;
