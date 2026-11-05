//! Demo originals inherit process policy and retain their exact first-matching stream rules.
use crate::case_opengl_process_case::observed_inherited_stream;
use std::error::Error;

/// Run the complete demo without installing a CPU policy; `run_demo("case", body)`.
pub(crate) fn run_demo(identity: &str, body: impl FnOnce() -> Result<(), Box<dyn Error>>) {
    let Some(report) = observed_inherited_stream(identity, body) else {
        return;
    };
    assert!(
        DemoLogRules::accepted(&report),
        "demo original stream rejected: report={report:?}; expected END and exact demo/common rules"
    );
}

struct DemoLogRules;
impl DemoLogRules {
    fn accepted(report: &str) -> bool {
        report
            .lines()
            .filter(|line| *line == "TEST COMPLETED")
            .count()
            == 1
            && report.lines().all(Self::accepted_line)
    }
    fn accepted_line(line: &str) -> bool {
        if line.contains("Wanted exception") {
            return true;
        }
        if line.contains("Failure message") {
            return false;
        }
        crate::case_common_bop_rules::accepted_inherited_common_line_with(
            line,
            crate::case_word_profile::word_character,
        )
    }
}

#[test]
fn demo_rules_preserve_ok_precedence_over_common_errors_and_failure() {
    assert!(DemoLogRules::accepted(
        "Error Failure message Wanted exception\nTEST COMPLETED\n"
    ));
    assert!(!DemoLogRules::accepted("Failure message\nTEST COMPLETED\n"));
}
#[test]
fn demo_rules_require_end_and_keep_unexpected_common_failures() {
    assert!(!DemoLogRules::accepted("Wanted exception\n"));
    assert!(!DemoLogRules::accepted(
        "Error: genuine defect\nTEST COMPLETED\n"
    ));
    assert!(DemoLogRules::accepted("Done.\nTEST COMPLETED\n"));
}

#[test]
fn demo_rules_refuse_embedded_or_duplicate_completion() {
    assert!(!DemoLogRules::accepted("prefix TEST COMPLETED suffix\n"));
    assert!(!DemoLogRules::accepted("TEST COMPLETED\nTEST COMPLETED\n"));
}
