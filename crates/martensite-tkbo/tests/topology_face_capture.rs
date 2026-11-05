//! Existing geometry assertions rendered by the standalone owned topology closure.
#![cfg(target_os = "linux")]
#[path = "support/case_view_process_limit.rs"]
mod process_limit;
#[path = "topology_geometry_capture/support.rs"]
mod support;
use support::replay;
mod face_section_cases;
#[path = "topology_geometry_capture/isolation.rs"]
mod isolation;

#[test]
fn first_face_section() {
    replay("first_face_section", "bug23985_case_section", || {
        face_section_cases::checked_result(23985)
    });
}
#[test]
fn second_face_section() {
    replay("second_face_section", "bug24496_case_section", || {
        face_section_cases::checked_result(24496)
    });
}
#[test]
fn pick_results_belong_to_each_owned_drawable() {
    isolation::check_pick_isolation();
}
