//! Exact runtime class descriptor identity, immediate parents and inherited partition filtering.
use crate::object_model_document::ObjectFixture;
use martensite_tkernel::standard::{
    Handle, RuntimeType, RuntimeTyped, StandardTransient, Transient,
};
use martensite_tktobj::object_model::{
    BackReferences, DocumentApplication, DocumentApplicationApi, HiddenPartition, ModelObject,
    ModelPartition, ObjectIterator, StandardObjectIterator, StandardPartition,
    TypedSequenceIterator,
};
fn assert_runtime_parent(descriptor: &RuntimeType, name: &str, parent: &str) {
    assert_eq!(descriptor.name(), name);
    assert_eq!(descriptor.parent().unwrap().name(), parent);
    assert_ne!(descriptor.name(), descriptor.parent().unwrap().name());
}
#[test]
fn concrete_application_descriptor_has_runtime_immediate_base_and_own_class_identity() {
    let application = DocumentApplication::get_instance().unwrap();
    let descriptor = DocumentApplication::static_type();
    assert_runtime_parent(
        descriptor,
        "martensite_object_model_DocumentApplication",
        "TDocStd_Application",
    );
    assert!(std::ptr::eq(
        descriptor,
        <dyn DocumentApplicationApi>::static_type()
    ));
    assert!(application.is_instance(descriptor));
    assert!(application.is_kind_handle(DocumentApplication::static_type_ref()));
    assert!(
        application
            .cast::<dyn DocumentApplicationApi>()
            .cast::<DocumentApplication>()
            == application
    );
}
#[test]
fn concrete_partition_descriptor_has_runtime_object_parent_and_own_class_identity() {
    let fixture = ObjectFixture::set_up();
    let partition = fixture.my_partition.cast::<StandardPartition>();
    assert!(!partition.is_null());
    let descriptor = StandardPartition::static_type();
    assert_runtime_parent(
        descriptor,
        "martensite_object_model_ModelPartition",
        "martensite_object_model_ModelObject",
    );
    assert!(std::ptr::eq(
        descriptor,
        <dyn ModelPartition>::static_type()
    ));
    assert!(partition.is_instance(descriptor));
    assert!(partition.is_kind(<dyn ModelObject>::static_type()));
    assert!(
        partition
            .cast::<dyn ModelPartition>()
            .cast::<StandardPartition>()
            == partition
    );
}
#[test]
fn concrete_empty_iterator_descriptor_has_runtime_transient_parent_and_identity() {
    let iterator = Handle::new(StandardObjectIterator::default());
    let descriptor = StandardObjectIterator::static_type();
    assert_runtime_parent(
        descriptor,
        "martensite_object_model_ObjectIterator",
        "Standard_Transient",
    );
    assert!(std::ptr::eq(
        descriptor,
        <dyn ObjectIterator>::static_type()
    ));
    assert!(iterator.is_instance_handle(StandardObjectIterator::static_type_ref()));
    assert!(iterator.is_kind(StandardTransient::static_type()));
    assert!(
        iterator
            .cast::<dyn ObjectIterator>()
            .cast::<StandardObjectIterator>()
            == iterator
    );
}
#[test]
fn runtime_partition_type_filter_includes_the_hidden_partition_subclass() {
    let fixture = ObjectFixture::set_up();
    let hidden = fixture
        .command(|| HiddenPartition::new(&fixture.my_partition.new_label().unwrap()).unwrap());
    let sequence = Handle::new(BackReferences::default());
    sequence.append(fixture.my_partition.cast());
    sequence.append(hidden.cast());
    let iterator = TypedSequenceIterator::new(&sequence, Some(StandardPartition::static_type()));
    assert!(iterator.more());
    assert!(iterator.value() == fixture.my_partition);
    iterator.next().unwrap();
    assert!(iterator.more());
    assert!(iterator.value() == hidden);
    iterator.next().unwrap();
    assert!(!iterator.more());
}

fn assert_class_descriptor<T: RuntimeTyped + ?Sized>(name: &str, parent: &str) {
    let descriptor = T::static_type();
    assert_runtime_parent(descriptor, name, parent);
    assert_eq!(T::TYPE_NAME, name);
    let slot = T::static_type_ref();
    assert!(std::ptr::eq(slot, T::static_type_ref()));
    assert!(std::ptr::eq(slot.get().unwrap(), descriptor));
    assert!(std::ptr::eq(
        T::static_type_handle().get().unwrap(),
        descriptor
    ));
    assert!(std::ptr::eq(descriptor.class_handle_ref(), slot));
}
#[test]
fn model_object_and_application_class_descriptors_preserve_runtime_names_and_parents() {
    use martensite_tktobj::object_model::DocumentModel;
    assert_class_descriptor::<dyn DocumentModel>(
        "martensite_object_model_DocumentModel",
        "Standard_Transient",
    );
    assert_class_descriptor::<dyn ModelObject>(
        "martensite_object_model_ModelObject",
        "Standard_Transient",
    );
    assert_class_descriptor::<dyn DocumentApplicationApi>(
        "martensite_object_model_DocumentApplication",
        "TDocStd_Application",
    );
}
#[test]
fn partitions_and_model_checker_preserve_runtime_class_descriptors() {
    use martensite_tktobj::object_model::ModelIntegrityCheck;
    assert_class_descriptor::<dyn ModelPartition>(
        "martensite_object_model_ModelPartition",
        "martensite_object_model_ModelObject",
    );
    assert_class_descriptor::<HiddenPartition>(
        "martensite_object_model_HiddenPartition",
        "martensite_object_model_ModelPartition",
    );
    assert_class_descriptor::<ModelIntegrityCheck>(
        "martensite_object_model_ModelIntegrityCheck",
        "Message_Algorithm",
    );
}
#[test]
fn abstract_and_model_iterators_preserve_runtime_class_descriptors() {
    use martensite_tktobj::object_model::{LabelIterator, ModelIterator};
    assert_class_descriptor::<dyn ObjectIterator>(
        "martensite_object_model_ObjectIterator",
        "Standard_Transient",
    );
    assert_class_descriptor::<dyn LabelIterator>(
        "martensite_object_model_LabelIterator",
        "martensite_object_model_ObjectIterator",
    );
    assert_class_descriptor::<ModelIterator>(
        "martensite_object_model_ModelIterator",
        "martensite_object_model_ObjectIterator",
    );
}
#[test]
fn concrete_filtered_iterators_preserve_runtime_class_descriptors() {
    use martensite_tktobj::object_model::{ObjectTreeIterator, ReferenceIterator};
    assert_class_descriptor::<TypedSequenceIterator>(
        "martensite_object_model_TypedSequenceIterator",
        "martensite_object_model_ObjectIterator",
    );
    assert_class_descriptor::<ObjectTreeIterator>(
        "martensite_object_model_ObjectTreeIterator",
        "martensite_object_model_LabelIterator",
    );
    assert_class_descriptor::<ReferenceIterator>(
        "martensite_object_model_ReferenceIterator",
        "martensite_object_model_LabelIterator",
    );
}
#[test]
fn object_model_and_reference_attributes_preserve_runtime_class_descriptors() {
    use martensite_tktobj::object_model::{ModelAttribute, ObjectAttribute, ReferenceAttribute};
    assert_class_descriptor::<ObjectAttribute>(
        "martensite_object_model_ObjectAttribute",
        "TDF_Attribute",
    );
    assert_class_descriptor::<ModelAttribute>(
        "martensite_object_model_ModelAttribute",
        "TDF_Attribute",
    );
    assert_class_descriptor::<ReferenceAttribute>(
        "martensite_object_model_ReferenceAttribute",
        "TDF_Attribute",
    );
}
#[test]
fn coordinate_sparse_and_name_attributes_preserve_runtime_class_descriptors() {
    use martensite_tktobj::object_model::{CoordinateAttribute, IntegerSparseArray, NameIndex};
    assert_class_descriptor::<CoordinateAttribute>(
        "martensite_object_model_CoordinateAttribute",
        "TDF_Attribute",
    );
    assert_class_descriptor::<IntegerSparseArray>(
        "martensite_object_model_IntegerSparseArray",
        "TDF_Attribute",
    );
    assert_class_descriptor::<NameIndex>("martensite_object_model_NameIndex", "TDF_Attribute");
}
