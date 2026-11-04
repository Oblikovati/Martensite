//! Five complete manipulation programs reuse their unchanged authored operation bodies.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code, unused_imports)]
mod v3d_manipulator_cases;

#[test]
fn bug30669() {
    case_v3d_process_case::run_case_within("bug30669", 1000, v3d_manipulator_cases::child_drag);
}

#[test]
fn bug33514() {
    case_v3d_process_case::run_case_within("bug33514", 1000, v3d_manipulator_cases::held_trihedron);
}

#[test]
fn drag_in_2d_view() {
    case_v3d_process_case::run_case_within(
        "drag_in_2d_view",
        1000,
        v3d_manipulator_cases::drag_in_2d,
    );
}

#[test]
fn flat() {
    case_v3d_process_case::run_case_within("flat", 1000, v3d_manipulator_cases::flat_skin);
}

#[test]
fn view_affinity() {
    case_v3d_process_case::run_case_within(
        "view_affinity",
        1000,
        v3d_manipulator_cases::view_affinity,
    );
}
