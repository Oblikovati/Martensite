//! Compile-time parent equality and root interface identity.

use martensite_tkernel::standard::{Handle, RuntimeTyped, Transient};
use martensite_tkxsbase::{
    interface::{
        InterfaceGlobalNodeOfGeneralLib, InterfaceGlobalNodeOfReaderLib, InterfaceNodeOfGeneralLib,
        InterfaceNodeOfReaderLib,
    },
    lib_ctl::LibCtlNodeBaseType,
};

fn assert_node_parent<T>()
where
    T: LibCtlNodeBaseType<BaseType = dyn Transient>
        + Default
        + RuntimeTyped
        + Transient
        + Send
        + Sync,
{
    let owned = Handle::new(T::default());
    let parent: Handle<T::BaseType> = owned.upcast();
    assert_eq!(parent.downcast::<T>(), owned);
    assert_eq!(
        T::static_type().parent().unwrap().name(),
        T::BaseType::TYPE_NAME
    );
    let parent = parent.get().unwrap();
    assert!(std::ptr::addr_eq(parent, owned.get().unwrap()));
    assert!(std::ptr::eq(parent.dynamic_type(), T::static_type()));
}

#[test]
fn general_global_node_base_type_is_its_root_interface() {
    assert_node_parent::<InterfaceGlobalNodeOfGeneralLib>();
    let node = InterfaceGlobalNodeOfGeneralLib::new();
    assert!(node.module().is_null());
    assert!(node.protocol().is_null());
    assert!(node.next().is_none());
}

#[test]
fn general_selected_node_base_type_is_its_root_interface() {
    assert_node_parent::<InterfaceNodeOfGeneralLib>();
    let node = InterfaceNodeOfGeneralLib::new();
    assert!(node.module().is_null());
    assert!(node.protocol().is_null());
    assert!(node.next().is_none());
}

#[test]
fn reader_global_node_base_type_is_its_root_interface() {
    assert_node_parent::<InterfaceGlobalNodeOfReaderLib>();
    let node = InterfaceGlobalNodeOfReaderLib::new();
    assert!(node.module().is_null());
    assert!(node.protocol().is_null());
    assert!(node.next().is_none());
}

#[test]
fn reader_selected_node_base_type_is_its_root_interface() {
    assert_node_parent::<InterfaceNodeOfReaderLib>();
    let node = InterfaceNodeOfReaderLib::new();
    assert!(node.module().is_null());
    assert!(node.protocol().is_null());
    assert!(node.next().is_none());
}
