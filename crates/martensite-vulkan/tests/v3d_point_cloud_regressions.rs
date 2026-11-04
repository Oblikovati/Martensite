//! Point cloud regressions of the V3d group, each in an isolated worker.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_point_cloud_cases;
use std::error::Error;
use v3d_point_cloud_cases as cases;

fn run(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case(identity, body);
}

#[test]
fn sensitivebox() {
    run("sensitivebox", cases::clipped_box_detection);
}
