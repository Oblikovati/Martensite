//! One CPU-limited child transport exposes finalized full streams to caller-owned source policies.
use martensite_process_policy::case_runner::{CaseRunner, CaseSpec, FactorProfile};
use std::{cell::Cell, error::Error, sync::Mutex};
#[path = "../report_rules/naming_report_stream.rs"]
mod naming_report_stream;

// Serialization protects source drawing windows, not mutable algorithm state.
static CASE_SOURCE_VIEW_LEASE: Mutex<()> = Mutex::new(());

/// Return complete finalized child output only to the parent; `observed_stream("case", 60, body)`.
/// Callers apply their own source rule chain to `Some`; `None` means this is the child.
pub(crate) fn observed_stream(
    identity: &str,
    seconds: i64,
    body: impl FnOnce() -> Result<(), Box<dyn Error>>,
) -> Option<String> {
    let spec = CaseSpec::new(identity, seconds, FactorProfile::RequireUnset);
    observed_spec(identity, spec, || {
        assert_eq!(martensite_process_policy::reported_process_limit(), seconds);
        body()
    })
}

/// Preserve a dedicated child's inherited policy; `observed_inherited_stream("case", body)`.
/// This source has no CPU command: no finite budget or watchdog may be introduced here.
#[allow(dead_code)] // Only some targets that share this file use it.
pub(crate) fn observed_inherited_stream(
    identity: &str,
    body: impl FnOnce() -> Result<(), Box<dyn Error>>,
) -> Option<String> {
    let spec = CaseSpec::inherit_process_policy(identity, FactorProfile::RequireUnset);
    observed_spec(identity, spec, body)
}

fn observed_spec(
    identity: &str,
    spec: CaseSpec<'_>,
    body: impl FnOnce() -> Result<(), Box<dyn Error>>,
) -> Option<String> {
    let executed = Cell::new(false);
    let root = std::env::temp_dir().join("martensite-topalgo-view-process-cases");
    let receipt = CaseRunner::new(&root, Some(&CASE_SOURCE_VIEW_LEASE))
        .run(spec, || {
            executed.set(true);
            body()
        })
        .unwrap_or_else(|error| panic!("topology view case {identity}: {error}"));
    (!executed.get()).then(|| naming_report_stream::case_stream(&receipt, identity))
}
