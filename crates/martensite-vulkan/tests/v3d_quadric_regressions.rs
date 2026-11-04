//! Triangulated quadric regressions of the V3d group, each in an isolated worker.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code)] // Only the quadric regressions display primitive-array objects.
#[path = "support/triangle_array_object.rs"]
mod triangle_array_object;
mod v3d_quadric_cases;
use std::error::Error;
use v3d_quadric_cases as cases;

fn run(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case(identity, body);
}

#[test]
fn cylinder() {
    run("cylinder", cases::cylinder_cone_frustum);
}

#[test]
fn sphere() {
    run("sphere", cases::complete_sphere);
}

#[test]
fn torus() {
    run("torus", cases::torus_segments);
}
