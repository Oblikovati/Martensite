//! Three complete procedural programs use the source CPU1000 and mandatory-image lifecycle.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_procedural_presentation_cases;

#[test]
fn bug27617() {
    case_v3d_process_case::run_case_within(
        "bug27617",
        1000,
        v3d_procedural_presentation_cases::lit_dynamic_highlight,
    );
}
#[test]
fn bug30412() {
    case_v3d_process_case::run_case_within(
        "bug30412",
        1000,
        v3d_procedural_presentation_cases::camera_frustum_boxes,
    );
}
#[test]
fn bug27700() {
    case_v3d_process_case::run_case_within(
        "bug27700",
        1000,
        v3d_procedural_presentation_cases::point_frame_before_text,
    );
}
