//! Automatic triangulation and actual culled primitive statistics cases.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_mesh_counter_cases;

#[test]
fn bug23200() {
    case_v3d_process_case::run_case("bug23200", v3d_mesh_counter_cases::automatic_mesh);
}

#[test]
fn bug30437() {
    case_v3d_process_case::run_case("bug30437", v3d_mesh_counter_cases::culled_elements);
}
