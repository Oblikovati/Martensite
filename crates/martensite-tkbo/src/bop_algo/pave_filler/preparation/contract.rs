//! Real base initialization and parent-only preparation boundaries.
use super::fixture::{front_indicator, plane_input};
use super::*;
use martensite_tkernel::message::Alert;
use std::sync::atomic::Ordering;

#[test]
fn empty_init_reports_before_named_scope_and_never_polls() {
    let (indicator, polls, events) = front_indicator(1);
    let range = indicator.start();
    let mut filler = BOPAlgoPaveFiller::new();
    filler.initialize_with_progress(&range).unwrap();
    assert!(
        filler
            .options()
            .has_error(crate::bop_algo::AlertTooFewArguments.dynamic_type())
    );
    assert!(filler.data_structure().is_none());
    assert_eq!(polls.load(Ordering::SeqCst), 0);
    assert_eq!(
        events.lock().unwrap().as_slice(),
        &[vec![("<null>".into(), 1.0, 0.0)]]
    );
}

#[test]
fn null_init_reports_inside_named_scope_without_poll_or_clear() {
    let (indicator, polls, events) = front_indicator(1);
    let range = indicator.start();
    let mut filler = BOPAlgoPaveFiller::new();
    filler.add_argument(&Shape::default());
    filler.initialize_with_progress(&range).unwrap();
    assert!(
        filler
            .options()
            .has_error(crate::bop_algo::AlertNullInputShapes.dynamic_type())
    );
    assert!(filler.data_structure().is_none());
    assert_eq!(polls.load(Ordering::SeqCst), 0);
    assert_named_initialization(&events.lock().unwrap());
}

#[test]
fn valid_init_populates_real_structure_even_when_first_poll_would_cancel() {
    let (indicator, polls, events) = front_indicator(1);
    let range = indicator.start();
    let mut filler = BOPAlgoPaveFiller::new();
    filler.add_argument(&plane_input().face);
    filler.initialize_with_progress(&range).unwrap();
    assert!(!filler.options().has_errors());
    assert!(filler.data_structure().unwrap().nb_source_shapes() > 1);
    assert!(filler.iterator().is_some());
    assert_eq!(polls.load(Ordering::SeqCst), 0);
    assert_named_initialization(&events.lock().unwrap());
}

fn assert_named_initialization(events: &[Vec<(String, f64, f64)>]) {
    let expected = vec![
        (
            "Initialization of Intersection algorithm".to_owned(),
            1.0,
            1.0,
        ),
        ("<null>".to_owned(), 1.0, 1.0),
    ];
    assert_eq!(events.len(), 2);
    assert_eq!(events[1], expected);
}

#[test]
fn non_destructive_prepare_returns_before_scope_and_structure_access() {
    let (indicator, polls, events) = front_indicator(1);
    let range = indicator.start();
    let observed = ProgressCancellation::default();
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_non_destructive(true);
    assert!(
        filler
            .prepare_with_progress(&range, &observed)
            .unwrap()
            .is_empty()
    );
    assert_eq!(polls.load(Ordering::SeqCst), 0);
    assert_eq!(events.lock().unwrap().len(), 1);
    assert!(!observed.is_cancelled());
}

#[test]
fn destructive_prepare_missing_structure_keeps_real_failure_not_empty_success() {
    let range = ProgressRange::default();
    let observed = ProgressCancellation::default();
    let mut filler = BOPAlgoPaveFiller::new();
    assert!(filler.prepare_with_progress(&range, &observed).is_err());
    assert!(!filler.options().has_errors());
    assert!(!observed.is_cancelled());
}

#[test]
fn actual_prepare_collects_candidate_faces_and_publishes_missing_planar_curve() {
    for parallel in [false, true] {
        let input = plane_input();
        super::fixture::remove_stored_pcurve(&input);
        let mut filler = BOPAlgoPaveFiller::new();
        filler.options_mut().set_run_parallel(parallel);
        filler.add_argument(&input.face);
        filler.add_argument(&plane_input().face);
        filler.initialize().unwrap();
        let observed = ProgressCancellation::default();
        let results = filler
            .prepare_with_progress(&ProgressRange::default(), &observed)
            .unwrap();
        assert!(results.is_empty());
        assert!(!observed.is_cancelled());
        assert!(!filler.options().has_errors());
        assert_stored(&input);
        assert!(!filler.iterator().unwrap().more());
    }
}

fn assert_stored(input: &PlanarPcurveInput) {
    assert!(
        BRepTool::curve_on_face(&input.edge, &input.face)
            .unwrap()
            .is_stored
    );
}

#[test]
fn task_and_parent_cancellation_keep_distinct_native_report_boundaries() {
    for target in 1..=3 {
        let input = plane_input();
        super::fixture::remove_stored_pcurve(&input);
        let edge = input.edge.clone();
        let face = input.face.clone();
        let (indicator, polls, _) = front_indicator(target);
        let range = indicator.start();
        let outer = ProgressScope::unnamed(&range, 1.0, false);
        let observed = ProgressCancellation::default();
        let mut filler = BOPAlgoPaveFiller::new();
        let stops = filler
            .complete_planar_preparation(vec![input], &outer, &observed)
            .unwrap();
        assert_cancelled_preparation(&filler, &observed, stops.len(), target);
        assert_eq!(polls.load(Ordering::SeqCst), expected_cancel_polls(target));
        assert!(!BRepTool::curve_on_face(&edge, &face).unwrap().is_stored);
    }
}

fn expected_cancel_polls(target: usize) -> usize {
    match target {
        2 => 2,
        _ => 3,
    }
}

fn assert_cancelled_preparation(
    filler: &BOPAlgoPaveFiller,
    observed: &ProgressCancellation,
    worker_stops: usize,
    target: usize,
) {
    assert!(observed.is_cancelled());
    assert_eq!(worker_stops, usize::from(target == 1));
    assert_eq!(filler.options().has_errors(), target != 1);
    assert_eq!(
        filler
            .options()
            .has_error(crate::bop_algo::AlertUserBreak.dynamic_type()),
        target != 1
    );
}
