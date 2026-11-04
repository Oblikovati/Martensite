//! View cube regressions of the V3d group, each in an isolated worker.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_viewcube_cases;
use std::error::Error;
use v3d_viewcube_cases as cases;

fn run(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case(identity, body);
}

#[test]
fn default() {
    run("default", cases::default_cube_clicks);
}

#[test]
fn largedist() {
    run("largedist", cases::far_origin_cube);
}

#[test]
fn orthopers() {
    run("orthopers", cases::orthographic_cube_clicks);
}

#[test]
fn style() {
    run("style", cases::styled_cubes);
}

#[test]
fn view() {
    run("view", cases::per_view_visibility);
}
