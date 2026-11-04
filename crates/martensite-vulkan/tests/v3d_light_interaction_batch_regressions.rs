//! Three complete interactive-light programs reuse the original authored bodies.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code, unused_imports)]
mod v3d_light_source_cases;

#[test]
fn display_all() {
    case_v3d_process_case::run_case_within(
        "display_all",
        1000,
        v3d_light_source_cases::every_light_kind,
    );
}

#[test]
fn dyn_highlight() {
    case_v3d_process_case::run_case_within(
        "dyn_highlight",
        1000,
        v3d_light_source_cases::sphere_detection,
    );
}

#[test]
fn manipulator() {
    case_v3d_process_case::run_case_within(
        "manipulator",
        1000,
        v3d_light_source_cases::light_manipulator,
    );
}
