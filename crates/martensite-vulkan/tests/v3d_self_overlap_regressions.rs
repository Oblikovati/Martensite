//! Actual 900-box and sphere scene with triangulation-based self-intersection output.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_self_overlap;
#[test]
fn bug26180() {
    case_bugs_process_case::run_bugs_case("bug26180", v3d_self_overlap::self_overlap_grid_view);
}
