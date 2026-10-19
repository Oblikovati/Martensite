//! Complete point-distribution families preserve requests, state transitions and all retained fields.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/point_distribution_corpus/inputs-00.txt"),
        include_str!("fixtures/point_distribution_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/point_distribution_corpus/inputs-01.txt"),
        include_str!("fixtures/point_distribution_corpus/expected-01.txt"),
    ),
    (
        include_str!("fixtures/point_distribution_corpus/inputs-02.txt"),
        include_str!("fixtures/point_distribution_corpus/expected-02.txt"),
    ),
    (
        include_str!("fixtures/point_distribution_corpus/inputs-03.txt"),
        include_str!("fixtures/point_distribution_corpus/expected-03.txt"),
    ),
    (
        include_str!("fixtures/point_distribution_corpus/inputs-04.txt"),
        include_str!("fixtures/point_distribution_corpus/expected-04.txt"),
    ),
    (
        include_str!("fixtures/point_distribution_corpus/inputs-05.txt"),
        include_str!("fixtures/point_distribution_corpus/expected-05.txt"),
    ),
    (
        include_str!("fixtures/point_distribution_corpus/inputs-06.txt"),
        include_str!("fixtures/point_distribution_corpus/expected-06.txt"),
    ),
    (
        include_str!("fixtures/point_distribution_corpus/inputs-07.txt"),
        include_str!("fixtures/point_distribution_corpus/expected-07.txt"),
    ),
    (
        include_str!("fixtures/point_distribution_corpus/inputs-08.txt"),
        include_str!("fixtures/point_distribution_corpus/expected-08.txt"),
    ),
];
#[test]
fn gcp_length_complete_family() {
    super::replay_family(CHUNKS, "gcp_length", 192);
}
#[test]
fn cpnts_abscissa_complete_family() {
    super::replay_family(CHUNKS, "cpnts_abscissa", 608);
}
#[test]
fn cpnts_udefl_complete_family() {
    super::replay_family(CHUNKS, "cpnts_udefl", 256);
}
#[test]
fn gcp_abscissa_complete_family() {
    super::replay_family(CHUNKS, "gcp_abscissa", 640);
}
#[test]
fn gcp_uabs_complete_family() {
    super::replay_family(CHUNKS, "gcp_uabs", 32);
}
#[test]
fn gcp_quabs_complete_family() {
    super::replay_family(CHUNKS, "gcp_quabs", 32);
}
#[test]
fn gcp_udefl_complete_family() {
    super::replay_family(CHUNKS, "gcp_udefl", 35);
}
#[test]
fn gcp_qudefl_complete_family() {
    super::replay_family(CHUNKS, "gcp_qudefl", 35);
}
#[test]
fn gcp_tdefl_complete_family() {
    super::replay_family(CHUNKS, "gcp_tdefl", 32);
}
#[test]
fn gcp_arc_step_complete_family() {
    super::replay_family(CHUNKS, "gcp_arc_step", 252);
}
#[test]
fn gcp_dist_complete_family() {
    super::replay_family(CHUNKS, "gcp_dist", 96);
}
#[test]
fn point_distribution_inputs_and_answers_stay_frozen() {
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(q, _)| *q)),
        0xb966e54e5e1d89ee
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, a)| *a)),
        0x40d3cad86189f8d0
    );
}
fn assert_unit_speed_lengths(request: &[f64]) {
    let output = super::construction_corpus::run_values("gcp_length", request).unwrap();
    let expected: String = [4.0_f64, 4.0, 2.0, 2.0, 4.0, 4.0, 2.0, 2.0]
        .iter()
        .map(|value| format!("{} ", value.to_bits()))
        .collect();
    assert_eq!(output, expected + "\n");
}
#[test]
fn planar_line_lengths_match_independent_parameter_differences() {
    assert_unit_speed_lengths(&[2.0, 1.0, 0.0, 4.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 3.0, 1e-6]);
}
#[test]
fn spatial_line_lengths_match_independent_parameter_differences() {
    assert_unit_speed_lengths(&[
        3.0, 1.0, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 3.0, 1e-6,
    ]);
}
