//! Real parent/subviews and actual selection affinity preserve both source dump kinds.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_subview_affinity;
#[test]
fn bug32978() {
    case_v3d_process_case::run_case_within(
        "bug32978",
        1000,
        v3d_subview_affinity::subview_selection_affinity,
    );
}
