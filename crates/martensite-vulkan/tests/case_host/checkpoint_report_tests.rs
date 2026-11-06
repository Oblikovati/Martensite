//! Check the actual checkpoint output through the finalized original worker/report seam.
use super::check_point;
use crate::case_opengl_process_case::observed_stream;
use crate::case_v3d_policy::accepted_report;

const NAN_CASE: &str = "session_run::checkpoint_report_tests::nan_rejects_finalized_report";

#[test]
fn nan_rejects_finalized_report() {
    let Some(report) = checkpoint_report(NAN_CASE, [f64::NAN, 200., 0.], [100., 200., 0.], 0.001)
    else {
        return;
    };
    assert!(
        !accepted_report(&report),
        "NaN checkpoint must reject the finalized original report, got {report:?}"
    );
    assert!(report.contains("Error: point1, (NaN 200 0)"));
}

fn checkpoint_report(
    identity: &str,
    value: [f64; 3],
    expected: [f64; 3],
    tolerance: f64,
) -> Option<String> {
    observed_stream(identity, 300, || {
        check_point("point1", value, expected, tolerance);
        println!("TEST COMPLETED");
        Ok(())
    })
}

#[test]
fn finite_point_keeps_report_accepted() {
    require_checkpoint_case(
        "finite_point_keeps_report_accepted",
        [100., 200., 0.],
        [100., 200., 0.],
        0.001,
        true,
    );
}

#[test]
fn outside_point_rejects_finalized_report() {
    require_checkpoint_case(
        "outside_point_rejects_finalized_report",
        [0.25, 0., 0.],
        [0.; 3],
        0.125,
        false,
    );
}

#[test]
fn exact_tolerance_boundary_keeps_report_accepted() {
    require_checkpoint_case(
        "exact_tolerance_boundary_keeps_report_accepted",
        [0.125, 0., 0.],
        [0.; 3],
        0.125,
        true,
    );
}

#[test]
fn positive_infinity_rejects_finalized_report() {
    require_checkpoint_case(
        "positive_infinity_rejects_finalized_report",
        [f64::INFINITY, 200., 0.],
        [100., 200., 0.],
        0.001,
        false,
    );
}

#[test]
fn negative_infinity_rejects_finalized_report() {
    require_checkpoint_case(
        "negative_infinity_rejects_finalized_report",
        [f64::NEG_INFINITY, 200., 0.],
        [100., 200., 0.],
        0.001,
        false,
    );
}

#[test]
fn nan_expected_rejects_finalized_report() {
    require_checkpoint_case(
        "nan_expected_rejects_finalized_report",
        [100., 200., 0.],
        [f64::NAN, 200., 0.],
        0.001,
        false,
    );
}

fn require_checkpoint_case(
    case: &str,
    value: [f64; 3],
    expected: [f64; 3],
    tolerance: f64,
    accepted: bool,
) {
    let identity = format!("session_run::checkpoint_report_tests::{case}");
    let Some(report) = checkpoint_report(&identity, value, expected, tolerance) else {
        return;
    };
    assert_checkpoint_grade(&report, accepted);
}

fn assert_checkpoint_grade(report: &str, accepted: bool) {
    assert!(report.contains("TEST COMPLETED"));
    assert_eq!(
        accepted_report(report),
        accepted,
        "unexpected actual finalized checkpoint report {report:?}"
    );
    let prefix = if accepted {
        "Check of point1 OK:"
    } else {
        "Error: point1,"
    };
    assert!(
        report.contains(prefix),
        "report={report:?}; expected actual checkpoint diagnostic {prefix:?}"
    );
}
