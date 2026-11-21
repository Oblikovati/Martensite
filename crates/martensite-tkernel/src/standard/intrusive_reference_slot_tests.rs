//! Owned-handle and actual slot observations are separate during callback release.
use crate::standard::{Handle, IntrusiveReferenceSlot, StandardTransient};

#[path = "intrusive_reference_slot_retirement_tests.rs"]
mod retirement;

#[test]
fn reference_slot_constructor_moves_exactly_one_existing_native_owner() {
    let target = Handle::new(StandardTransient);
    let observed = target.downgrade();
    let pointer = target.object_address();
    let slot = IntrusiveReferenceSlot::from_owned(target);
    assert_eq!(observed.ref_count(), 1);
    assert_eq!(slot.object_address(), pointer);
    assert_eq!(slot.observe().ref_count(), 2);
    slot.clear().unwrap();
    assert!(slot.is_null());
    assert_eq!(observed.ref_count(), 0);
    assert!(IntrusiveReferenceSlot::<StandardTransient>::from_owned(Handle::default()).is_null());
}
#[test]
fn reference_slot_actual_pointer_and_ownership_remain_native_separate() {
    let slot = IntrusiveReferenceSlot::<StandardTransient>::default();
    let first = Handle::new(StandardTransient);
    assert!(slot.is_null());
    slot.assign(&first).unwrap();
    let address = slot.reference_address();
    assert_reference_slot_ownership(&slot, &first);
    slot.assign(&first).unwrap();
    assert_eq!(first.ref_count(), 2);
    slot.clear().unwrap();
    assert!(slot.is_null());
    assert_eq!(slot.reference_address(), address);
    assert_eq!(first.ref_count(), 1);
}
#[test]
fn reference_slot_retiring_pointer_is_actual_and_conflicts_fail_explicitly() {
    let slot = IntrusiveReferenceSlot::<StandardTransient>::default();
    let old = Handle::new(StandardTransient);
    let next = Handle::new(StandardTransient);
    slot.assign(&old).unwrap();
    let retiring = slot.begin_retirement(next.object_address()).unwrap();
    assert_eq!(slot.object_address(), old.object_address());
    assert!(
        slot.assign(&next)
            .unwrap_err()
            .message()
            .contains("expected idle slot")
    );
    assert_eq!(next.ref_count(), 1);
    slot.assign(&old).unwrap();
    drop(retiring);
    slot.finish_retirement(next.downgrade(), false);
    assert_eq!(slot.object_address(), next.object_address());
    assert_eq!(next.ref_count(), 2);
}

fn assert_reference_slot_ownership(
    slot: &IntrusiveReferenceSlot<StandardTransient>,
    first: &Handle<StandardTransient>,
) {
    assert_eq!(first.ref_count(), 2);
    assert_eq!(slot.ref_count(), 2);
    assert_eq!(slot.object_address(), first.object_address());
    assert_eq!(slot.observe().object_address(), first.object_address());
}
#[test]
fn reference_slot_const_transport_preserves_actual_slot_and_same_target_count() {
    use crate::standard::HandleReference;
    let slot = IntrusiveReferenceSlot::<StandardTransient>::default();
    let target = Handle::new(StandardTransient);
    slot.assign(&target).unwrap();
    let borrowed: &dyn HandleReference<StandardTransient> = &slot;
    assert!(!borrowed.is_null());
    assert_eq!(borrowed.object_address(), target.object_address());
    assert_eq!(borrowed.reference_address(), slot.reference_address());
    assert_eq!(borrowed.observe().ref_count(), 3);
    slot.assign(borrowed).unwrap();
    assert_eq!(target.ref_count(), 2);
}
#[test]
fn reference_slot_retiring_projection_refuses_a_fabricated_live_pointee() {
    use crate::standard::catch_native_failure;
    let slot = IntrusiveReferenceSlot::<StandardTransient>::default();
    let target = Handle::new(StandardTransient);
    slot.assign(&target).unwrap();
    let storage = target.downgrade();
    drop(target);
    drop(slot.begin_retirement(std::ptr::null()).unwrap());
    assert!(!slot.is_null());
    assert_eq!(slot.ref_count(), 0);
    let failure = catch_native_failure(|| Ok(slot.observe().is_null())).unwrap_err();
    assert_retiring_projection_failure(&failure);
    assert_eq!(storage.ref_count(), 0);
    drop(storage);
    slot.finish_retirement(Handle::<StandardTransient>::default().downgrade(), false);
}

fn assert_retiring_projection_failure(failure: &crate::standard::Failure) {
    // The observer must refuse the retired target instead of fabricating a null slot.
    let message = failure.message();
    assert!(message.contains("expected a projectable live pointee"));
}

#[test]
fn reference_slot_retiring_reborrow_checks_actual_pointer_and_phase() {
    let slot = IntrusiveReferenceSlot::<StandardTransient>::default();
    let target = Handle::new(StandardTransient);
    let foreign = Handle::new(StandardTransient);
    slot.assign(&target).unwrap();
    assert!(slot.with_retiring_pointee(&*target, |_| ()).is_err());
    let ownership = slot.begin_retirement(std::ptr::null()).unwrap();
    assert!(slot.with_retiring_pointee(&*foreign, |_| ()).is_err());
    assert!(
        slot.with_retiring_pointee(&*target, |p| std::ptr::eq(p, &*target))
            .unwrap()
    );
    drop(ownership);
    slot.finish_retirement(Handle::<StandardTransient>::default().downgrade(), false);
}

#[test]
fn reference_slot_lease_preserves_one_owner_through_reentrant_replacement() {
    let first = Handle::new(StandardTransient);
    let weak = first.downgrade();
    let slot = IntrusiveReferenceSlot::from_owned_shared(first);
    let old = slot.observe();
    assert_eq!(old.ref_count(), 1);
    let next = Handle::new(StandardTransient);
    slot.assign(&next).unwrap();
    assert_eq!(old.object_address(), weak.object_address());
    assert_eq!(weak.ref_count(), 1);
    assert_eq!(slot.observe().object_address(), next.object_address());
    assert_eq!(next.ref_count(), 2);
    drop(old);
    assert_eq!(weak.ref_count(), 0);
    slot.clear().unwrap();
    assert_eq!(next.ref_count(), 1);
}
