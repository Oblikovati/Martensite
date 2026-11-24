//! Concrete generated-node identities, independently enumerated from their declarations.
use crate::interface::{
    InterfaceGlobalNodeOfGeneralLib, InterfaceGlobalNodeOfReaderLib, InterfaceNodeOfGeneralLib,
    InterfaceNodeOfReaderLib,
};
use martensite_tkernel::standard::{Handle, RuntimeTyped, Transient};

fn check_descriptor<T: Default + RuntimeTyped + Transient>(name: &str) {
    let owned = Handle::new(T::default());
    assert_eq!(T::TYPE_NAME, name);
    assert_eq!(owned.get().unwrap().dynamic_type().name(), name);
    assert_eq!(
        owned.get().unwrap().dynamic_type().parent().unwrap().name(),
        "Standard_Transient"
    );
    assert!(std::ptr::eq(
        owned.get().unwrap().dynamic_type(),
        T::static_type()
    ));
    check_handle_roundtrip(&owned);
}

fn check_handle_roundtrip<T: RuntimeTyped + Transient>(owned: &Handle<T>) {
    let erased: Handle<dyn Transient> = owned.cast();
    let restored = erased.downcast::<T>();
    assert!(!restored.is_null());
    assert_eq!(&restored, owned);
    assert!(erased.is_kind_name("Standard_Transient"));
    assert!(!erased.is_kind_name("InterfaceCheck"));
}

#[test]
fn general_global_node_has_its_concrete_descriptor() {
    check_descriptor::<InterfaceGlobalNodeOfGeneralLib>("InterfaceGlobalNodeOfGeneralLib");
}

#[test]
fn general_node_has_its_concrete_descriptor() {
    check_descriptor::<InterfaceNodeOfGeneralLib>("InterfaceNodeOfGeneralLib");
}

#[test]
fn reader_global_node_has_its_concrete_descriptor() {
    check_descriptor::<InterfaceGlobalNodeOfReaderLib>("InterfaceGlobalNodeOfReaderLib");
}

#[test]
fn reader_node_has_its_concrete_descriptor() {
    check_descriptor::<InterfaceNodeOfReaderLib>("InterfaceNodeOfReaderLib");
}

#[test]
fn distinct_node_kinds_never_downcast_to_each_other() {
    let general: Handle<dyn Transient> =
        Handle::new(InterfaceNodeOfGeneralLib::new()).into_upcast();
    assert!(general.downcast::<InterfaceNodeOfReaderLib>().is_null());
    assert!(
        general
            .downcast::<InterfaceGlobalNodeOfGeneralLib>()
            .is_null()
    );
    assert!(!std::ptr::eq(
        InterfaceNodeOfGeneralLib::static_type(),
        InterfaceNodeOfReaderLib::static_type()
    ));
}
