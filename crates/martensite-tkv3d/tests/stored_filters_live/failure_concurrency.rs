//! Failure and interleaved reads retain explicit ownership rather than a callback lock.
use super::probe::{LiveProbeFilter, LiveTrace};
use martensite_tkernel::standard::Handle;
use martensite_tkv3d::select_mgr::*;
use std::sync::{Arc, atomic::Ordering};

#[test]
fn callback_failure_releases_lease_and_keeps_case_membership() {
    let parent = SelectMgrAndFilter::default();
    let trace = Arc::new(LiveTrace::default());
    let child = LiveProbeFilter::new(&trace, 7, true).cast();
    parent.add(&child);
    trace.fail.store(true, Ordering::Release);
    let owner = SelectMgrEntityOwner::new(0).cast();
    assert!(parent.is_ok(&owner).is_err());
    assert_eq!(parent.stored_filters().len(), 1);
    assert_eq!(child.ref_count(), 2);
    parent.remove(&child);
    assert_eq!(child.ref_count(), 1);
}

fn observed_step(cursor: &mut SelectMgrStoredFilterIter) -> bool {
    match cursor.try_next() {
        Ok(Some(lease)) => {
            assert!(lease.is_some());
            true
        }
        Ok(None) => false,
        Err(failure) => {
            assert!(failure.message().contains("phase=Clear"));
            false
        }
    }
}

fn reader_steps(collection: SelectMgrStoredFilters) {
    for _ in 0..128 {
        let mut cursor = collection.iter();
        while observed_step(&mut cursor) {}
        std::thread::yield_now();
    }
}

#[test]
fn concurrent_steps_retain_safe_storage_without_snapshot_determinism_claim() {
    let parent = Handle::new(SelectMgrAndFilter::default());
    let trace = Arc::new(LiveTrace::default());
    let child = LiveProbeFilter::new(&trace, 7, true).cast();
    std::thread::scope(|scope| {
        for _ in 0..3 {
            let collection = parent.stored_filters();
            scope.spawn(move || reader_steps(collection));
        }
        for _ in 0..128 {
            parent.add(&child);
            parent.clear();
        }
    });
    assert!(parent.is_empty());
    assert_eq!(child.ref_count(), 1);
}
