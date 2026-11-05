//! Independent census processes cannot consume other concurrency controls' worker reservations.
use std::{ffi::OsStr, process::Command};
const CENSUS_CHILD: &str = "MARTENSITE_PROGRESS_CENSUS_CHILD";
/// Execute every assertion with a fresh process-global pool; `if run_isolated_progress_census(name) { return; }`.
pub(crate) fn run_isolated_progress_census(test: &str) -> bool {
    if std::env::var_os(CENSUS_CHILD).as_deref() == Some(OsStr::new(test)) {
        return false;
    }
    let executable =
        std::env::current_exe().expect("census executable: expected current test binary");
    let output = Command::new(&executable)
        .args([test, "--exact", "--nocapture"])
        .env(CENSUS_CHILD, test)
        .output()
        .expect("census child: expected executable test process");
    print!("{}", String::from_utf8_lossy(&output.stdout));
    assert!(
        output.status.success(),
        "census child {executable:?} test {test:?} exited {}; expected success, stderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    true
}
