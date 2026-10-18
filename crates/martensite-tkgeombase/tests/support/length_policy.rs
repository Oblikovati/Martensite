//! Exact shared CheckCommands.tcl length policy; expected constants belong to each case.
pub const LENGTH_COMPUTE_TOLERANCE: f64 = 1e-4;
pub fn check_original_length(actual: f64, expected: f64) {
    // CheckCommands.tcl:705–776 rejects zero and a relative difference strictly greater than 1%.
    assert!(
        actual.is_finite() && actual != 0.0,
        "length {actual}; expected finite nonzero value near {expected}"
    );
    assert!(
        ((expected - actual) / expected).abs() <= 0.01,
        "length {actual}; expected {expected} within original 1% relative comparison"
    );
}
pub fn record_original_length(case: &str, actual: f64, expected: f64) {
    check_original_length(actual, expected);
    println!(
        "{{\"case\":\"{case}\",\"length_bits\":{},\"expected_bits\":{}}}",
        actual.to_bits(),
        expected.to_bits()
    );
}
