//! Native-valid clear callbacks and explicitly owned cleanup of the observed native leak differ.
mod probe;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkv3d::select_mgr::*;
use probe::{DropAction, DropProbeFilter, DropRecord, DropTrace};
use std::{
    panic::{AssertUnwindSafe, catch_unwind},
    sync::{Arc, atomic::Ordering},
};

type Parent = Handle<dyn SelectMgrCompositionFilter>;

fn source_children(count: u8) -> (Parent, Arc<DropTrace>) {
    let parent: Parent = Handle::new(SelectMgrAndFilter::default()).cast();
    let trace = Arc::new(DropTrace::default());
    *trace.parent.write() = parent.downgrade();
    for identity in 1..=count {
        parent.add(&DropProbeFilter::new(&trace, identity).cast());
    }
    (parent, trace)
}

fn reset_is_usable(parent: &Parent) {
    assert!(parent.is_empty());
    assert!(parent.stored_filters().iter().try_next().unwrap().is_none());
    let extra: Handle<dyn SelectMgrFilter> = Handle::new(SelectMgrAndFilter::default()).cast();
    parent.add(&extra);
    parent.clear();
    assert_eq!(extra.ref_count(), 1);
}

#[test]
fn actual_destructors_see_native_size_until_source_clear_completes() {
    let (parent, trace) = source_children(2);
    let owner = SelectMgrEntityOwner::new(0).cast();
    assert!(parent.is_ok(&owner).unwrap());
    let before = parent.stored_filters().len();
    parent.clear();
    let actual = native_clear_records(before, &trace, parent.stored_filters().len());
    assert_eq!(
        actual,
        include_str!("../../fixtures/composition-filter-live/clear-native.txt")
    );
    println!("{actual}");
}

fn native_clear_records(before: usize, trace: &DropTrace, after: usize) -> String {
    let mut actual = format!("before {before}\n");
    for record in trace.records.read().iter() {
        let id = record.identity;
        let length = record
            .parent_length
            .expect("parent=None; expected live source parent");
        let empty = i32::from(record.parent_empty.unwrap());
        actual += &format!(
            "destructor {id} len {length} empty {empty} refs {}\n",
            record.references
        );
    }
    actual + &format!("after {after}\n")
}

fn append_control(count: u8) -> i32 {
    let (parent, trace) = source_children(count);
    let extra: Handle<dyn SelectMgrFilter> = Handle::new(SelectMgrAndFilter::default()).cast();
    *trace.extra.write() = extra.downgrade();
    *trace.action.write() = DropAction::Append;
    parent.clear();
    assert_eq!(
        trace.records.read()[0],
        expected_record(1, usize::from(count))
    );
    reset_is_usable(&parent);
    extra.ref_count()
}

#[test]
fn nonterminal_callback_append_matches_native_owned_counter() {
    let observed = append_control(2);
    let expected_native_external_owner = 1;
    assert_eq!(observed, expected_native_external_owner);
}

#[test]
fn terminal_callback_append_uses_nonleaking_policy_with_open_native_difference() {
    let observed_rust = append_control(1);
    assert_eq!(
        observed_rust, 1,
        "one external owner remains after owned cleanup"
    );
    let observed_native_leak = native_terminal_owner_count();
    assert_ne!(
        observed_rust, observed_native_leak,
        "native orphan compatibility is explicitly OPEN"
    );
}

#[test]
fn panic_cleanup_drops_remaining_owners_and_restores_collection_phase() {
    let (parent, trace) = source_children(2);
    trace.panic_once.store(true, Ordering::Release);
    let failure = catch_unwind(AssertUnwindSafe(|| parent.clear())).unwrap_err();
    assert_eq!(
        failure.downcast_ref::<Failure>().unwrap().exception_type(),
        "Standard_ProgramError"
    );
    assert_eq!(trace.records.read().len(), 2);
    reset_is_usable(&parent);
}

#[test]
fn recursive_clear_fails_typed_and_restores_owned_state() {
    let (parent, trace) = source_children(2);
    *trace.action.write() = DropAction::RecursiveClear;
    let failure = catch_unwind(AssertUnwindSafe(|| parent.clear())).unwrap_err();
    assert!(
        failure
            .downcast_ref::<Failure>()
            .unwrap()
            .message()
            .contains("recursive Clear")
    );
    reset_is_usable(&parent);
}

#[test]
fn staged_slots_never_escape_as_synthetic_null_handles() {
    let (parent, trace) = source_children(2);
    *trace.action.write() = DropAction::InspectTraversal;
    parent.clear();
    assert!(trace.traversal_failed.load(Ordering::Acquire));
    reset_is_usable(&parent);
}

#[test]
fn removing_cached_next_is_typed_invalidation_not_silent_truncation() {
    let (parent, trace) = source_children(1);
    let extra: Handle<dyn SelectMgrFilter> = Handle::new(SelectMgrAndFilter::default()).cast();
    parent.add(&extra);
    *trace.extra.write() = extra.downgrade();
    *trace.action.write() = DropAction::Remove;
    let failure = catch_unwind(AssertUnwindSafe(|| parent.clear())).unwrap_err();
    assert!(
        failure
            .downcast_ref::<Failure>()
            .unwrap()
            .message()
            .contains("cached-next")
    );
    assert_eq!(extra.ref_count(), 1);
    reset_is_usable(&parent);
}

#[test]
fn removing_a_later_non_cached_node_keeps_defined_native_clear_progress() {
    let (parent, trace) = source_children(2);
    let extra: Handle<dyn SelectMgrFilter> = Handle::new(SelectMgrAndFilter::default()).cast();
    parent.add(&extra);
    *trace.extra.write() = extra.downgrade();
    *trace.action.write() = DropAction::Remove;
    parent.clear();
    assert_eq!(
        *trace.records.read(),
        [expected_record(1, 3), expected_record(2, 2)]
    );
    assert_eq!(extra.ref_count(), 1);
}

fn native_terminal_owner_count() -> i32 {
    let original = include_str!("../../fixtures/composition-filter-live/clear-append-native.txt");
    let terminal = original
        .lines()
        .find(|line| line.starts_with("initial 1 "))
        .unwrap();
    terminal.split_whitespace().last().unwrap().parse().unwrap()
}

fn expected_record(identity: u8, length: usize) -> DropRecord {
    DropRecord {
        identity,
        parent_length: Some(length),
        parent_empty: Some(length == 0),
        references: 0,
    }
}

#[test]
fn retained_view_keeps_entries_owned_after_parent_destruction_without_dangling_parent() {
    let (parent, trace) = source_children(2);
    let retained = parent.stored_filters();
    drop(parent);
    assert!(trace.records.read().is_empty());
    drop(retained);
    assert_eq!(trace.records.read().len(), 2);
    assert!(
        trace
            .records
            .read()
            .iter()
            .all(|record| record.parent_length.is_none())
    );
}
