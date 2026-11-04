//! Complete manipulator programs share their existing authored operation bodies.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code, unused_imports)]
mod v3d_manipulator_cases;

#[test]
fn dragg() {
    case_v3d_process_case::run_case_within("dragg", 1000, v3d_manipulator_cases::dragged_object);
}

#[test]
fn rotate() {
    case_v3d_process_case::run_case_within("rotate", 1000, v3d_manipulator_cases::rotated_object);
}

#[test]
fn scale() {
    case_v3d_process_case::run_case_within("scale", 1000, v3d_manipulator_cases::scaled_object);
}

#[test]
fn translate() {
    case_v3d_process_case::run_case_within(
        "translate",
        1000,
        v3d_manipulator_cases::translated_object,
    );
}
