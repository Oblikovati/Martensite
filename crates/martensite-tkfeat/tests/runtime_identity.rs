//! Runtime descriptors preserve actual ancestry, casts and independent handle counters.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{
    Handle, RuntimeType, RuntimeTyped, StandardTransient, Transient,
};
use std::sync::Arc;
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
    // Descriptor handles became process-lived borrows; each call returns the same singleton.
    let generated = <dyn GeneratedShape>::static_type();
    let glued = GluedShape::static_type();
    let wires = WiresOnShape::static_type();
    assert!(std::ptr::eq(generated, <dyn GeneratedShape>::static_type()));
    assert!(std::ptr::eq(glued, GluedShape::static_type()));
    assert!(std::ptr::eq(wires, WiresOnShape::static_type()));
}

#[test]
fn dynamic_descriptor_handles_borrow_the_exact_class_singletons() {
    let glued = GluedShape::default();
    let generated: &dyn GeneratedShape = &glued;
    let wires = WiresOnShape::new(&Shape::default());
    assert!(std::ptr::eq(
        generated.dynamic_type(),
        GluedShape::static_type()
    ));
    assert!(std::ptr::eq(
        wires.dynamic_type(),
        WiresOnShape::static_type()
    ));
}

#[test]
fn generated_shape_handle_preserves_most_derived_type_and_downcast_identity() {
    let glued = Handle::new(GluedShape::default());
    let generated = glued.upcast::<dyn GeneratedShape>();
    let root = generated.upcast::<dyn Transient>();
    assert_eq!(
        generated.get().unwrap().dynamic_type().name(),
        "LocOpe_GluedShape"
    );
    assert!(generated.is_kind(<dyn GeneratedShape>::static_type()));
    assert!(generated.is_kind(StandardTransient::static_type()));
    assert!(!generated.is_kind(WiresOnShape::static_type()));
    assert_eq!(generated.downcast::<GluedShape>(), glued);
    assert!(generated.downcast::<WiresOnShape>().is_null());
    assert_eq!(root.downcast::<GluedShape>(), glued);
    assert_eq!(glued.ref_count(), 3);
}

#[test]
fn cloned_glued_shape_starts_with_an_independent_zero_reference_counter() {
    let original = Handle::new(GluedShape::default());
    let cloned = original.get().unwrap().clone();
    // The count lives in the handle now: the clone is a plain value that no owner counts.
    assert_eq!(original.ref_count(), 1);
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
    // The count lives in the handle now: the clone is a plain value that no owner counts.
    assert_eq!(original.ref_count(), 1);
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
    let descriptor: &'static RuntimeType = <dyn martensite_tkfeat::loc_ope::GeneratedShape as martensite_tkernel::standard::RuntimeTyped>::static_type();
    let dynamic: &'static RuntimeType = <dyn martensite_tkfeat::loc_ope::GeneratedShape as martensite_tkernel::standard::Transient>::dynamic_type(value);
    assert_generated_descriptor_values(name, descriptor, dynamic, "LocOpe_GeneratedShape");
}

fn assert_generated_descriptor_values(
    name: &str,
    descriptor: &RuntimeType,
    dynamic: &RuntimeType,
    expected: &str,
) {
    assert_eq!(name, expected);
    let (declared, actual) = (descriptor, dynamic);
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
    let source = Handle::new(GluedShape::default());
    let source_alias = source.clone();
    let mut target = Arc::new(GluedShape::default());
    // The owner keeps the count beside the object: assignment through it cannot reach it.
    Arc::get_mut(&mut target)
        .unwrap()
        .clone_from(source.get().unwrap());
    let target = Handle::from_arc(target);
    assert_eq!(target.ref_count(), 1);
    assert_eq!(source.ref_count(), 2);
    drop(source_alias);
    assert_eq!(source.ref_count(), 1);
}

#[test]
fn wire_assignment_preserves_the_target_reference_counter() {
    let source = Handle::new(WiresOnShape::new(&Shape::default()));
    let source_alias = source.clone();
    let mut target = Arc::new(WiresOnShape::new(&Shape::default()));
    // The owner keeps the count beside the object: assignment through it cannot reach it.
    Arc::get_mut(&mut target)
        .unwrap()
        .clone_from(source.get().unwrap());
    let target = Handle::from_arc(target);
    assert_eq!(target.ref_count(), 1);
    assert_eq!(source.ref_count(), 2);
    drop(source_alias);
    assert_eq!(source.ref_count(), 1);
}
