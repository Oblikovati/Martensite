//! Transient runtime type queries and persistent objects.
use martensite_tkernel::ncollection::{AllocatorObject, HeapAllocatorObject};
use martensite_tkernel::standard::{
    Handle, RuntimeTyped, StandardPersistent, StandardTransient, Transient, type_queries,
};
use std::sync::atomic::Ordering;

#[test]
fn is_instance_and_is_kind_by_descriptor_and_name() {
    let object = HeapAllocatorObject::default();
    assert!(object.is_instance(HeapAllocatorObject::static_type()));
    assert!(!object.is_instance(AllocatorObject::static_type()));
    assert!(object.is_kind(AllocatorObject::static_type()));
    assert!(object.is_instance_name("NCollection_HeapAllocator"));
    assert!(object.is_instance_name("NCollection_HeapAllocator\0tail"));
    assert!(!object.is_instance_name("NCollection_BaseAllocator"));
    assert!(object.is_kind_name("NCollection_BaseAllocator"));
    assert!(!object.is_kind_name("Standard_Persistent"));
}
#[test]
fn type_queries_reject_null_objects() {
    let object = StandardPersistent::default();
    assert!(type_queries::is_kind::<StandardTransient>(Some(&object)));
    assert!(type_queries::is_instance::<StandardPersistent>(Some(
        &object
    )));
    assert!(!type_queries::is_instance::<StandardTransient>(Some(
        &object
    )));
    assert!(!type_queries::is_kind::<StandardTransient>(None));
    let null = Handle::<StandardTransient>::default();
    assert!(!type_queries::is_kind_handle::<StandardTransient, _>(&null));
    assert!(!type_queries::is_instance_handle::<StandardTransient, _>(
        &null
    ));
    let handle = Handle::new(StandardPersistent::default()).upcast::<dyn Transient>();
    assert!(type_queries::is_kind_handle::<StandardPersistent, _>(
        &handle
    ));
    assert!(type_queries::is_instance_handle::<StandardPersistent, _>(
        &handle
    ));
    assert!(type_queries::is_kind::<StandardTransient>(
        handle.view::<dyn Transient>().transient()
    ));
}
#[test]
fn persistent_numbers_start_at_zero_and_are_shared() {
    let handle = Handle::new(StandardPersistent::default());
    assert_eq!(0, handle.type_num().load(Ordering::Relaxed));
    assert_eq!(0, handle.ref_num().load(Ordering::Relaxed));
    let copy = handle.clone();
    copy.type_num().store(7, Ordering::Relaxed);
    assert_eq!(7, handle.type_num().load(Ordering::Relaxed));
    assert_eq!(
        "Standard_Transient",
        handle.dynamic_type().unwrap().parent().unwrap().name()
    );
}
