//! Every geometry-projection request retains its original observations.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/surface_projection_corpus/inputs-00.txt"),
        include_str!("fixtures/surface_projection_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/surface_projection_corpus/inputs-01.txt"),
        include_str!("fixtures/surface_projection_corpus/expected-01.txt"),
    ),
    (
        include_str!("fixtures/surface_projection_corpus/inputs-02.txt"),
        include_str!("fixtures/surface_projection_corpus/expected-02.txt"),
    ),
    (
        include_str!("fixtures/surface_projection_corpus/inputs-03.txt"),
        include_str!("fixtures/surface_projection_corpus/expected-03.txt"),
    ),
    (
        include_str!("fixtures/surface_projection_corpus/inputs-04.txt"),
        include_str!("fixtures/surface_projection_corpus/expected-04.txt"),
    ),
    (
        include_str!("fixtures/surface_projection_corpus/inputs-05.txt"),
        include_str!("fixtures/surface_projection_corpus/expected-05.txt"),
    ),
    (
        include_str!("fixtures/surface_projection_corpus/inputs-06.txt"),
        include_str!("fixtures/surface_projection_corpus/expected-06.txt"),
    ),
];

#[test]
fn fixture_bytes_and_all_911_rows_are_preserved() {
    let rows = (
        CHUNKS.iter().map(|(r, _)| r.lines().count()).sum::<usize>(),
        CHUNKS.iter().map(|(_, r)| r.lines().count()).sum::<usize>(),
    );
    let bytes = (
        super::fixture_hash(CHUNKS.iter().map(|(r, _)| *r)),
        super::fixture_hash(CHUNKS.iter().map(|(_, r)| *r)),
    );
    assert_eq!(rows, (911, 911));
    assert_eq!(bytes, (3065578449484268981, 9664045463093483268));
}

#[test]
fn gpl_curve2d_complete_family() {
    super::replay_family(CHUNKS, "gpl_curve2d", 295);
}

#[test]
fn gpl_project_complete_family() {
    super::replay_family(CHUNKS, "gpl_project", 306);
}

#[test]
fn gpl_on_plane_complete_family() {
    super::replay_family(CHUNKS, "gpl_on_plane", 310);
}

#[path = "support/surface_projection_controls.rs"]
mod independent_controls;
