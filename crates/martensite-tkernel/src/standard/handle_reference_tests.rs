//! Erased aliases preserve slot identity, and snapshots keep the object they observed.
use super::*;
use crate::standard::{SharedState, StandardTransient};
struct ReferenceSlots {
    first: Handle<StandardTransient>,
    second: Handle<StandardTransient>,
}
fn first_slot(slots: &ReferenceSlots) -> &Handle<StandardTransient> {
    &slots.first
}
fn second_slot(slots: &ReferenceSlots) -> &Handle<StandardTransient> {
    &slots.second
}
#[test]
fn erased_local_slot_preserves_identity_and_explicit_ownership() {
    let target = Handle::new(StandardTransient);
    let reference: &dyn HandleReference<StandardTransient> = &target;
    assert_eq!(
        reference.reference_address(),
        std::ptr::from_ref(&target).cast()
    );
    assert_eq!(reference.object_address(), target.object_address());
    assert!(!reference.is_null());
    assert_eq!(reference.observe().ref_count(), 1);
    let owned = reference.clone_handle();
    assert_eq!(owned.ref_count(), 2);
    drop(owned);
    assert_eq!(target.ref_count(), 1);
}
#[test]
fn erased_field_alias_rereads_replacement_while_snapshots_own_old_targets() {
    let first = Handle::new(StandardTransient);
    let next = Handle::new(StandardTransient);
    let fields = reference_slot_pair(&first);
    let alias = HandleFieldRef::new(&fields, first_slot);
    let other = HandleFieldRef::new(&fields, second_slot);
    let reference: &dyn HandleReference<StandardTransient> = &alias;
    assert_ne!(reference.reference_address(), other.reference_address());
    let address = reference.reference_address();
    assert_eq!(reference.ref_count(), 3);
    let projected = ProjectedHandleReference::<StandardTransient, StandardTransient>::new(&alias);
    assert_eq!(projected.ref_count(), 3);
    let observed = reference.observe();
    fields.write().first = next.clone();
    assert_eq!(projected.ref_count(), 2);
    assert_eq!(reference.reference_address(), address);
    assert_eq!(reference.object_address(), next.object_address());
    assert_eq!(observed.object_address(), first.object_address());
    assert_eq!(first.ref_count(), 3);
    drop(observed);
    assert_eq!(first.ref_count(), 2);
    fields.write().first = Handle::default();
    assert_eq!(projected.ref_count(), 0);
    assert!(reference.is_null() && reference.clone_handle().is_null());
}

fn reference_slot_pair(first: &Handle<StandardTransient>) -> SharedState<ReferenceSlots> {
    SharedState::new(ReferenceSlots {
        first: first.clone(),
        second: first.clone(),
    })
}
#[test]
fn weak_upgrade_owns_live_storage_and_refuses_expired_or_null_pointers() {
    let target = Handle::new(StandardTransient);
    let weak = target.downgrade();
    let observed = weak.upgrade();
    assert_eq!(observed.ref_count(), 2);
    assert_eq!(target.ref_count(), 2);
    assert_eq!(observed.object_address(), target.object_address());
    drop(observed);
    assert_eq!(target.ref_count(), 1);
    drop(target);
    assert!(weak.upgrade().is_null());
    assert!(
        crate::standard::WeakHandle::<StandardTransient>::default()
            .upgrade()
            .is_null()
    );
}
#[test]
fn weak_static_handle_does_not_add_an_owner() {
    static TARGET: std::sync::LazyLock<Handle<StandardTransient>> =
        std::sync::LazyLock::new(|| Handle::new(StandardTransient));
    let target = &*TARGET;
    let weak = target.downgrade();
    let observed = weak.upgrade();
    assert_eq!(target.ref_count(), 2);
    assert_eq!(observed.object_address(), target.object_address());
    drop(observed);
    assert_eq!(target.ref_count(), 1);
}

struct FakeMetadataHandleReference<'a> {
    source: &'a dyn HandleReference<StandardTransient>,
}
impl HandleReference<StandardTransient> for FakeMetadataHandleReference<'_> {
    fn observe(&self) -> HandleObserver<'_, StandardTransient> {
        panic!("observed original object; expected pointer metadata only")
    }
    fn reference_address(&self) -> *const () {
        self.source.reference_address()
    }
    fn object_address(&self) -> *const () {
        self.source.object_address()
    }
    fn is_null(&self) -> bool {
        self.source.is_null()
    }
}
#[test]
fn projected_reference_metadata_never_observes_the_original_object() {
    let target = Handle::new(StandardTransient);
    let fields = reference_slot_pair(&target);
    let slot = HandleFieldRef::new(&fields, first_slot);
    let source = FakeMetadataHandleReference { source: &slot };
    let projected = ProjectedHandleReference::<StandardTransient, StandardTransient>::new(&source);
    assert_eq!(projected.reference_address(), slot.reference_address());
    assert_eq!(projected.object_address(), target.object_address());
    assert!(!projected.is_null());
    fields.write().first = Handle::default();
    assert!(projected.is_null());
    assert!(projected.object_address().is_null());
    assert_eq!(target.ref_count(), 2);
}
