//! Actual V3d commands precede descriptor/end controls through the fixed CPU1000 parent.
use super::{
    source_parent_refusal_controls::{Incident, StreamProbe},
    v3d_buc60701,
};
use crate::case_v3d_process_case::run_case;
use std::{
    error::Error,
    io::Write,
    panic::{AssertUnwindSafe, catch_unwind},
};

fn reject_descriptor(identity: &str, incident: Incident) {
    let probe = StreamProbe::new_with_events(
        "V3d original final stream rejected:",
        &["TEST COMPLETED", "Faulty:"],
    );
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        run_case(identity, || probe.inject(v3d_buc60701::case_body, incident));
    }));
    probe.require_refusal(outcome);
}

#[test]
fn actual_buc60701_stdout_fatal_after_end_is_rejected() {
    let identity =
        "opengl::v3d_stream_controls::actual_buc60701_stdout_fatal_after_end_is_rejected";
    reject_descriptor(identity, Incident::StdoutFaulty);
}

#[test]
fn actual_buc60701_stderr_fatal_after_end_is_rejected() {
    let identity =
        "opengl::v3d_stream_controls::actual_buc60701_stderr_fatal_after_end_is_rejected";
    reject_descriptor(identity, Incident::StderrFaulty);
}

#[test]
fn actual_buc60701_missing_end_is_rejected_after_capture() {
    let identity =
        "opengl::v3d_stream_controls::actual_buc60701_missing_end_is_rejected_after_capture";
    reject_missing_end(identity, v3d_buc60701::commands_without_end);
}

/// Reject missing END only after actual commands and capture; `reject_missing_end(identity, body)`.
pub(super) fn reject_missing_end(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    let probe = StreamProbe::new("V3d original final stream rejected:");
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        run_case(identity, || {
            probe.execute(body)?;
            probe.missing_end()
        });
    }));
    probe.require_refusal(outcome);
}

#[test]
fn actual_buc60701_duplicate_end_is_accepted() {
    let identity = "opengl::v3d_stream_controls::actual_buc60701_duplicate_end_is_accepted";
    let probe = StreamProbe::new("V3d original final stream rejected:");
    run_case(identity, || {
        probe.inject(v3d_buc60701::case_body, Incident::DuplicateEnd)
    });
}

#[test]
fn actual_buc60701_benign_descriptors_are_accepted() {
    let identity = "opengl::v3d_stream_controls::actual_buc60701_benign_descriptors_are_accepted";
    run_case(identity, || {
        v3d_buc60701::case_body()?;
        std::io::stdout().write_all(b"Warning: benign V3d stdout\n")?;
        std::io::stderr().write_all(b"Warning: benign V3d stderr\n")?;
        Ok(())
    });
}

#[test]
fn actual_buc60701_named_failure_is_not_a_policy_refusal() {
    let identity =
        "opengl::v3d_stream_controls::actual_buc60701_named_failure_is_not_a_policy_refusal";
    let probe = StreamProbe::new_with_events(
        "topology view case opengl::v3d_stream_controls::actual_buc60701_named_failure_is_not_a_policy_refusal:",
        &["child code=Some(101)", "named full-stream child failure"],
    );
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        run_case(identity, || {
            probe.execute(v3d_buc60701::case_body)?;
            probe.failed_child()
        });
    }));
    probe.require_refusal(outcome);
}
