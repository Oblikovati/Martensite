//! Existing geometry assertions rendered by the standalone owned topology closure.
#![cfg(target_os = "linux")]
#[path = "support/case_view_process_limit.rs"]
mod process_limit;
#[path = "topology_geometry_capture/support.rs"]
mod support;
use support::replay;
mod prepared_13116;

#[test]
fn prepared_union() {
    replay("prepared_union", "bug13116_1_case_fuse", || {
        prepared_13116::checked_result(1)
    });
}
#[test]
fn prepared_common() {
    replay("prepared_common", "bug13116_2_case_common", || {
        prepared_13116::checked_result(2)
    });
}
#[test]
fn prepared_cut() {
    replay("prepared_cut", "bug13116_3_case_cut", || {
        prepared_13116::checked_result(3)
    });
}
#[test]
fn prepared_reverse_cut() {
    replay(
        "prepared_reverse_cut",
        "bug13116_4_case_reverse_cut",
        || prepared_13116::checked_result(4),
    );
}
