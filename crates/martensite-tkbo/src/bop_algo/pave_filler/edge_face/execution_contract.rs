//! These controls use real prepared tasks, including cancellation before their geometry.
use super::super::progress_fixture::prepared_crossing_tasks;
use super::*;
use crate::bop_algo::progress_test_support::before_indicator;
use std::sync::atomic::Ordering;

static_assertions::assert_impl_all!(EdgeFaceInvocation<'static>: Send);
static_assertions::assert_not_impl_any!(EdgeFaceInvocation<'static>: Sync);

#[test]
fn ef_worker_takes_one_owned_range_then_refuses_reexecution() {
    let (tasks, context) = prepared_crossing_tasks();
    let (indicator, trace) = before_indicator();
    let range = indicator.start();
    let scope = ProgressScope::unnamed(&range, 1., false);
    let mut work = allocated_edge_face_invocations(tasks, &scope, "PerformEF");
    work[0].perform(&context).unwrap();
    assert_eq!(trace.queries.load(Ordering::SeqCst), 1);
    assert!(matches!(
        work[0].completion,
        Some(ProgressCompletion::Cancelled { .. })
    ));
    let failure = work[0].perform(&context).unwrap_err();
    assert!(
        failure.message().contains("EF task 0 has no range"),
        "actual {failure:?}; expected consumed-range refusal"
    );
}

#[test]
fn ef_unexecuted_worker_refuses_a_completed_output() {
    let (tasks, _) = prepared_crossing_tasks();
    let scope = ProgressScope::default();
    let mut work = allocated_edge_face_invocations(tasks, &scope, "PerformEF");
    let failure = work.pop().unwrap().into_output().err().unwrap();
    assert!(
        failure.message().contains("has no completion"),
        "actual {failure:?}; expected missing worker result refusal"
    );
}
