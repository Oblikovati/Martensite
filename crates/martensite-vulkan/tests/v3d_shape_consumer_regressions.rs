//! Complete shape-consumer cases retain their authored CPU budgets.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code)]
#[path = "v3d_object_hlr_cases/commands.rs"]
mod hlr_commands;
#[allow(dead_code)]
#[path = "support/shape_report_checks.rs"]
mod shape_report_checks;
mod v3d_shape_consumer_cases;

#[test]
fn bug31144() {
    case_bugs_process_case::run_bugs_case_within(
        "bug31144",
        10,
        v3d_shape_consumer_cases::outer_wire_termination,
    );
}

#[test]
fn bug27782() {
    case_bugs_process_case::run_bugs_case(
        "bug27782",
        v3d_shape_consumer_cases::filleted_exact_hidden_lines,
    );
}
