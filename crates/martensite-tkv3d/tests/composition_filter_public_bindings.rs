//! Public composition filter descriptors preserve their declared runtime ancestry.
use martensite_tkernel::standard::{Handle, RuntimeType};
use martensite_tkv3d::select_mgr::{SelectMgrAndFilter, SelectMgrCompositionFilter};

fn assert_composition_static(descriptor: &'static Handle<RuntimeType>) {
    assert_eq!(descriptor.name(), "SelectMgrCompositionFilter");
    assert_eq!(descriptor.parent().unwrap().name(), "SelectMgrFilter");
    assert!(descriptor.is_subtype_name("Standard_Transient"));
    assert!(std::ptr::eq(descriptor.class_handle_ref(), descriptor));
}

fn assert_composition_dynamic(descriptor: &'static Handle<RuntimeType>) {
    let parent = <dyn SelectMgrCompositionFilter as martensite_tkernel::standard::RuntimeTyped>::static_type_ref();
    assert_eq!(descriptor.name(), "SelectMgrAndFilter");
    assert_eq!(
        descriptor.parent().unwrap().name(),
        "SelectMgrCompositionFilter"
    );
    assert!(descriptor.is_subtype_name("SelectMgrFilter"));
    assert!(std::ptr::eq(descriptor.parent().unwrap(), &**parent));
    assert!(std::ptr::eq(descriptor.class_handle_ref(), descriptor));
}

fn emit_composition_binding(identity: &str, name: &str) {
    let assertion =
        format!("MARTENSITE_TYPED_ASSERTION {{\"identity\":\"{identity}\",\"name\":\"{name}\"}}");
    println!("{assertion}");
}

#[test]
fn composition_dynamic_type_public_binding() {
    let owner = Handle::new(SelectMgrAndFilter::default());
    let composition: Handle<dyn SelectMgrCompositionFilter> = owner.cast();
    let value: &dyn SelectMgrCompositionFilter = &*composition;
    let dynamic_descriptor: &'static Handle<RuntimeType> = <dyn martensite_tkv3d::select_mgr::SelectMgrCompositionFilter as martensite_tkernel::standard::Transient>::dynamic_type_ref(value);
    assert_composition_dynamic(dynamic_descriptor);
    emit_composition_binding(
        "composition_dynamic_type_public_binding",
        "SelectMgrAndFilter",
    );
}

#[test]
fn composition_static_type_public_binding() {
    let static_descriptor: &'static Handle<RuntimeType> = <dyn martensite_tkv3d::select_mgr::SelectMgrCompositionFilter as martensite_tkernel::standard::RuntimeTyped>::static_type_ref();
    assert_composition_static(static_descriptor);
    emit_composition_binding(
        "composition_static_type_public_binding",
        "SelectMgrCompositionFilter",
    );
}

#[test]
fn composition_type_name_public_binding() {
    let composition_name: &'static str = <dyn martensite_tkv3d::select_mgr::SelectMgrCompositionFilter as martensite_tkernel::standard::RuntimeTyped>::TYPE_NAME;
    assert_eq!(composition_name, "SelectMgrCompositionFilter");
    emit_composition_binding(
        "composition_type_name_public_binding",
        "SelectMgrCompositionFilter",
    );
}
