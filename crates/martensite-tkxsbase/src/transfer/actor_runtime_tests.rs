//! One native actor-parent class has one descriptor across storage and virtual views.
use super::*;
use martensite_tkernel::standard::{Handle, RuntimeTyped, StandardTransient};

fn assert_actor_views<K, Child, Parent>(owner: Handle<Child>)
where
    K: TransferProcessKind,
    Child: RuntimeTyped + TransferActorOps<K>,
    Parent: RuntimeTyped + TransferActorOps<K>,
    dyn TransferActorOps<K>: RuntimeTyped,
{
    let erased: Handle<StandardTransient> = owner.cast();
    let virtual_view: Handle<dyn TransferActorOps<K>> = erased.cast();
    let parent: Handle<Parent> = erased.cast();
    assert!(!virtual_view.is_null() && !parent.is_null());
    assert_eq!(Handle::<Child>::down_cast(&virtual_view), owner);
    assert_eq!(Handle::<Parent>::down_cast(&virtual_view), parent);
    virtual_view.set_last(true);
    assert!(owner.is_last());
    parent.set_last(false);
    assert!(!owner.is_last());
}

#[test]
fn transient_actor_keeps_parent_root_and_specific_interface_casters() {
    let owner = Handle::new(TransferActorOfTransientProcess::new());
    assert_actor_views::<TransferTransientKind, _, TransferActorOfProcessForTransient>(
        owner.clone(),
    );
    let specific: Handle<dyn TransferActorOfTransientProcessOps> = owner.cast();
    assert!(!specific.is_null());
    assert_eq!(
        Handle::<TransferActorOfTransientProcess>::down_cast(&specific),
        owner
    );
    assert!(std::ptr::eq(
        TransferActorOfTransientProcessBaseType::static_type_ref(),
        TransferActorOfProcessForTransient::static_type_ref()
    ));
}

#[test]
fn finder_actor_keeps_parent_root_and_specific_interface_casters() {
    let owner = Handle::new(TransferActorOfFinderProcess::new());
    assert_actor_views::<TransferFinderKind, _, TransferActorOfProcessForFinder>(owner.clone());
    let specific: Handle<dyn TransferActorOfFinderProcessOps> = owner.cast();
    assert!(!specific.is_null());
    assert_eq!(
        Handle::<TransferActorOfFinderProcess>::down_cast(&specific),
        owner
    );
    assert!(std::ptr::eq(
        TransferActorOfFinderProcessBaseType::static_type_ref(),
        TransferActorOfProcessForFinder::static_type_ref()
    ));
}

#[test]
fn actor_parent_interface_is_kind_queries_use_the_real_class_descriptor() {
    let finder = Handle::new(TransferActorOfFinderProcess::new());
    let transient = Handle::new(TransferActorOfTransientProcess::new());
    assert!(finder.is_kind(TransferActorOfFinderProcessBaseType::static_type()));
    assert!(transient.is_kind(TransferActorOfTransientProcessBaseType::static_type()));
    assert!(!finder.is_kind(TransferActorOfTransientProcessBaseType::static_type()));
    assert!(!transient.is_kind(TransferActorOfFinderProcessBaseType::static_type()));
}

#[test]
fn class_interface_registration_orders_share_descriptors_without_cycles() {
    std::thread::scope(|scope| {
        for index in 0..8 {
            scope.spawn(move || {
                for _ in 0..100 {
                    assert_registration_pair(index % 2 == 0);
                }
            });
        }
    });
}

fn assert_registration_pair(class_first: bool) {
    if class_first {
        TransferActorOfProcessForTransient::static_type();
        TransferActorOfProcessForFinder::static_type();
    } else {
        TransferActorOfTransientProcessBaseType::static_type();
        TransferActorOfFinderProcessBaseType::static_type();
    }
    assert!(std::ptr::eq(
        TransferActorOfProcessForTransient::static_type_ref(),
        TransferActorOfTransientProcessBaseType::static_type_ref()
    ));
    assert!(std::ptr::eq(
        TransferActorOfProcessForFinder::static_type_ref(),
        TransferActorOfFinderProcessBaseType::static_type_ref()
    ));
}
