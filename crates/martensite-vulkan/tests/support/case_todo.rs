//! the `TODO` command lines: a case's known, expected failure on this platform.
//!
//! `puts "TODO <bug> Linux: <regexp>"` tells the test command that output lines matching
//! `<regexp>` are expected. The case then finishes as TODO, not FAILED, when every failing
//! line matches a TODO; a TODO that matches no failing line turns it into an IMPROVEMENT.
//! `puts "REQUIRED All: <regexp>"` names output the case must print: a matching line is not a
//! failure, and the case fails when no line matches.
//! The patterns of the translated cases use literal text where `.` matches any character;
//! any other regular-expression syntax is refused rather than misread.

/// The expected outcome of a case with TODO lines.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum TodoOutcome {
    /// Every failing line is expected and every TODO occurred.
    Todo,
    /// The named TODO did not occur.
    Improvement(&'static str),
    /// The listed failing lines match no TODO.
    Failed(Vec<String>),
}

/// Grade `report` against `todos`, with `failing` the group policy's verdict on one line.
/// Example: `todo_outcome(report, &["Error : The length"], |line| line.contains("Error"))`.
pub(crate) fn todo_outcome(
    report: &str,
    todos: &[&'static str],
    failing: impl Fn(&str) -> bool,
) -> TodoOutcome {
    let failed: Vec<&str> = report
        .lines()
        .filter(|line| !todo_declaration(line) && failing(line))
        .collect();
    let unexpected: Vec<String> = failed
        .iter()
        .filter(|line| !todos.iter().any(|todo| todo_matches(todo, line)))
        .map(|line| line.to_string())
        .collect();
    if !unexpected.is_empty() {
        return TodoOutcome::Failed(unexpected);
    }
    match todos
        .iter()
        .find(|todo| !failed.iter().any(|line| todo_matches(todo, line)))
    {
        Some(todo) => TodoOutcome::Improvement(todo),
        None => TodoOutcome::Todo,
    }
}

/// command-host consumes declaration records before failure rules; `todo_declaration("TODO bug ALL: Error")`.
fn todo_declaration(line: &str) -> bool {
    let Some((keyword, remainder)) = line.trim_start().split_once(' ') else {
        return false;
    };
    keyword.eq_ignore_ascii_case("TODO") && remainder.contains(':')
}

/// Whether the TODO regexp `pattern` occurs in `line`; `.` matches any character.
/// Example: `todo_matches("is 3025.4", "The length is 3025.43")` is true.
pub(crate) fn todo_matches(pattern: &str, line: &str) -> bool {
    let special = pattern.chars().find(|c| "\\^$*+?()[]{}|".contains(*c));
    assert!(
        special.is_none(),
        "TODO pattern {pattern:?} uses {special:?}; expected literal text with `.` wildcards"
    );
    let pattern: Vec<char> = pattern.chars().collect();
    let line: Vec<char> = line.chars().collect();
    (0..=line.len().saturating_sub(pattern.len()))
        .filter(|_| line.len() >= pattern.len())
        .any(|start| matches_at(&pattern, &line[start..]))
}

fn matches_at(pattern: &[char], text: &[char]) -> bool {
    pattern
        .iter()
        .zip(text)
        .all(|(expected, actual)| *expected == '.' || expected == actual)
}

/// A case with command-host TODO lines: its stream must finish as TODO under the V3d policy.
/// Example: `run_case_expecting("case", 1000, &["Error: The length"], body)`.
pub(crate) fn run_case_expecting(
    identity: &str,
    cpu_seconds: i64,
    todos: &[&'static str],
    body: impl FnOnce() -> Result<(), Box<dyn std::error::Error>>,
) {
    let Some(report) =
        crate::case_opengl_process_case::observed_stream(identity, cpu_seconds, body)
    else {
        return;
    };
    assert!(
        report.contains("TEST COMPLETED"),
        "V3d original stream report={report:?}; expected source completion"
    );
    let failing =
        |line: &str| !crate::case_v3d_policy::accepted_report(&format!("{line}\nTEST COMPLETED"));
    let outcome = todo_outcome(&report, todos, failing);
    assert_eq!(
        outcome,
        TodoOutcome::Todo,
        "V3d original stream report={report:?}; expected exactly the TODO failures {todos:?}"
    );
}

/// The lines of `report` that fail under `failing` and match no REQUIRED pattern, or the
/// first pattern that matches no line. Example:
/// `required_outcome(report, &["Error: dimension geometry is invalid"], |l| l.contains("Error"))`.
pub(crate) fn required_outcome(
    report: &str,
    required: &[&'static str],
    failing: impl Fn(&str) -> bool,
) -> Result<(), String> {
    if let Some(missing) = required
        .iter()
        .find(|pattern| !report.lines().any(|line| todo_matches(pattern, line)))
    {
        return Err(format!("REQUIRED {missing:?} matched no line"));
    }
    let unexpected: Vec<&str> = report
        .lines()
        .filter(|line| failing(line) && !required.iter().any(|p| todo_matches(p, line)))
        .collect();
    match unexpected.is_empty() {
        true => Ok(()),
        false => Err(format!("failing lines {unexpected:?}")),
    }
}

/// A case with command-host REQUIRED lines: its stream must print them and pass the V3d policy
/// otherwise. Example: `run_case_requiring("case", 1000, &["Error: invalid"], body)`.
pub(crate) fn run_case_requiring(
    identity: &str,
    cpu_seconds: i64,
    required: &[&'static str],
    body: impl FnOnce() -> Result<(), Box<dyn std::error::Error>>,
) {
    let Some(report) =
        crate::case_opengl_process_case::observed_stream(identity, cpu_seconds, body)
    else {
        return;
    };
    assert_required_report(&report, required);
}

fn assert_required_report(report: &str, required: &[&'static str]) {
    assert!(
        report.contains("TEST COMPLETED"),
        "V3d original stream report={report:?}; expected source completion"
    );
    let failing =
        |line: &str| !crate::case_v3d_policy::accepted_report(&format!("{line}\nTEST COMPLETED"));
    if let Err(problem) = required_outcome(report, required, failing) {
        panic!(
            "V3d original stream report={report:?}: {problem}; expected the REQUIRED lines {required:?}"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn error_line(line: &str) -> bool {
        line.contains("Error")
    }

    const DECLARATIONS: &str = "TODO bug ALL: Error : first\n  todo bug Linux: Error : second";
    const EXPECTED_ERRORS: &[&str] = &["Error : first", "Error : second"];

    #[test]
    fn declaration_headers_cannot_satisfy_actual_failures() {
        assert_eq!(
            todo_outcome(DECLARATIONS, EXPECTED_ERRORS, error_line),
            TodoOutcome::Improvement("Error : first")
        );
    }

    #[test]
    fn one_actual_failure_leaves_second_todo_as_improvement() {
        let report = format!("{DECLARATIONS}\nError : first");
        assert_eq!(
            todo_outcome(&report, EXPECTED_ERRORS, error_line),
            TodoOutcome::Improvement("Error : second")
        );
    }

    #[test]
    fn both_actual_failures_are_required_and_unexpected_errors_refuse() {
        let report = format!("{DECLARATIONS}\nError : first\nError : second");
        assert_eq!(
            todo_outcome(&report, EXPECTED_ERRORS, error_line),
            TodoOutcome::Todo
        );
        let unexpected = format!("{report}\nError : extra");
        assert!(matches!(
            todo_outcome(&unexpected, EXPECTED_ERRORS, error_line),
            TodoOutcome::Failed(_)
        ));
    }

    #[test]
    fn only_anchored_declaration_syntax_is_consumed() {
        for line in [
            "TODO bug ALL: Error",
            "  todo bug Linux: Error",
            "TODO : Error",
        ] {
            assert!(todo_declaration(line), "{line:?}");
        }
        for line in [
            "Error before TODO bug ALL: Error",
            "TODO Error without colon",
            "TODO\tbug ALL: Error",
        ] {
            assert!(!todo_declaration(line), "{line:?}");
        }
    }

    #[test]
    fn dot_matches_any_character_and_the_rest_is_literal() {
        assert!(todo_matches(
            "is 3025.4",
            "Error : The length is 3025.43, expected"
        ));
        assert!(todo_matches("is 3025.4", "is 3025x4"));
        assert!(!todo_matches("is 3025.4", "is 3026.4"));
        assert!(!todo_matches("a longer pattern", "short"));
    }

    #[test]
    fn expected_failures_finish_as_todo() {
        let report =
            "Error : The length of result shape is 3025.4, expected 3025.49\nTEST COMPLETED";
        let todos = ["Error : The length of result shape is 3025.4"];
        assert_eq!(todo_outcome(report, &todos, error_line), TodoOutcome::Todo);
    }

    #[test]
    fn a_todo_without_failure_is_an_improvement_and_other_errors_fail() {
        let todos = ["Error : The length of result shape is 302"];
        assert_eq!(
            todo_outcome("TEST COMPLETED", &todos, error_line),
            TodoOutcome::Improvement(todos[0])
        );
        let report = "Error : The area is 0\nTEST COMPLETED";
        assert_eq!(
            todo_outcome(report, &todos, error_line),
            TodoOutcome::Failed(vec!["Error : The area is 0".to_string()])
        );
    }

    #[test]
    fn required_lines_are_not_failures_and_must_occur() {
        let required = ["Error: dimension geometry is invalid"];
        let report = "Error: dimension geometry is invalid, -length dimension\nTEST COMPLETED";
        assert_eq!(required_outcome(report, &required, error_line), Ok(()));
        assert!(required_outcome("TEST COMPLETED", &required, error_line).is_err());
        let other = "Error: dimension geometry is invalid\nError : other\nTEST COMPLETED";
        assert!(required_outcome(other, &required, error_line).is_err());
    }

    #[test]
    #[should_panic(expected = "expected literal text")]
    fn other_regular_expression_syntax_is_refused() {
        todo_matches("value [0-9]+", "value 12");
    }
}
