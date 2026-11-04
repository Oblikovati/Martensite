//! Actual surface isoline catch preserves the authored missing result binding.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_isoline_catch;
#[test]
fn bug22910_1() {
    case_bugs_process_case::run_bugs_case_expecting(
        "bug22910_1",
        &[
            "An exception was caught",
            "Faulty OCC22910",
            "Error: The command cannot be built",
        ],
        v3d_isoline_catch::isoline_catch_view,
    );
}
