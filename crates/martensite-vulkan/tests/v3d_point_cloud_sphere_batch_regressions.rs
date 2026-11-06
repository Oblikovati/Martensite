//! Both complete point-cloud cases share their actual mesh/node construction.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code, unused_imports)]
mod v3d_point_cloud_cases;

#[test]
fn sphere() {
    case_v3d_process_case::run_case_within(
        "sphere",
        1000,
        v3d_point_cloud_cases::colored_textured_sphere,
    );
}

#[test]
fn sensitivebox() {
    case_v3d_process_case::run_case_within(
        "sensitivebox",
        1000,
        v3d_point_cloud_cases::clipped_box_detection,
    );
}

#[cfg(feature = "vertex-output-diagnostics")]
#[test]
fn sphere_green_vertex_output_diagnostic() {
    case_v3d_process_case::run_case_within(
        "sphere_green_vertex_output_diagnostic",
        1000,
        v3d_point_cloud_cases::green_vertex_output_prefix,
    );
}
