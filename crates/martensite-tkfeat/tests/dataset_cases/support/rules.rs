//! The verdict of a whole case stream: the parse rules of the test harness, line by line.
//!
//! A `TODO platforms: pattern` line declares an expected problem and a `REQUIRED platforms:
//! pattern` line a message that must appear. A line matching a bad word fails the case
//! unless a TODO matches it; a TODO no line matches is an improvement. Exactly one completion
//! line is required; the harness only asks for one, the single run of the end script is
//! this contract's addition.
use super::todo_pattern::TodoPattern;

/// The platform a `TODO` or `REQUIRED` line must name, besides `all`.
const PLATFORM: &str = "Linux";
/// What starts the line of a script stopped by an error.
pub const EXCEPTION_PREFIX: &str = "ScriptStep Exception: ";
/// The line the group `end` writes.
pub const COMPLETED: &str = "TEST COMPLETED";

/// What a rule makes of a line that matches it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleStatus {
    Failed,
    Skipped,
    Ignore,
    Ok,
}

/// How a rule recognizes a line.
#[derive(Clone, Copy, Debug)]
pub enum RuleMatch {
    /// A whole word accepted by the predicate.
    Word(fn(&str) -> bool),
    /// A literal anywhere in the line.
    Contains(&'static str),
    /// A script stopped by a missing input: an exception line naming a file not found.
    MissingFile,
}

/// One line of a `parse.rules` file.
#[derive(Clone, Copy, Debug)]
pub struct Rule {
    pub status: RuleStatus,
    pub matching: RuleMatch,
}

const fn rule(status: RuleStatus, matching: RuleMatch) -> Rule {
    Rule { status, matching }
}

fn is_faulty(word: &str) -> bool {
    word == "Faulty"
}

fn is_exception(word: &str) -> bool {
    word == "exception" || word == "Exception"
}

fn is_error(word: &str) -> bool {
    word.eq_ignore_ascii_case("error")
}

fn is_fail_digit(word: &str) -> bool {
    let bytes = word.as_bytes();
    bytes.len() == 5 && bytes[4].is_ascii_digit() && word[..4].eq_ignore_ascii_case("fail")
}

fn is_failed(word: &str) -> bool {
    word.eq_ignore_ascii_case("failed")
}

fn is_failure(word: &str) -> bool {
    word.eq_ignore_ascii_case("failure")
}

/// The rules of `tests/boolean`, read before [`COMMON_RULES`].
pub const BOOLEAN_RULES: &[Rule] = &[
    rule(RuleStatus::Failed, RuleMatch::Word(is_faulty)),
    rule(
        RuleStatus::Ok,
        RuleMatch::Contains("Relative error of mass computation"),
    ),
];

/// The rules every group reads last.
pub const COMMON_RULES: &[Rule] = &[
    rule(RuleStatus::Skipped, RuleMatch::MissingFile),
    rule(
        RuleStatus::Ignore,
        RuleMatch::Contains("ScriptStep Exception: ** Exception **"),
    ),
    rule(
        RuleStatus::Ignore,
        RuleMatch::Contains("Relative error of mass computation :"),
    ),
    rule(RuleStatus::Failed, RuleMatch::Word(is_exception)),
    rule(RuleStatus::Failed, RuleMatch::Word(is_error)),
    rule(RuleStatus::Failed, RuleMatch::Word(is_fail_digit)),
    rule(RuleStatus::Failed, RuleMatch::Word(is_failed)),
    rule(RuleStatus::Failed, RuleMatch::Word(is_failure)),
    rule(
        RuleStatus::Failed,
        RuleMatch::Contains("Process killed by CPU limit"),
    ),
    rule(
        RuleStatus::Failed,
        RuleMatch::Contains("Process killed by elapsed limit"),
    ),
];

impl Rule {
    fn matches(&self, line: &str) -> bool {
        match self.matching {
            RuleMatch::Word(accepted) => line
                .split(|c: char| !(c.is_alphanumeric() || c == '_'))
                .any(accepted),
            RuleMatch::Contains(text) => line.contains(text),
            RuleMatch::MissingFile => names_a_missing_file(line),
        }
    }
}

fn names_a_missing_file(line: &str) -> bool {
    let Some((_, rest)) = line.split_once(EXCEPTION_PREFIX) else {
        return false;
    };
    ["file ", "File "].into_iter().any(|word| {
        rest.split_once(word)
            .is_some_and(|(_, tail)| tail.contains(" could not be found"))
    })
}

/// The harness verdict of one case stream.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// No problem.
    Ok,
    /// Only the problems its TODO lines declare.
    KnownProblem,
    /// A bad word, a missing message or a wrong number of completion lines.
    Failed(String),
    /// A missing input.
    Skipped(String),
    /// A declared problem did not appear.
    Improvement(String),
}

impl Verdict {
    /// The two verdicts a passing test may carry; `Verdict::ok.passes()`.
    pub fn passes(&self) -> bool {
        matches!(self, Verdict::Ok | Verdict::KnownProblem)
    }
}

struct Declared {
    pattern: TodoPattern,
    text: String,
    unstable: bool,
    count: usize,
}

/// The pattern of a `TODO`/`REQUIRED` line for this platform: `(platforms, pattern text)`.
fn declaration<'a>(line: &'a str, keyword: &str) -> Option<(&'a str, &'a str)> {
    let rest = strip_prefix_ignore_case(line.trim_start(), keyword)?;
    let (platforms, pattern) = rest.split_once(':')?;
    names_this_platform(platforms).then_some((platforms, pattern.trim()))
}

fn strip_prefix_ignore_case<'a>(line: &'a str, keyword: &str) -> Option<&'a str> {
    let head = line.get(..keyword.len())?;
    head.eq_ignore_ascii_case(keyword)
        .then(|| &line[keyword.len()..])
}

fn names_this_platform(platforms: &str) -> bool {
    platforms
        .split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .any(|word| word.eq_ignore_ascii_case("all") || word.eq_ignore_ascii_case(PLATFORM))
}

fn is_declaration(line: &str, keyword: &str) -> bool {
    strip_prefix_ignore_case(line.trim_start(), keyword).is_some_and(|rest| rest.contains(':'))
}

/// One pass over a stream, in the harness's order of precedence.
#[derive(Default)]
struct Grading {
    todos: Vec<Declared>,
    required: Vec<Declared>,
    status: Option<Verdict>,
    incomplete_declared: bool,
}

impl Grading {
    fn read_line(&mut self, line: &str, rules: &[&Rule]) {
        if is_declaration(line, "TODO ") {
            self.declare_todo(line);
            return;
        }
        if is_declaration(line, "REQUIRED ") {
            self.declare_required(line);
            return;
        }
        if self.required_matches(line) {
            return;
        }
        if let Some(rule) = rules.iter().find(|rule| rule.matches(line)) {
            self.apply_rule(rule, line);
        }
    }

    fn declare_todo(&mut self, line: &str) {
        let Some((platforms, text)) = declaration(line, "TODO ") else {
            return;
        };
        self.incomplete_declared |= text == "TEST INCOMPLETE";
        self.todos.push(Declared {
            pattern: TodoPattern::compile(text),
            text: text.to_owned(),
            unstable: platforms.contains('?'),
            count: 0,
        });
    }

    fn declare_required(&mut self, line: &str) {
        let Some((_, text)) = declaration(line, "REQUIRED ") else {
            return;
        };
        self.required.push(Declared {
            pattern: TodoPattern::compile(text),
            text: text.to_owned(),
            unstable: false,
            count: 0,
        });
    }

    fn required_matches(&mut self, line: &str) -> bool {
        let mut marked = false;
        for required in &mut self.required {
            if required.pattern.matches(line) {
                required.count += 1;
                marked = true;
            }
        }
        marked
    }

    fn apply_rule(&mut self, rule: &Rule, line: &str) {
        if let Some(todo) = self
            .todos
            .iter_mut()
            .find(|todo| todo.pattern.matches(line))
        {
            todo.count += 1;
            return;
        }
        if self.status.is_some() {
            return;
        }
        self.status = match rule.status {
            RuleStatus::Failed => Some(Verdict::Failed(line.to_owned())),
            RuleStatus::Skipped => Some(Verdict::Skipped(line.to_owned())),
            RuleStatus::Ok | RuleStatus::Ignore => None,
        };
    }

    fn completion_problem(&mut self, stream: &str) -> Option<Verdict> {
        let completed = stream.matches(COMPLETED).count();
        if completed == 1 {
            return None;
        }
        if completed == 0 && self.incomplete_declared {
            for todo in self
                .todos
                .iter_mut()
                .filter(|t| t.text == "TEST INCOMPLETE")
            {
                todo.count += 1;
            }
            return None;
        }
        Some(Verdict::Failed(format!(
            "{completed} lines of {COMPLETED:?}; expected exactly one"
        )))
    }

    fn missing_required(&self) -> Option<Verdict> {
        let missing = self.required.iter().find(|required| required.count == 0)?;
        Some(Verdict::Failed(format!(
            "REQUIRED statement {:?} is not found",
            missing.text
        )))
    }

    fn undetected_todo(&self) -> Option<Verdict> {
        let stale = self.todos.iter().find(|t| !t.unstable && t.count == 0)?;
        Some(Verdict::Improvement(format!(
            "expected problem TODO {:?} is not detected",
            stale.text
        )))
    }

    fn finish(mut self, stream: &str) -> Verdict {
        if let Some(status) = self.status.take() {
            return status;
        }
        if let Some(problem) = self.completion_problem(stream) {
            return problem;
        }
        if let Some(problem) = self.missing_required().or_else(|| self.undetected_todo()) {
            return problem;
        }
        if self.todos.iter().any(|todo| todo.count > 0) {
            return Verdict::KnownProblem;
        }
        Verdict::Ok
    }
}

/// Grades a complete stream with the group's rules, then the common ones.
/// Example: `grade("TEST COMPLETED\n", BOOLEAN_RULES) == Verdict::ok`.
pub fn grade(stream: &str, group_rules: &[Rule]) -> Verdict {
    let rules: Vec<&Rule> = group_rules.iter().chain(COMMON_RULES).collect();
    let mut grading = Grading::default();
    for line in stream.split('\n') {
        grading.read_line(line, &rules);
    }
    grading.finish(stream)
}

#[cfg(test)]
#[path = "rules_tests.rs"]
mod tests;
