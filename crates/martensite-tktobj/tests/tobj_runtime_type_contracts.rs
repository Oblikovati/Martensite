//! Exact native class descriptor identity, immediate parents and inherited partition filtering.
use crate::tobj_model::ObjectFixture;
use martensite_tkernel::standard::{
    Handle, RuntimeType, RuntimeTyped, StandardTransient, Transient,
};
use martensite_tktobj::tobj::{
    TObjApplication, TObjBackReferences, TObjHiddenPartition, TObjObject, TObjObjectIterator,
    TObjPartition, TObjSequenceIterator, TObjStandardApplication, TObjStandardObjectIterator,
    TObjStandardPartition,
};
fn assert_native_parent(descriptor: &RuntimeType, name: &str, parent: &str) {
    assert_eq!(descriptor.name(), name);
    assert_eq!(descriptor.parent().unwrap().name(), parent);
    assert_ne!(descriptor.name(), descriptor.parent().unwrap().name());
}
#[test]
fn concrete_application_descriptor_has_native_immediate_base_and_own_class_identity() {
    let application = TObjStandardApplication::get_instance().unwrap();
    let descriptor = TObjStandardApplication::static_type();
    assert_native_parent(descriptor, "TObj_Application", "TDocStd_Application");
    assert!(std::ptr::eq(
        descriptor,
        <dyn TObjApplication>::static_type()
    ));
    assert!(application.is_instance(descriptor));
    assert!(application.is_kind_handle(TObjStandardApplication::static_type_ref()));
    assert!(
        application
            .cast::<dyn TObjApplication>()
            .cast::<TObjStandardApplication>()
            == application
    );
}
#[test]
fn concrete_partition_descriptor_has_native_object_parent_and_own_class_identity() {
    let fixture = ObjectFixture::set_up();
    let partition = fixture.my_partition.cast::<TObjStandardPartition>();
    assert!(!partition.is_null());
    let descriptor = TObjStandardPartition::static_type();
    assert_native_parent(descriptor, "TObj_Partition", "TObj_Object");
    assert!(std::ptr::eq(descriptor, <dyn TObjPartition>::static_type()));
    assert!(partition.is_instance(descriptor));
    assert!(partition.is_kind(<dyn TObjObject>::static_type()));
    assert!(
        partition
            .cast::<dyn TObjPartition>()
            .cast::<TObjStandardPartition>()
            == partition
    );
}
#[test]
fn concrete_empty_iterator_descriptor_has_native_transient_parent_and_identity() {
    let iterator = Handle::new(TObjStandardObjectIterator::default());
    let descriptor = TObjStandardObjectIterator::static_type();
    assert_native_parent(descriptor, "TObj_ObjectIterator", "Standard_Transient");
    assert!(std::ptr::eq(
        descriptor,
        <dyn TObjObjectIterator>::static_type()
    ));
    assert!(iterator.is_instance_handle(TObjStandardObjectIterator::static_type_ref()));
    assert!(iterator.is_kind(StandardTransient::static_type()));
    assert!(
        iterator
            .cast::<dyn TObjObjectIterator>()
            .cast::<TObjStandardObjectIterator>()
            == iterator
    );
}
#[test]
fn native_partition_type_filter_includes_the_hidden_partition_subclass() {
    let fixture = ObjectFixture::set_up();
    let hidden = fixture
        .command(|| TObjHiddenPartition::new(&fixture.my_partition.new_label().unwrap()).unwrap());
    let sequence = Handle::new(TObjBackReferences::default());
    sequence.append(fixture.my_partition.cast());
    sequence.append(hidden.cast());
    let iterator = TObjSequenceIterator::new(&sequence, Some(TObjStandardPartition::static_type()));
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
    assert_native_parent(descriptor, name, parent);
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
fn model_object_and_application_class_descriptors_preserve_native_names_and_parents() {
    use martensite_tktobj::tobj::TObjModel;
    assert_class_descriptor::<dyn TObjModel>("TObj_Model", "Standard_Transient");
    assert_class_descriptor::<dyn TObjObject>("TObj_Object", "Standard_Transient");
    assert_class_descriptor::<dyn TObjApplication>("TObj_Application", "TDocStd_Application");
}
#[test]
fn partitions_and_model_checker_preserve_native_class_descriptors() {
    use martensite_tktobj::tobj::TObjCheckModel;
    assert_class_descriptor::<dyn TObjPartition>("TObj_Partition", "TObj_Object");
    assert_class_descriptor::<TObjHiddenPartition>("TObj_HiddenPartition", "TObj_Partition");
    assert_class_descriptor::<TObjCheckModel>("TObj_CheckModel", "Message_Algorithm");
}
#[test]
fn abstract_and_model_iterators_preserve_native_class_descriptors() {
    use martensite_tktobj::tobj::{TObjLabelIterator, TObjModelIterator};
    assert_class_descriptor::<dyn TObjObjectIterator>("TObj_ObjectIterator", "Standard_Transient");
    assert_class_descriptor::<dyn TObjLabelIterator>("TObj_LabelIterator", "TObj_ObjectIterator");
    assert_class_descriptor::<TObjModelIterator>("TObj_ModelIterator", "TObj_ObjectIterator");
}
#[test]
fn concrete_filtered_iterators_preserve_native_class_descriptors() {
    use martensite_tktobj::tobj::{TObjOcafObjectIterator, TObjReferenceIterator};
    assert_class_descriptor::<TObjSequenceIterator>("TObj_SequenceIterator", "TObj_ObjectIterator");
    assert_class_descriptor::<TObjOcafObjectIterator>(
        "TObj_OcafObjectIterator",
        "TObj_LabelIterator",
    );
    assert_class_descriptor::<TObjReferenceIterator>(
        "TObj_ReferenceIterator",
        "TObj_LabelIterator",
    );
}
#[test]
fn object_model_and_reference_attributes_preserve_native_class_descriptors() {
    use martensite_tktobj::tobj::{TObjTModel, TObjTObject, TObjTReference};
    assert_class_descriptor::<TObjTObject>("TObj_TObject", "TDF_Attribute");
    assert_class_descriptor::<TObjTModel>("TObj_TModel", "TDF_Attribute");
    assert_class_descriptor::<TObjTReference>("TObj_TReference", "TDF_Attribute");
}
#[test]
fn coordinate_sparse_and_name_attributes_preserve_native_class_descriptors() {
    use martensite_tktobj::tobj::{TObjTIntSparseArray, TObjTNameContainer, TObjTXyz};
    assert_class_descriptor::<TObjTXyz>("TObj_TXYZ", "TDF_Attribute");
    assert_class_descriptor::<TObjTIntSparseArray>("TObj_TIntSparseArray", "TDF_Attribute");
    assert_class_descriptor::<TObjTNameContainer>("TObj_TNameContainer", "TDF_Attribute");
}
