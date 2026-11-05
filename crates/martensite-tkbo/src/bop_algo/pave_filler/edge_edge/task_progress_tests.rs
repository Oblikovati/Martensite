//! Completion tests retain actual typed causes ahead of a separately observed cancellation.
use super::*;
use crate::bop_algo::pave_filler::edge_edge::progress_fixture::unstarted_ee_task;
use martensite_tkernel::standard::Failure;

#[test]
fn retained_edge_failure_precedes_a_later_cancellation_without_changing_its_cause() {
    let mut task = unstarted_ee_task();
    let cause = Failure::with_type("Standard_DomainError", "owned EE failure", "owned EE stack");
    task.failure = Some(cause.clone());
    let observed = ProgressCancellation::default();
    observed.record_cancelled();
    let completion = task.edge_task_completion(&observed, 7, "PerformEE");
    let ProgressCompletion::Failed {
        cause: actual,
        class,
        context,
        ..
    } = completion
    else {
        panic!("actual {completion:?}; expected retained kernel failure")
    };
    assert_eq!(actual, Some(cause));
    assert_eq!(class, ProgressFailureClass::KernelFailure);
    assert_eq!(context.unwrap().input_ordinal, Some(7));
}

#[test]
fn unfinished_edge_task_never_reports_completed_geometry() {
    let task = unstarted_ee_task();
    let observed = ProgressCancellation::default();
    let completion = task.edge_task_completion(&observed, 2, "PerformEE");
    assert!(
        matches!(
            completion,
            ProgressCompletion::Failed {
                class: ProgressFailureClass::AlgorithmReport,
                cause: None,
                ..
            }
        ),
        "actual {completion:?}; expected unfinished solver failure"
    );
}
