//! Every original matrix-provider request and independent state/serialization contracts.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/continuity_matrix_corpus/inputs-00.txt"),
        include_str!("fixtures/continuity_matrix_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/continuity_matrix_corpus/inputs-01.txt"),
        include_str!("fixtures/continuity_matrix_corpus/expected-01.txt"),
    ),
    (
        include_str!("fixtures/continuity_matrix_corpus/inputs-02.txt"),
        include_str!("fixtures/continuity_matrix_corpus/expected-02.txt"),
    ),
    (
        include_str!("fixtures/continuity_matrix_corpus/inputs-03.txt"),
        include_str!("fixtures/continuity_matrix_corpus/expected-03.txt"),
    ),
];
#[test]
fn acc_matrix_complete_family() {
    super::replay_family(CHUNKS, "acc_matrix", 945);
}
#[test]
fn matrix_provider_inputs_and_retained_answers_stay_frozen() {
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(q, _)| *q)),
        0xb2b7a0cc313f6001
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, a)| *a)),
        0x77a44f7e5010cf7f
    );
}
#[test]
fn shifted_mass_provider_matches_independent_linear_bernstein_integrals() {
    let output =
        super::construction_corpus::run_values("acc_matrix", &[0.0, 2.0, 0.0, -2.0, 5.0, 2.0, 2.0])
            .unwrap();
    let values: String = [1.0f64 / 3.0, 1.0 / 6.0, 1.0 / 6.0, 1.0 / 3.0]
        .iter()
        .map(|v| format!("{} ", v.to_bits()))
        .collect();
    assert_eq!(output, format!("-2 -1 5 6 {values}\n"));
}
#[test]
fn short_output_reports_failure_after_preserving_its_first_retained_write() {
    let output =
        super::construction_corpus::run_values("acc_matrix", &[0.0, 2.0, 0.0, 1.0, 1.0, 1.0, 1.0])
            .unwrap();
    assert_eq!(
        output,
        format!(
            "Standard_OutOfRange 1 1 1 1 {} \n",
            (1.0f64 / 3.0).to_bits()
        )
    );
}
#[test]
fn canonical_matrix_tokens_preserve_bounds_order_and_special_bits() {
    use super::construction_corpus::{matrix_tokens, print::Output};
    use martensite_tkmath::math::Matrix;
    let mut matrix = Matrix::with_bounds(-2, -1, 5, 6).unwrap();
    matrix[(-2, 5)] = 0.0;
    matrix[(-2, 6)] = -0.0;
    matrix[(-1, 5)] = f64::INFINITY;
    matrix[(-1, 6)] = f64::from_bits(0x7ff8_0000_0000_1234);
    let output = Output::default();
    matrix_tokens::matrix(&output, &matrix);
    assert_eq!(
        output.finish(),
        "-2 -1 5 6 0 9223372036854775808 9218868437227405312 9221120237041095220 "
    );
}
