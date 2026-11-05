//! Runtime descriptors preserve actual ancestry, casts and independent handle counters.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{
    Handle, RuntimeType, RuntimeTyped, StandardTransient, Transient,
};
use martensite_tkfeat::{
    brep_feat::BRepFeatBuilder,
    loc_ope::{CSIntersector, GeneratedShape, GluedShape, WiresOnShape},
};

// Reifying the bound proves each public type without an empty runtime helper.
struct LocalOperationThreadProof<T: ?Sized + Send + Sync>(std::marker::PhantomData<T>);

#[test]
fn local_operation_descriptors_have_source_names_and_direct_parents() {
    assert_hierarchy(
        <dyn GeneratedShape>::static_type(),
        "LocOpe_GeneratedShape",
        "Standard_Transient",
    );
    assert_hierarchy(
        GluedShape::static_type(),
        "LocOpe_GluedShape",
        "LocOpe_GeneratedShape",
    );
    assert_hierarchy(
        WiresOnShape::static_type(),
        "LocOpe_WiresOnShape",
        "Standard_Transient",
    );
}

fn assert_hierarchy(descriptor: &RuntimeType, name: &str, parent: &str) {
    assert_eq!(descriptor.name(), name);
    assert_eq!(descriptor.parent().unwrap().name(), parent);
    assert!(descriptor.is_subtype(StandardTransient::static_type()));
}

#[test]
fn local_operation_static_type_names_equal_registered_names() {
    assert_eq!(<dyn GeneratedShape>::TYPE_NAME, "LocOpe_GeneratedShape");
    assert_eq!(GluedShape::TYPE_NAME, "LocOpe_GluedShape");
    assert_eq!(WiresOnShape::TYPE_NAME, "LocOpe_WiresOnShape");
}

#[test]
fn local_operation_descriptor_handles_are_process_stable() {
    let generated = <dyn GeneratedShape>::static_type_ref();
    let glued = GluedShape::static_type_ref();
    let wires = WiresOnShape::static_type_ref();
    assert!(std::ptr::eq(
        generated,
        <dyn GeneratedShape>::static_type_ref()
    ));
    assert!(std::ptr::eq(glued, GluedShape::static_type_ref()));
    assert!(std::ptr::eq(wires, WiresOnShape::static_type_ref()));
    assert!(std::ptr::eq(
        glued.get().unwrap(),
        GluedShape::static_type()
    ));
    assert!(std::ptr::eq(
        wires.get().unwrap(),
        WiresOnShape::static_type()
    ));
}

#[test]
fn dynamic_descriptor_handles_borrow_the_exact_class_singletons() {
    let glued = GluedShape::default();
    let generated: &dyn GeneratedShape = &glued;
    let wires = WiresOnShape::new(&Shape::default());
    assert!(std::ptr::eq(
        generated.dynamic_type_ref(),
        GluedShape::static_type_ref()
    ));
    assert!(std::ptr::eq(
        wires.dynamic_type_ref(),
        WiresOnShape::static_type_ref()
    ));
}

#[test]
fn generated_shape_handle_preserves_most_derived_type_and_downcast_identity() {
    let glued = Handle::new(GluedShape::default());
    let generated = glued.cast::<dyn GeneratedShape>();
    let root = generated.cast::<StandardTransient>();
    assert_eq!(
        generated.get().unwrap().dynamic_type().name(),
        "LocOpe_GluedShape"
    );
    assert!(generated.is_kind(<dyn GeneratedShape>::static_type()));
    assert!(generated.is_kind(StandardTransient::static_type()));
    assert!(!generated.is_kind(WiresOnShape::static_type()));
    assert_eq!(generated.cast::<GluedShape>(), glued);
    assert!(generated.cast::<WiresOnShape>().is_null());
    assert_eq!(root.cast::<GluedShape>(), glued);
    assert_eq!(glued.ref_count(), 3);
}

#[test]
fn cloned_glued_shape_starts_with_an_independent_zero_reference_counter() {
    let original = Handle::new(GluedShape::default());
    let cloned = original.get().unwrap().clone();
    assert_eq!(cloned.transient_base().ref_count(), 0);
    let copied = Handle::new(cloned);
    assert_ne!(original, copied);
    assert_eq!(original.ref_count(), 1);
    assert_eq!(copied.ref_count(), 1);
    assert!(std::ptr::eq(
        original.get().unwrap().dynamic_type(),
        copied.get().unwrap().dynamic_type()
    ));
}

#[test]
fn cloned_wire_bindings_have_independent_handles_and_preserve_runtime_class() {
    let original = Handle::new(WiresOnShape::new(&Shape::default()));
    let cloned = original.get().unwrap().clone();
    assert_eq!(cloned.transient_base().ref_count(), 0);
    let copied = Handle::new(cloned);
    assert_ne!(original, copied);
    assert_eq!(original.ref_count(), 1);
    assert_eq!(copied.ref_count(), 1);
    assert_eq!(
        copied.get().unwrap().dynamic_type().name(),
        "LocOpe_WiresOnShape"
    );
    assert!(copied.is_kind(StandardTransient::static_type()));
    assert!(copied.cast::<dyn GeneratedShape>().is_null());
}

#[test]
fn local_operation_runtime_objects_are_send_and_sync() {
    let _: Option<LocalOperationThreadProof<GluedShape>> = None;
    let _: Option<LocalOperationThreadProof<WiresOnShape>> = None;
    let _: Option<LocalOperationThreadProof<CSIntersector>> = None;
    let _: Option<LocalOperationThreadProof<BRepFeatBuilder>> = None;
    let _: Option<LocalOperationThreadProof<dyn GeneratedShape>> = None;
    let _: Option<LocalOperationThreadProof<Handle<dyn GeneratedShape>>> = None;
}

#[test]
fn abstract_generated_metadata_compiler_calls() {
    let value: &dyn GeneratedShape = &GluedShape::default();
    let name: &'static str = <dyn martensite_tkfeat::loc_ope::GeneratedShape as martensite_tkernel::standard::RuntimeTyped>::TYPE_NAME;
    let descriptor: &'static Handle<RuntimeType> = <dyn martensite_tkfeat::loc_ope::GeneratedShape as martensite_tkernel::standard::RuntimeTyped>::static_type_ref();
    let dynamic: &'static Handle<RuntimeType> = <dyn martensite_tkfeat::loc_ope::GeneratedShape as martensite_tkernel::standard::Transient>::dynamic_type_ref(value);
    assert_generated_descriptor_values(name, descriptor, dynamic, "LocOpe_GeneratedShape");
}

fn assert_generated_descriptor_values(
    name: &str,
    descriptor: &Handle<RuntimeType>,
    dynamic: &Handle<RuntimeType>,
    expected: &str,
) {
    assert_eq!(name, expected);
    let (declared, actual) = (descriptor.get().unwrap(), dynamic.get().unwrap());
    assert_eq!(declared.name(), expected);
    assert_eq!(declared.parent().unwrap().name(), "Standard_Transient");
    assert_eq!(actual.name(), "LocOpe_GluedShape");
    assert!(actual.is_subtype(declared));
    assert!(actual.is_subtype(StandardTransient::static_type()));
    assert!(!actual.is_subtype(WiresOnShape::static_type()));
    assert!(std::ptr::eq(actual, GluedShape::static_type()));
    println!(
        "MARTENSITE_TYPED_ASSERTION {{\"identity\":\"runtime_identity::abstract_generated_metadata_compiler_calls\",\"name\":\"{name}\",\"dynamic\":\"{}\"}}",
        actual.name()
    );
}

#[test]
fn glued_assignment_preserves_the_target_reference_counter() {
    let source = GluedShape::default();
    let mut target = GluedShape::default();
    target.transient_base().increment_ref_counter();
    target.transient_base().increment_ref_counter();
    target.clone_from(&source);
    assert_eq!(target.transient_base().ref_count(), 2);
    assert_eq!(source.transient_base().ref_count(), 0);
    assert_eq!(target.transient_base().decrement_ref_counter(), 1);
    assert_eq!(target.transient_base().decrement_ref_counter(), 0);
}

#[test]
fn wire_assignment_preserves_the_target_reference_counter() {
    let source = WiresOnShape::new(&Shape::default());
    let mut target = WiresOnShape::new(&Shape::default());
    target.transient_base().increment_ref_counter();
    target.transient_base().increment_ref_counter();
    target.clone_from(&source);
    assert_eq!(target.transient_base().ref_count(), 2);
    assert_eq!(source.transient_base().ref_count(), 0);
    assert_eq!(target.transient_base().decrement_ref_counter(), 1);
    assert_eq!(target.transient_base().decrement_ref_counter(), 0);
}
