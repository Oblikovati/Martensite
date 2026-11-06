//! Complete manipulator programs share their existing authored operation bodies.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code, unused_imports)]
mod v3d_manipulator_cases;

#[test]
fn drag_pan_zoom() {
    case_v3d_process_case::run_case_within(
        "drag_pan_zoom",
        1000,
        v3d_manipulator_cases::gesture_drags,
    );
}

#[test]
fn rotate2() {
    case_v3d_process_case::run_case_within(
        "rotate2",
        1000,
        v3d_manipulator_cases::stepped_rotation,
    );
}

#[test]
fn translate2() {
    case_v3d_process_case::run_case_within(
        "translate2",
        1000,
        v3d_manipulator_cases::turned_camera_translation,
    );
}

#[test]
fn zoom_persistence() {
    case_v3d_process_case::run_case_within(
        "zoom_persistence",
        1000,
        v3d_manipulator_cases::zoom_persistence,
    );
}
