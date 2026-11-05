//! Compile-time parent equality and unchanged concrete node projection.

use martensite_tkernel::standard::{Handle, RuntimeTyped, StandardTransient, Transient};
use martensite_tkxsbase::{
    interface::{
        InterfaceGlobalNodeOfGeneralLib, InterfaceGlobalNodeOfReaderLib, InterfaceNodeOfGeneralLib,
        InterfaceNodeOfReaderLib,
    },
    lib_ctl::LibCtlNodeBaseType,
};

fn assert_node_parent<T>()
where
    T: LibCtlNodeBaseType<BaseType = StandardTransient>
        + Default
        + RuntimeTyped
        + Transient
        + Send
        + Sync,
{
    let owned = Handle::new(T::default());
    let parent: Handle<T::BaseType> = owned.cast();
    assert_eq!(Handle::<T>::down_cast(&parent), owned);
    assert_eq!(
        T::static_type().parent().unwrap().name(),
        T::BaseType::TYPE_NAME
    );
    let object = owned.get().unwrap();
    assert!(std::ptr::eq(parent.get().unwrap(), object.transient_base()));
}

#[test]
fn general_global_node_base_type_is_its_embedded_transient() {
    assert_node_parent::<InterfaceGlobalNodeOfGeneralLib>();
    let node = InterfaceGlobalNodeOfGeneralLib::new();
    assert!(node.module().is_null());
    assert!(node.protocol().is_null());
    assert!(node.next().is_none());
}

#[test]
fn general_selected_node_base_type_is_its_embedded_transient() {
    assert_node_parent::<InterfaceNodeOfGeneralLib>();
    let node = InterfaceNodeOfGeneralLib::new();
    assert!(node.module().is_null());
    assert!(node.protocol().is_null());
    assert!(node.next().is_none());
}

#[test]
fn reader_global_node_base_type_is_its_embedded_transient() {
    assert_node_parent::<InterfaceGlobalNodeOfReaderLib>();
    let node = InterfaceGlobalNodeOfReaderLib::new();
    assert!(node.module().is_null());
    assert!(node.protocol().is_null());
    assert!(node.next().is_none());
}

#[test]
fn reader_selected_node_base_type_is_its_embedded_transient() {
    assert_node_parent::<InterfaceNodeOfReaderLib>();
    let node = InterfaceNodeOfReaderLib::new();
    assert!(node.module().is_null());
    assert!(node.protocol().is_null());
    assert!(node.next().is_none());
}
