//! Complete point/curve and deflection families retain original cache/query ordering.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/point_curve_corpus/inputs-00.txt"),
        include_str!("fixtures/point_curve_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/point_curve_corpus/inputs-01.txt"),
        include_str!("fixtures/point_curve_corpus/expected-01.txt"),
    ),
    (
        include_str!("fixtures/point_curve_corpus/inputs-02.txt"),
        include_str!("fixtures/point_curve_corpus/expected-02.txt"),
    ),
    (
        include_str!("fixtures/point_curve_corpus/inputs-03.txt"),
        include_str!("fixtures/point_curve_corpus/expected-03.txt"),
    ),
    (
        include_str!("fixtures/point_curve_corpus/inputs-04.txt"),
        include_str!("fixtures/point_curve_corpus/expected-04.txt"),
    ),
    (
        include_str!("fixtures/point_curve_corpus/inputs-05.txt"),
        include_str!("fixtures/point_curve_corpus/expected-05.txt"),
    ),
    (
        include_str!("fixtures/point_curve_corpus/inputs-06.txt"),
        include_str!("fixtures/point_curve_corpus/expected-06.txt"),
    ),
    (
        include_str!("fixtures/point_curve_corpus/inputs-07.txt"),
        include_str!("fixtures/point_curve_corpus/expected-07.txt"),
    ),
    (
        include_str!("fixtures/point_curve_corpus/inputs-08.txt"),
        include_str!("fixtures/point_curve_corpus/expected-08.txt"),
    ),
];
#[test]
fn fixture_byte_hashes_and_all_rows_are_preserved() {
    assert_eq!(
        CHUNKS.iter().map(|(r, _)| r.lines().count()).sum::<usize>(),
        2505
    );
    assert_eq!(
        CHUNKS.iter().map(|(_, a)| a.lines().count()).sum::<usize>(),
        2505
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(r, _)| *r)),
        5590748732455745430
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, a)| *a)),
        4568719870956088793
    );
}

#[test]
fn pc2d_curve_complete_family() {
    super::replay_family(CHUNKS, "pc2d_curve", 824);
}

#[test]
fn pc2d_grid_complete_family() {
    super::replay_family(CHUNKS, "pc2d_grid", 14);
}

#[test]
fn pc2d_uniform_complete_family() {
    super::replay_family(CHUNKS, "pc2d_uniform", 7);
}

#[test]
fn pc2d_distance_complete_family() {
    super::replay_family(CHUNKS, "pc2d_distance", 10);
}

#[test]
fn pc2d_endpoints_complete_family() {
    super::replay_family(CHUNKS, "pc2d_endpoints", 252);
}

#[test]
fn gcpnts_td2d_complete_family() {
    super::replay_family(CHUNKS, "gcpnts_td2d", 146);
}

#[test]
fn pc_curve_complete_family() {
    super::replay_family(CHUNKS, "pc_curve", 975);
}

#[test]
fn pc_grid_complete_family() {
    super::replay_family(CHUNKS, "pc_grid", 10);
}

#[test]
fn pc_distance_complete_family() {
    super::replay_family(CHUNKS, "pc_distance", 4);
}

#[test]
fn pc_endpoints_complete_family() {
    super::replay_family(CHUNKS, "pc_endpoints", 144);
}

#[test]
fn pc_project_complete_family() {
    super::replay_family(CHUNKS, "pc_project", 27);
}

#[test]
fn gcpnts_td3d_complete_family() {
    super::replay_family(CHUNKS, "gcpnts_td3d", 92);
}

#[path = "support/point_curve_controls.rs"]
mod independent_controls;
