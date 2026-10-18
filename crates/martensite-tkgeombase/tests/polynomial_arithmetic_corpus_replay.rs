//! Original MathBase request order is essential: two rows change process precision.
#[path = "support/construction_corpus/mod.rs"]
mod construction_corpus;
#[path = "support/polynomial_arithmetic_corpus.rs"]
mod polynomial_arithmetic_corpus;

#[test]
fn math_base_sequence_and_independent_invariants() {
    polynomial_arithmetic_corpus::replay_sequence();
    polynomial_arithmetic_corpus::independent_invariants();
    polynomial_arithmetic_corpus::independent_layout_and_reader();
    polynomial_arithmetic_corpus::restoration_on_unwind();
}
