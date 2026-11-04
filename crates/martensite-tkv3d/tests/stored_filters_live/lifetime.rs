//! Storage leases are safe Rust extensions where native dangling iterators have no valid use.
use super::probe::{LiveProbeFilter, LiveTrace};
use martensite_tkernel::standard::{Handle, HandleObserver};
use martensite_tkv3d::select_mgr::{
    SelectMgrAndFilter, SelectMgrCompositionFilter, SelectMgrStoredFilterIter,
    SelectMgrStoredFilters,
};
use std::sync::{Arc, atomic::Ordering};

fn assert_send_sync<T: Send + Sync>() {
    let proof: fn(T) -> T = std::convert::identity::<T>;
    std::hint::black_box(proof);
}

#[test]
fn public_live_collection_types_have_automatic_send_sync_contracts() {
    assert_send_sync::<SelectMgrStoredFilters>();
    assert_send_sync::<SelectMgrStoredFilterIter>();
    assert_send_sync::<HandleObserver<dyn martensite_tkv3d::select_mgr::SelectMgrFilter>>();
}

#[test]
fn retained_collection_owns_entries_after_case_composition_drops() {
    let parent = Handle::new(SelectMgrAndFilter::default());
    let trace = Arc::new(LiveTrace::default());
    let child = LiveProbeFilter::new(&trace, 7, true);
    parent.add(&child.cast());
    let retained = parent.stored_filters();
    drop(parent);
    assert_eq!(retained.len(), 1);
    assert_eq!(child.ref_count(), 2);
    drop(retained);
    assert_eq!(child.ref_count(), 1);
}

#[test]
fn yielded_const_handle_lease_keeps_storage_without_intrusive_ownership() {
    let parent = Handle::new(SelectMgrAndFilter::default());
    let trace = Arc::new(LiveTrace::default());
    let child = LiveProbeFilter::new(&trace, 7, true);
    parent.add(&child.cast());
    let retained = parent.stored_filters();
    let lease = retained.iter().next().unwrap();
    assert_eq!(child.ref_count(), 2);
    parent.clear();
    drop(child);
    assert_eq!(lease.ref_count(), 0);
    assert_eq!(trace.destructions.load(Ordering::Acquire), 0);
    assert!(retained.is_empty());
    drop(lease);
    assert_eq!(trace.destructions.load(Ordering::Acquire), 1);
}

#[test]
fn separate_compositions_share_filter_objects_but_not_collection_membership() {
    let first = SelectMgrAndFilter::default();
    let second = SelectMgrAndFilter::default();
    let trace = Arc::new(LiveTrace::default());
    let child = LiveProbeFilter::new(&trace, 7, true).cast();
    first.add(&child);
    second.add(&child);
    first.clear();
    assert!(first.stored_filters().is_empty());
    assert_eq!(second.stored_filters().len(), 1);
    second.remove(&child);
    assert_eq!(child.ref_count(), 1);
}

#[test]
fn completed_cursor_stays_complete_and_fresh_cursor_observes_append() {
    let parent = SelectMgrAndFilter::default();
    let mut cursor = parent.stored_filters().iter();
    assert!(cursor.next().is_none());
    let trace = Arc::new(LiveTrace::default());
    let child = LiveProbeFilter::new(&trace, 7, true).cast();
    parent.add(&child);
    assert!(cursor.next().is_none());
    assert!(parent.stored_filters().iter().next().is_some());
}
