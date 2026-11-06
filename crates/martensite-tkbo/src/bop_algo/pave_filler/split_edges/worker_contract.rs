//! Owned range misuse and typed failure metadata do not claim a geometric exception.
use super::*;
static_assertions::assert_impl_all!(DataStructure: Send,Sync);
static_assertions::assert_impl_all!(SplitEdgeInvocation<'static>: Send);
static_assertions::assert_not_impl_any!(SplitEdgeInvocation<'static>: Sync);
#[test]
fn original_failure_precedes_separately_observed_cancellation() {
    let cause = Failure::with_message("Standard_DomainError", "split worker sentinel");
    let completion = split_worker_completion(&Err(cause.clone()), true, 7);
    let ProgressCompletion::Failed {
        cause: actual,
        context: Some(context),
        ..
    } = completion
    else {
        panic!("expected retained split worker cause");
    };
    assert_eq!(actual, Some(cause));
    assert_eq!(context.label, "MakeSplitEdges");
    assert_eq!(context.input_ordinal, Some(7));
}

fn prepared_worker_tasks() -> BOPAlgoPaveFiller {
    super::super::tests::crossing_case(false)
}
#[test]
fn split_worker_consumes_one_range_then_refuses_reexecution() {
    let mut filler = prepared_worker_tasks();
    let tasks = filler
        .collect_split_edge_tasks(&ProgressScope::default(), &ProgressCancellation::default())
        .unwrap()
        .unwrap();
    let (indicator, trace) = crate::bop_algo::progress_test_support::before_indicator();
    let range = indicator.start();
    let scope = ProgressScope::unnamed(&range, 4., false);
    let mut work = allocated_split_invocations(tasks, &scope);
    work[0]
        .perform(filler.data_structure().unwrap(), &filler.context)
        .unwrap();
    assert!(matches!(
        work[0].completion,
        Some(ProgressCompletion::Cancelled { .. })
    ));
    assert!(work[0].task.split.is_same(&work[0].task.edge));
    assert!(work[0].task.bounds.is_void());
    let failure = work[0]
        .perform(filler.data_structure().unwrap(), &filler.context)
        .unwrap_err();
    assert!(failure.message().contains("split task 0 has no range"));
    assert_eq!(trace.queries.load(std::sync::atomic::Ordering::SeqCst), 1);
}
#[test]
fn split_worker_without_terminal_packet_refuses_a_completed_output() {
    let mut filler = prepared_worker_tasks();
    let tasks = filler
        .collect_split_edge_tasks(&ProgressScope::default(), &ProgressCancellation::default())
        .unwrap()
        .unwrap();
    let scope = ProgressScope::default();
    let mut work = allocated_split_invocations(tasks, &scope);
    let failure = work.pop().unwrap().into_output().err().unwrap();
    assert!(failure.message().contains("split task 3 has no completion"));
}

#[test]
fn actual_null_edge_failure_crosses_the_context_launcher_without_ds_storage() {
    for parallel in [false, true] {
        let mut filler = prepared_worker_tasks();
        let mut tasks = filler
            .collect_split_edge_tasks(&ProgressScope::default(), &ProgressCancellation::default())
            .unwrap()
            .unwrap();
        tasks[0].edge = Edge::default();
        let scope = ProgressScope::default();
        let failure = execute_split_edge_tasks(
            tasks,
            &scope,
            filler.data_structure().unwrap(),
            &filler.context,
            parallel,
        )
        .err()
        .unwrap();
        assert_eq!(failure.exception_type(), "Standard_ProgramError");
        assert_eq!(
            failure.message(),
            "Standard_NullObject: TopoDS_Shape::EmptyCopy: the shape is null; expected a shape with a TShape"
        );
        assert_eq!(failure.stack_string(), "");
        assert_eq!(filler.data_structure().unwrap().nb_shapes(), 7);
        for list in filler.data_structure().unwrap().pave_blocks_pool() {
            assert!(list.iter().all(|block| !block.lock().unwrap().has_edge()));
        }
    }
}
