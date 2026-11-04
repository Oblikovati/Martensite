//! Complete original copy/mesh regressions use actual shape copies and measured baselines.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_mesh_copy_cases;

#[test]
fn bug25055() {
    case_bugs_process_case::run_bugs_case(
        "bug25055",
        v3d_mesh_copy_cases::independently_remeshed_copy,
    );
}

#[test]
fn bug26897() {
    case_bugs_process_case::run_bugs_case("bug26897", v3d_mesh_copy_cases::retained_conical_mesh);
}
