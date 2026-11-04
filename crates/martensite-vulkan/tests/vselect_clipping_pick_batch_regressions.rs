//! Complete clipping selection cases reuse the actual shared plane and scene bodies.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "support/selection_occurrence_rank.rs"]
mod selection_occurrence_rank;
#[allow(dead_code, unused_imports)]
mod vselect_clipping_cases;

#[test]
fn bug30777() {
    case_v3d_process_case::run_case_within(
        "bug30777",
        300,
        vselect_clipping_cases::clipped_sphere_point,
    );
}

#[test]
fn bug30906() {
    case_v3d_process_case::run_case_within(
        "bug30906",
        300,
        vselect_clipping_cases::override_does_not_leak,
    );
}

#[test]
fn bug31650() {
    case_v3d_process_case::run_case_within(
        "bug31650",
        300,
        vselect_clipping_cases::shared_global_local_plane,
    );
}
