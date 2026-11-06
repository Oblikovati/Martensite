//! Actual meshed-shape proximity and reported overlapping face compounds.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code)]
#[path = "support/shape_report_checks.rs"]
mod shape_report_checks;
mod v3d_proximity;
#[test]
fn bug25398() {
    case_bugs_process_case::run_bugs_case("bug25398", v3d_proximity::sphere_cone_overlap);
}
