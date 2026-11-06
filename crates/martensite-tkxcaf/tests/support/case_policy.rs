//! Process policy shared by original document regressions.

use martensite_process_policy::case_runner::{CaseRunner, CaseSpec, FactorProfile};

/// Runs one source case after its inherited `cpulimit 1000` policy.
pub fn run_with_case_policy(identity: &'static str, body: impl FnOnce()) {
    let root = std::env::temp_dir().join("martensite-xcaf-original-case-policy");
    let spec = CaseSpec::new(identity, 1000, FactorProfile::RequireUnset);
    CaseRunner::new(&root, None)
        .run(spec, || {
            assert_eq!(martensite_process_policy::reported_process_limit(), 1000);
            body();
            Ok(())
        })
        .unwrap_or_else(|error| panic!("original case {identity}: {error}"));
}
