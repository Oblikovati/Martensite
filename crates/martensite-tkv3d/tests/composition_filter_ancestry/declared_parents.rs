//! Declared parents project to the same object and all public filter views remain Send+Sync.
use martensite_tkernel::standard::{Handle, RuntimeTyped};
use martensite_tkv3d::select_mgr::*;
use std::{any::TypeId, marker::PhantomData};

fn thread_contract<T: ?Sized + Send + Sync>() {
    let marker = PhantomData::<&T>;
    std::hint::black_box(marker);
}

#[test]
fn declared_filter_parent_aliases_match_native_static_assertions() {
    let expected = TypeId::of::<dyn SelectMgrCompositionFilter>();
    assert_eq!(TypeId::of::<SelectMgrAndFilterBaseType>(), expected);
    assert_eq!(TypeId::of::<SelectMgrOrFilterBaseType>(), expected);
    assert_eq!(TypeId::of::<SelectMgrAndOrFilterBaseType>(), expected);
    assert_eq!(
        TypeId::of::<SelectMgrCompositionFilterBaseType>(),
        TypeId::of::<dyn SelectMgrFilter>()
    );
}

#[test]
fn public_filter_parent_interfaces_and_concrete_filters_remain_send_sync() {
    thread_contract::<SelectMgrCompositionFilterBaseType>();
    thread_contract::<SelectMgrAndFilterBaseType>();
    thread_contract::<SelectMgrOrFilterBaseType>();
    thread_contract::<SelectMgrAndOrFilterBaseType>();
    thread_contract::<SelectMgrAndFilter>();
    thread_contract::<SelectMgrOrFilter>();
    thread_contract::<SelectMgrAndOrFilter>();
}

#[test]
fn parent_views_retain_one_owner_and_mutate_the_actual_stored_filters() {
    let owner = Handle::new(SelectMgrAndFilter::default());
    let composition: Handle<SelectMgrAndFilterBaseType> = owner.cast();
    let parent: Handle<SelectMgrCompositionFilterBaseType> = composition.cast();
    assert_eq!(owner.object_address(), parent.object_address());
    assert_eq!(owner.object_address(), composition.object_address());
    let child = Handle::new(SelectMgrOrFilter::default()).cast();
    composition.add(&child);
    drop(owner);
    let recovered = Handle::<SelectMgrAndFilter>::down_cast(&parent);
    assert!(recovered.is_in(&child));
    composition.remove(&child);
    assert!(recovered.is_empty());
    assert!(parent.is_kind(<dyn SelectMgrFilter>::static_type()));
    assert!(parent.cast::<dyn SelectMgrSelectableObject>().is_null());
}
