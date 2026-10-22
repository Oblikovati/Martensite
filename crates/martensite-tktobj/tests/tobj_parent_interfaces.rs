//! Declared parent aliases retain inherited dispatch and allocation identity.
use crate::tobj_model::ObjectFixture;
use martensite_tkernel::standard::{Handle, RuntimeTyped, Transient};
use martensite_tklcaf::tdf::{TdfAttribute, TdfAttributeState};
use martensite_tktobj::tobj::*;

fn parent_roundtrip<T: RuntimeTyped + ?Sized, B: Transient + RuntimeTyped + ?Sized>(
    source: &Handle<T>,
    parent_name: &str,
) -> Handle<B> {
    let base = source.cast::<B>();
    assert!(!base.is_null());
    assert_eq!(B::static_type().name(), parent_name);
    assert!(std::ptr::eq(
        T::static_type().parent().unwrap(),
        B::static_type()
    ));
    assert!(base.cast::<T>() == *source);
    assert!(std::ptr::eq(
        base.get().unwrap().dynamic_type(),
        T::static_type()
    ));
    base
}
#[test]
fn application_parent_alias_preserves_resources_virtual_dispatch() {
    let application = TObjStandardApplication::get_instance().unwrap();
    let base = parent_roundtrip::<_, TObjApplicationBaseType>(&application, "TDocStd_Application");
    let borrowed: &TObjApplicationBaseType = application.get().unwrap();
    assert_eq!(borrowed.resources_name(), "TObj");
    assert_eq!(base.resources_name(), borrowed.resources_name());
}
#[test]
fn partition_parent_aliases_preserve_object_and_partition_dispatch() {
    let fixture = ObjectFixture::set_up();
    let partition = fixture.my_partition.cast::<TObjStandardPartition>();
    let base = parent_roundtrip::<_, TObjPartitionBaseType>(&partition, "TObj_Object");
    let borrowed: &TObjPartitionBaseType = partition.get().unwrap();
    assert_eq!(base.get_label(), borrowed.get_label());
    let hidden =
        fixture.command(|| TObjHiddenPartition::new(&partition.new_label().unwrap()).unwrap());
    let base = parent_roundtrip::<_, TObjHiddenPartitionBaseType>(&hidden, "TObj_Partition");
    let borrowed: &TObjHiddenPartitionBaseType = hidden.get().unwrap();
    assert_eq!(base.get_label(), borrowed.get_label());
    assert_eq!(base.get_label(), hidden.get_label());
}
#[test]
fn sequence_parent_alias_exposes_live_inherited_iterator_state() {
    let fixture = ObjectFixture::set_up();
    let objects = Handle::new(TObjBackReferences::default());
    objects.append(fixture.my_partition.cast());
    let iterator = Handle::new(TObjSequenceIterator::new(&objects, None));
    let base =
        parent_roundtrip::<_, TObjSequenceIteratorBaseType>(&iterator, "TObj_ObjectIterator");
    let borrowed: &TObjSequenceIteratorBaseType = iterator.get().unwrap();
    assert!(borrowed.more());
    assert!(base.checked_value().unwrap() == fixture.my_partition);
    base.next().unwrap();
    assert!(!borrowed.more());
}
#[test]
fn model_and_label_parent_aliases_share_virtual_iteration() {
    let fixture = ObjectFixture::set_up();
    let iterator = TObjModelIterator::new(&fixture.my_model().clone().cast()).unwrap();
    let base = parent_roundtrip::<_, TObjModelIteratorBaseType>(&iterator, "TObj_ObjectIterator");
    let borrowed: &TObjModelIteratorBaseType = iterator.get().unwrap();
    assert!(borrowed.value() == fixture.my_partition && base.more());
    let object = fixture.command(|| fixture.create_object());
    check_label_parent_alias(&fixture, &object);
}
fn check_label_parent_alias(
    fixture: &ObjectFixture,
    object: &Handle<crate::tobj_model::TObjTestObject>,
) {
    let label = fixture.my_partition.get_child_label().unwrap();
    let iterator = TObjOcafObjectIterator::new(&label, None, true, false).unwrap();
    let label_base =
        parent_roundtrip::<_, TObjOcafObjectIteratorBaseType>(&iterator, "TObj_LabelIterator");
    let inherited: Handle<TObjLabelIteratorBaseType> = label_base.cast();
    assert!(inherited.cast::<TObjOcafObjectIterator>() == iterator);
    let borrowed: &TObjLabelIteratorBaseType = label_base.get().unwrap();
    assert_eq!(
        <TObjLabelIteratorBaseType>::static_type().name(),
        "TObj_ObjectIterator"
    );
    assert!(borrowed.more() && borrowed.value() == *object);
    assert_eq!(label_base.label_value(), object.get_label());
    inherited.next().unwrap();
    assert!(!borrowed.more() && label_base.label_value().is_null());
}
#[test]
fn reference_parent_alias_preserves_inherited_label_and_value() {
    let fixture = ObjectFixture::set_up();
    let holder = fixture.command(|| {
        let holder = fixture.create_object();
        holder
            .set_reference(&fixture.my_partition.cast(), 1, 0)
            .unwrap();
        holder
    });
    let iterator =
        TObjReferenceIterator::new(&holder.get_reference_label().unwrap(), None, true).unwrap();
    let base =
        parent_roundtrip::<_, TObjReferenceIteratorBaseType>(&iterator, "TObj_LabelIterator");
    let borrowed: &TObjReferenceIteratorBaseType = iterator.get().unwrap();
    assert!(!borrowed.label_value().is_null());
    assert_eq!(base.label_value(), borrowed.label_value());
    assert!(base.value() == fixture.my_partition);
}
fn attribute_parent<T: RuntimeTyped + TdfAttribute, B: RuntimeTyped + TdfAttribute + ?Sized>(
    attribute: T,
) {
    let attribute = Handle::new(attribute);
    let base = parent_roundtrip::<_, B>(&attribute, "TDF_Attribute");
    let borrowed: &B = base.get().unwrap();
    assert_eq!(attribute.id(), borrowed.id());
    assert!(base.label().is_null());
    assert!(borrowed.label().is_null());
    assert!(base.new_empty().is_instance(T::static_type()));
}
#[test]
fn model_object_and_reference_attribute_parent_aliases_dispatch() {
    attribute_parent::<_, TObjTModelBaseType>(TObjTModel::new());
    attribute_parent::<_, TObjTObjectBaseType>(TObjTObject::new());
    attribute_parent::<_, TObjTReferenceBaseType>(TObjTReference::new());
}
#[test]
fn coordinate_sparse_and_name_attribute_parent_aliases_dispatch() {
    attribute_parent::<_, TObjTXyzBaseType>(TObjTXyz::new());
    attribute_parent::<_, TObjTIntSparseArrayBaseType>(TObjTIntSparseArray::new());
    attribute_parent::<_, TObjTNameContainerBaseType>(TObjTNameContainer::new());
}

#[test]
fn checker_algorithm_alias_preserves_borrowed_dynamic_type_and_shared_status() {
    use martensite_tkernel::message::Status;
    let checker = Handle::new(TObjCheckModel::new(&Handle::default()));
    let base = parent_roundtrip::<_, TObjCheckModelBaseType>(&checker, "Message_Algorithm");
    let borrowed: &TObjCheckModelBaseType = checker.algorithm();
    assert!(std::ptr::eq(
        borrowed.dynamic_type(),
        TObjCheckModel::static_type()
    ));
    borrowed.set_status(Status::WARN1);
    assert!(base.get_status().is_set(Status::WARN1));
    assert!(!checker.perform().unwrap());
    assert!(borrowed.get_status().is_set(Status::FAIL1));
    assert!(!base.get_status().is_set(Status::WARN1));
}
