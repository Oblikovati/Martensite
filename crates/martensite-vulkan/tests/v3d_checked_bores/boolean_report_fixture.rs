//! TEST-ONLY report sentinels detect metadata loss; they are not native warning evidence.
use super::super::boolean_outcome::{
    BooleanCause, BooleanIssue, CheckedLocalBody, KernelDiagnostics, RefusalClass,
};
use martensite_tkbo::bop_algo::{ProgressCompletion, ProgressFailureClass};

const PRIOR_WARNINGS: &str = "TEST_ONLY_PRIOR_WARNING_FIRST\nTEST_ONLY_PRIOR_WARNING_SECOND\n";
const PRIOR_ERRORS: &str = "TEST_ONLY_PRIOR_ERROR_FIRST\nTEST_ONLY_PRIOR_ERROR_SECOND\n";

pub(super) fn append_prior_sentinels(
    previous: CheckedLocalBody,
) -> (CheckedLocalBody, KernelDiagnostics) {
    let mut expected = previous.diagnostics().clone();
    let sentinels = KernelDiagnostics {
        warnings: PRIOR_WARNINGS.into(),
        errors: PRIOR_ERRORS.into(),
    };
    expected.warnings.push_str(PRIOR_WARNINGS);
    expected.errors.push_str(PRIOR_ERRORS);
    let previous = previous.append_test_only_prior_reports(sentinels);
    (previous, expected)
}

pub(super) fn assert_failed_successor_reports(issue: &BooleanIssue, prior: &KernelDiagnostics) {
    // The existing real null-tool path emits this alert; it is separate from the fixture.
    assert_report_channels(issue, prior, "Error: No or null input shapes\n");
    assert_eq!(issue.class, RefusalClass::Failed);
    assert!(matches!(
        &*issue.cause,
        BooleanCause::Execution(ProgressCompletion::Failed {
            class: ProgressFailureClass::AlgorithmReport,
            ..
        })
    ));
}

pub(super) fn assert_conversion_refusal_reports(issue: &BooleanIssue, prior: &KernelDiagnostics) {
    assert_report_channels(issue, prior, "");
    assert_eq!(issue.class, RefusalClass::Incomplete);
    assert!(matches!(
        &*issue.cause,
        BooleanCause::Validation { entity, expected }
            if entity == "named failure-only dependency"
                && *expected == "actual completed conversion/map/measurement"
    ));
}

fn assert_report_channels(issue: &BooleanIssue, prior: &KernelDiagnostics, successor_error: &str) {
    let returned = issue.diagnostics();
    assert_eq!(returned.warnings, prior.warnings);
    assert_eq!(
        returned.errors,
        format!("{}{successor_error}", prior.errors)
    );
    assert_sentinels_once(&returned.warnings, PRIOR_WARNINGS);
    assert_sentinels_once(&returned.errors, PRIOR_ERRORS);
}

fn assert_sentinels_once(returned: &str, ordered: &str) {
    assert!(
        returned.contains(ordered),
        "report={returned:?}; expected ordered test-only fixture"
    );
    for sentinel in ordered.lines() {
        assert_eq!(
            returned.matches(sentinel).count(),
            1,
            "report={returned:?}; expected one {sentinel:?}"
        );
    }
}
