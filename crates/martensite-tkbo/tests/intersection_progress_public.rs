//! Ordinary external callers exercise actual ranged intersection and self-check receivers.
use martensite_tkbo::bop_algo;
use martensite_tkbo::bop_algo::{
    BOPAlgoCheckerSI, BOPAlgoPaveFiller, ProgressCompletion, ProgressFailureClass,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;
use std::sync::atomic::Ordering;
#[allow(dead_code)] // This target uses part of the recording helpers.
#[path = "../src/bop_algo/progress_test_support/recording.rs"]
mod recording;
fn vertex(x: f64) -> Shape {
    BRepBuilderAPIMakeVertex::new(Point3::new(x, 0., 0.))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape()
}
#[test]
fn both_ranged_entry_points_have_their_actual_receiver_function_type() {
    let _: for<'a, 'b, 'p> fn(
        &'a mut BOPAlgoPaveFiller,
        &'b ProgressRange<'p>,
    ) -> ProgressCompletion = BOPAlgoPaveFiller::perform_with_progress;
    let _: for<'a, 'b, 'p> fn(
        &'a mut BOPAlgoCheckerSI,
        &'b ProgressRange<'p>,
    ) -> ProgressCompletion = BOPAlgoCheckerSI::perform_with_progress;
}
#[test]
fn external_intersection_keeps_the_callers_parent_range_and_real_diagnostics() {
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(&[vertex(0.), vertex(10.)].into_iter().collect());
    let (indicator, trace) = recording::indicator(0);
    let root = indicator.start();
    let caller = ProgressScope::with_literal(&root, "External intersection caller", 10., false);
    assert_eq!(
        BOPAlgoPaveFiller::perform_with_progress(&mut filler, &caller.next(3.)),
        ProgressCompletion::Completed
    );
    assert!(!filler.options().has_errors());
    assert_eq!(filler.data_structure().unwrap().nb_source_shapes(), 2);
    require_ancestor(
        &trace,
        "Performing intersection of shapes",
        "External intersection caller",
    );
}
#[test]
fn external_checker_uses_self_initialization_and_the_actual_checker_parent() {
    let mut checker = BOPAlgoCheckerSI::new();
    checker.set_arguments(&[vertex(0.)].into_iter().collect());
    let (indicator, trace) = recording::indicator(0);
    let root = indicator.start();
    let caller = ProgressScope::with_literal(&root, "External checker caller", 10., false);
    assert_eq!(
        BOPAlgoCheckerSI::perform_with_progress(&mut checker, &caller.next(3.)),
        ProgressCompletion::Completed
    );
    assert!(!checker.options().has_errors());
    assert_eq!(checker.data_structure().unwrap().nb_source_shapes(), 1);
    require_ancestor(
        &trace,
        "Checking shape on self-intersection",
        "External checker caller",
    );
}
#[test]
fn external_one_pulse_cancellation_cannot_become_completed_execution() {
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(&[vertex(0.), vertex(0.)].into_iter().collect());
    let (indicator, trace) = recording::pulse_indicator(1);
    assert!(matches!(
        filler.perform_with_progress(&indicator.start()),
        ProgressCompletion::Cancelled { .. }
    ));
    assert!(trace.break_observed.load(Ordering::SeqCst));
    let mut checker = BOPAlgoCheckerSI::new();
    checker.set_arguments(&[vertex(0.)].into_iter().collect());
    let (indicator, trace) = recording::pulse_indicator(1);
    assert!(matches!(
        checker.perform_with_progress(&indicator.start()),
        ProgressCompletion::Cancelled { .. }
    ));
    assert!(trace.break_observed.load(Ordering::SeqCst));
}
#[test]
fn external_rejected_input_returns_failure_and_never_a_completed_empty_tag() {
    let mut filler = BOPAlgoPaveFiller::new();
    assert!(matches!(
        filler.perform_with_progress(&ProgressRange::default()),
        ProgressCompletion::Failed {
            class: ProgressFailureClass::AlgorithmReport,
            ..
        }
    ));
    let mut checker = BOPAlgoCheckerSI::new();
    assert!(matches!(
        checker.perform_with_progress(&ProgressRange::default()),
        ProgressCompletion::Failed {
            class: ProgressFailureClass::AlgorithmReport,
            ..
        }
    ));
}
fn require_ancestor(trace: &recording::Trace, operation: &str, caller: &str) {
    let events = trace.events.lock().unwrap();
    assert!(
        events.iter().any(|(_, names)| names
            .windows(2)
            .any(|pair| pair[0] == operation && pair[1] == caller)),
        "operation={operation} caller={caller}; expected directly retained named parent"
    );
}
