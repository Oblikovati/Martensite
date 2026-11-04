//! The perf group: the shared worker transport under its 500 s CPU limit (set in
//! the group's begin hook) and its own rules, which accept `Error: performance` lines before
//! the inherited common rules.
use crate::case_opengl_process_case::observed_stream;
use std::error::Error;

/// Run a perf-group regression in an isolated worker and grade its report.
/// Example: `run_perf_case("bug23650", body)`.
pub(crate) fn run_perf_case(identity: &str, body: impl FnOnce() -> Result<(), Box<dyn Error>>) {
    let Some(report) = observed_stream(identity, 500, body) else {
        return;
    };
    assert!(
        report.contains("TEST COMPLETED")
            && crate::case_perf_bop_rules::accepted_perf_bop_report(&report),
        "perf original final stream rejected: report={report:?}; expected source completion and accepted perf/common diagnostics"
    );
}
