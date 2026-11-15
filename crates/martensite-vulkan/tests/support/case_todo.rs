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
support_unit_tests! {
    #[path = "case_todo_tests.rs"]
    mod tests;
}
