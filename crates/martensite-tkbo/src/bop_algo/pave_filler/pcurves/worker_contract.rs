//! Consumed ranges, terminal packets and original caught causes remain owned and explicit.
use super::*;
static_assertions::assert_impl_all!(PCurveTask: Send, Sync);
static_assertions::assert_impl_all!(PCurveInvocation<'static>: Send);
static_assertions::assert_not_impl_any!(PCurveInvocation<'static>: Sync);
fn one_pcurve_task() -> (BOPAlgoPaveFiller, Vec<PCurveTask>) {
    let case = super::super::fixture::pcurve_case(4, false);
    let tasks = case
        .filler
        .face_block_pcurve_tasks(&ProgressScope::default(), &ProgressCancellation::default())
        .unwrap()
        .unwrap();
    (case.filler, tasks)
}
#[test]
fn pcurve_worker_consumes_one_range_and_refuses_another_execution() {
    let (filler, tasks) = one_pcurve_task();
    let (progress, trace) = crate::bop_algo::progress_test_support::pulse_indicator(1);
    let range = progress.start();
    let scope = ProgressScope::unnamed(&range, 1., false);
    let mut work = allocate_pcurve_invocations(tasks, &scope);
    work[0].perform(&filler.context).unwrap();
    assert!(matches!(
        work[0].completion,
        Some(ProgressCompletion::Cancelled { .. })
    ));
    assert!(work[0].task.failed);
    let failure = work[0].perform(&filler.context).unwrap_err();
    assert!(failure.message().contains("pcurve task 0 has no range"));
    assert_eq!(trace.queries.load(std::sync::atomic::Ordering::SeqCst), 1);
}
#[test]
fn pcurve_worker_without_a_terminal_packet_refuses_completed_output() {
    let (_, tasks) = one_pcurve_task();
    let scope = ProgressScope::default();
    let mut work = allocate_pcurve_invocations(tasks, &scope);
    let failure = work.pop().unwrap().into_output().err().unwrap();
    assert!(
        failure
            .message()
            .contains("pcurve task 0 has no completion")
    );
}
#[test]
fn pcurve_computation_does_not_attach_to_the_source_before_storage() {
    for parallel in [false, true] {
        let (mut filler, tasks) = one_pcurve_task();
        let source = tasks[0].edge.clone();
        let face = tasks[0].face.clone();
        let outputs =
            execute_pcurve_tasks(tasks, &ProgressScope::default(), &filler.context, parallel)
                .unwrap();
        let (task, outcome) = outputs.into_iter().next().unwrap();
        assert_eq!(outcome, ProgressCompletion::Completed);
        assert!(!task.pcurve.is_null());
        assert!(
            BRepTool::curve_on_face(&source, &face)
                .unwrap()
                .pcurve
                .is_null()
        );
        filler.store_pcurve(task).unwrap();
        assert!(
            !BRepTool::curve_on_face(&source, &face)
                .unwrap()
                .pcurve
                .is_null()
        );
    }
}
#[test]
fn actual_null_face_failure_is_caught_with_original_typed_cause_in_both_settings() {
    for parallel in [false, true] {
        let (filler, mut tasks) = one_pcurve_task();
        tasks[0].face = Face::default();
        let outputs =
            execute_pcurve_tasks(tasks, &ProgressScope::default(), &filler.context, parallel)
                .unwrap();
        assert_caught_failure(&outputs[0]);
    }
}
fn assert_caught_failure(output: &(PCurveTask, ProgressCompletion)) {
    let (task, outcome) = output;
    let ProgressCompletion::Failed {
        class: ProgressFailureClass::KernelFailure,
        cause: Some(cause),
        context: Some(context),
        ..
    } = outcome
    else {
        panic!("pcurve result {outcome:?}, expected typed caught failure");
    };
    assert!(task.failed);
    assert_eq!(cause.exception_type(), "Standard_NullObject");
    assert!(!cause.message().is_empty());
    assert_eq!(context.label, "MakePCurves");
    assert_eq!(context.input_ordinal, Some(0));
}
#[test]
fn a_caught_original_cause_precedes_cancellation_and_a_report_has_no_invented_cause() {
    let cause = Failure::with_message("Standard_DomainError", "pcurve worker sentinel");
    let completion = pcurve_worker_completion(&Err(cause.clone()), true, true, 7);
    let ProgressCompletion::Failed {
        cause: actual,
        context: Some(context),
        ..
    } = completion
    else {
        panic!("expected caught pcurve failure");
    };
    assert_eq!(actual, Some(cause));
    assert_eq!(context.input_ordinal, Some(7));
    assert!(matches!(
        pcurve_worker_completion(&Ok(()), true, false, 3),
        ProgressCompletion::Failed {
            class: ProgressFailureClass::AlgorithmReport,
            cause: None,
            ..
        }
    ));
}
