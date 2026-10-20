//! The test log a regression case writes and the verdict the test harness draws from it
//! (`testgrid`, `_run_test` and `_check_log`).
//!
//! `testgrid` runs each case in its own process in echo mode (`_run_test ... t`) and
//! takes that process's standard output as the log: `puts` text, and with `decho on` every
//! command's arguments and result. While `dlog on`, a command's arguments, output
//! and result go to the `dlog` buffer instead. The verdict scans the log line by line
//! against the `parse.rules` of the grid, the group and the root, in that order: a line
//! matching a bad word is a failure unless a `TODO` pattern matches it; a `TODO` that
//! never matches is an "IMPROVEMENT", and a log without `TEST COMPLETED` failed.
use super::pattern::Pattern;

/// The standard output of one case and its `decho`/`dlog` state.
pub struct ScriptLog {
    stdout: String,
    echo: bool,
    logging: bool,
    buffer: String,
}

impl Default for ScriptLog {
    fn default() -> Self {
        Self {
            stdout: String::new(),
            echo: true,
            logging: false,
            buffer: String::new(),
        }
    }
}

impl ScriptLog {
    /// script `puts text`: the text and a newline on standard output.
    pub fn add(&mut self, text: &str) {
        self.stdout.push_str(text);
        self.stdout.push('\n');
    }

    /// `std::cout` output of a running command: the `dlog` buffer while logging, else
    /// standard output.
    pub fn capture(&mut self, text: &str) {
        if self.logging {
            self.buffer.push_str(text);
        } else {
            self.stdout.push_str(text);
        }
    }

    /// Whether a command echoes its arguments and result (`decho on` or `dlog on`).
    pub fn echoes(&self) -> bool {
        self.echo || self.logging
    }

    /// `decho on` / `decho off`.
    pub fn set_echo(&mut self, on: bool) {
        self.echo = on;
    }

    /// `dlog on` / `dlog off`.
    pub fn set_logging(&mut self, on: bool) {
        self.logging = on;
    }

    /// `dlog reset`.
    pub fn reset(&mut self) {
        self.buffer.clear();
    }

    /// `dlog get`.
    pub fn buffer(&self) -> &str {
        &self.buffer
    }

    /// The case's standard output, which the harness checks.
    pub fn text(&self) -> &str {
        &self.stdout
    }
}

/// How a `parse.rules` line classifies a matching log line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleStatus {
    Failed,
    Skipped,
    Ignore,
    Ok,
}

/// One `parse.rules` line: `STATUS /regexp/ comment`.
pub type Rule = (RuleStatus, &'static str);

/// tests/parse.rules.
pub const ROOT_RULES: &[Rule] = &[
    (
        RuleStatus::Skipped,
        r"ScriptStep Exception: .*[fF]ile .* could not be found",
    ),
    (
        RuleStatus::Ignore,
        r"ScriptStep Exception: [*][*] Exception [*][*]",
    ),
    (RuleStatus::Ignore, r"Relative error of mass computation :"),
    (RuleStatus::Failed, r"\b[Ee]xception\b"),
    (RuleStatus::Failed, r"\b[Ee][Rr][Rr][Oo][Rr]\b"),
    (RuleStatus::Failed, r"\b[Ff][Aa][Ii][Ll][0-9]\b"),
    (RuleStatus::Failed, r"\b[Ff][Aa][Ii][Ll][Ee][Dd]\b"),
    (RuleStatus::Failed, r"\b[Ff][Aa][Ii][Ll][Uu][Rr][Ee]\b"),
    (RuleStatus::Failed, r"Process killed by CPU limit"),
    (RuleStatus::Failed, r"Process killed by elapsed limit"),
];

/// `bugs_parse`.rules.
pub const BUGS_RULES: &[Rule] = &[
    (RuleStatus::Failed, r"\bFaulty\b"),
    (RuleStatus::Failed, r"failure reading attribute"),
    (
        RuleStatus::Skipped,
        r"Error: unsupported locale specification",
    ),
    (RuleStatus::Ok, r"Relative error of mass computation"),
];

/// `lowalgos_parse`.rules.
pub const LOWALGOS_RULES: &[Rule] = &[(RuleStatus::Failed, r"\bFaulty\b")];

/// `geometry_parse`.rules.
pub const GEOMETRY_RULES: &[Rule] = &[(RuleStatus::Ok, r"Relative error of mass computation")];

/// `perf_parse`.rules.
pub const PERF_RULES: &[Rule] = &[
    (RuleStatus::Failed, r"\bFaulty\b"),
    (RuleStatus::Ignore, r"Error: performance"),
];

/// The harness verdict of a log.
#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    /// `OK`.
    Passed,
    /// `BAD (known problem)`: every failure is an expected `TODO` problem.
    KnownProblem,
    /// Any other status, with the harness reason.
    Failed(String),
}

/// `_check_log`: the verdict of `log` under `rules` (group rules first, then the root).
pub fn verdict(log: &str, rules: &[Rule]) -> Verdict {
    let rules: Vec<(RuleStatus, Pattern)> = rules
        .iter()
        .map(|(status, source)| (*status, Pattern::new(source)))
        .collect();
    let mut scan = LogScan::default();
    for line in log.split('\n') {
        scan.line(line, &rules);
    }
    scan.finish(log)
}

#[derive(Default)]
struct LogScan {
    todos: Vec<(Pattern, String, usize)>,
    required: Vec<(Pattern, String, usize)>,
    status: Option<String>,
}

impl LogScan {
    fn line(&mut self, line: &str, rules: &[(RuleStatus, Pattern)]) {
        if let Some(applies) = statement(line, "TODO") {
            if let Some(pattern) = applies {
                self.todos
                    .push((Pattern::new(pattern.trim()), line.into(), 0));
            }
            return;
        }
        if let Some(applies) = statement(line, "REQUIRED") {
            if let Some(pattern) = applies {
                self.required
                    .push((Pattern::new(pattern.trim_start()), line.into(), 0));
            }
            return;
        }
        if self.mark_required(line) {
            return;
        }
        let Some((status, _)) = rules.iter().find(|(_, pattern)| pattern.is_match(line)) else {
            return;
        };
        if let Some(todo) = self.todos.iter_mut().find(|todo| todo.0.is_match(line)) {
            todo.2 += 1;
            return;
        }
        if self.status.is_none() && matches!(status, RuleStatus::Failed | RuleStatus::Skipped) {
            self.status = Some(format!("{status:?}: {line}"));
        }
    }

    fn mark_required(&mut self, line: &str) -> bool {
        let mut marked = false;
        for required in self.required.iter_mut().filter(|r| r.0.is_match(line)) {
            required.2 += 1;
            marked = true;
        }
        marked
    }

    fn finish(self, log: &str) -> Verdict {
        if let Some(status) = self.status {
            return Verdict::Failed(status);
        }
        if !log.contains("TEST COMPLETED") {
            return Verdict::Failed("FAILED (no final message is found)".into());
        }
        if let Some(required) = self.required.iter().find(|r| r.2 == 0) {
            return Verdict::Failed(format!(
                "FAILED (REQUIRED statement {:?} is not found)",
                required.1
            ));
        }
        if let Some(todo) = self.todos.iter().find(|t| t.2 == 0) {
            return Verdict::Failed(format!(
                "IMPROVEMENT (expected problem {:?} is not detected)",
                todo.1
            ));
        }
        if self.todos.is_empty() {
            Verdict::Passed
        } else {
            Verdict::KnownProblem
        }
    }
}

/// A `TODO platforms: pattern` / `REQUIRED platforms: pattern` line: `Some(Some(pattern))`
/// for all platforms or Linux, `Some(None)` for other platforms (the harness skips the
/// line), `None` for any other line.
fn statement<'a>(line: &'a str, keyword: &str) -> Option<Option<&'a str>> {
    let rest = line.trim_start().strip_prefix(keyword)?.strip_prefix(' ')?;
    let (platforms, pattern) = rest.split_once(':')?;
    let platforms = platforms.to_ascii_lowercase();
    let applies = platforms
        .split(|c: char| !c.is_alphanumeric())
        .any(|word| word == "all" || word == "linux");
    Some(applies.then_some(pattern))
}

#[test]
fn verdict_follows_the_harness() {
    let rules = [LOWALGOS_RULES, ROOT_RULES].concat();
    assert_eq!(verdict("x\nTEST COMPLETED\n", &rules), Verdict::Passed);
    assert!(matches!(
        verdict("Error: x\nTEST COMPLETED\n", &rules),
        Verdict::Failed(_)
    ));
    let todo = "TODO OCC1 ALL: Faulty OCC236\nFaulty OCC236\nTEST COMPLETED\n";
    assert_eq!(verdict(todo, &rules), Verdict::KnownProblem);
    let improvement = "TODO OCC1 ALL: Faulty OCC236\nOK\nTEST COMPLETED\n";
    assert!(matches!(verdict(improvement, &rules), Verdict::Failed(_)));
    assert!(matches!(verdict("ok\n", &rules), Verdict::Failed(_)));
    let other_platform = "TODO OCC1 Windows: Faulty\nTEST COMPLETED\n";
    assert_eq!(verdict(other_platform, &rules), Verdict::Passed);
    let required = "REQUIRED ALL: Error: cannot\nError: cannot make\nTEST COMPLETED\n";
    assert_eq!(verdict(required, &rules), Verdict::Passed);
}
