//! Complete hidden conic-tangent command, including its authored zero-based solution loop.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_conic_tangents;

#[test]
fn buc60842() {
    case_bugs_process_case::run_bugs_case(
        "buc60842",
        v3d_conic_tangents::conic_tangent_catch_regression,
    );
}
