//! Hidden wire-abscissa command and three distinct interactive pixel observations.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code)]
#[path = "support/shape_report_checks.rs"]
mod shape_report_checks;
mod v3d_wire_detection_cases;

#[test]
fn bug105() {
    case_bugs_process_case::run_bugs_case(
        "bug105",
        v3d_wire_detection_cases::wire_abscissa_regression,
    );
}

#[test]
fn bug22039() {
    case_bugs_process_case::run_bugs_case(
        "bug22039",
        v3d_wire_detection_cases::detection_pixel_regression,
    );
}
