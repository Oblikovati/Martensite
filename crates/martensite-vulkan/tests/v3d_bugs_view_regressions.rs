//! View regressions of the V3d bugs group, each in an isolated worker under the V3d policy.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_bugs_view_cases;
use std::error::Error;
use v3d_bugs_view_cases as cases;

fn run(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case(identity, body);
}

#[test]
fn bug25867() {
    run("bug25867", cases::zoom_persistent_anchor_moves);
}

#[test]
fn bug27374() {
    run("bug27374", cases::zoom_persistent_boxes_fit);
}

#[test]
fn bug27536() {
    run("bug27536", cases::zoom_persistent_box_survives_view_resize);
}

#[test]
fn bug27629() {
    run("bug27629", cases::zoom_persistent_normals_stay_unit);
}
