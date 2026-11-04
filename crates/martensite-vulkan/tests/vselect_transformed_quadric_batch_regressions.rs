//! Three transformed analytic/triangulated selections reuse their complete authored bodies.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code, unused_imports)]
mod vselect_quadric_cases;

#[test]
fn trsf_cone() {
    case_v3d_process_case::run_case_within(
        "trsf_cone",
        300,
        vselect_quadric_cases::transformed_cone,
    );
}

#[test]
fn trsf_cyl() {
    case_v3d_process_case::run_case_within(
        "trsf_cyl",
        300,
        vselect_quadric_cases::transformed_cylinder,
    );
}

#[test]
fn trsf() {
    case_v3d_process_case::run_case_within("trsf", 300, vselect_quadric_cases::sphere_trsf);
}
