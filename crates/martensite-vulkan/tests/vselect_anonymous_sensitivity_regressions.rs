//! Anonymous edge-selection sensitivity with the complete CPU300 selection-group lifecycle.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod vselect_anonymous_sensitivity;

#[test]
fn bug26462_2() {
    case_v3d_process_case::run_case_within(
        "bug26462_2",
        300,
        vselect_anonymous_sensitivity::complete_anonymous_sensitivity,
    );
}
