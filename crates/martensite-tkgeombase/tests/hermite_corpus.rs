//! Complete reparametrization corpus and independent constant-weight polynomial checks.
const CHUNKS: &[(&str, &str)] = &[(
    include_str!("fixtures/hermite_corpus/inputs.txt"),
    include_str!("fixtures/hermite_corpus/native.txt"),
)];
#[test]
fn hermit_complete_family() {
    super::replay_family(CHUNKS, "hermit", 134);
}
#[test]
fn hermit_requests_and_answers_stay_frozen() {
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(q, _)| *q)),
        0xb39ddb0f438d5736
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, a)| *a)),
        0xacd730c4f27ed2dc
    );
}
fn assert_constant_unit_polynomial(request: &[f64]) {
    // A cubic with endpoint values one and endpoint derivatives zero is the constant one.
    let output = super::construction_corpus::run_values("hermit", request).unwrap();
    let one = 1.0f64.to_bits();
    assert_eq!(
        output,
        format!("3 4 0 {one} 0 {one} 0 {one} 0 {one} 2 0 4 {one} 4 \n")
    );
}
#[test]
fn unit_weight_2d_spline_has_constant_compensation() {
    assert_constant_unit_polynomial(&[
        1.0, 1e-6, 1e-6, 0.0, 1.0, 6.0, 2.0, 0.0, 3.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.0, 1.0,
        2.0, 0.0, 3.0, 1.0, 3.0,
    ]);
}
#[test]
fn unit_weight_3d_spline_has_constant_compensation() {
    assert_constant_unit_polynomial(&[
        0.0, 1e-6, 1e-6, 0.0, 1.0, 6.0, 2.0, 0.0, 3.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 2.0,
        0.0, 0.0, 1.0, 2.0, 0.0, 3.0, 1.0, 3.0,
    ]);
}
