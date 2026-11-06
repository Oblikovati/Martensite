//! Viewer animation regressions of the V3d group, each in an isolated worker.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_anim_cases;
use std::error::Error;
use v3d_anim_cases as cases;

fn run(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case(identity, body);
}

#[test]
fn objects() {
    run("objects", cases::moved_objects);
}

#[test]
fn rotate() {
    run("rotate", cases::camera_rotation);
}

#[test]
fn scale() {
    run("scale", cases::camera_zoom);
}

#[test]
fn translate() {
    run("translate", cases::camera_translation);
}

#[test]
fn propeller() {
    run("propeller", cases::propeller_tour);
}
