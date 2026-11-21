//! Handle ownership, type projection and actual resource behavior.
use martensite_tkernel::ncollection::{
    AllocatorObject, AllocatorSource, HeapAllocatorObject, IncrementalAllocatorObject,
};
use martensite_tkernel::standard::{Handle, RuntimeTyped, StandardTransient, Transient};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
#[repr(C)]
struct DropTracked {
    drops: Arc<AtomicUsize>,
}
martensite_tkernel::impl_transient!(DropTracked, "DropTracked", StandardTransient);
impl Drop for DropTracked {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}
#[test]
fn casts_borrowed_views_and_nullification_preserve_lifetime() {
    let drops: Arc<AtomicUsize> = Arc::default();
    let handle = Handle::new(DropTracked {
        drops: drops.clone(),
    });
    assert_eq!(handle.ref_count(), 1);
    let view = handle.view::<dyn Transient>();
    assert!(!view.is_null());
    assert_eq!(handle.ref_count(), 1);
    let mut base = handle.upcast::<dyn Transient>();
    assert_eq!(handle.ref_count(), 2);
    assert_eq!(view.object_address(), base.object_address());
    assert_eq!(base.clone().ref_count(), 3);
    base.nullify();
    assert!(base.is_null());
    assert_eq!(handle.ref_count(), 1);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(handle);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
}
#[test]
fn descriptors_exact_names_and_failed_views() {
    let handle = Handle::new(IncrementalAllocatorObject::default());
    assert!(handle.is_instance_name("NCollection_IncAllocator"));
    assert!(!handle.is_instance_name("NCollection_BaseAllocator"));
    assert!(handle.is_kind(AllocatorObject::static_type()));
    assert!(!handle.is_kind(HeapAllocatorObject::static_type()));
    assert!(handle.view::<HeapAllocatorObject>().is_null());
    assert!(handle.view::<HeapAllocatorObject>().as_ptr().is_null());
    assert_eq!(
        handle.dynamic_type().unwrap().parent().unwrap().name(),
        "NCollection_BaseAllocator"
    );
    assert!(
        Handle::<StandardTransient>::default()
            .dynamic_type()
            .is_none()
    );
    assert!(!Handle::<StandardTransient>::default().is_kind_name("Standard_Transient"));
}
#[test]
fn cloning_and_dropping_owner_counts_is_thread_safe() {
    let handle = Handle::new(StandardTransient);
    std::thread::scope(|scope| {
        for _ in 0..4 {
            let borrowed = &handle;
            scope.spawn(move || {
                for _ in 0..1000 {
                    let copy = borrowed.clone();
                    assert!(copy.ref_count() >= 2);
                }
            });
        }
    });
    assert_eq!(handle.ref_count(), 1);
    assert_eq!(handle.downgrade().ref_count(), 1);
}
#[test]
fn polymorphic_allocator_objects_own_real_allocations() {
    let incremental = Handle::new(IncrementalAllocatorObject::default());
    let base = incremental.upcast::<dyn AllocatorSource>();
    assert_eq!(
        incremental.resource(),
        base.allocator_object_base().resource()
    );
    let allocation = base
        .allocator_object_base()
        .resource()
        .allocate_value(42_i32);
    assert_eq!(*allocation, 42);
    assert!(incremental.resource().reset(false).is_err());
    drop(allocation);
    incremental.resource().reset(false).unwrap();
    let heap = Handle::new(HeapAllocatorObject::default());
    assert_eq!(
        heap.resource()
            .allocate(16)
            .expect("malloc succeeds")
            .as_slice()
            .len(),
        16
    );
    assert_ne!(
        heap.resource(),
        &martensite_tkernel::ncollection::BaseAllocator::global_heap()
    );
}
