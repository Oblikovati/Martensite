//! The rules of these scripts' groups, read before the shared common rules.
pub use martensite_boolean_cases::support::rules::Rule;
use martensite_boolean_cases::support::rules::{RuleMatch, RuleStatus};

const fn rule(status: RuleStatus, matching: RuleMatch) -> Rule {
    Rule { status, matching }
}

fn is_faulty(word: &str) -> bool {
    word == "Faulty"
}

fn is_invalid(word: &str) -> bool {
    word == "invalid"
}

/// The rules of the document scripts.
pub const DOCUMENT_RULES: &[Rule] = &[
    rule(
        RuleStatus::Ignore,
        RuleMatch::Contains("on screen buffer is used for image dump"),
    ),
    // The script catches this refusal: querying a label whose presentation was unset.
    rule(
        RuleStatus::Ignore,
        RuleMatch::Contains("Syntax error: '0:2' label has no presentation"),
    ),
    rule(RuleStatus::Failed, RuleMatch::Word(is_invalid)),
];

/// The rules of the scripts that do not test a document's transactions.
pub const BUG_RULES: &[Rule] = &[
    rule(RuleStatus::Failed, RuleMatch::Word(is_faulty)),
    rule(
        RuleStatus::Failed,
        RuleMatch::Contains("failure reading attribute"),
    ),
    rule(
        RuleStatus::Skipped,
        RuleMatch::Contains("Error: unsupported locale specification"),
    ),
    rule(
        RuleStatus::Ok,
        RuleMatch::Contains("Relative error of mass computation"),
    ),
];

/// The rules of the viewer scripts.
pub const VIEWER_RULES: &[Rule] = &[rule(RuleStatus::Failed, RuleMatch::Word(is_faulty))];

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_boolean_cases::support::rules::{Verdict, grade};

    #[test]
    fn the_document_rules_grade_what_the_scripts_write() {
        let completed = "TEST COMPLETED\n";
        assert_eq!(grade(completed, DOCUMENT_RULES), Verdict::Ok);
        let refused = format!("Syntax error: '0:2' label has no presentation\n{completed}");
        assert_eq!(grade(&refused, DOCUMENT_RULES), Verdict::Ok);
        let invalid = format!("RED light is invalid\n{completed}");
        assert!(!grade(&invalid, DOCUMENT_RULES).passes());
        let error = format!("AISColor command: Error\n{completed}");
        assert!(!grade(&error, DOCUMENT_RULES).passes());
        assert!(!grade("", DOCUMENT_RULES).passes());
    }

    #[test]
    fn the_bug_and_viewer_rules_fail_a_faulty_line() {
        let faulty = "Faulty OCC1\nTEST COMPLETED\n";
        assert!(!grade(faulty, BUG_RULES).passes());
        assert!(!grade(faulty, VIEWER_RULES).passes());
    }
}
