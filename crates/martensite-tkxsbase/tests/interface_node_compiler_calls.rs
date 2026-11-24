//! Explicit concrete-alias calls with independently enumerated descriptor identities.
use martensite_tkernel::standard::{Handle, RuntimeType, RuntimeTyped, Transient};
use martensite_tkxsbase::interface::{
    InterfaceGlobalNodeOfGeneralLib, InterfaceGlobalNodeOfReaderLib, InterfaceNodeOfGeneralLib,
    InterfaceNodeOfReaderLib,
};

fn assert_alias_descriptor_values(
    name: &'static str,
    descriptor: &'static RuntimeType,
    dynamic: &'static RuntimeType,
    expected: &str,
) {
    let actual = descriptor;
    assert_eq!(name, expected);
    assert_eq!(actual.name(), expected);
    assert_eq!(actual.parent().unwrap().name(), "Standard_Transient");
    assert!(std::ptr::eq(
        actual.parent().unwrap(),
        <dyn Transient>::static_type()
    ));
    assert!(std::ptr::eq(descriptor, dynamic));
}

#[test]
fn general_global_alias_calls_preserve_its_concrete_identity() {
    let node = InterfaceGlobalNodeOfGeneralLib::new();
    let value: &InterfaceGlobalNodeOfGeneralLib = &node;
    let name: &'static str = <martensite_tkxsbase::interface::InterfaceGlobalNodeOfGeneralLib as martensite_tkernel::standard::RuntimeTyped>::TYPE_NAME;
    let descriptor: &'static RuntimeType = <martensite_tkxsbase::interface::InterfaceGlobalNodeOfGeneralLib as martensite_tkernel::standard::RuntimeTyped>::static_type();
    let dynamic: &'static RuntimeType = <martensite_tkxsbase::interface::InterfaceGlobalNodeOfGeneralLib as martensite_tkernel::standard::Transient>::dynamic_type(value);
    assert_alias_descriptor_values(name, descriptor, dynamic, "InterfaceGlobalNodeOfGeneralLib");
}

#[test]
fn general_selected_alias_calls_preserve_its_concrete_identity() {
    let node = InterfaceNodeOfGeneralLib::new();
    let value: &InterfaceNodeOfGeneralLib = &node;
    let name: &'static str = <martensite_tkxsbase::interface::InterfaceNodeOfGeneralLib as martensite_tkernel::standard::RuntimeTyped>::TYPE_NAME;
    let descriptor: &'static RuntimeType = <martensite_tkxsbase::interface::InterfaceNodeOfGeneralLib as martensite_tkernel::standard::RuntimeTyped>::static_type();
    let dynamic: &'static RuntimeType = <martensite_tkxsbase::interface::InterfaceNodeOfGeneralLib as martensite_tkernel::standard::Transient>::dynamic_type(value);
    assert_alias_descriptor_values(name, descriptor, dynamic, "InterfaceNodeOfGeneralLib");
}

#[test]
fn reader_global_alias_calls_preserve_its_concrete_identity() {
    let node = InterfaceGlobalNodeOfReaderLib::new();
    let value: &InterfaceGlobalNodeOfReaderLib = &node;
    let name: &'static str = <martensite_tkxsbase::interface::InterfaceGlobalNodeOfReaderLib as martensite_tkernel::standard::RuntimeTyped>::TYPE_NAME;
    let descriptor: &'static RuntimeType = <martensite_tkxsbase::interface::InterfaceGlobalNodeOfReaderLib as martensite_tkernel::standard::RuntimeTyped>::static_type();
    let dynamic: &'static RuntimeType = <martensite_tkxsbase::interface::InterfaceGlobalNodeOfReaderLib as martensite_tkernel::standard::Transient>::dynamic_type(value);
    assert_alias_descriptor_values(name, descriptor, dynamic, "InterfaceGlobalNodeOfReaderLib");
}

#[test]
fn reader_selected_alias_calls_preserve_its_concrete_identity() {
    let node = InterfaceNodeOfReaderLib::new();
    let value: &InterfaceNodeOfReaderLib = &node;
    let name: &'static str = <martensite_tkxsbase::interface::InterfaceNodeOfReaderLib as martensite_tkernel::standard::RuntimeTyped>::TYPE_NAME;
    let descriptor: &'static RuntimeType = <martensite_tkxsbase::interface::InterfaceNodeOfReaderLib as martensite_tkernel::standard::RuntimeTyped>::static_type();
    let dynamic: &'static RuntimeType = <martensite_tkxsbase::interface::InterfaceNodeOfReaderLib as martensite_tkernel::standard::Transient>::dynamic_type(value);
    assert_alias_descriptor_values(name, descriptor, dynamic, "InterfaceNodeOfReaderLib");
}

#[test]
fn each_alias_downcasts_only_to_its_actual_specialization() {
    let values = four_concrete_alias_handles();
    assert_eq!(
        downcast_membership::<InterfaceGlobalNodeOfGeneralLib>(&values),
        [true, false, false, false]
    );
    assert_eq!(
        downcast_membership::<InterfaceNodeOfGeneralLib>(&values),
        [false, true, false, false]
    );
    assert_eq!(
        downcast_membership::<InterfaceGlobalNodeOfReaderLib>(&values),
        [false, false, true, false]
    );
    assert_eq!(
        downcast_membership::<InterfaceNodeOfReaderLib>(&values),
        [false, false, false, true]
    );
}

fn downcast_membership<T: RuntimeTyped + Transient>(
    values: &[Handle<dyn Transient>; 4],
) -> [bool; 4] {
    std::array::from_fn(|index| {
        let restored = values[index].downcast::<T>();
        let matches = !restored.is_null();
        if matches {
            assert_eq!(restored.cast::<dyn Transient>(), values[index]);
        }
        matches
    })
}

fn four_concrete_alias_handles() -> [Handle<dyn Transient>; 4] {
    [
        Handle::new(InterfaceGlobalNodeOfGeneralLib::new()).into_upcast::<dyn Transient>(),
        Handle::new(InterfaceNodeOfGeneralLib::new()).into_upcast::<dyn Transient>(),
        Handle::new(InterfaceGlobalNodeOfReaderLib::new()).into_upcast::<dyn Transient>(),
        Handle::new(InterfaceNodeOfReaderLib::new()).into_upcast::<dyn Transient>(),
    ]
}
