//! The test log of one case and its verdict.
//!
//! The test harness runs scenario with `dlog on`, so the result of every scenario command is
//! logged, even inside `set v [cmd ...]`, together with the script's `puts` lines. The
//! log is then read line by line against the grid, group and common `parse.rules`: a
//! `TODO platforms: pattern` line declares an expected problem, a `REQUIRED platforms:
//! pattern` line a message that must appear; a line matching a bad word fails the case
//! unless a TODO matches it; a TODO that matches no bad line is an IMPROVEMENT, which
//! fails here too, as the TKBRep cases treat a disappeared defect.
//!
//! The cases log with [`puts`] (a script `puts`) and [`result`] (a scenario command result);
//! TODO and REQUIRED patterns are literal texts here (none of the regression cases uses a
//! regexp metacharacter in them).
use std::cell::RefCell;

thread_local! {
    static LOG: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

/// A script `puts text`.
pub fn puts(text: &str) {
    println!("{text}");
    LOG.with(|log| log.borrow_mut().push(text.to_string()));
}

/// The logged result of a scenario command (nothing for an empty result).
pub fn result(text: &str) {
    if !text.is_empty() {
        puts(text);
    }
}

/// `puts "TODO All: pattern"` (the platforms of the regression cases include Linux).
pub fn todo(pattern: &str) {
    puts(&format!("TODO OCC12345 ALL: {pattern}"));
}

/// `puts "REQUIRED All: pattern"`.
pub fn required(pattern: &str) {
    puts(&format!("REQUIRED All: {pattern}"));
}

/// The case's log so far.
pub fn text() -> String {
    LOG.with(|log| log.borrow().join("\n"))
}

/// The status of a rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Rule {
    Failed,
    Ignore,
    Ok,
    Skipped,
}

/// A bad-word rule: a literal, matched case-insensitively or not, with word boundaries
/// on both sides or not.
struct BadWord {
    status: Rule,
    text: &'static str,
    ignore_case: bool,
    word: bool,
    /// `\b[Ff][Aa][Ii][Ll][0-9]\b`: the literal then one digit.
    digit: bool,
}

const fn bw(status: Rule, text: &'static str, ignore_case: bool, word: bool) -> BadWord {
    BadWord {
        status,
        text,
        ignore_case,
        word,
        digit: false,
    }
}

/// tests/parse.rules, in order.
const COMMON: &[BadWord] = &[
    bw(Rule::Skipped, "Tcl Exception: ", false, false),
    bw(Rule::Ignore, "Tcl Exception: ** Exception **", false, false),
    bw(
        Rule::Ignore,
        "Relative error of mass computation :",
        false,
        false,
    ),
    bw(Rule::Failed, "exception", true, true),
    bw(Rule::Failed, "error", true, true),
    BadWord {
        status: Rule::Failed,
        text: "fail",
        ignore_case: true,
        word: true,
        digit: true,
    },
    bw(Rule::Failed, "failed", true, true),
    bw(Rule::Failed, "failure", true, true),
    bw(Rule::Failed, "Process killed by CPU limit", false, false),
    bw(
        Rule::Failed,
        "Process killed by elapsed limit",
        false,
        false,
    ),
];

/// The group rules (tests/<group>/parse.rules) the regression groups read before COMMON.
fn group_rules(group: &str) -> &'static [BadWord] {
    const FAULTY: BadWord = bw(Rule::Failed, "Faulty", false, true);
    const RELATIVE: BadWord = bw(Rule::Ok, "Relative error of mass computation", false, false);
    const BUGS: &[BadWord] = &[
        FAULTY,
        bw(Rule::Failed, "failure reading attribute", false, false),
        bw(
            Rule::Skipped,
            "Error: unsupported locale specification",
            false,
            false,
        ),
        RELATIVE,
    ];
    const GEOMETRY: &[BadWord] = &[RELATIVE];
    const FAULTY_ONLY: &[BadWord] = &[FAULTY];
    const PERF: &[BadWord] = &[FAULTY, bw(Rule::Ignore, "Error: performance", false, false)];
    match group {
        "bugs" => BUGS,
        "geometry" => GEOMETRY,
        "heal" | "lowalgos" => FAULTY_ONLY,
        "perf" => PERF,
        other => panic!("no parse rules for group {other}"),
    }
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

impl BadWord {
    fn matches(&self, line: &str) -> bool {
        if self.text == "Tcl Exception: " {
            // A missing data file produces this skipped-case result.
            return line.contains("Tcl Exception: ") && line.contains("could not be found");
        }
        let (hay, needle) = if self.ignore_case {
            (line.to_lowercase(), self.text.to_lowercase())
        } else {
            (line.to_string(), self.text.to_string())
        };
        let chars: Vec<char> = hay.chars().collect();
        let needle: Vec<char> = needle.chars().collect();
        let extra = usize::from(self.digit);
        if chars.len() < needle.len() + extra {
            return false;
        }
        (0..=chars.len() - needle.len() - extra).any(|at| {
            if chars[at..at + needle.len()] != needle[..] {
                return false;
            }
            let end = at + needle.len() + extra;
            if self.digit && !chars[end - 1].is_ascii_digit() {
                return false;
            }
            !self.word
                || ((at == 0 || !is_word_char(chars[at - 1]))
                    && (end == chars.len() || !is_word_char(chars[end])))
        })
    }
}

/// The parse-rule verdict of the case's log in `group`, after the group `end` script's
/// `puts "TEST COMPLETED"`: panics unless the status is OK or BAD (known problem).
pub fn finish(group: &str) {
    puts("TEST COMPLETED");
    let lines: Vec<String> = LOG.with(|log| log.borrow().clone());
    let rules: Vec<&BadWord> = group_rules(group).iter().chain(COMMON.iter()).collect();
    let mut todos: Vec<(String, usize)> = Vec::new();
    let mut requs: Vec<(String, usize)> = Vec::new();
    let mut failures = Vec::new();
    for line in lines.iter().flat_map(|text| text.split('\n')) {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("TODO ") {
            let (_, pattern) = rest.split_once(':').expect("TODO platforms: pattern");
            todos.push((pattern.trim().to_string(), 0));
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("REQUIRED ") {
            let (_, pattern) = rest.split_once(':').expect("REQUIRED platforms: pattern");
            requs.push((pattern.trim_start().to_string(), 0));
            continue;
        }
        if let Some(required) = requs.iter_mut().find(|(p, _)| line.contains(p.as_str())) {
            required.1 += 1;
            continue;
        }
        if let Some(rule) = rules.iter().find(|rule| rule.matches(line)) {
            if let Some(todo) = todos.iter_mut().find(|(p, _)| line.contains(p.as_str())) {
                todo.1 += 1;
            } else if rule.status == Rule::Failed || rule.status == Rule::Skipped {
                failures.push(line.to_string());
            }
        }
    }
    assert!(
        failures.is_empty(),
        "FAILED: {}\n--- log ---\n{}",
        failures.join(" | "),
        lines.join("\n")
    );
    for (pattern, count) in &requs {
        assert!(
            *count > 0,
            "FAILED (REQUIRED statement `{pattern}` is not found)"
        );
    }
    for (pattern, count) in &todos {
        assert!(
            *count > 0,
            "IMPROVEMENT (expected problem TODO `{pattern}` is not detected)"
        );
    }
}

#[test]
fn bad_words_follow_the_parse_rules() {
    let rules: Vec<&BadWord> = group_rules("bugs").iter().chain(COMMON.iter()).collect();
    let first = |line: &str| rules.iter().find(|r| r.matches(line)).map(|r| r.status);
    assert_eq!(
        first("Faulty shapes in variables faulty_1"),
        Some(Rule::Failed)
    );
    assert_eq!(
        first("Relative error of mass computation : 1e-05"),
        Some(Rule::Ok)
    );
    assert_eq!(
        first("Error : The area of result shape is 0"),
        Some(Rule::Failed)
    );
    assert_eq!(first("nb alone Vertices : 0 (errors)"), None);
    assert_eq!(first("checkshape exception"), Some(Rule::Failed));
    assert_eq!(first("step FAIL1 here"), Some(Rule::Failed));
    assert_eq!(first("This shape seems to be valid"), None);
    assert_eq!(first("NotFaulty"), None);
}
