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
    let report = "Error : The length of result shape is 3025.4, expected 3025.49\nTEST COMPLETED";
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
