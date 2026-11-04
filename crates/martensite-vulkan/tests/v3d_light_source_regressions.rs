//! Light source regressions of the V3d group, each in an isolated worker.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_light_source_cases;
use std::error::Error;
use v3d_light_source_cases as cases;

fn run(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case(identity, body);
}

#[test]
fn headlight() {
    run("headlight", cases::headlight_spot_and_positional);
}

#[test]
fn display_all() {
    run("display_all", cases::every_light_kind);
}

#[test]
fn dyn_highlight() {
    run("dyn_highlight", cases::sphere_detection);
}

#[test]
fn manipulator() {
    run("manipulator", cases::light_manipulator);
}
