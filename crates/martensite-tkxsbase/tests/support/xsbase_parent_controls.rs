//! Each original exercises the same asserting parent with genuine commands and descriptor mutations.
#[path = "source_parent_refusal_controls.rs"]
mod source_parent_refusal_controls;
use source_parent_refusal_controls::{Incident, StreamProbe};
use std::{
    error::Error,
    panic::{AssertUnwindSafe, catch_unwind},
};
type CaseBody = fn() -> Result<(), Box<dyn Error>>;

/// Require actual descriptor text after the original END to fail the same parent; `reject_descriptor(id, body, false)`.
pub(super) fn reject_descriptor(identity: &str, body: CaseBody, stderr: bool) {
    let probe = StreamProbe::new_with_events(
        "martensite-tkxsbase original:",
        &["TEST COMPLETED", "Faulty:"],
    );
    let incident = if stderr {
        Incident::StderrFaulty
    } else {
        Incident::StdoutFaulty
    };
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        crate::source_process::run_case(identity, || probe.inject(body, incident));
    }));
    probe.require_refusal(outcome);
}

/// Run real commands before requiring missing completion refusal; `reject_missing_end(id, commands)`.
pub(super) fn reject_missing_end(identity: &str, commands: CaseBody) {
    let probe = StreamProbe::new("martensite-tkxsbase original:");
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        crate::source_process::run_case(identity, || {
            probe.execute(commands)?;
            probe.missing_end()
        });
    }));
    probe.require_refusal(outcome);
}

/// Keep source completion presence rather than inventing exactly-once; `accept_duplicate(id, body)`.
pub(super) fn accept_duplicate(identity: &str, body: CaseBody) {
    let probe = StreamProbe::new("martensite-tkxsbase original:");
    let incident: Incident = Incident::DuplicateEnd;
    crate::source_process::run_case(identity, || probe.inject(body, incident));
}

/// Retain benign diagnostics on both actual descriptors; `accept_benign(id, body)`.
pub(super) fn accept_benign(identity: &str, body: CaseBody) {
    crate::source_process::run_case(identity, || {
        body()?;
        crate::source_process::emit_source("Warning: benign exchange diagnostic\n")?;
        std::io::Write::write_all(&mut std::io::stderr(), b"Warning: benign exchange stderr\n")?;
        Ok(())
    });
}

/// Distinguish an actual failed worker from semantic grading; `reject_failed_child(id, body)`.
pub(super) fn reject_failed_child(identity: &str, body: CaseBody) {
    let probe = StreamProbe::new_with_events(
        "topology view case ",
        &["child code=Some(101)", "named full-stream child failure"],
    );
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        crate::source_process::run_case(identity, || {
            probe.execute(body)?;
            probe.failed_child()
        });
    }));
    probe.require_refusal(outcome);
}
