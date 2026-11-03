//! Complete independent represented-face certificates, never captured-pass promotion.
mod fixture;
mod replay;
mod variants;
use martensite_tkbrep::brep::face_input_capture::{
    FaceCaptureLimits, capture_closed_extrusion_face,
};
use martensite_tktopalgo::face_validation::*;
pub(super) fn check(fixture: &fixture::StripFixture, policy: ClosedFacePolicy) -> ClosedFaceReport {
    let input =
        capture_closed_extrusion_face(&fixture.face, FaceCaptureLimits::bounded(), || false)
            .unwrap();
    certify_closed_extrusion_face(input, policy, || false)
}
#[test]
fn polynomial_strip_certifies_all_ten_support_obligations_and_zero_residuals() {
    let fixture = fixture::strip(1e-7, 0.);
    let report = check(&fixture, ClosedFacePolicy::bounded());
    assert!(
        matches!(report.completion(), ClosedFaceCompletion::Certified),
        "{:?}",
        report.completion()
    );
    assert!(report.certificate().is_some());
    assert_eq!(report.work().support_obligations, 10);
    assert_eq!(report.work().support_finished, 10);
    assert!(
        report
            .error_bounds()
            .iter()
            .all(|bound| bound.squared_upper.compare_f64(0.).unwrap() == std::cmp::Ordering::Equal)
    );
    assert_eq!(report.history(), ClosedFaceHistory::NoGeometryChanges);
}
#[test]
fn cancellation_and_exhausted_allowances_never_publish_a_certificate() {
    let fixture = fixture::strip(1e-7, 0.);
    let input =
        capture_closed_extrusion_face(&fixture.face, FaceCaptureLimits::bounded(), || false)
            .unwrap();
    let report = certify_closed_extrusion_face(input, ClosedFacePolicy::bounded(), || true);
    assert!(matches!(
        report.completion(),
        ClosedFaceCompletion::Incomplete(ClosedFaceIncomplete::Cancelled)
    ));
    assert!(report.certificate().is_none());
}
#[test]
fn exhausted_query_allowance_never_publishes_a_certificate() {
    let fixture = fixture::strip(1e-7, 0.);
    let policy = ClosedFacePolicy {
        max_queries: 0,
        ..ClosedFacePolicy::bounded()
    };
    let report = check(&fixture, policy);
    assert!(matches!(
        report.completion(),
        ClosedFaceCompletion::Incomplete(ClosedFaceIncomplete::Limit { .. })
    ));
    assert!(report.certificate().is_none());
}
#[test]
fn nonzero_cap_residuals_respect_existing_raw_limits() {
    let delta = 1. / 1_073_741_824.;
    let fixture = fixture::strip(delta, delta);
    let report = check(&fixture, ClosedFacePolicy::bounded());
    assert!(
        matches!(report.completion(), ClosedFaceCompletion::Certified),
        "{:?}",
        report.completion()
    );
    assert_positive_error(&report);
    let too_far = fixture::strip(delta, 4. * delta);
    let refused = check(&too_far, ClosedFacePolicy::bounded());
    assert!(!matches!(
        refused.completion(),
        ClosedFaceCompletion::Certified
    ));
    assert!(refused.certificate().is_none());
}

#[test]
fn mutated_observable_uv_cache_cannot_receive_a_certificate() {
    let fixture = fixture::strip(1e-7, 0.);
    fixture::set_cache(&fixture.edges[1], [[1., 2.], [4., 2.]], None);
    let report = check(&fixture, ClosedFacePolicy::bounded());
    assert!(
        matches!(report.completion(), ClosedFaceCompletion::Invalid(_)),
        "{:?}",
        report.completion()
    );
    assert!(report.certificate().is_none());
}

fn assert_positive_error(report: &ClosedFaceReport) {
    assert!(
        report.error_bounds().iter().any(
            |bound| bound.squared_upper.compare_f64(0.).unwrap() == std::cmp::Ordering::Greater
        )
    );
}
