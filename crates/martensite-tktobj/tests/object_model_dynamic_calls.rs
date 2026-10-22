//! Direct, explicitly typed expressions omitted by pinned rustdoc for dyn Self.
use crate::object_model_document::ObjectFixture;
use martensite_tkernel::standard::{Handle, RuntimeType};
use martensite_tktobj::object_model::{
    DocumentApplication, DocumentApplicationApi, DocumentModel, LabelIterator, ModelObject,
    ModelPartition, ObjectIterator, ObjectTreeIterator, StandardObjectIterator,
};

fn assert_typed_descriptor_values(
    name: &'static str,
    descriptor: &'static Handle<RuntimeType>,
    dynamic: &'static Handle<RuntimeType>,
    expected: &str,
    expected_dynamic: &str,
) {
    assert_eq!(name, expected);
    assert_eq!(descriptor.get().unwrap().name(), expected);
    assert_eq!(dynamic.get().unwrap().name(), expected_dynamic);
    assert!(std::ptr::eq(
        descriptor.get().unwrap().class_handle_ref(),
        descriptor
    ));
    assert!(std::ptr::eq(
        dynamic.get().unwrap().class_handle_ref(),
        dynamic
    ));
    emit_typed_assertion_marker(name, expected_dynamic);
}

fn emit_typed_assertion_marker(name: &str, dynamic: &str) {
    let current = std::thread::current();
    let identity = current.name().unwrap();
    println!(
        "MARTENSITE_TYPED_ASSERTION {{\"identity\":\"{identity}\",\"name\":\"{name}\",\"dynamic\":\"{dynamic}\"}}"
    );
}

#[test]
fn model_dyn_self_calls_use_the_model_interface_and_external_dynamic_class() {
    let fixture = ObjectFixture::set_up();
    let value: &dyn DocumentModel = fixture.my_model().get().unwrap();
    let name: &'static str = <dyn martensite_tktobj::object_model::DocumentModel as martensite_tkernel::standard::RuntimeTyped>::TYPE_NAME;
    let descriptor: &'static Handle<RuntimeType> =
        <dyn martensite_tktobj::object_model::DocumentModel as martensite_tkernel::standard::RuntimeTyped>::static_type_ref();
    let dynamic: &'static Handle<RuntimeType> =
        <dyn martensite_tktobj::object_model::DocumentModel as martensite_tkernel::standard::Transient>::dynamic_type_ref(value);
    assert_typed_descriptor_values(
        name,
        descriptor,
        dynamic,
        "martensite_object_model_DocumentModel",
        "TestDocumentModel",
    );
}

#[test]
fn object_dyn_self_calls_use_the_object_interface_and_external_dynamic_class() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.create_object();
    let value: &dyn ModelObject = object.get().unwrap();
    let name: &'static str = <dyn martensite_tktobj::object_model::ModelObject as martensite_tkernel::standard::RuntimeTyped>::TYPE_NAME;
    let descriptor: &'static Handle<RuntimeType> =
        <dyn martensite_tktobj::object_model::ModelObject as martensite_tkernel::standard::RuntimeTyped>::static_type_ref();
    let dynamic: &'static Handle<RuntimeType> =
        <dyn martensite_tktobj::object_model::ModelObject as martensite_tkernel::standard::Transient>::dynamic_type_ref(value);
    assert_typed_descriptor_values(
        name,
        descriptor,
        dynamic,
        "martensite_object_model_ModelObject",
        "TestModelObject",
    );
}

#[test]
fn application_dyn_self_calls_keep_the_canonical_application_descriptor() {
    let application = DocumentApplication::get_instance().unwrap();
    let value: &dyn DocumentApplicationApi = application.get().unwrap();
    let name: &'static str = <dyn martensite_tktobj::object_model::DocumentApplicationApi as martensite_tkernel::standard::RuntimeTyped>::TYPE_NAME;
    let descriptor: &'static Handle<RuntimeType> =
        <dyn martensite_tktobj::object_model::DocumentApplicationApi as martensite_tkernel::standard::RuntimeTyped>::static_type_ref();
    let dynamic: &'static Handle<RuntimeType> =
        <dyn martensite_tktobj::object_model::DocumentApplicationApi as martensite_tkernel::standard::Transient>::dynamic_type_ref(value);
    assert_typed_descriptor_values(
        name,
        descriptor,
        dynamic,
        "martensite_object_model_DocumentApplication",
        "martensite_object_model_DocumentApplication",
    );
}

#[test]
fn partition_dyn_self_calls_keep_the_canonical_partition_descriptor() {
    let fixture = ObjectFixture::set_up();
    let value: &dyn ModelPartition = fixture.my_partition.get().unwrap();
    let name: &'static str = <dyn martensite_tktobj::object_model::ModelPartition as martensite_tkernel::standard::RuntimeTyped>::TYPE_NAME;
    let descriptor: &'static Handle<RuntimeType> =
        <dyn martensite_tktobj::object_model::ModelPartition as martensite_tkernel::standard::RuntimeTyped>::static_type_ref();
    let dynamic: &'static Handle<RuntimeType> =
        <dyn martensite_tktobj::object_model::ModelPartition as martensite_tkernel::standard::Transient>::dynamic_type_ref(value);
    assert_typed_descriptor_values(
        name,
        descriptor,
        dynamic,
        "martensite_object_model_ModelPartition",
        "martensite_object_model_ModelPartition",
    );
}

#[test]
fn object_iterator_dyn_self_calls_keep_the_canonical_iterator_descriptor() {
    let iterator = StandardObjectIterator::default();
    let value: &dyn ObjectIterator = &iterator;
    let name: &'static str = <dyn martensite_tktobj::object_model::ObjectIterator as martensite_tkernel::standard::RuntimeTyped>::TYPE_NAME;
    let descriptor: &'static Handle<RuntimeType> =
        <dyn martensite_tktobj::object_model::ObjectIterator as martensite_tkernel::standard::RuntimeTyped>::static_type_ref();
    let dynamic: &'static Handle<RuntimeType> =
        <dyn martensite_tktobj::object_model::ObjectIterator as martensite_tkernel::standard::Transient>::dynamic_type_ref(value);
    assert_typed_descriptor_values(
        name,
        descriptor,
        dynamic,
        "martensite_object_model_ObjectIterator",
        "martensite_object_model_ObjectIterator",
    );
}

#[test]
fn label_iterator_dyn_self_calls_keep_the_actual_concrete_iterator_descriptor() {
    let fixture = ObjectFixture::set_up();
    let label = fixture.my_partition.new_label().unwrap();
    let iterator = ObjectTreeIterator::new(&label, None, false, false).unwrap();
    let value: &dyn LabelIterator = iterator.get().unwrap();
    let name: &'static str = <dyn martensite_tktobj::object_model::LabelIterator as martensite_tkernel::standard::RuntimeTyped>::TYPE_NAME;
    let descriptor: &'static Handle<RuntimeType> =
        <dyn martensite_tktobj::object_model::LabelIterator as martensite_tkernel::standard::RuntimeTyped>::static_type_ref();
    let dynamic: &'static Handle<RuntimeType> =
        <dyn martensite_tktobj::object_model::LabelIterator as martensite_tkernel::standard::Transient>::dynamic_type_ref(value);
    assert_typed_descriptor_values(
        name,
        descriptor,
        dynamic,
        "martensite_object_model_LabelIterator",
        "martensite_object_model_ObjectTreeIterator",
    );
}
