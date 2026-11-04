//! Exact compiler calls qualify the real dyn interface when rustdoc61 omits its foreign impl items.
use martensite_tkernel::standard::{Handle, RuntimeType};
use martensite_tkv3d::select_mgr::{SelectMgrAndFilter, SelectMgrCompositionFilter};

fn assert_composition_descriptors(
    name: &'static str,
    descriptor: &'static Handle<RuntimeType>,
    dynamic: &'static Handle<RuntimeType>,
) {
    assert_eq!(name, "SelectMgrCompositionFilter");
    assert_eq!(descriptor.name(), name);
    assert_eq!(descriptor.parent().unwrap().name(), "SelectMgrFilter");
    assert_eq!(dynamic.name(), "SelectMgrAndFilter");
    assert!(std::ptr::eq(dynamic.parent().unwrap(), &**descriptor));
    assert!(std::ptr::eq(descriptor.class_handle_ref(), descriptor));
    emit_composition_assertion(name, dynamic.name());
}

fn emit_composition_assertion(name: &str, dynamic: &str) {
    let current = std::thread::current();
    let identity = current.name().unwrap();
    println!(
        "MARTENSITE_TYPED_ASSERTION {{\"identity\":\"{identity}\",\"name\":\"{name}\",\"dynamic\":\"{dynamic}\"}}"
    );
}

#[test]
fn composition_interface_static_and_dynamic_identity() {
    let owner = Handle::new(SelectMgrAndFilter::default());
    let composition: Handle<dyn SelectMgrCompositionFilter> = owner.cast();
    let value: &dyn SelectMgrCompositionFilter = &*composition;
    let name: &'static str = <dyn martensite_tkv3d::select_mgr::SelectMgrCompositionFilter as martensite_tkernel::standard::RuntimeTyped>::TYPE_NAME;
    let descriptor: &'static Handle<RuntimeType> = <dyn martensite_tkv3d::select_mgr::SelectMgrCompositionFilter as martensite_tkernel::standard::RuntimeTyped>::static_type_ref();
    let dynamic: &'static Handle<RuntimeType> = <dyn martensite_tkv3d::select_mgr::SelectMgrCompositionFilter as martensite_tkernel::standard::Transient>::dynamic_type_ref(value);
    assert_composition_descriptors(name, descriptor, dynamic);
}
