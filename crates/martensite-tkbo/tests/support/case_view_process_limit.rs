//! Preserve inherited process policy before the original topology and view work.
use martensite_process_policy::case_runner::{CaseRunner, CaseSpec, FactorProfile};
use std::{error::Error, sync::Mutex};

// One inclusion per standalone host serializes graphical child lifetimes in that host.
static TOPOLOGY_VIEW_LEASE: Mutex<()> = Mutex::new(());

/// Run an original body, e.g. `run_with_limit("case_view", 600, || Ok(()))`.
pub(super) fn run_with_limit(
    identity: &str,
    seconds: i64,
    body: impl FnOnce() -> Result<(), Box<dyn Error>>,
) {
    let root = std::env::temp_dir().join("martensite-topalgo-view-process-cases");
    let spec = CaseSpec::new(identity, seconds, FactorProfile::RequireUnset);
    CaseRunner::new(&root, Some(&TOPOLOGY_VIEW_LEASE))
        .run(spec, || {
            assert_eq!(martensite_process_policy::reported_process_limit(), seconds);
            body()
        })
        .unwrap_or_else(|error| panic!("topology view case {identity}: {error}"));
}
