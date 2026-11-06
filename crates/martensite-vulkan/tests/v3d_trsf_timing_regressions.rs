//! Timed instance regressions of the V3d trsf group. The cases time operations of the
//! release build, so the host is built optimized (the release cargo profile).
//! The limits only hold for optimized code, so the
//! cases exist only in optimized builds.
#![cfg(all(target_os = "linux", not(debug_assertions)))]
include!("case_host/session_modules.rs");
mod v3d_trsf_timing_cases;
use std::error::Error;
use v3d_trsf_timing_cases as cases;

fn run(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case(identity, body);
}

#[test]
fn bug26029() {
    run("bug26029", cases::linear_instance_time);
}

#[test]
fn bug26199() {
    run("bug26199", cases::instance_clear_time);
}
