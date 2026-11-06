//! The bugs group: the shared worker transport under its 1000 s CPU limit and its own rules,
//! which refuse attribute-reading failures before the inherited common rules.
use crate::case_opengl_process_case::observed_stream;
use std::error::Error;

/// Run a bugs-group regression in an isolated worker and grade its report.
/// Example: `run_bugs_case("bug25044_1", body)`.
pub(crate) fn run_bugs_case(identity: &str, body: impl FnOnce() -> Result<(), Box<dyn Error>>) {
    run_bugs_case_within(identity, 1000, body);
}

/// The same under the case's own CPU limit in seconds.
/// Example: `run_bugs_case_within("bug32601", 30, body)`.
pub(crate) fn run_bugs_case_within(
    identity: &str,
    cpu_seconds: i64,
    body: impl FnOnce() -> Result<(), Box<dyn Error>>,
) {
    let Some(report) = observed_stream(identity, cpu_seconds, body) else {
        return;
    };
    let word = crate::case_word_profile::word_character;
    let accepted = crate::case_bugs_bop_rules::accepted_bugs_bop_report_with(&report, word);
    assert!(
        report.contains("TEST COMPLETED") && accepted,
        "bugs original final stream rejected: report={report:?}; expected source completion and accepted bugs/common diagnostics"
    );
}

/// A bugs-group regression that declares `TODO` lines: its report must fail exactly
/// as expected under the bugs rules.
/// Example: `run_bugs_case_expecting("bug264_1", &["Error : The area"], body)`.
pub(crate) fn run_bugs_case_expecting(
    identity: &str,
    todos: &[&'static str],
    body: impl FnOnce() -> Result<(), Box<dyn Error>>,
) {
    use crate::case_todo::{TodoOutcome, todo_outcome};
    let Some(report) = observed_stream(identity, 1000, body) else {
        return;
    };
    assert!(
        report.contains("TEST COMPLETED"),
        "bugs original stream report={report:?}; expected source completion"
    );
    let word = crate::case_word_profile::word_character;
    let failing = |line: &str| {
        let single = format!("{line}\nTEST COMPLETED");
        !crate::case_bugs_bop_rules::accepted_bugs_bop_report_with(&single, word)
    };
    let outcome = todo_outcome(&report, todos, failing);
    assert_eq!(
        outcome,
        TodoOutcome::Todo,
        "bugs original stream report={report:?}; expected exactly the TODO failures {todos:?}"
    );
}
