//! All original coefficient indices and offset classes use direct owned Rust dispatch.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/continuity_tables_corpus/inputs-00.txt"),
        include_str!("fixtures/continuity_tables_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/continuity_tables_corpus/inputs-01.txt"),
        include_str!("fixtures/continuity_tables_corpus/expected-01.txt"),
    ),
];
#[test]
fn acc_table_complete_family() {
    super::replay_family(CHUNKS, "acc_table", 254);
}
#[test]
fn acc_offsets_complete_family() {
    super::replay_family(CHUNKS, "acc_offsets", 34);
}
#[test]
fn coefficient_requests_and_retained_answers_stay_frozen() {
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(q, _)| *q)),
        0xc7ae517e828dae3b
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, a)| *a)),
        0xcb996566ca2183ca
    );
}
fn assert_coefficient_block(kind: f64, expected: &[f64]) {
    let output =
        super::construction_corpus::run_values("acc_table", &[kind, 0.0, expected.len() as f64])
            .unwrap();
    let expected: String = expected
        .iter()
        .map(|v| format!("{} ", v.to_bits()))
        .collect();
    assert_eq!(output, expected + "\n");
}
#[test]
fn linear_bernstein_mass_entries_match_independent_exact_integrals() {
    // Integrals of (1-t)^2, t(1-t), t(1-t), t² on [0,1].
    assert_coefficient_block(0.0, &[1.0 / 3.0, 1.0 / 6.0, 1.0 / 6.0, 1.0 / 3.0]);
}
#[test]
fn inverse_linear_mass_entries_match_independent_two_by_two_inverse() {
    assert_coefficient_block(1.0, &[4.0, -2.0, -2.0, 4.0]);
}
#[test]
fn offset_blocks_match_independent_sums_of_square_dimensions() {
    for (class, expected) in [(4.0, "13 1 0 \n"), (7.0, "90 30 5 \n"), (-3.0, "0 0 0 \n")] {
        assert_eq!(
            super::construction_corpus::run_values("acc_offsets", &[class]).unwrap(),
            expected
        );
    }
}
