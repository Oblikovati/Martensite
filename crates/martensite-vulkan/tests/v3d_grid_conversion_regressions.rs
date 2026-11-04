//! Complete detection and conversion source retains every numeric tolerance and image.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_grid_conversion;
#[test]
fn bug25671() {
    case_v3d_process_case::run_case_within("bug25671", 1000, v3d_grid_conversion::grid_conversion);
}
