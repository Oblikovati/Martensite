//! Complete selection performance cases retain the perf group's finite 500-second budget.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_selection_perf_cases;

#[test]
fn bug24623_1() {
    case_perf_process_case::run_perf_case("bug24623_1", v3d_selection_perf_cases::sphere_grid);
}

#[test]
fn bug24623_2() {
    case_perf_process_case::run_perf_case("bug24623_2", v3d_selection_perf_cases::spiral);
}
