//! Unstarted FF envelopes retire their assigned ranges without inventing worker scopes.
use super::*;

#[test]
fn unstarted_ff_range_drop_advances_only_its_actual_parent() {
    let (indicator, events) = face_progress_indicator(false);
    let range = indicator.start();
    let scope = ProgressScope::with_literal(&range, "Unstarted FF task", 1.0, false);
    let work = prepared_face_work(vec![prepared_plane_input()], &scope);
    assert!(work[0].input.is_some());
    assert!(work[0].output.is_none());
    assert_initial_root_then_observe_retirement(&events);
    drop(work);
    assert_parent_only_close(&events, "Unstarted FF task", 1.0);
}

fn assert_initial_root_then_observe_retirement(events: &FaceProgressEvents) {
    let mut events = events.lock().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].position, 0.0);
    assert_eq!(events[0].chain, vec![("<null>".to_owned(), 1.0, 0.0)]);
    // Native Start reports the root before any assigned range is retired.
    events.clear();
}

#[test]
fn partially_started_ff_batch_retires_remaining_ranges_without_worker_scopes() {
    let (indicator, events) = face_progress_indicator(false);
    let range = indicator.start();
    let scope = ProgressScope::with_literal(&range, "Partially started FF batch", 4.0, false);
    let mut work = prepared_face_work(prepared_inputs(false), &scope);
    execute_face_work(&mut work[0]).unwrap();
    assert_eq!(
        work[0].output.as_ref().unwrap().completion,
        ProgressCompletion::Completed
    );
    assert!(
        work[1..]
            .iter()
            .all(|item| item.output.is_none() && item.input.is_some())
    );
    events.lock().unwrap().clear();
    drop(work);
    assert_parent_only_close(&events, "Partially started FF batch", 4.0);
}

fn assert_parent_only_close(events: &FaceProgressEvents, name: &str, maximum: f64) {
    let events = events.lock().unwrap();
    let expected = vec![
        (name.to_owned(), maximum, maximum),
        ("<null>".to_owned(), 1.0, 1.0),
    ];
    let event = events.iter().find(|event| event.position == 1.0).unwrap();
    // Native generic unstarted_range.cpp observes the assigning parent, not a task scope.
    assert_eq!(event.chain, expected);
    assert!(events.iter().all(|event| event.chain.len() == 2));
}

#[test]
fn completed_ff_envelope_refuses_a_second_callback_without_replacing_output() {
    let (indicator, events) = face_progress_indicator(false);
    let range = indicator.start();
    let scope = ProgressScope::unnamed(&range, 1.0, false);
    let mut work = prepared_face_work(vec![prepared_plane_input()], &scope);
    execute_face_work(&mut work[0]).unwrap();
    let before = events.lock().unwrap().len();
    let failure = execute_face_work(&mut work[0]).unwrap_err();
    assert!(failure.message().contains("FF work item 0 has no input"));
    assert!(work[0].progress.is_none());
    assert_eq!(
        work[0].output.as_ref().unwrap().completion,
        ProgressCompletion::Completed
    );
    assert_eq!(events.lock().unwrap().len(), before);
}
