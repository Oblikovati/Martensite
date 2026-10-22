//! Declared parent aliases retain inherited dispatch and allocation identity.
use crate::object_model_document::ObjectFixture;
use martensite_tkernel::standard::{Handle, RuntimeTyped, Transient};
use martensite_tklcaf::tdf::{TdfAttribute, TdfAttributeState};
use martensite_tktobj::object_model::*;

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
    let application = DocumentApplication::get_instance().unwrap();
    let base =
        parent_roundtrip::<_, DocumentApplicationParent>(&application, "TDocStd_Application");
    let borrowed: &DocumentApplicationParent = application.get().unwrap();
    assert_eq!(borrowed.resources_name(), "ObjectModel");
    assert_eq!(base.resources_name(), borrowed.resources_name());
}
#[test]
fn partition_parent_aliases_preserve_object_and_partition_dispatch() {
    let fixture = ObjectFixture::set_up();
    let partition = fixture.my_partition.cast::<StandardPartition>();
    let base = parent_roundtrip::<_, ModelPartitionParent>(
        &partition,
        "martensite_object_model_ModelObject",
    );
    let borrowed: &ModelPartitionParent = partition.get().unwrap();
    assert_eq!(base.get_label(), borrowed.get_label());
    let hidden = fixture.command(|| HiddenPartition::new(&partition.new_label().unwrap()).unwrap());
    let base = parent_roundtrip::<_, HiddenPartitionParent>(
        &hidden,
        "martensite_object_model_ModelPartition",
    );
    let borrowed: &HiddenPartitionParent = hidden.get().unwrap();
    assert_eq!(base.get_label(), borrowed.get_label());
    assert_eq!(base.get_label(), hidden.get_label());
}
#[test]
fn sequence_parent_alias_exposes_live_inherited_iterator_state() {
    let fixture = ObjectFixture::set_up();
    let objects = Handle::new(BackReferences::default());
    objects.append(fixture.my_partition.cast());
    let iterator = Handle::new(TypedSequenceIterator::new(&objects, None));
    let base = parent_roundtrip::<_, TypedSequenceIteratorParent>(
        &iterator,
        "martensite_object_model_ObjectIterator",
    );
    let borrowed: &TypedSequenceIteratorParent = iterator.get().unwrap();
    assert!(borrowed.more());
    assert!(base.checked_value().unwrap() == fixture.my_partition);
    base.next().unwrap();
    assert!(!borrowed.more());
}
#[test]
fn model_and_label_parent_aliases_share_virtual_iteration() {
    let fixture = ObjectFixture::set_up();
    let iterator = ModelIterator::new(&fixture.my_model().clone().cast()).unwrap();
    let base = parent_roundtrip::<_, ModelIteratorParent>(
        &iterator,
        "martensite_object_model_ObjectIterator",
    );
    let borrowed: &ModelIteratorParent = iterator.get().unwrap();
    assert!(borrowed.value() == fixture.my_partition && base.more());
    let object = fixture.command(|| fixture.create_object());
    check_label_parent_alias(&fixture, &object);
}
fn check_label_parent_alias(
    fixture: &ObjectFixture,
    object: &Handle<crate::object_model_document::TestModelObject>,
) {
    let label = fixture.my_partition.get_child_label().unwrap();
    let iterator = ObjectTreeIterator::new(&label, None, true, false).unwrap();
    let label_base = parent_roundtrip::<_, ObjectTreeIteratorParent>(
        &iterator,
        "martensite_object_model_LabelIterator",
    );
    let inherited: Handle<LabelIteratorParent> = label_base.cast();
    assert!(inherited.cast::<ObjectTreeIterator>() == iterator);
    let borrowed: &LabelIteratorParent = label_base.get().unwrap();
    assert_eq!(
        <LabelIteratorParent>::static_type().name(),
        "martensite_object_model_ObjectIterator"
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
        ReferenceIterator::new(&holder.get_reference_label().unwrap(), None, true).unwrap();
    let base = parent_roundtrip::<_, ReferenceIteratorParent>(
        &iterator,
        "martensite_object_model_LabelIterator",
    );
    let borrowed: &ReferenceIteratorParent = iterator.get().unwrap();
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
    attribute_parent::<_, ModelAttributeParent>(ModelAttribute::new());
    attribute_parent::<_, ObjectAttributeParent>(ObjectAttribute::new());
    attribute_parent::<_, ReferenceAttributeParent>(ReferenceAttribute::new());
}
#[test]
fn coordinate_sparse_and_name_attribute_parent_aliases_dispatch() {
    attribute_parent::<_, CoordinateAttributeParent>(CoordinateAttribute::new());
    attribute_parent::<_, IntegerSparseArrayParent>(IntegerSparseArray::new());
    attribute_parent::<_, NameIndexParent>(NameIndex::new());
}

#[test]
fn checker_algorithm_alias_preserves_borrowed_dynamic_type_and_shared_status() {
    use martensite_tkernel::message::Status;
    let checker = Handle::new(ModelIntegrityCheck::new(&Handle::default()));
    let base = parent_roundtrip::<_, ModelIntegrityCheckParent>(&checker, "Message_Algorithm");
    let borrowed: &ModelIntegrityCheckParent = checker.algorithm();
    assert!(std::ptr::eq(
        borrowed.dynamic_type(),
        ModelIntegrityCheck::static_type()
    ));
    borrowed.set_status(Status::WARN1);
    assert!(base.get_status().is_set(Status::WARN1));
    assert!(!checker.perform().unwrap());
    assert!(borrowed.get_status().is_set(Status::FAIL1));
    assert!(!base.get_status().is_set(Status::WARN1));
}
