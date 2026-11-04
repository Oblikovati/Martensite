//! Whole, local and colored-shape highlighting retain all five authored captures.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_discretization;
#[test]
fn bug25540() {
    case_v3d_process_case::run_case_within(
        "bug25540",
        1000,
        v3d_discretization::highlight_discretization,
    );
}
