//! Complete mesh algorithm comparisons and the required reused-status original.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_mesh_strategy_cases;

#[test]
fn bug28089_1() {
    case_bugs_process_case::run_bugs_case(
        "bug28089_1",
        v3d_mesh_strategy_cases::terrain_algorithms,
    );
}

#[test]
fn bug28089_2() {
    case_bugs_process_case::run_bugs_case(
        "bug28089_2",
        v3d_mesh_strategy_cases::primitive_algorithms,
    );
}

#[test]
fn bug32692_2() {
    case_todo::run_case_requiring(
        "bug32692_2",
        1000,
        &["Meshing statuses: Reused"],
        v3d_mesh_strategy_cases::existing_invalid_mesh,
    );
}
