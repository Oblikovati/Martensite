//! Explicit concrete-alias calls with independently enumerated descriptor identities.
use martensite_tkernel::standard::{
    Handle, RuntimeType, RuntimeTyped, StandardTransient, Transient,
};
use martensite_tkxsbase::interface::{
    InterfaceGlobalNodeOfGeneralLib, InterfaceGlobalNodeOfReaderLib, InterfaceNodeOfGeneralLib,
    InterfaceNodeOfReaderLib,
};

fn assert_alias_descriptor_values(
    name: &'static str,
    descriptor: &'static Handle<RuntimeType>,
    dynamic: &'static Handle<RuntimeType>,
    expected: &str,
) {
    let actual = descriptor.get().unwrap();
    assert_eq!(name, expected);
    assert_eq!(actual.name(), expected);
    assert_eq!(actual.parent().unwrap().name(), "Standard_Transient");
    assert!(std::ptr::eq(
        actual.parent().unwrap(),
        StandardTransient::static_type()
    ));
    assert!(std::ptr::eq(descriptor, dynamic));
    assert!(std::ptr::eq(actual.class_handle_ref(), descriptor));
}

#[test]
fn general_global_alias_calls_preserve_its_concrete_identity() {
    let node = InterfaceGlobalNodeOfGeneralLib::new();
    let value: &InterfaceGlobalNodeOfGeneralLib = &node;
    let name: &'static str = <martensite_tkxsbase::interface::InterfaceGlobalNodeOfGeneralLib as martensite_tkernel::standard::RuntimeTyped>::TYPE_NAME;
    let descriptor: &'static Handle<RuntimeType> = <martensite_tkxsbase::interface::InterfaceGlobalNodeOfGeneralLib as martensite_tkernel::standard::RuntimeTyped>::static_type_ref();
    let dynamic: &'static Handle<RuntimeType> = <martensite_tkxsbase::interface::InterfaceGlobalNodeOfGeneralLib as martensite_tkernel::standard::Transient>::dynamic_type_ref(value);
    assert_alias_descriptor_values(name, descriptor, dynamic, "InterfaceGlobalNodeOfGeneralLib");
}

#[test]
fn general_selected_alias_calls_preserve_its_concrete_identity() {
    let node = InterfaceNodeOfGeneralLib::new();
    let value: &InterfaceNodeOfGeneralLib = &node;
    let name: &'static str = <martensite_tkxsbase::interface::InterfaceNodeOfGeneralLib as martensite_tkernel::standard::RuntimeTyped>::TYPE_NAME;
    let descriptor: &'static Handle<RuntimeType> = <martensite_tkxsbase::interface::InterfaceNodeOfGeneralLib as martensite_tkernel::standard::RuntimeTyped>::static_type_ref();
    let dynamic: &'static Handle<RuntimeType> = <martensite_tkxsbase::interface::InterfaceNodeOfGeneralLib as martensite_tkernel::standard::Transient>::dynamic_type_ref(value);
    assert_alias_descriptor_values(name, descriptor, dynamic, "InterfaceNodeOfGeneralLib");
}

#[test]
fn reader_global_alias_calls_preserve_its_concrete_identity() {
    let node = InterfaceGlobalNodeOfReaderLib::new();
    let value: &InterfaceGlobalNodeOfReaderLib = &node;
    let name: &'static str = <martensite_tkxsbase::interface::InterfaceGlobalNodeOfReaderLib as martensite_tkernel::standard::RuntimeTyped>::TYPE_NAME;
    let descriptor: &'static Handle<RuntimeType> = <martensite_tkxsbase::interface::InterfaceGlobalNodeOfReaderLib as martensite_tkernel::standard::RuntimeTyped>::static_type_ref();
    let dynamic: &'static Handle<RuntimeType> = <martensite_tkxsbase::interface::InterfaceGlobalNodeOfReaderLib as martensite_tkernel::standard::Transient>::dynamic_type_ref(value);
    assert_alias_descriptor_values(name, descriptor, dynamic, "InterfaceGlobalNodeOfReaderLib");
}

#[test]
fn reader_selected_alias_calls_preserve_its_concrete_identity() {
    let node = InterfaceNodeOfReaderLib::new();
    let value: &InterfaceNodeOfReaderLib = &node;
    let name: &'static str = <martensite_tkxsbase::interface::InterfaceNodeOfReaderLib as martensite_tkernel::standard::RuntimeTyped>::TYPE_NAME;
    let descriptor: &'static Handle<RuntimeType> = <martensite_tkxsbase::interface::InterfaceNodeOfReaderLib as martensite_tkernel::standard::RuntimeTyped>::static_type_ref();
    let dynamic: &'static Handle<RuntimeType> = <martensite_tkxsbase::interface::InterfaceNodeOfReaderLib as martensite_tkernel::standard::Transient>::dynamic_type_ref(value);
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
    values: &[Handle<StandardTransient>; 4],
) -> [bool; 4] {
    std::array::from_fn(|index| {
        let restored = Handle::<T>::down_cast(&values[index]);
        let matches = !restored.is_null();
        if matches {
            assert_eq!(restored.cast::<StandardTransient>(), values[index]);
        }
        matches
    })
}

fn four_concrete_alias_handles() -> [Handle<StandardTransient>; 4] {
    [
        Handle::new(InterfaceGlobalNodeOfGeneralLib::new()).cast::<StandardTransient>(),
        Handle::new(InterfaceNodeOfGeneralLib::new()).cast::<StandardTransient>(),
        Handle::new(InterfaceGlobalNodeOfReaderLib::new()).cast::<StandardTransient>(),
        Handle::new(InterfaceNodeOfReaderLib::new()).cast::<StandardTransient>(),
    ]
}
