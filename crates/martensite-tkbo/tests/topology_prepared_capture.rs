//! Existing geometry assertions rendered by the standalone owned topology closure.
#![cfg(target_os = "linux")]
#[path = "support/case_view_process_limit.rs"]
mod process_limit;
#[path = "topology_geometry_capture/support.rs"]
mod support;
use support::replay;
mod prepared_intersection_cases;

#[test]
fn prepared_section() {
    replay(
        "prepared_section",
        "bug23998_case_intersection",
        prepared_intersection_cases::checked_section,
    );
}
#[test]
fn prepared_second_cut() {
    replay(
        "prepared_second_cut",
        "bug24811_case_intersection",
        prepared_intersection_cases::checked_cut,
    );
}
