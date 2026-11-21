//! The sticky custom-source flag of an allocator registration (#584).
#![allow(unsafe_code)] // A custom source class implements the unsafe allocator contract.
use crate::ncollection::{
    AllocatorObject, AllocatorSource, BaseAllocator, HeapAllocatorObject,
    IncrementalAllocatorObject,
};
use crate::standard::Handle;

#[derive(Default)]
struct CustomSource {
    parent: AllocatorObject,
}
crate::impl_transient!(
    CustomSource,
    "Martensite_RegistrationCustomSource",
    AllocatorObject,
    interfaces: [dyn AllocatorSource]
);
// SAFETY: allocation stays on the embedded common policy.
unsafe impl AllocatorSource for CustomSource {
    fn allocator_object_base(&self) -> &AllocatorObject {
        &self.parent
    }
}

#[test]
fn builtin_classes_never_mark_the_registration_custom() {
    let incremental = Handle::new(IncrementalAllocatorObject::default());
    let heap = Handle::new(HeapAllocatorObject::default());
    for allocator in [
        BaseAllocator::from_handle(&incremental),
        BaseAllocator::from_handle(&heap),
    ] {
        assert!(allocator.fields.registration.is_registered());
        assert!(!allocator.fields.registration.ever_custom());
    }
}

#[test]
fn a_custom_class_marks_the_registration_after_it_retires() {
    let source = Handle::new(CustomSource::default());
    let lease = BaseAllocator::from_handle(&source).storage_lease();
    assert!(lease.fields.registration.ever_custom());
    drop(source);
    assert!(lease.fields.registration.observe().is_null());
    assert!(lease.fields.registration.is_registered());
    assert!(lease.fields.registration.ever_custom());
}

#[test]
fn a_policy_without_a_source_is_neither_registered_nor_custom() {
    let allocator = BaseAllocator::incremental_policy(4096);
    assert!(!allocator.fields.registration.is_registered());
    assert!(!allocator.fields.registration.ever_custom());
}
