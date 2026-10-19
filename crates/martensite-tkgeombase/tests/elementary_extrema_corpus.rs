//! Complete elementary extrema families preserve original requests and accessor/error order.
const CHUNKS: &[(&str, &str)] = &[(
    include_str!("fixtures/elementary_extrema_corpus/inputs.txt"),
    include_str!("fixtures/elementary_extrema_corpus/native.txt"),
)];
#[test]
fn fixture_bytes_and_row_counts_are_locked() {
    assert_eq!(CHUNKS[0].0.lines().count(), 212);
    assert_eq!(CHUNKS[0].1.lines().count(), 212);
    assert_eq!(
        super::fixture_hash([CHUNKS[0].0].into_iter()),
        11972031338667257913
    );
    assert_eq!(
        super::fixture_hash([CHUNKS[0].1].into_iter()),
        788904924921980280
    );
}

#[test]
fn ex_pon_complete_family() {
    super::replay_family(CHUNKS, "ex_pon", 2);
}

#[test]
fn ex_pelc_complete_family() {
    super::replay_family(CHUNKS, "ex_pelc", 33);
}

#[test]
fn ex_pelc2d_complete_family() {
    super::replay_family(CHUNKS, "ex_pelc2d", 18);
}

#[test]
fn ex_elc_complete_family() {
    super::replay_family(CHUNKS, "ex_elc", 71);
}

#[test]
fn ex_elc2d_complete_family() {
    super::replay_family(CHUNKS, "ex_elc2d", 49);
}

#[test]
fn ex_pels_complete_family() {
    super::replay_family(CHUNKS, "ex_pels", 24);
}

#[test]
fn ex_elcs_complete_family() {
    super::replay_family(CHUNKS, "ex_elcs", 14);
}

#[test]
fn ex_elss_complete_family() {
    super::replay_family(CHUNKS, "ex_elss", 1);
}

#[path = "support/elementary_extrema_controls.rs"]
mod independent_controls;
