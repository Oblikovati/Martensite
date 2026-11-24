use super::*;
use crate::backend::resource_lifecycle_test_support::create_resource_renderer;

#[test]
fn native_completion_recycles_the_same_handles_without_sharing_an_active_slot() {
    let renderer = create_resource_renderer();
    let runtime = &renderer.runtime;
    let first = CommandSubmission::new(runtime).unwrap();
    let handles = (first.pool, first.command, first.fence);
    let concurrent = CommandSubmission::new(runtime).unwrap();
    assert_ne!(first.pool, concurrent.pool);
    assert_ne!(first.command, concurrent.command);
    assert_ne!(first.fence, concurrent.fence);
    first.finish().unwrap();
    drop(first);
    let reopened = CommandSubmission::new(runtime).unwrap();
    assert_eq!((reopened.pool, reopened.command, reopened.fence), handles);
    concurrent.finish().unwrap();
    reopened.finish().unwrap();
    drop((concurrent, reopened));
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
fn unsubmitted_recordings_do_not_replenish_the_slot_pool() {
    let renderer = create_resource_renderer();
    let runtime = &renderer.runtime;
    let recording = CommandSubmission::new(runtime).unwrap();
    drop(recording);
    assert!(runtime.command_slots.take().is_none());
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
fn completed_slots_above_the_limit_are_destroyed() {
    let renderer = create_resource_renderer();
    let runtime = &renderer.runtime;
    let recordings: Vec<_> = (0..=RETAINED_COMMAND_SLOTS)
        .map(|_| CommandSubmission::new(runtime).unwrap())
        .collect();
    for recording in recordings {
        recording.finish().unwrap();
    }
    let slots = runtime.command_slots.0.lock().unwrap();
    assert_eq!(slots.iter().flatten().count(), RETAINED_COMMAND_SLOTS);
    assert!(renderer.validation_errors().unwrap().is_empty());
}
