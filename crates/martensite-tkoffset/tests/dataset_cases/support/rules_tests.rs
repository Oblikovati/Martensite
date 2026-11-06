//! The parse rules line by line, and the harness's order of precedence over a stream.
use super::*;

fn graded(stream: &str) -> Verdict {
    grade(stream, BOOLEAN_RULES)
}

fn failed_on(stream: &str, line: &str) {
    assert_eq!(graded(stream), Verdict::Failed(line.to_owned()), "{stream}");
}

#[test]
fn a_clean_stream_with_warnings_is_ok() {
    let stream = "Warning: Unable to orient the shape correctly\nTEST COMPLETED\n";
    assert_eq!(graded(stream), Verdict::Ok);
}

#[test]
fn every_common_bad_word_fails() {
    for line in [
        "Error : The area of result shape is 3, expected 4",
        "an ERROR here",
        "An exception was raised",
        "Exception raised",
        "step FAIL1 here",
        "it failed.",
        "Failure of a step",
        "Process killed by CPU limit",
        "Process killed by elapsed limit",
    ] {
        failed_on(&format!("{line}\nTEST COMPLETED\n"), line);
    }
}

#[test]
fn only_whole_words_of_the_right_case_fail() {
    for line in [
        "EXCEPTION in upper case is not a rule",
        "errors in the plural are not the word",
        "nb alone Vertices : 0 (errorless)",
        "NotFaulty",
        "faulty in lower case",
        "fail12 has two digits",
        "This shape seems to be valid",
    ] {
        assert_eq!(
            graded(&format!("{line}\nTEST COMPLETED\n")),
            Verdict::Ok,
            "{line}"
        );
    }
}

#[test]
fn the_group_rule_on_faulty_comes_before_the_ignored_mass_error() {
    failed_on(
        "Relative error of mass computation : Faulty\nTEST COMPLETED\n",
        "Relative error of mass computation : Faulty",
    );
    let harmless = "Relative error of mass computation : Error\nTEST COMPLETED\n";
    assert_eq!(graded(harmless), Verdict::Ok);
}

#[test]
fn a_missing_input_skips_and_a_script_exception_is_ignored() {
    let missing = "ScriptStep Exception: File a.brep could not be found";
    assert_eq!(
        graded(&format!("{missing}\nTEST COMPLETED\n")),
        Verdict::Skipped(missing.to_owned())
    );
    let ignored = "ScriptStep Exception: ** Exception ** \nTEST COMPLETED\n";
    assert_eq!(graded(ignored), Verdict::Ok);
}

#[test]
fn only_a_file_not_found_skips() {
    let other = "ScriptStep Exception: shape could not be found";
    let verdict = graded(&format!("{other}\nTEST COMPLETED\n"));
    assert_eq!(verdict, Verdict::Failed(other.to_owned()));
}

#[test]
fn the_first_bad_line_gives_the_status() {
    failed_on(
        "Error first\nFaulty second\nTEST COMPLETED\n",
        "Error first",
    );
}

#[test]
fn the_completion_line_must_appear_exactly_once() {
    assert!(matches!(graded("no end\n"), Verdict::Failed(line) if line.contains("0 lines")));
    let twice = "TEST COMPLETED\nTEST COMPLETED\n";
    assert!(matches!(graded(twice), Verdict::Failed(line) if line.contains("2 lines")));
}

#[test]
fn a_bad_word_outranks_a_missing_completion() {
    failed_on("Error first\n", "Error first");
}

#[test]
fn a_matched_todo_is_a_known_problem() {
    let stream = "TODO 1 ALL: Faulty shapes in variables\n\
        Faulty shapes in variables faulty_1 to faulty_2 \nTEST COMPLETED\n";
    assert_eq!(graded(stream), Verdict::KnownProblem);
}

#[test]
fn a_todo_only_excuses_its_own_line() {
    let stream = "TODO 1 ALL: Faulty shapes in variables\n\
        Faulty shapes in variables\nError : other\nTEST COMPLETED\n";
    failed_on(stream, "Error : other");
}

#[test]
fn a_todo_that_nothing_matches_is_an_improvement() {
    let stream = "TODO 1 ALL: Error : The area of result shape is\nTEST COMPLETED\n";
    assert!(matches!(graded(stream), Verdict::Improvement(_)));
}

#[test]
fn an_unstable_todo_may_stay_unmatched() {
    let stream = "TODO ?1 ALL: Error : The area\nTEST COMPLETED\n";
    assert_eq!(graded(stream), Verdict::Ok);
}

#[test]
fn a_todo_for_another_platform_excuses_nothing() {
    failed_on("TODO 1 Windows: Error\nError\nTEST COMPLETED\n", "Error");
    let here = "TODO CR33225 Linux: Error : The area\nError : The area\nTEST COMPLETED\n";
    assert_eq!(graded(here), Verdict::KnownProblem);
}

#[test]
fn a_declared_incomplete_case_needs_no_completion_line() {
    assert_eq!(
        graded("TODO ALL: TEST INCOMPLETE\nnot done\n"),
        Verdict::KnownProblem
    );
}

#[test]
fn a_required_message_must_appear_and_outranks_the_bad_words() {
    let stream = "REQUIRED All: Error: expected text\nError: expected text\nTEST COMPLETED\n";
    assert_eq!(graded(stream), Verdict::Ok);
    let missing = "REQUIRED All: Error: expected text\nTEST COMPLETED\n";
    assert!(matches!(graded(missing), Verdict::Failed(line) if line.contains("REQUIRED")));
    let elsewhere = "REQUIRED Windows: Error: expected text\nTEST COMPLETED\n";
    assert_eq!(graded(elsewhere), Verdict::Ok);
}
