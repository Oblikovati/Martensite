//! Authored expected failures run as ordinary original workers, without recorded-failure skips.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code)]
#[path = "support/shape_report_checks.rs"]
mod shape_report_checks;
mod v3d_shown_mesh_cases;
use v3d_shown_mesh_cases as cases;

#[test]
fn bug109() {
    case_bugs_process_case::run_bugs_case_expecting(
        "bug109",
        &["Error: Number of triangles", "Error: Number of nodes"],
        cases::shaded_count_regression,
    );
}

#[test]
fn bug264_1() {
    case_bugs_process_case::run_bugs_case_expecting(
        "bug264_1",
        &[
            "Error: Number of triangles",
            "Error: Number of nodes",
            "Error : The area of result shape is",
        ],
        cases::cleared_count_regression,
    );
}

#[test]
fn bug287() {
    case_bugs_process_case::run_bugs_case_expecting(
        "bug287",
        &["Error : The area of result shape is"],
        cases::incremental_count_regression,
    );
}
