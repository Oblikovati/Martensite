//! Actual completed rendering cannot hide fatal descriptors from the same source parent.
use super::{
    opengl_hatch_bug28107,
    source_parent_refusal_controls::{Incident, StreamProbe},
};
use crate::case_opengl_process_case::run_case;
use std::{
    error::Error,
    io::Write,
    panic::{AssertUnwindSafe, catch_unwind},
};

const SECOND_PIXEL_EVENTS: &[&str] = &[
    "control_first_named_pixel",
    "Error: no hatching in second view",
    "control_second_named_pixel",
    "TEST COMPLETED",
];

/// Grade the real second-view failure after both captures; `reject_second_pixel_after("case", body)`.
pub(super) fn reject_second_pixel_after(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    let probe = StreamProbe::new_with_events(
        "OpenGL original final stream rejected:",
        SECOND_PIXEL_EVENTS,
    );
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        run_case(identity, || probe.execute(body));
    }));
    probe.require_refusal(outcome);
}

fn reject_descriptor(identity: &str, incident: Incident) {
    let probe = StreamProbe::new_with_events(
        "OpenGL original final stream rejected:",
        &["TEST COMPLETED", "Faulty:"],
    );
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        run_case(identity, || {
            probe.inject(opengl_hatch_bug28107::case_body, incident)
        });
    }));
    probe.require_refusal(outcome);
}

#[test]
fn actual_hatch_stdout_fatal_after_end_is_rejected() {
    let identity = "opengl::stream_controls::actual_hatch_stdout_fatal_after_end_is_rejected";
    reject_descriptor(identity, Incident::StdoutFaulty);
}

#[test]
fn actual_hatch_stderr_fatal_after_end_is_rejected() {
    let identity = "opengl::stream_controls::actual_hatch_stderr_fatal_after_end_is_rejected";
    reject_descriptor(identity, Incident::StderrFaulty);
}

/// Keep the genuine rendering commands before checking missing completion.
/// Example: `reject_missing_end_after_case("case", opengl_hatch_bug28107::commands_without_end)`.
pub(super) fn reject_missing_end_after_case(
    identity: &str,
    body: fn() -> Result<(), Box<dyn Error>>,
) {
    let probe = StreamProbe::new("OpenGL original final stream rejected:");
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        run_case(identity, || {
            probe.execute(body)?;
            probe.missing_end()
        });
    }));
    probe.require_refusal(outcome);
}

#[test]
fn actual_hatch_missing_end_is_rejected_after_real_capture() {
    let identity =
        "opengl::stream_controls::actual_hatch_missing_end_is_rejected_after_real_capture";
    reject_missing_end_after_case(identity, opengl_hatch_bug28107::commands_without_end);
}

#[test]
fn actual_hatch_duplicate_end_is_accepted_by_source_presence_rule() {
    let probe = StreamProbe::new("OpenGL original final stream rejected:");
    run_case(
        "opengl::stream_controls::actual_hatch_duplicate_end_is_accepted_by_source_presence_rule",
        || probe.inject(opengl_hatch_bug28107::case_body, Incident::DuplicateEnd),
    );
}

#[test]
fn actual_hatch_benign_stdout_and_stderr_are_accepted() {
    run_case(
        "opengl::stream_controls::actual_hatch_benign_stdout_and_stderr_are_accepted",
        || {
            opengl_hatch_bug28107::case_body()?;
            std::io::stdout().write_all(b"Warning: benign source rendering diagnostic\n")?;
            std::io::stderr().write_all(b"Warning: benign source rendering stderr\n")?;
            Ok(())
        },
    );
}

#[test]
fn actual_failed_child_is_distinct_from_source_policy_refusal() {
    let identity =
        "opengl::stream_controls::actual_failed_child_is_distinct_from_source_policy_refusal";
    let probe = StreamProbe::new_with_events(
        "topology view case opengl::stream_controls::actual_failed_child_is_distinct_from_source_policy_refusal:",
        &["child code=Some(101)", "named full-stream child failure"],
    );
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        run_case(identity, || probe.failed_child())
    }));
    probe.require_refusal(outcome);
}
