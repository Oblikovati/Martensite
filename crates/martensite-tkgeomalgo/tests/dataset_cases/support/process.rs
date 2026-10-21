//! How the test harness runs a case: in a dedicated child process under the `cpulimit` of
//! its group's `begin` script (martensite-process-policy), the body, then the group's
//! `end` script, and the verdict of the log against the group's `parse.rules`.
use super::log::{
    BUGS_RULES, GEOMETRY_RULES, LOWALGOS_RULES, PERF_RULES, ROOT_RULES, Rule, Verdict, verdict,
};
use super::script::{Script, ScriptStep};
use super::session::Session;
#[path = "../../common/isolated_case.rs"]
mod isolated_case;

/// A test group of tests/: its `begin` CPU limit, `end` script and `parse.rules`.
#[derive(Clone, Copy, Debug)]
pub enum Group {
    /// the `bugs` group: `cpulimit 1000`.
    Bugs,
    /// the `geometry` group: `cpulimit 600`.
    Geometry,
    /// the `lowalgos` group: `cpulimit 60`.
    Lowalgos,
    /// the `perf` group: `cpulimit 500`.
    Perf,
}

impl Group {
    /// The `cpulimit` of the group's `begin` script.
    pub const fn seconds(self) -> i64 {
        match self {
            Group::Bugs => 1000,
            Group::Geometry => 600,
            Group::Lowalgos => 60,
            Group::Perf => 500,
        }
    }

    /// The group's rules, then the root rules (no grid of the cases has its own).
    fn rules(self) -> Vec<Rule> {
        let group = match self {
            Group::Bugs => BUGS_RULES,
            Group::Geometry => GEOMETRY_RULES,
            Group::Lowalgos => LOWALGOS_RULES,
            Group::Perf => PERF_RULES,
        };
        [group, ROOT_RULES].concat()
    }
}

/// A case body: the script of one test case.
pub type Body = fn(&mut Script) -> ScriptStep;

/// One `#[test]` per case body of `crate::cases::<grid>`, run by `runner(identity, group,
/// body)` with a fresh `session` in the child; the libtest name is `<grid>::<case>`.
/// `script_cases!(run_case, NoView, Group::Bugs, bugs_filling; bug16119);`
///
/// With `ignore = "reason"` first, the tests are `#[ignore]`d with that reason: a case
/// whose translation is complete but whose verdict is not OK yet, run explicitly with
/// `--ignored`.
macro_rules! script_cases {
    (ignore = $reason:literal, $runner:path, $session:expr, $group:expr, $grid:ident; $($case:ident),* $(,)?) => {$(
        $crate::support::process::script_cases!(@one [#[ignore = $reason]] $runner, $session, $group, $grid, $case);
    )*};
    ($runner:path, $session:expr, $group:expr, $grid:ident; $($case:ident),* $(,)?) => {$(
        $crate::support::process::script_cases!(@one [] $runner, $session, $group, $grid, $case);
    )*};
    (@one [$($attribute:tt)*] $runner:path, $session:expr, $group:expr, $grid:ident, $case:ident) => {
        #[test]
        $($attribute)*
        fn $case() {
            let identity = concat!(stringify!($grid), "::", stringify!($case));
            $runner(identity, $group, || {
                let mut session = $session;
                let body = crate::cases::$grid::$case;
                crate::support::process::run_script(&mut session, identity, $group, body);
            });
        }
    };
}
pub(crate) use script_cases;

/// `_run_test`: the body, the `end` script (`TEST COMPLETED`), or `ScriptStep Exception: ...`
/// when the body raised an uncaught error; then `_check_log`, which must give `OK` or
/// `BAD (known problem)`. With `MARTENSITE_CASE_LOG_DIR` set, the log is also written to
/// `<dir>/<identity>.log`.
pub fn run_script(session: &mut dyn Session, identity: &str, group: Group, body: Body) {
    let mut draw = Script::new(session);
    match body(&mut draw) {
        Ok(()) => draw.puts("TEST COMPLETED"),
        Err(error) if error.0.is_empty() => draw.puts("ScriptStep Exception: EMPTY"),
        Err(error) => draw.puts(&format!("ScriptStep Exception: {}", error.0)),
    }
    let log = draw.log.text().to_string();
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

/// `cpulimit seconds` inside a script: a new process limit and elapsed clock.
pub fn cpulimit(seconds: i64) {
    isolated_case::restart_budget(seconds);
}

/// Run `body` as the libtest case `identity` (its path below the target root) alone in a
/// child process under the group's elapsed budget; `run_case("bugs_filling::bug16119", Group::Bugs, || {})`.
pub fn run_case(identity: &str, group: Group, body: impl FnOnce()) {
    let seconds = group.seconds();
    isolated_case::run_isolated(identity, seconds, body);
}
