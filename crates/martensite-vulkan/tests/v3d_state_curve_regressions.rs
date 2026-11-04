//! Complete state/curve bodies keep their source group worker and completion contracts.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code, unused_imports)]
// Reuse paused playback without registering sibling animation cases.
mod v3d_anim_cases;
mod v3d_state_curve_cases;

#[test]
fn bug32717() {
    case_bugs_process_case::run_bugs_case(
        "bug32717",
        v3d_state_curve_cases::triangulation_only_copy,
    );
}

#[test]
fn bug32570() {
    case_v3d_process_case::run_case("bug32570", v3d_state_curve_cases::axis_rotation_midpoint);
}
#[test]
fn bug31794() {
    case_v3d_process_case::run_case("bug31794", v3d_state_curve_cases::child_mode_reactivation);
}
#[test]
fn bug31688() {
    case_v3d_process_case::run_case("bug31688", v3d_state_curve_cases::periodic_face_isolines);
}
