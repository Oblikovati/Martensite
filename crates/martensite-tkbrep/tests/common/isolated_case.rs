//! Runs one regression case alone in a fresh child process of the current test executable,
//! under the elapsed-time budget of its group.
//!
//! The parent re-runs the executable with the case's exact test name and a marker
//! variable; only the child whose marker matches runs the body. Inside the child a
//! watchdog thread ends the process with status 2 once the budget is spent, and a case can
//! restart the budget with [`restart_budget`]. The parent fails unless the child ran
//! exactly that one test and passed.
#![allow(dead_code)] // Each case family uses the subset it needs.
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::sync::{Mutex, PoisonError};
use std::time::{Duration, Instant};

/// The marker naming the case a child process runs.
const CHILD_CASE: &str = "MARTENSITE_ISOLATED_CASE";
/// The start of the current budget.
static STARTED: Mutex<Option<Instant>> = Mutex::new(None);
/// The current budget in seconds; zero or negative means none.
static BUDGET_SECONDS: AtomicI64 = AtomicI64::new(0);
/// The watchdog thread is started by the first budget only.
static WATCHDOG_STARTED: AtomicBool = AtomicBool::new(false);

/// Run `body` as the test `identity` (its full libtest name) in a fresh child process with
/// a budget of `seconds`; `run_isolated("cases::point", 1000, || assert_eq!(1 + 1, 2))`.
pub fn run_isolated(identity: &str, seconds: i64, body: impl FnOnce()) {
    if std::env::var(CHILD_CASE).as_deref() == Ok(identity) {
        restart_budget(seconds);
        assert_eq!(budget_seconds(), seconds, "group time budget");
        body();
        return;
    }
    let executable = std::env::current_exe()
        .unwrap_or_else(|error| panic!("case {identity}: no test executable path: {error}"));
    let output = Command::new(&executable)
        .args([identity, "--exact", "--nocapture", "--test-threads=1"])
        .env(CHILD_CASE, identity)
        .output()
        .unwrap_or_else(|error| panic!("case {identity}: cannot start {executable:?}: {error}"));
    let stdout = String::from_utf8_lossy(&output.stdout);
    print!("{stdout}");
    eprint!("{}", String::from_utf8_lossy(&output.stderr));
    assert!(
        output.status.success(),
        "case {identity}: child process ended with {}; expected success",
        output.status
    );
    assert!(
        stdout.contains("test result: ok. 1 passed"),
        "case {identity}: the child ran no single passing test of that name; expected exactly one"
    );
}

/// Restart the elapsed budget at `seconds` from now (zero or negative removes it), as a
/// case script does before a long step; `restart_budget(20)`.
pub fn restart_budget(seconds: i64) {
    *STARTED.lock().unwrap_or_else(PoisonError::into_inner) = Some(Instant::now());
    BUDGET_SECONDS.store(seconds, Ordering::SeqCst);
    if !WATCHDOG_STARTED.swap(true, Ordering::SeqCst) {
        std::thread::spawn(watch_budget);
    }
}

/// The current budget in seconds (zero before the first one).
pub fn budget_seconds() -> i64 {
    BUDGET_SECONDS.load(Ordering::SeqCst)
}

/// Every second: end the process with status 2 once a positive budget is spent.
fn watch_budget() {
    loop {
        std::thread::sleep(Duration::from_secs(1));
        let elapsed = STARTED
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .map_or(0, |started| started.elapsed().as_secs());
        let limit = budget_seconds();
        if limit > 0 && elapsed >= limit.unsigned_abs() {
            println!("Process killed by elapsed limit  ({limit} sec)");
            std::process::exit(2);
        }
    }
}
