//! Complete color tables and the shared sample material scene retain authored commands.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code, unused_imports)] // This host runs a part of the shared cases.
mod v3d_materials_cases;
#[test]
fn colors() {
    case_v3d_process_case::run_case_within("colors", 1000, v3d_materials_cases::colors);
}
#[test]
fn phong1() {
    case_v3d_process_case::run_case_within("phong1", 1000, v3d_materials_cases::phong);
}
#[test]
fn raytrace1() {
    case_v3d_process_case::run_case_within("raytrace1", 1000, v3d_materials_cases::raytrace);
}
