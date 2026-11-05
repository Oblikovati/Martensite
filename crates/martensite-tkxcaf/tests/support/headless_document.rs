//! Exact process isolation for document cases that have no viewer operations.
use martensite_process_policy::case_runner::{CaseRunner, CaseSpec, FactorProfile};

/// Apply the declared case budget before any document fixture is constructed.
/// Example: `run("document_regressions::case", 1000, || assert_eq!(1, 1))`.
pub(super) fn run(identity: &str, seconds: i64, body: impl FnOnce()) {
    let root = std::env::temp_dir().join("martensite-xcaf-original-case-receipts");
    let runner = CaseRunner::new(&root, None);
    let spec = CaseSpec::new(identity, seconds, FactorProfile::RequireUnset);
    runner
        .run(spec, || {
            body();
            Ok(())
        })
        .unwrap_or_else(|failure| panic!("{failure}"));
}
