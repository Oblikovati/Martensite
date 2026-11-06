//! Typed terminal controls do not claim that an injected cause exercised the solver.
use super::super::progress_fixture::prepared_crossing_tasks;
use super::*;
use martensite_tkernel::standard::Failure;

#[test]
fn ef_retained_kernel_failure_precedes_later_cancellation() {
    let (mut tasks, _) = prepared_crossing_tasks();
    let task = &mut tasks[0];
    let cause = Failure::with_type("Standard_DomainError", "owned EF failure", "owned EF stack");
    task.failure = Some(cause.clone());
    let observed = ProgressCancellation::default();
    observed.record_cancelled();
    let completion = task.edge_face_task_completion(&observed, 7, "PerformEF");
    assert_ef_kernel_cause(completion, cause);
}

fn assert_ef_kernel_cause(completion: ProgressCompletion, cause: Failure) {
    let ProgressCompletion::Failed {
        cause: actual,
        class,
        context,
        ..
    } = completion
    else {
        panic!("actual {completion:?}; expected retained EF kernel failure");
    };
    assert_eq!(actual, Some(cause));
    assert_eq!(class, ProgressFailureClass::KernelFailure);
    let context = context.unwrap();
    assert_eq!(context.input_ordinal, Some(7));
    assert_eq!(context.label, "PerformEF");
}

#[test]
fn ef_unstarted_solver_is_not_a_completed_empty_result() {
    let (tasks, _) = prepared_crossing_tasks();
    let completion =
        tasks[0].edge_face_task_completion(&ProgressCancellation::default(), 2, "PerformEF");
    assert!(
        matches!(
            completion,
            ProgressCompletion::Failed {
                class: ProgressFailureClass::AlgorithmReport,
                cause: None,
                ..
            }
        ),
        "actual {completion:?}; expected unexecuted solver failure"
    );
}
