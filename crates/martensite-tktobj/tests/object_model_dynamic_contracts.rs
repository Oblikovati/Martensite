//! Dynamic descriptor dispatch for every concrete object-model class family.
use crate::object_model_document::ObjectFixture;
use martensite_tkernel::standard::{Handle, RuntimeTyped, Transient};
use martensite_tktobj::object_model::{
    BackReferences, CoordinateAttribute, DocumentModel, HiddenPartition, IntegerSparseArray,
    LabelIterator, ModelAttribute, ModelIntegrityCheck, ModelIterator, ModelObject, NameIndex,
    ObjectAttribute, ObjectTreeIterator, ReferenceAttribute, ReferenceIterator,
    TypedSequenceIterator,
};

fn assert_dynamic_class<T: RuntimeTyped + Transient>(object: &T, name: &str) {
    let erased: &dyn Transient = object;
    assert_eq!(erased.dynamic_type().name(), name);
    assert!(std::ptr::eq(erased.dynamic_type(), T::static_type()));
    assert!(std::ptr::eq(
        erased.dynamic_type_ref(),
        T::static_type_ref()
    ));
    assert!(std::ptr::eq(
        erased.dynamic_type_handle().get().unwrap(),
        T::static_type()
    ));
}

#[test]
fn attribute_dynamic_dispatch_preserves_each_concrete_class() {
    assert_dynamic_class(
        &ObjectAttribute::new(),
        "martensite_object_model_ObjectAttribute",
    );
    assert_dynamic_class(
        &ModelAttribute::new(),
        "martensite_object_model_ModelAttribute",
    );
    assert_dynamic_class(
        &ReferenceAttribute::default(),
        "martensite_object_model_ReferenceAttribute",
    );
    assert_dynamic_class(
        &CoordinateAttribute::new(),
        "martensite_object_model_CoordinateAttribute",
    );
    assert_dynamic_class(&NameIndex::new(), "martensite_object_model_NameIndex");
    assert_dynamic_class(
        &IntegerSparseArray::new(),
        "martensite_object_model_IntegerSparseArray",
    );
}

#[test]
fn sequence_iterator_dynamic_dispatch_retains_its_own_descriptor() {
    let objects = Handle::new(BackReferences::default());
    let iterator = TypedSequenceIterator::new(&objects, None);
    assert_dynamic_class(&iterator, "martensite_object_model_TypedSequenceIterator");
}

#[test]
fn model_checker_and_iterator_dispatch_retain_distinct_descriptors() {
    let fixture = ObjectFixture::set_up();
    let model: Handle<dyn DocumentModel> = fixture.my_model().cast();
    let checker = ModelIntegrityCheck::new(&model);
    let iterator = ModelIterator::new(&model).unwrap();
    assert_dynamic_class(&checker, "martensite_object_model_ModelIntegrityCheck");
    assert_dynamic_class(
        iterator.get().unwrap(),
        "martensite_object_model_ModelIterator",
    );
}

#[test]
fn label_iterator_subclasses_keep_their_concrete_dynamic_descriptors() {
    let fixture = ObjectFixture::set_up();
    let label = fixture.my_partition.new_label().unwrap();
    let objects = ObjectTreeIterator::new(&label, None, false, false).unwrap();
    let references = ReferenceIterator::new(&label, None, false).unwrap();
    assert_dynamic_class(
        objects.get().unwrap(),
        "martensite_object_model_ObjectTreeIterator",
    );
    assert_dynamic_class(
        references.get().unwrap(),
        "martensite_object_model_ReferenceIterator",
    );
}

#[test]
fn hidden_partition_dynamic_dispatch_does_not_collapse_to_its_parent() {
    let fixture = ObjectFixture::set_up();
    let hidden = fixture
        .command(|| HiddenPartition::new(&fixture.my_partition.new_label().unwrap()).unwrap());
    assert_dynamic_class(
        hidden.get().unwrap(),
        "martensite_object_model_HiddenPartition",
    );
}

fn assert_erased_dispatch(value: &dyn Transient, concrete: &dyn Transient, name: &str) {
    assert_eq!(value.dynamic_type().name(), name);
    assert!(std::ptr::eq(value.dynamic_type(), concrete.dynamic_type()));
    assert!(std::ptr::eq(
        value.dynamic_type_ref(),
        concrete.dynamic_type_ref()
    ));
}

#[test]
fn abstract_model_interface_dispatches_the_external_model_descriptor() {
    let fixture = ObjectFixture::set_up();
    let concrete = fixture.my_model();
    let model: Handle<dyn DocumentModel> = concrete.cast();
    assert_erased_dispatch(
        model.get().unwrap(),
        concrete.get().unwrap(),
        "TestDocumentModel",
    );
}

#[test]
fn abstract_object_interface_dispatches_the_external_object_descriptor() {
    let fixture = ObjectFixture::set_up();
    let concrete = fixture.create_object();
    let object: Handle<dyn ModelObject> = concrete.cast();
    assert_erased_dispatch(
        object.get().unwrap(),
        concrete.get().unwrap(),
        "TestModelObject",
    );
}

#[test]
fn abstract_label_iterator_dispatches_the_concrete_iterator_descriptor() {
    let fixture = ObjectFixture::set_up();
    let label = fixture.my_partition.new_label().unwrap();
    let concrete = ObjectTreeIterator::new(&label, None, false, false).unwrap();
    let iterator: Handle<dyn LabelIterator> = concrete.cast();
    assert_erased_dispatch(
        iterator.get().unwrap(),
        concrete.get().unwrap(),
        "martensite_object_model_ObjectTreeIterator",
    );
}
