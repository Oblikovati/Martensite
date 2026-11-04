//! Complete clipping selection cases reuse the actual shared plane and scene bodies.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "support/selection_occurrence_rank.rs"]
mod selection_occurrence_rank;
#[allow(dead_code, unused_imports)]
mod vselect_clipping_cases;

#[test]
fn bug26973() {
    case_v3d_process_case::run_case_within(
        "bug26973",
        300,
        vselect_clipping_cases::four_plane_targets,
    );
}

#[test]
fn bug29791() {
    case_v3d_process_case::run_case_within(
        "bug29791",
        300,
        vselect_clipping_cases::orthogonal_and_negative_depth,
    );
}

#[test]
fn bug30695() {
    case_v3d_process_case::run_case_within(
        "bug30695",
        300,
        vselect_clipping_cases::rectangle_containment_overlap,
    );
}
