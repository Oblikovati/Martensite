//! Three complete ray-picking cases reuse the existing bodies and source predicates.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod vselect_axis_batch;

#[test]
fn axis_a1() {
    case_v3d_process_case::run_case_within("axis_a1", 300, vselect_axis_batch::axis_a1);
}

#[test]
fn axis_a2() {
    case_v3d_process_case::run_case_within("axis_a2", 300, vselect_axis_batch::axis_a2);
}

#[test]
fn axis_a3() {
    case_v3d_process_case::run_case_within("axis_a3", 300, vselect_axis_batch::axis_a3);
}
