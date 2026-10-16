//! The verdict of a regression case that checks the log it prints.
//!
//! A case is decided from its log: each line is matched against the case-specific rules,
//! then the common rules, in that order, and the first matching rule gives its status.
//! TODO predicates declare failures the case is known to produce; a TODO that matches no
//! failing line is an unexpected improvement. Each case rebuilds its log from API calls
//! and checks it with these rules.
#![allow(dead_code)]

/// The status a matching rule gives a log line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleStatus {
    Failed,
    Skipped,
    Ignore,
    Ok,
}

/// One rule: its status and its pattern as a predicate on a log line.
#[derive(Clone, Copy)]
pub struct ParseRule {
    pub status: RuleStatus,
    pub pattern: &'static str,
    pub matches: fn(&str) -> bool,
}

/// Whether `word` occurs in `line` between word boundaries.
pub fn has_word(line: &str, word: &str, ignore_case: bool) -> bool {
    let (haystack, needle) = if ignore_case {
        (line.to_ascii_lowercase(), word.to_ascii_lowercase())
    } else {
        (line.to_owned(), word.to_owned())
    };
    haystack.match_indices(&needle).any(|(start, _)| {
        let before = haystack[..start].chars().next_back();
        let after = haystack[start + needle.len()..].chars().next();
        !before.is_some_and(is_word_char) && !after.is_some_and(is_word_char)
    })
}

fn is_word_char(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

/// `\b[Ff][Aa][Ii][Ll][0-9]\b`: "fail" followed by exactly one digit, as a whole word.
fn fail_digit(line: &str) -> bool {
    (0..=9).any(|digit| has_word(line, &format!("fail{digit}"), true))
}

/// `\b[Ee]xception\b`: only the first letter is case-insensitive.
fn exception_word(line: &str) -> bool {
    has_word(line, "exception", false) || has_word(line, "Exception", false)
}

fn file_not_found(line: &str) -> bool {
    line.contains("Tcl Exception: ")
        && (line.contains("file ") || line.contains("File "))
        && line.contains(" could not be found")
}

/// The rules shared by every case.
pub const COMMON_RULES: &[ParseRule] = &[
    ParseRule {
        status: RuleStatus::Skipped,
        pattern: "Tcl Exception: .*[fF]ile .* could not be found",
        matches: file_not_found,
    },
    ParseRule {
        status: RuleStatus::Ignore,
        pattern: "Tcl Exception: [*][*] Exception [*][*]",
        matches: |l| l.contains("Tcl Exception: ** Exception **"),
    },
    ParseRule {
        status: RuleStatus::Ignore,
        pattern: "Relative error of mass computation :",
        matches: |l| l.contains("Relative error of mass computation :"),
    },
    ParseRule {
        status: RuleStatus::Failed,
        pattern: "\\b[Ee]xception\\b",
        matches: exception_word,
    },
    ParseRule {
        status: RuleStatus::Failed,
        pattern: "\\b[Ee][Rr][Rr][Oo][Rr]\\b",
        matches: |l| has_word(l, "error", true),
    },
    ParseRule {
        status: RuleStatus::Failed,
        pattern: "\\b[Ff][Aa][Ii][Ll][0-9]\\b",
        matches: fail_digit,
    },
    ParseRule {
        status: RuleStatus::Failed,
        pattern: "\\b[Ff][Aa][Ii][Ll][Ee][Dd]\\b",
        matches: |l| has_word(l, "failed", true),
    },
    ParseRule {
        status: RuleStatus::Failed,
        pattern: "\\b[Ff][Aa][Ii][Ll][Uu][Rr][Ee]\\b",
        matches: |l| has_word(l, "failure", true),
    },
    ParseRule {
        status: RuleStatus::Failed,
        pattern: "Process killed by CPU limit",
        matches: |l| l.contains("Process killed by CPU limit"),
    },
    ParseRule {
        status: RuleStatus::Failed,
        pattern: "Process killed by elapsed limit",
        matches: |l| l.contains("Process killed by elapsed limit"),
    },
];

/// The rules of the regression cases.
pub const BUGS_RULES: &[ParseRule] = &[
    ParseRule {
        status: RuleStatus::Failed,
        pattern: "\\bFaulty\\b",
        matches: |l| has_word(l, "Faulty", false),
    },
    ParseRule {
        status: RuleStatus::Failed,
        pattern: "failure reading attribute",
        matches: |l| l.contains("failure reading attribute"),
    },
    ParseRule {
        status: RuleStatus::Skipped,
        pattern: "Error: unsupported locale specification",
        matches: |l| l.contains("Error: unsupported locale specification"),
    },
    ParseRule {
        status: RuleStatus::Ok,
        pattern: "Relative error of mass computation",
        matches: |l| l.contains("Relative error of mass computation"),
    },
];

/// The rules of the performance cases.
pub const PERF_RULES: &[ParseRule] = &[
    ParseRule {
        status: RuleStatus::Failed,
        pattern: "\\bFaulty\\b",
        matches: |l| has_word(l, "Faulty", false),
    },
    ParseRule {
        status: RuleStatus::Ignore,
        pattern: "Error: performance",
        matches: |l| l.contains("Error: performance"),
    },
];

/// The rules of the demonstration cases.
pub const DEMO_RULES: &[ParseRule] = &[
    ParseRule {
        status: RuleStatus::Ok,
        pattern: "Wanted exception",
        matches: |l| l.contains("Wanted exception"),
    },
    ParseRule {
        status: RuleStatus::Failed,
        pattern: "Failure message",
        matches: |l| l.contains("Failure message"),
    },
];

/// The first rule of the ordered rule sets that matches `line`, if any.
pub fn first_match<'a>(rule_sets: &[&'a [ParseRule]], line: &str) -> Option<&'a ParseRule> {
    rule_sets
        .iter()
        .flat_map(|rules| rules.iter())
        .find(|rule| (rule.matches)(line))
}

/// Checks a case log: every failing line must be covered by a TODO
/// predicate and every TODO must cover a failing line; returns the offending lines.
pub fn check_log(
    rule_sets: &[&[ParseRule]],
    todos: &[fn(&str) -> bool],
    log: &[String],
) -> Result<(), Vec<String>> {
    let failing: Vec<&String> = log
        .iter()
        .filter(|line| {
            first_match(rule_sets, line).is_some_and(|rule| rule.status == RuleStatus::Failed)
        })
        .collect();
    let mut problems: Vec<String> = failing
        .iter()
        .filter(|line| !todos.iter().any(|todo| todo(line)))
        .map(|line| format!("unexpected failure: {line}"))
        .collect();
    let unmatched = todos
        .iter()
        .filter(|todo| !failing.iter().any(|line| todo(line)))
        .count();
    if unmatched > 0 {
        problems.push(format!(
            "{unmatched} TODO statement(s) matched no failing line (improvement)"
        ));
    }
    if problems.is_empty() {
        Ok(())
    } else {
        Err(problems)
    }
}
