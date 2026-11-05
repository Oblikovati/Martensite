//! One-shot ranges and deferred real planar projection remain independently observable.
use super::super::fixture::{front_indicator, plane_input, remove_stored_pcurve};
use super::*;
use std::sync::atomic::Ordering;

#[test]
fn planar_worker_is_send_but_its_consumable_range_is_not_sync() {
    static_assertions::assert_impl_all!(PlanarPcurveWork<'static>: Send);
    static_assertions::assert_not_impl_any!(PlanarPcurveWork<'static>: Sync);
    static_assertions::assert_impl_all!(PlanarPcurveInput: Send, Sync);
}

#[test]
fn completed_projection_is_deferred_until_parent_store_and_has_closed_form_uv() {
    let input = plane_input();
    remove_stored_pcurve(&input);
    let edge = input.edge.clone();
    let face = input.face.clone();
    let range = ProgressRange::default();
    let scope = ProgressScope::unnamed(&range, 1.0, false);
    let mut work =
        execute_planar_inputs(vec![input], &scope, false, &ProgressCancellation::default())
            .unwrap();
    assert!(!BRepTool::curve_on_face(&edge, &face).unwrap().is_stored);
    assert_completed(&mut work[0]);
    work[0].store_curve().unwrap();
    let found = BRepTool::curve_on_face(&edge, &face).unwrap();
    assert!(found.is_stored);
    assert_eq!(found.pcurve.get().unwrap().value(0.0).unwrap().x(), -2.0);
    assert_eq!(found.pcurve.get().unwrap().value(0.0).unwrap().y(), 0.0);
}

#[test]
fn worker_cancel_is_silent_latched_and_not_mistaken_for_completion() {
    let (indicator, polls, _) = front_indicator(1);
    let range = indicator.start();
    let scope = ProgressScope::unnamed(&range, 1.0, false);
    let observed = ProgressCancellation::default();
    let mut work = execute_planar_inputs(vec![plane_input()], &scope, false, &observed).unwrap();
    assert!(observed.is_cancelled());
    assert_eq!(polls.load(Ordering::SeqCst), 1);
    assert_eq!(work[0].take_completion().unwrap(), cancelled_planar_work(0));
    assert!(!work[0].to_update);
    assert!(work[0].curve.is_null());
}

#[test]
fn repeated_worker_and_missing_completion_fail_without_an_extra_poll() {
    let (indicator, polls, _) = front_indicator(0);
    let range = indicator.start();
    let scope = ProgressScope::unnamed(&range, 1.0, false);
    let mut work = reserved_planar_work(vec![plane_input()], &scope);
    assert_failure_contains(
        work[0].take_completion().unwrap_err(),
        "actual terminal callback",
    );
    work[0].perform(&ProgressCancellation::default()).unwrap();
    assert_eq!(polls.load(Ordering::SeqCst), 1);
    let failure = work[0]
        .perform(&ProgressCancellation::default())
        .unwrap_err();
    assert_failure_contains(failure, "unconsumed worker range");
    assert_eq!(polls.load(Ordering::SeqCst), 1);
    assert_completed(&mut work[0]);
}

fn assert_completed(work: &mut PlanarPcurveWork<'_>) {
    assert_eq!(
        work.take_completion().unwrap(),
        ProgressCompletion::Completed
    );
}

fn assert_failure_contains(failure: Failure, expected: &str) {
    assert!(
        failure.message().contains(expected),
        "{failure:?}, expected {expected:?}"
    );
}

#[test]
fn unstarted_planar_work_retires_only_the_actual_assigning_scope() {
    let (indicator, _, events) = front_indicator(0);
    let range = indicator.start();
    let scope = ProgressScope::with_literal(&range, "Unstarted planar work", 1.0, false);
    let work = reserved_planar_work(vec![plane_input()], &scope);
    assert_eq!(events.lock().unwrap().len(), 1);
    events.lock().unwrap().clear();
    drop(work);
    assert_eq!(
        *events.lock().unwrap(),
        vec![vec![
            ("Unstarted planar work".to_owned(), 1.0, 1.0),
            ("<null>".to_owned(), 1.0, 1.0),
        ]]
    );
}

#[test]
fn singleton_parallel_keeps_the_actual_direct_typed_failure() {
    for parallel in [false, true] {
        let input = PlanarPcurveInput {
            edge: Edge::default(),
            face: plane_input().face,
        };
        let expected = pcurve_for_edge_on_plane(&input.edge, &input.face)
            .err()
            .unwrap();
        let range = ProgressRange::default();
        let scope = ProgressScope::unnamed(&range, 1.0, false);
        let observed = ProgressCancellation::default();
        let actual = execute_planar_inputs(vec![input], &scope, parallel, &observed)
            .err()
            .unwrap();
        assert_eq!(actual, expected);
        assert!(!observed.is_cancelled());
    }
}
