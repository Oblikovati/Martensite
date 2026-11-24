//! Canonical names and parents for the foundation descriptors.
use martensite_tkernel::standard::{
    Handle, Persistent, RuntimeType, RuntimeTyped, StandardPersistent, Transient,
};

#[test]
fn standard_type_rtti_accessors_select_the_runtime_type_descriptor() {
    let descriptor = RuntimeType::static_type();
    assert_eq!("Standard_Type", RuntimeType::TYPE_NAME);
    assert_eq!("Standard_Type", descriptor.name());
    assert_eq!("Standard_Transient", descriptor.parent().unwrap().name());
    assert!(std::ptr::eq(descriptor, descriptor.dynamic_type()));
}

#[test]
fn standard_persistent_rtti_accessors_select_the_persistent_descriptor() {
    let descriptor = StandardPersistent::static_type();
    let object = Handle::new(StandardPersistent::default()).into_upcast::<dyn Persistent>();
    assert_eq!("Standard_Persistent", StandardPersistent::TYPE_NAME);
    assert_eq!("Standard_Persistent", descriptor.name());
    assert_eq!("Standard_Transient", descriptor.parent().unwrap().name());
    assert!(std::ptr::eq(descriptor, object.dynamic_type().unwrap()));
    assert!(std::ptr::eq(descriptor, <dyn Persistent>::static_type()));
    assert_eq!(object.ref_count(), 1);
}
