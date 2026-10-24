//! Fixture groups define CPU budgets and completion markers.
//! Budgeted cases run in a dedicated child with the limit applied before the body.
//! The runner evaluates the body, group teardown and recorded log verdict.
use super::draw::{Draw, Tcl};
use super::log::{BUGS_RULES, ROOT_RULES, Rule, RuleStatus, Verdict, verdict};
use martensite_process_policy::{
    case_runner::{CaseRunner, CaseSpec, FactorProfile},
    reported_process_limit,
};

/// tests/demo/parse.rules.
const DEMO_RULES: &[Rule] = &[
    (RuleStatus::Ok, r"Wanted exception"),
    (RuleStatus::Failed, r"Failure message"),
];

/// A test group of the cases.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Group {
    /// tests/bugs: `begin` sets `cpulimit 1000`; `end` prints `TEST COMPLETED`.
    Bugs,
    /// tests/demo: no `begin` or `end` script; the case prints `TEST COMPLETED` itself.
    Demo,
}

impl Group {
    /// The `cpulimit` of the group's `begin` script, if it sets one.
    pub const fn seconds(self) -> Option<i64> {
        match self {
            Group::Bugs => Some(1000),
            Group::Demo => None,
        }
    }

    /// Whether the group's `end` script prints `TEST COMPLETED`.
    const fn end_completes(self) -> bool {
        matches!(self, Group::Bugs)
    }

    /// The group's rules, then the root rules (the grids have none of their own).
    fn rules(self) -> Vec<Rule> {
        let group = match self {
            Group::Bugs => BUGS_RULES,
            Group::Demo => DEMO_RULES,
        };
        [group, ROOT_RULES].concat()
    }
}

/// A case body: the script of one test case.
pub type Body = fn(&mut Draw) -> Tcl;

/// One `#[test]` per case body of `crate::cases::<grid>`, run by `runner(identity, group,
/// body)`; the libtest name is `<grid>::<case>`.
/// `draw_cases!(run_case, Group::Bugs, bugs_moddata_1; bug20627);`
macro_rules! draw_cases {
    ($runner:path, $group:expr, $grid:ident; $($case:ident),* $(,)?) => {$(
        #[test]
        fn $case() {
            let identity = concat!(stringify!($grid), "::", stringify!($case));
            $runner(identity, $group, || {
                let body = crate::cases::$grid::$case;
                crate::support::process::run_script(identity, $group, body);
            });
        }
    )*};
}
pub(crate) use draw_cases;

/// `_run_test`: the body, the group `end` script, or `Tcl Exception: ...` when the body
/// raised an uncaught error; then `_check_log`, which must give `OK` or `BAD (known
/// problem)`. With `MARTENSITE_scenario_LOG_DIR` set, the log is also written to
/// `<dir>/<identity>.log`.
pub fn run_script(identity: &str, group: Group, body: impl FnOnce(&mut Draw) -> Tcl) {
    let mut draw = Draw::default();
    match body(&mut draw) {
        Ok(()) if group.end_completes() => draw.puts("TEST COMPLETED"),
        Ok(()) => {}
        Err(error) if error.0.is_empty() => draw.puts("Tcl Exception: EMPTY"),
        Err(error) => draw.puts(&format!("Tcl Exception: {}", error.0)),
    }
    let log = draw.log.text().to_string();
    println!("{log}");
    if let Some(directory) = std::env::var_os("MARTENSITE_DRAW_LOG_DIR") {
        let path = std::path::Path::new(&directory).join(format!("{identity}.log"));
        std::fs::write(&path, &log).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    }
    let verdict = verdict(&log, &group.rules());
    assert!(
        matches!(verdict, Verdict::Passed | Verdict::KnownProblem),
        "case verdict {verdict:?}; expected OK or BAD (known problem)"
    );
}

/// The glibc tunable that turns off the per-thread chunk cache (tcache).
const NO_TCACHE: &str = "glibc.malloc.tcache_count=0";

/// [`run_case`] for a case that measures `meminfo h` (glibc `mallinfo2().uordblks`):
/// glibc counts the chunks parked in its per-thread cache as in use, so the counter of a
/// leak-free loop drifts by up to a few KiB per round while the cache fills, as the
/// allocation order of the Rust kernel varies between rounds (address-hashed maps). With
/// the cache off the counter is the bytes the program holds, which the case means to
/// measure; its thresholds are unchanged. The case therefore re-runs itself in a process
/// started with that tunable (glibc reads tunables only at start-up), which then runs the
/// usual policy child; `run_case_exact_heap("bugs_moddata_1::bug20627", Group::Bugs, || {})`.
pub fn run_case_exact_heap(identity: &str, group: Group, body: impl FnOnce()) {
    let tunables = std::env::var("GLIBC_TUNABLES").unwrap_or_default();
    if tunables.split(':').any(|tunable| tunable == NO_TCACHE) {
        run_case(identity, group, body);
        return;
    }
    let executable = std::env::current_exe().expect("test executable path");
    let joined = if tunables.is_empty() {
        NO_TCACHE.to_string()
    } else {
        format!("{tunables}:{NO_TCACHE}")
    };
    let output = std::process::Command::new(executable)
        .args(["--exact", identity, "--test-threads=1", "--nocapture"])
        .env("GLIBC_TUNABLES", joined)
        .output()
        .unwrap_or_else(|error| panic!("{identity}: cannot start the exact-heap process: {error}"));
    let stdout = String::from_utf8_lossy(&output.stdout);
    print!("{stdout}");
    eprint!("{}", String::from_utf8_lossy(&output.stderr));
    assert!(
        output.status.success() && stdout.contains("1 passed"),
        "{identity}: exact-heap process {}; expected one passing case",
        output.status
    );
}

/// Run `body` as the libtest case `identity`: in a child process under the group's CPU
/// and elapsed budget when its `begin` sets one, else in place;
/// `run_case("bugs_moddata_1::bug20627", Group::Bugs, || {})`.
pub fn run_case(identity: &str, group: Group, body: impl FnOnce()) {
    let Some(seconds) = group.seconds() else {
        body();
        return;
    };
    let artifacts = std::env::temp_dir().join("martensite-tkmesh-draw-case-receipts");
    let spec = CaseSpec::new(identity, seconds, FactorProfile::RequireUnset);
    let receipt = CaseRunner::new(&artifacts, None)
        .run(spec, || {
            assert_eq!(reported_process_limit(), seconds, "group cpulimit");
            body();
            Ok(())
        })
        .unwrap_or_else(|failure| panic!("{failure}"));
    println!(
        "{{\"case\":{identity:?},\"receipt_directory\":{:?}}}",
        receipt.directory
    );
}
