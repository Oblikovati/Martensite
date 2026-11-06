//! Full priority and transformed-quadric cases retain their CPU300 selection lifecycle.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "support/case_v3d_policy_controls.rs"]
mod case_v3d_policy_controls;
#[allow(dead_code, unused_imports)]
// Reuse the whole reviewed priority subtree without importing unrelated selection cases.
#[path = "vselect_bugs_cases/persistent_priority/mod.rs"]
mod persistent_priority;
#[allow(dead_code, unused_imports)]
// Keep all shared quadric programs unchanged; this host selects the three complete bodies.
mod vselect_quadric_cases;
use std::error::Error;

fn run(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case_within(identity, 300, body);
}

#[test]
fn bug30484_1() {
    run("bug30484_1", persistent_priority::priority_boxes);
}

#[test]
fn bug30484_2() {
    run("bug30484_2", persistent_priority::priority_arrays);
}

#[test]
fn bug30484_3() {
    run("bug30484_3", persistent_priority::priority_point);
}

#[test]
fn bug30484_4() {
    run("bug30484_4", persistent_priority::priority_quad);
}

#[test]
fn hollow_cone_cyl() {
    run("hollow_cone_cyl", vselect_quadric_cases::hollow_cone_cyl);
}

#[test]
fn trsf_cone() {
    run("trsf_cone", vselect_quadric_cases::transformed_cone);
}

#[test]
fn trsf_cyl() {
    run("trsf_cyl", vselect_quadric_cases::transformed_cylinder);
}
