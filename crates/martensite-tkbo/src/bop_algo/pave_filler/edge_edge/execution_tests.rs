//! Synthetic ownership controls exercise actual ranges without claiming an intersection result.
use super::*;
use crate::bop_algo::pave_filler::edge_edge::progress_fixture::unstarted_ee_task;
use crate::bop_algo::progress_test_support::before_indicator;
use std::sync::atomic::Ordering;

static_assertions::assert_impl_all!(super::super::store::EdgeEdgeStore: Send, Sync);
static_assertions::assert_impl_all!(super::super::progress::EdgeEdgeBoundary<'static, 'static>: Send, Sync);
static_assertions::assert_impl_all!(EdgeEdgeInvocation<'static>: Send);
static_assertions::assert_not_impl_any!(EdgeEdgeInvocation<'static>: Sync);

#[test]
fn edge_worker_consumes_one_range_and_rejects_a_second_invocation() {
    let (indicator, trace) = before_indicator();
    let range = indicator.start();
    let scope = ProgressScope::unnamed(&range, 1.0, false);
    let mut work = allocated_edge_invocations(vec![unstarted_ee_task()], &scope, "PerformEE");
    work[0].perform().unwrap();
    assert_eq!(trace.queries.load(Ordering::SeqCst), 1);
    assert!(matches!(
        work[0].completion,
        Some(ProgressCompletion::Cancelled { .. })
    ));
    assert!(work[0].task.parts.is_empty());
    let failure = work[0].perform().unwrap_err();
    assert!(
        failure.message().contains("EE task 0 has no range"),
        "actual {failure:?}; expected consumed range refusal"
    );
}

#[test]
fn unexecuted_edge_worker_is_incomplete_instead_of_a_completed_output() {
    let scope = ProgressScope::default();
    let mut work = allocated_edge_invocations(vec![unstarted_ee_task()], &scope, "PerformEE");
    let failure = work.pop().unwrap().into_output().err().unwrap();
    assert!(
        failure.message().contains("EE task 0 has no completion"),
        "actual {failure:?}; expected missing worker result refusal"
    );
}

#[test]
fn cancelled_edge_workers_preserve_input_ordinals_in_serial_and_parallel() {
    assert_cancelled_edge_mode(false);
    assert_cancelled_edge_mode(true);
}

fn assert_cancelled_edge_mode(parallel: bool) {
    let (indicator, trace) = before_indicator();
    let range = indicator.start();
    let scope = ProgressScope::unnamed(&range, 2.0, false);
    let tasks = vec![unstarted_ee_task(), unstarted_ee_task()];
    let output = execute_edge_tasks(tasks, &scope, parallel, "PerformEE").unwrap();
    assert_eq!(trace.queries.load(Ordering::SeqCst), 2);
    for (ordinal, (_, completion)) in output.into_iter().enumerate() {
        assert_cancelled_ordinal(completion, ordinal);
    }
}

fn assert_cancelled_ordinal(completion: ProgressCompletion, ordinal: usize) {
    let ProgressCompletion::Cancelled {
        context: Some(context),
        ..
    } = completion
    else {
        panic!("actual {completion:?}; expected cancelled EE worker")
    };
    assert_eq!(context.input_ordinal, Some(ordinal));
    assert_eq!(context.label, "PerformEE");
}
