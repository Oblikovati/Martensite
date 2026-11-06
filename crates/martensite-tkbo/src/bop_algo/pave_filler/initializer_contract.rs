//! The private pipeline must execute the actual derived initializer exactly once.
use super::{BOPAlgoPaveFiller, perform_pipeline_with};
use crate::bop_algo::{AlertIntersectionFailed, AlertTooFewArguments};
use martensite_tkernel::message::{Alert, ProgressRange};
use martensite_tkernel::standard::Failure;

#[test]
fn actual_initializer_receives_original_filler_and_stack_child_range_once() {
    let mut filler = BOPAlgoPaveFiller::new();
    let range = ProgressRange::default();
    let mut calls = 0;
    let actual_address = std::ptr::from_ref(&filler);
    let result = perform_pipeline_with(&mut filler, &range, |actual, supplied| {
        calls += 1;
        assert_eq!(std::ptr::from_ref(actual), actual_address);
        assert!(
            !std::ptr::eq(supplied, &range),
            "initializer must receive the actual five-unit child range"
        );
        actual.initialize()
    });
    assert_empty_initializer(&filler, result, calls);
}

#[test]
fn initializer_failure_keeps_exact_typed_cause_before_native_catch() {
    let mut filler = BOPAlgoPaveFiller::new();
    let range = ProgressRange::default();
    let cause = Failure::with_message("Standard_DomainError", "initializer sentinel");
    let result = perform_pipeline_with(&mut filler, &range, |_, _| Err(cause.clone()));
    assert_eq!(
        result,
        Err(cause),
        "private driver must retain original typed failure"
    );
    assert!(
        !filler.options().has_errors(),
        "internal failure must not invent an alert"
    );
    assert!(filler.data_structure().is_none());
}

#[test]
fn ranged_catch_keeps_native_intersection_failure_report() {
    let mut filler = BOPAlgoPaveFiller::new();
    filler.perform_on_progress_with_initializer(
        &ProgressRange::default(),
        &crate::bop_algo::ProgressCancellation::default(),
        |_, _| Err(Failure::new("initializer sentinel")),
    );
    assert!(
        filler
            .options()
            .has_error(AlertIntersectionFailed.dynamic_type())
    );
    assert!(
        filler.data_structure().is_none(),
        "failed initializer must not run stages"
    );
}

fn assert_empty_initializer(filler: &BOPAlgoPaveFiller, result: Result<(), Failure>, calls: usize) {
    assert_eq!(result, Ok(()), "empty initializer must preserve raw report");
    assert_eq!(calls, 1, "actual initializer must execute once");
    let alert = AlertTooFewArguments;
    assert!(filler.options().has_error(alert.dynamic_type()));
    assert!(filler.data_structure().is_none());
}

#[test]
fn active_checker_initializer_executes_once_without_base_argument_rejection() {
    let mut filler = single_vertex_filler();
    let mut calls = 0;
    let observed = crate::bop_algo::ProgressCancellation::default();
    let completion = filler.perform_on_progress_with_initializer(
        &ProgressRange::default(),
        &observed,
        |actual, _| {
            calls += 1;
            crate::bop_algo::checker_si::initialize_checker_for_progress_test(actual, 0)
        },
    );
    assert_eq!(calls, 1);
    assert_eq!(completion, crate::bop_algo::ProgressCompletion::Completed);
    assert!(!filler.options().has_errors());
    assert_eq!(filler.data_structure().unwrap().nb_source_shapes(), 1);
}
#[test]
fn active_initializer_failure_keeps_its_typed_cause_and_native_report() {
    let mut filler = BOPAlgoPaveFiller::new();
    let cause = Failure::with_message("Standard_DomainError", "active initializer sentinel");
    let observed = crate::bop_algo::ProgressCancellation::default();
    let completion = filler.perform_on_progress_with_initializer(
        &ProgressRange::default(),
        &observed,
        |_, _| Err(cause.clone()),
    );
    assert!(
        matches!(completion, crate::bop_algo::ProgressCompletion::Failed { cause: Some(actual), .. } if actual==cause)
    );
    assert!(
        filler
            .options()
            .has_error(AlertIntersectionFailed.dynamic_type())
    );
    assert!(filler.data_structure().is_none());
}
#[test]
fn immutable_checker_pair_inputs_satisfy_scoped_worker_traits() {
    fn require_sync<T: Sync>() {}
    fn require_send<T: Send>() {}
    require_sync::<crate::bop_ds::DataStructure>();
    require_send::<martensite_tkernel::message::ProgressRange<'static>>();
    require_send::<martensite_tkbrep::topods::Face>();
}
fn single_vertex_filler() -> BOPAlgoPaveFiller {
    let vertex = martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex::new(
        martensite_tkmath::gp::Point3::new(0., 0., 0.),
    )
    .unwrap()
    .vertex()
    .unwrap()
    .into_shape();
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(&[vertex].into_iter().collect());
    filler
}
