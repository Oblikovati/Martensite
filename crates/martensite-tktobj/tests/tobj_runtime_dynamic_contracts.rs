//! Dynamic descriptor dispatch for every concrete object-model class family.
use crate::tobj_model::ObjectFixture;
use martensite_tkernel::standard::{Handle, RuntimeTyped, Transient};
use martensite_tktobj::tobj::{
    TObjBackReferences, TObjCheckModel, TObjHiddenPartition, TObjLabelIterator, TObjModel,
    TObjModelIterator, TObjObject, TObjOcafObjectIterator, TObjReferenceIterator,
    TObjSequenceIterator, TObjTIntSparseArray, TObjTModel, TObjTNameContainer, TObjTObject,
    TObjTReference, TObjTXyz,
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
    assert_dynamic_class(&TObjTObject::new(), "TObj_TObject");
    assert_dynamic_class(&TObjTModel::new(), "TObj_TModel");
    assert_dynamic_class(&TObjTReference::default(), "TObj_TReference");
    assert_dynamic_class(&TObjTXyz::new(), "TObj_TXYZ");
    assert_dynamic_class(&TObjTNameContainer::new(), "TObj_TNameContainer");
    assert_dynamic_class(&TObjTIntSparseArray::new(), "TObj_TIntSparseArray");
}

#[test]
fn sequence_iterator_dynamic_dispatch_retains_its_own_descriptor() {
    let objects = Handle::new(TObjBackReferences::default());
    let iterator = TObjSequenceIterator::new(&objects, None);
    assert_dynamic_class(&iterator, "TObj_SequenceIterator");
}

#[test]
fn model_checker_and_iterator_dispatch_retain_distinct_descriptors() {
    let fixture = ObjectFixture::set_up();
    let model: Handle<dyn TObjModel> = fixture.my_model().cast();
    let checker = TObjCheckModel::new(&model);
    let iterator = TObjModelIterator::new(&model).unwrap();
    assert_dynamic_class(&checker, "TObj_CheckModel");
    assert_dynamic_class(iterator.get().unwrap(), "TObj_ModelIterator");
}

#[test]
fn label_iterator_subclasses_keep_their_concrete_dynamic_descriptors() {
    let fixture = ObjectFixture::set_up();
    let label = fixture.my_partition.new_label().unwrap();
    let objects = TObjOcafObjectIterator::new(&label, None, false, false).unwrap();
    let references = TObjReferenceIterator::new(&label, None, false).unwrap();
    assert_dynamic_class(objects.get().unwrap(), "TObj_OcafObjectIterator");
    assert_dynamic_class(references.get().unwrap(), "TObj_ReferenceIterator");
}

#[test]
fn hidden_partition_dynamic_dispatch_does_not_collapse_to_its_parent() {
    let fixture = ObjectFixture::set_up();
    let hidden = fixture
        .command(|| TObjHiddenPartition::new(&fixture.my_partition.new_label().unwrap()).unwrap());
    assert_dynamic_class(hidden.get().unwrap(), "TObj_HiddenPartition");
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
    let model: Handle<dyn TObjModel> = concrete.cast();
    assert_erased_dispatch(
        model.get().unwrap(),
        concrete.get().unwrap(),
        "TObj_TestModel",
    );
}

#[test]
fn abstract_object_interface_dispatches_the_external_object_descriptor() {
    let fixture = ObjectFixture::set_up();
    let concrete = fixture.create_object();
    let object: Handle<dyn TObjObject> = concrete.cast();
    assert_erased_dispatch(
        object.get().unwrap(),
        concrete.get().unwrap(),
        "TObj_TestObject",
    );
}

#[test]
fn abstract_label_iterator_dispatches_the_concrete_iterator_descriptor() {
    let fixture = ObjectFixture::set_up();
    let label = fixture.my_partition.new_label().unwrap();
    let concrete = TObjOcafObjectIterator::new(&label, None, false, false).unwrap();
    let iterator: Handle<dyn TObjLabelIterator> = concrete.cast();
    assert_erased_dispatch(
        iterator.get().unwrap(),
        concrete.get().unwrap(),
        "TObj_OcafObjectIterator",
    );
}
