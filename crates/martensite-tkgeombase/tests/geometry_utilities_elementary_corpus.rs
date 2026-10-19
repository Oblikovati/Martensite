//! Complete elementary geometry/math families and independent derivative/array checks.
const CHUNKS: &[(&str, &str)] = &[(
    include_str!("fixtures/geometry_utilities_elementary_corpus/inputs.txt"),
    include_str!("fixtures/geometry_utilities_elementary_corpus/native.txt"),
)];
#[test]
fn gl_inertia_complete_family() {
    super::replay_family(CHUNKS, "gl_inertia", 16);
}
#[test]
fn gl_arrays_complete_family() {
    super::replay_family(CHUNKS, "gl_arrays", 76);
}
#[test]
fn gl_log_sample_complete_family() {
    super::replay_family(CHUNKS, "gl_log_sample", 6);
}
#[test]
fn gl_poly_func_complete_family() {
    super::replay_family(CHUNKS, "gl_poly_func", 12);
}
#[test]
fn elementary_geom_lib_inputs_and_retained_answers_stay_frozen() {
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(q, _)| *q)),
        0xe3bf94fa98cd9ac0
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, a)| *a)),
        0x100e9daa2786c1c8
    );
}
#[test]
fn quadratic_polynomial_derivatives_match_independent_closed_form() {
    // P(x)=2+3x+4x², so P'(2)=19 and P''(2)=8.
    let output =
        super::construction_corpus::run_values("gl_poly_func", &[1.0, 3.0, 2.0, 3.0, 4.0, 2.0])
            .unwrap();
    let (v, d) = (19.0f64.to_bits(), 8.0f64.to_bits());
    assert_eq!(output, format!("1 {v} 1 {d} 1 {v} {d} \n"));
}
#[test]
fn endpoint_array_densification_has_independent_integer_steps() {
    let output =
        super::construction_corpus::run_values("gl_arrays", &[1.0, 5.0, -2.0, 2.0, 0.0, 4.0])
            .unwrap();
    let values: String = [0.0f64, 1.0, 2.0, 3.0, 4.0]
        .iter()
        .map(|v| format!("{} ", v.to_bits()))
        .collect();
    assert_eq!(output, format!("1 5 {values}\n"));
}
#[test]
fn shared_array_reader_preserves_cursor_and_initialization_sentinel() {
    use super::construction_corpus::{
        common::Reader, geometry_utilities_elementary::read_reals, numeric_protocol::UNSET,
    };
    let values = [-2.0, 2.0, 0.0, 4.0, 99.0];
    let mut input = Reader::new(&values);
    let array = read_reals(&mut input);
    assert_eq!((array.lower(), array.upper()), (-2, -1));
    assert_eq!(array.as_slice(), &[0.0, 4.0]);
    assert_eq!(input.real(), 99.0);
    assert_eq!(UNSET.to_bits(), (-7.25f64).to_bits());
}
