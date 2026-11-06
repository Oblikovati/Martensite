//! Real descriptor/lifecycle negatives call the same asserting parent as the original.
use super::run_source;
use std::{
    cell::Cell,
    error::Error,
    io::Write,
    panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
};

#[derive(Clone, Copy)]
enum ChildProbe {
    Stdout,
    Stderr,
    MissingEnd,
    DuplicateEnd,
    NamedChildFailure,
}

fn original_child(probe: ChildProbe) -> Result<(), Box<dyn Error>> {
    match probe {
        ChildProbe::Stdout => std::io::stdout()
            .lock()
            .write_all(b"Error: performance Faulty injected stdout\n")?,
        ChildProbe::Stderr => std::io::stderr()
            .lock()
            .write_all(b"Faulty injected stderr\n")?,
        ChildProbe::NamedChildFailure => panic!("NAMED_CHILD_CAUSE_26542"),
        ChildProbe::MissingEnd => println!("NAMED_CHILD_MISSING_END_26542"),
        _ => {}
    }
    if !matches!(probe, ChildProbe::MissingEnd) {
        println!("TEST COMPLETED");
    }
    if matches!(probe, ChildProbe::DuplicateEnd) {
        println!("TEST COMPLETED");
    }
    Ok(())
}

fn rejected_by_same_parent(identity: &str, probe: ChildProbe, expected_prefix: &str) {
    let executed = Cell::new(false);
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        run_source(identity, || {
            executed.set(true);
            original_child(probe)
        })
    }));
    require_parent_outcome(outcome, executed.get(), expected_prefix);
}

fn require_parent_outcome(
    outcome: std::thread::Result<()>,
    child_executed: bool,
    expected_prefix: &str,
) {
    if child_executed {
        if let Err(cause) = outcome {
            resume_unwind(cause);
        }
        return;
    }
    let cause = outcome.expect_err("same parent: expected actual rejection");
    let message = panic_message(cause.as_ref());
    assert!(
        message.starts_with(expected_prefix),
        "parent failure {message:?}; expected prefix {expected_prefix:?}"
    );
    require_named_child_events(message, expected_prefix);
}

fn require_named_child_events(message: &str, prefix: &str) {
    if !prefix.starts_with("topology view case ") {
        return;
    }
    for expected in [
        "child code=Some(101), signal=None: expected normal exit 0",
        "NAMED_CHILD_CAUSE_26542",
    ] {
        assert!(
            message.contains(expected),
            "parent failure {message:?}; expected actual event {expected:?}"
        );
    }
}

fn panic_message(cause: &(dyn std::any::Any + Send)) -> &str {
    if let Some(message) = cause.downcast_ref::<String>() {
        return message;
    }
    cause
        .downcast_ref::<&str>()
        .copied()
        .expect("parent panic: expected String or &str")
}

#[test]
fn actual_stdout_faulty_precedes_performance_ignore() {
    let identity = "parent_controls::actual_stdout_faulty_precedes_performance_ignore";
    rejected_by_same_parent(
        identity,
        ChildProbe::Stdout,
        "original perf report rejected:",
    );
}

#[test]
fn actual_stderr_faulty_is_not_hidden_by_a_complete_stdout() {
    let identity = "parent_controls::actual_stderr_faulty_is_not_hidden_by_a_complete_stdout";
    rejected_by_same_parent(
        identity,
        ChildProbe::Stderr,
        "original perf report rejected:",
    );
}

#[test]
fn actual_missing_end_is_rejected_by_the_source_parent() {
    let identity = "parent_controls::actual_missing_end_is_rejected_by_the_source_parent";
    rejected_by_same_parent(
        identity,
        ChildProbe::MissingEnd,
        "original perf END rejected:",
    );
}

#[test]
fn actual_duplicate_end_is_rejected_by_the_source_parent() {
    let identity = "parent_controls::actual_duplicate_end_is_rejected_by_the_source_parent";
    rejected_by_same_parent(
        identity,
        ChildProbe::DuplicateEnd,
        "original perf END rejected:",
    );
}

#[test]
fn actual_named_child_failure_is_a_lifecycle_failure() {
    let identity = "parent_controls::actual_named_child_failure_is_a_lifecycle_failure";
    rejected_by_same_parent(
        identity,
        ChildProbe::NamedChildFailure,
        "topology view case parent_controls::actual_named_child_failure_is_a_lifecycle_failure:",
    );
}
