//! Four complete color, connection and child-selection programs reuse unchanged bodies.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "case_host/dump_diff.rs"]
mod dump_diff;
#[path = "support/selection_occurrence_rank.rs"]
mod selection_occurrence_rank;
// Query-only shared helpers use this legacy module path; no clipping programs are included.
use selection_occurrence_rank as vselect_clipping_cases;
#[allow(dead_code, unused_imports)]
mod vselect_bugs_cases;

#[test]
fn buc60587() {
    case_v3d_process_case::run_case_within(
        "buc60587",
        300,
        vselect_bugs_cases::recolored_selection,
    );
}

#[test]
fn bug12121() {
    case_v3d_process_case::run_case_within(
        "bug12121",
        300,
        vselect_bugs_cases::connected_instance_sub_shapes,
    );
}

#[test]
fn bug26462_1() {
    case_v3d_process_case::run_case_within(
        "bug26462_1",
        300,
        vselect_bugs_cases::sensitivity_follows_width,
    );
}

#[test]
fn bug30824() {
    case_v3d_process_case::run_case_within(
        "bug30824",
        300,
        vselect_bugs_cases::parent_selected_without_children,
    );
}
