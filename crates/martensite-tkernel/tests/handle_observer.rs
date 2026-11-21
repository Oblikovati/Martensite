//! Borrow-only handle reads and explicit ownership snapshots.
use martensite_tkernel::standard::{
    Handle, SharedState, StandardTransient, Transient, WeakHandleSlotRef,
};
use std::sync::Arc;

#[test]
fn borrowed_observation_adds_an_owner_only_when_made_owned() {
    let handle = Handle::new(StandardTransient);
    let observed = handle.observe();
    assert_eq!(observed.object_address(), handle.object_address());
    assert_eq!(handle.ref_count(), 1);
    let owned = observed.into_handle();
    assert_eq!(handle.ref_count(), 2);
    drop(owned);
    assert_eq!(handle.ref_count(), 1);
}

#[test]
fn null_observer_is_a_null_const_argument() {
    let null = Handle::<StandardTransient>::default();
    let observed = null.observe();
    assert!(observed.is_null());
    assert!(observed.get().is_none());
    assert!(observed.object_address().is_null());
}

#[test]
fn mutable_slot_snapshot_survives_replacement_with_one_explicit_owner() {
    let handle = Handle::new(StandardTransient);
    let weak = handle.downgrade();
    let slot = Arc::new(SharedState::new(handle));
    let alias = WeakHandleSlotRef::new(&slot);
    let snapshot = alias.observe();
    assert_eq!(weak.ref_count(), 2);
    *slot.write() = Handle::default();
    assert!(alias.is_null());
    assert_eq!(weak.ref_count(), 1);
    let owner = snapshot.into_handle();
    assert_eq!(owner.ref_count(), 1);
    drop(owner);
    assert!(weak.upgrade().is_null());
}

#[test]
fn borrowed_projection_preserves_identity_and_rejects_unrelated_types() {
    use martensite_tkernel::standard::RuntimeType;
    let handle = Handle::new(StandardTransient);
    let projected = handle.view::<dyn Transient>();
    let unrelated = handle.view::<RuntimeType>();
    assert_eq!(projected.object_address(), handle.object_address());
    assert_eq!(handle.ref_count(), 1);
    assert!(unrelated.is_null());
    assert!(
        Handle::<StandardTransient>::default()
            .view::<RuntimeType>()
            .is_null()
    );
}
