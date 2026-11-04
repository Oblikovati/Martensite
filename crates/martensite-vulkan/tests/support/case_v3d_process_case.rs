//! V3d shares the existing finalized worker transport, with its fixed CPU1000/group policy.
use crate::case_opengl_process_case::observed_stream;
use std::error::Error;

/// Run the real V3d lifecycle and grade all descriptors; `run_case("case", case_body)`.
pub(crate) fn run_case(identity: &str, body: impl FnOnce() -> Result<(), Box<dyn Error>>) {
    run_case_within(identity, 1000, body);
}

/// The same lifecycle under a group's own CPU limit in seconds; `run_case_within("case", 300, body)`.
pub(crate) fn run_case_within(
    identity: &str,
    cpu_seconds: i64,
    body: impl FnOnce() -> Result<(), Box<dyn Error>>,
) {
    let Some(report) = observed_stream(identity, cpu_seconds, body) else {
        return;
    };
    assert!(
        crate::case_v3d_policy::accepted_report(&report),
        "V3d original final stream rejected: report={report:?}; expected source completion and accepted V3d/common diagnostics"
    );
}
