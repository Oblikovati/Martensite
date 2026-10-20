//! The test group of the cases, its time budget and the harness run of one script: the
//! body, the group `end` script and the verdict of the log.
use super::log::{BUGS_RULES, ROOT_RULES, Rule, Verdict, verdict};
use super::script::{Script, ScriptStep};
#[path = "../../common/isolated_case.rs"]
mod isolated_case;

/// A test group of the cases.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Group {
    /// The bugs group: a 1000 s budget; its `end` script prints `TEST COMPLETED`.
    Bugs,
}

impl Group {
    /// The time budget of the group in seconds.
    pub const fn seconds(self) -> i64 {
        match self {
            Group::Bugs => 1000,
        }
    }

    /// The group's rules, then the root rules.
    fn rules(self) -> Vec<Rule> {
        [BUGS_RULES, ROOT_RULES].concat()
    }
}

/// The body, the group `end` script, or `Script exception: ...` when the body raised an
/// uncaught error; then the verdict, which must be `OK` or `BAD (known problem)`. With
/// `MARTENSITE_CASE_LOG_DIR` set, the log is also written to `<dir>/<identity>.log`.
pub fn run_script(identity: &str, group: Group, body: impl FnOnce(&mut Script) -> ScriptStep) {
    let mut script = Script::default();
    match body(&mut script) {
        Ok(()) => script.puts("TEST COMPLETED"),
        Err(error) if error.0.is_empty() => script.puts("Script exception: EMPTY"),
        Err(error) => script.puts(&format!("Script exception: {}", error.0)),
    }
    let log = script.log.text().to_string();
    println!("{log}");
    if let Some(directory) = std::env::var_os("MARTENSITE_CASE_LOG_DIR") {
        let path = std::path::Path::new(&directory).join(format!("{identity}.log"));
        std::fs::write(&path, &log).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    }
    let verdict = verdict(&log, &group.rules());
    assert!(
        matches!(verdict, Verdict::Passed | Verdict::KnownProblem),
        "case verdict {verdict:?}; expected OK or BAD (known problem)"
    );
}

/// Run `body` as the libtest case `identity` alone in a child process under the group's
/// elapsed budget; `run_case("bugs_modalg_6::bug25214", Group::Bugs, || {})`.
pub fn run_case(identity: &str, group: Group, body: impl FnOnce()) {
    isolated_case::run_isolated(identity, group.seconds(), body);
}
