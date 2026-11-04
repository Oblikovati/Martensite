//! The released isolated worker enforces CPU300 before the original rendering commands.
use std::error::Error;
#[path = "../case_support/case_source_process_case.rs"]
mod case_source_process_case;
// Share this one worker/lease instance; callers supply their own fixed source group and budget.
pub(crate) use case_source_process_case::observed_stream;

/// Qualify the real finalized stdout/stderr stream; `run_case("case", case_body)`.
pub(crate) fn run_case(identity: &str, body: impl FnOnce() -> Result<(), Box<dyn Error>>) {
    let Some(report) = case_source_process_case::observed_stream(identity, 300, body) else {
        return;
    };
    assert!(
        crate::case_opengl_policy::accepted_report(&report),
        "OpenGL original final stream rejected: report={report:?}; expected source completion and accepted group/common diagnostics"
    );
}

/// Preserve the authored absence of a CPU command with the same worker; `observed_inherited_stream("case", body)`.
#[allow(dead_code)] // Only some targets that share this file use it.
pub(crate) fn observed_inherited_stream(
    identity: &str,
    body: impl FnOnce() -> Result<(), Box<dyn Error>>,
) -> Option<String> {
    case_source_process_case::observed_inherited_stream(identity, body)
}
