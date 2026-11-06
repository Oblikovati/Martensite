//! Isolate inherited bugs-suite CPU limits from the parent test process.
use martensite_process_policy::case_runner::{CaseRunner, CaseSpec, FactorProfile};

/// Run a case under its inherited limit before restoring fixtures.
///
/// Example: `run_bug_case("module::case", || assert_eq!(2 + 2, 4));`.
// tests/bugs/begin installs this before any case fixture is restored.
pub fn run_bug_case(identity: &str, body: impl FnOnce()) {
    let artifacts = std::env::temp_dir().join("martensite-tkbo-bugs-cpu-limit");
    let specification = CaseSpec::new(identity, 1000, FactorProfile::RequireUnset);
    CaseRunner::new(&artifacts, None)
        .run(specification, || {
            assert_eq!(martensite_process_policy::reported_process_limit(), 1000);
            body();
            Ok(())
        })
        .unwrap_or_else(|error| {
            panic!("{identity}: expected success within cpulimit 1000: {error}")
        });
}
