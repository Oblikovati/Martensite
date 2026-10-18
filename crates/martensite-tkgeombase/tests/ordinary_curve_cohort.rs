//! Ordinary polynomial and nonlinear approximation operations retain every result field.
#[path = "support/ordinary_curve/mod.rs"]
mod ordinary;
fn run_group(kind: i32, prefix: &str, count: usize) {
    let mut executed = 0;
    for (spec, baseline) in ordinary::cases() {
        if spec.kind == kind && spec.name.starts_with(prefix) {
            ordinary::run_retained(&spec, &baseline);
            executed += 1;
        }
    }
    assert_eq!(executed, count);
}
#[test]
fn polynomial_all_orders_directions_derivatives_and_odd_even_rules() {
    run_group(0, "P_", 48);
}
#[test]
fn odd_rule_nonlinear_cut_cap_retains_middle_correction_behavior() {
    run_group(1, "C_", 4);
}
#[test]
fn even_rule_nonlinear_cutting_and_single_piece_warning() {
    run_group(1, "CE_", 4);
}
#[test]
fn zero_precision_zero_function_completes_in_both_directions() {
    run_group(2, "ZP_", 4);
}
