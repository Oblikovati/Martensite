//! Supplemental analytical membership and actual worker ownership do not replace legacy tests.
#[path = "progress_tests/fixtures.rs"]
mod fixtures;
#[path = "progress_tests/matrix.rs"]
mod matrix;
#[path = "progress_tests/rendezvous.rs"]
mod rendezvous;
#[path = "progress_tests/selection_failure.rs"]
mod selection_failure;
use super::InParts;
use super::{
    classify_faces_observed, classify_faces_with_progress, execution::ClassificationExecution,
};
use crate::{bop_algo::ProgressCompletion, int_tools::Context};
use fixtures::AnalyticalClassification;
use martensite_tkernel::{
    message::ProgressRange,
    osd::{Launcher, ThreadPool},
    standard::Failure,
};
use rendezvous::{AbortOnUnwind, ClassificationRendezvous};

#[test]
fn real_two_context_classification_matches_native_analytical_membership() {
    let fixture = AnalyticalClassification::new();
    let _: &(dyn Send + Sync) = &fixture;
    fixture.require_valid();
    let before = fixture.snapshot();
    assert_eq!(serial_membership(&fixture), vec![vec![0], vec![1]]);
    for _ in 0..8 {
        assert_eq!(parallel_membership(&fixture), vec![vec![0], vec![1]]);
    }
    assert_eq!(fixture.snapshot(), before);
}

fn serial_membership(fixture: &AnalyticalClassification) -> Vec<Vec<usize>> {
    let context = Context::default();
    let mut parts = InParts::default();
    let completion = classify_faces_with_progress(
        fixture.job(&context, &mut parts, false),
        &ProgressRange::default(),
    );
    assert_eq!(completion, ProgressCompletion::Completed);
    fixture.membership(&parts)
}

fn parallel_membership(fixture: &AnalyticalClassification) -> Vec<Vec<usize>> {
    let context = Context::default();
    let trace = ClassificationRendezvous::new(&context);
    let _: &(dyn Send + Sync) = &trace;
    let observe = |context: &Context| {
        let _unwind = AbortOnUnwind(&trace);
        let _: &(dyn Send + Sync) = &_unwind;
        trace.observe(context)
    };
    let (completion, parts) = with_owned_workers(fixture, &context, &observe);
    assert_eq!(completion, ProgressCompletion::Completed);
    trace.assert_two();
    fixture.membership(&parts)
}

fn with_owned_workers(
    fixture: &AnalyticalClassification,
    context: &Context,
    observe: &(dyn Fn(&Context) -> Result<(), Failure> + Sync),
) -> (ProgressCompletion, InParts) {
    with_owned_workers_on(fixture, context, observe, &ProgressRange::default())
}

fn with_owned_workers_on(
    fixture: &AnalyticalClassification,
    context: &Context,
    observe: &(dyn Fn(&Context) -> Result<(), Failure> + Sync),
    range: &ProgressRange<'_>,
) -> (ProgressCompletion, InParts) {
    let pool = ThreadPool::new(2);
    let launcher = Launcher::new(&pool, 2);
    assert_eq!(launcher.nb_threads(), 2);
    assert_eq!(launcher.upper_thread_index(), 1);
    let execution = ClassificationExecution {
        observer: observe,
        launcher: Some(&launcher),
    };
    let _: &(dyn Send + Sync) = &execution;
    run_observed(fixture, context, &execution, range)
}

fn run_observed(
    fixture: &AnalyticalClassification,
    context: &Context,
    execution: &ClassificationExecution<'_>,
    range: &ProgressRange<'_>,
) -> (ProgressCompletion, InParts) {
    let mut parts = InParts::default();
    let completion =
        classify_faces_observed(fixture.job(context, &mut parts, true), range, execution);
    (completion, parts)
}

#[test]
fn classification_worker_failure_aborts_rendezvous_and_publishes_no_parts() {
    let fixture = AnalyticalClassification::new();
    let before = fixture.snapshot();
    let context = Context::default();
    let trace = ClassificationRendezvous::new(&context);
    let observer = |actual: &Context| failing_entry(actual, &context, &trace, false);
    let (completion, parts) = with_owned_workers(&fixture, &context, &observer);
    assert_failed_without_parts(completion, &parts, "injected classification failure");
    assert_eq!(fixture.snapshot(), before);
}

#[test]
fn classification_worker_unwind_releases_peer_and_preserves_failure() {
    let fixture = AnalyticalClassification::new();
    let before = fixture.snapshot();
    let context = Context::default();
    let trace = ClassificationRendezvous::new(&context);
    let observer = |actual: &Context| failing_entry(actual, &context, &trace, true);
    let (completion, parts) = with_owned_workers(&fixture, &context, &observer);
    assert_failed_without_parts(completion, &parts, "injected classification unwind");
    assert_eq!(fixture.snapshot(), before);
}

fn failing_entry(
    actual: &Context,
    main: &Context,
    trace: &ClassificationRendezvous,
    unwind: bool,
) -> Result<(), Failure> {
    let _unwind = AbortOnUnwind(trace);
    if !std::ptr::eq(actual, main) {
        return trace.observe(actual);
    }
    if unwind {
        panic!("injected classification unwind");
    }
    trace.abort();
    Err(Failure::with_type(
        "Standard_ConstructionError",
        "injected classification failure",
        "kept source stack",
    ))
}

fn assert_failed_without_parts(completion: ProgressCompletion, parts: &InParts, message: &str) {
    let ProgressCompletion::Failed {
        cause: Some(cause), ..
    } = completion
    else {
        panic!("completion={completion:?}; expected typed Failed with actual cause");
    };
    assert_eq!(cause.exception_type(), "Standard_ProgramError");
    assert!(cause.message().contains(message), "{}", cause.message());
    assert!(parts.is_empty());
}
