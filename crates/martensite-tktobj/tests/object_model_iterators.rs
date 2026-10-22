//! The ObjectModel iterators and their protected state, against the semantics of
#![allow(non_snake_case)]
use crate::object_model_document::{ObjectFixture, TestModelObject};
use crate::object_model_references::TestContainer;
use martensite_tkernel::standard::{Handle, RuntimeTyped};
use martensite_tktobj::object_model::{
    BackReferences, DocumentModel, LabelIterator, LabelIteratorBase, ModelIterator,
    ModelObjectData, ModelObjectState, ObjectIterator, ObjectTreeIterator, ReferenceIterator,
    TypeRegistry, TypedSequenceIterator, label_iterator_more,
};

#[test]
fn a_sequence_iterator_skips_other_types_and_stops_at_a_null_entry() {
    let fixture = ObjectFixture::set_up();
    let (plain, container) = fixture.command(|| {
        let plain = fixture.create_object();
        let container = TestContainer::new(&fixture.my_partition.new_label().unwrap());
        (plain, container)
    });
    let sequence = Handle::new(BackReferences::default());
    for object in [
        container.cast(),
        plain.cast(),
        Handle::default(),
        plain.cast(),
    ] {
        sequence.append(object);
    }
    let kind = TestModelObject::static_type();
    let iterator = TypedSequenceIterator::new(&sequence, Some(kind));
    assert!(std::ptr::eq(iterator.kind().unwrap(), kind));
    assert!(iterator.objects() == sequence);
    // More moves past the container, as the runtime const member does.
    assert!(iterator.more());
    assert_eq!(iterator.index(), 2);
    assert!(iterator.value() == plain);
    assert!(iterator.checked_value().unwrap() == plain);
    iterator.next().unwrap();
    // A null entry ends the iteration, although an object follows it.
    assert!(!iterator.more());
    assert!(!TypedSequenceIterator::empty().more());
}

#[test]
fn object_tree_iterator_stops_at_the_first_object_level_unless_asked() {
    let fixture = ObjectFixture::set_up();
    let parent = fixture.command(|| {
        let parent = fixture.create_object();
        parent.add_child().add_child();
        parent
    });
    let label = parent.get_child_label().unwrap();
    let first_level = ObjectTreeIterator::new(&label, None, true, false).unwrap();
    assert!(first_level.kind().is_none() && !first_level.all_sub_children());
    let mut count = 0;
    while first_level.more() {
        count += 1;
        assert!(!first_level.label_value().is_null());
        first_level.next().unwrap();
    }
    assert_eq!(count, 1);
    let all = ObjectTreeIterator::new(&label, None, true, true).unwrap();
    assert!(all.all_sub_children());
    let mut count = 0;
    while all.more() {
        count += 1;
        all.next().unwrap();
    }
    assert_eq!(count, 2);
}

#[test]
fn a_reference_iterator_filters_the_referred_type() {
    let fixture = ObjectFixture::set_up();
    let (holder, container) = fixture.command(|| {
        let holder = fixture.create_object();
        let container = TestContainer::new(&fixture.my_partition.new_label().unwrap());
        holder.set_reference(&container.cast(), 1, 0).unwrap();
        holder
            .set_reference(&fixture.create_object().cast(), 2, 0)
            .unwrap();
        (holder, container)
    });
    let label = holder.get_reference_label().unwrap();
    let kind = TestModelObject::static_type();
    let iterator = ReferenceIterator::new(&label, Some(kind), true).unwrap();
    assert!(std::ptr::eq(iterator.kind().unwrap(), kind));
    assert!(iterator.more());
    assert!(iterator.value() != container);
    iterator.next().unwrap();
    assert!(!iterator.more());
    let any = ReferenceIterator::new(&label, None, true).unwrap();
    assert!(any.value() == container);
}

#[test]
fn a_label_iterator_base_starts_empty_or_on_a_label() {
    let fixture = ObjectFixture::set_up();
    let base = LabelIteratorBase::new();
    assert!(base.node().is_null() && base.object().is_null());
    assert!(!base.child_iterator().more());
    let object = fixture.command(|| fixture.create_object());
    let started =
        LabelIteratorBase::with_label(&fixture.my_partition.get_child_label().unwrap(), false)
            .unwrap();
    assert!(started.child_iterator().more());
    started.set_current(&object.cast(), &object.get_label());
    assert!(started.object() == object && started.node() == object.get_label());
    let failure = LabelIteratorBase::with_label(&Default::default(), false)
        .err()
        .unwrap();
    assert!(failure.is_kind("Standard_NullObject"));
    let iterator = ObjectTreeIterator::new(
        &fixture.my_partition.get_child_label().unwrap(),
        None,
        false,
        false,
    )
    .unwrap();
    assert!(label_iterator_more(&*iterator));
}

#[test]
fn a_model_iterator_keeps_a_stack_of_child_iterators() {
    let fixture = ObjectFixture::set_up();
    fixture.command(|| {
        fixture.create_object().add_child();
    });
    let model: Handle<dyn DocumentModel> = fixture.my_model().clone().cast();
    let iterator = ModelIterator::new(&model).unwrap();
    assert!(iterator.object() == fixture.my_partition);
    assert_eq!(iterator.iterators().len(), 1);
    iterator.next().unwrap();
    // The first object's children iterator is pushed on the stack.
    assert_eq!(iterator.iterators().len(), 2);
    iterator.next().unwrap();
    iterator.next().unwrap();
    assert!(!iterator.more() && iterator.iterators().is_empty());
}

#[test]
fn the_persistence_map_holds_the_registered_tools() {
    let fixture = ObjectFixture::set_up();
    let label = fixture.command(|| fixture.my_partition.new_label().unwrap());
    let has_partition = TypeRegistry::map_of_types()
        .items()
        .any(|(name, _)| name.as_bytes() == b"martensite_object_model_ModelPartition");
    assert!(has_partition);
    let created = TypeRegistry::create_new_object("TestModelObject", &label).unwrap();
    assert!(created.is_kind(TestModelObject::static_type()));
    assert!(!created.is_alive().unwrap());
}

#[test]
fn checked_sequence_value_distinguishes_null_slot_from_exhaustion() {
    let sequence = Handle::new(BackReferences::default());
    sequence.append(Handle::default());
    let iterator = TypedSequenceIterator::new(&sequence, None);
    let interface: &dyn ObjectIterator = &iterator;
    assert!(!interface.more());
    assert!(interface.checked_value().unwrap().is_null());
    interface.next().unwrap();
    let failure = interface.checked_value().unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_OutOfRange");
}

#[test]
fn checked_sequence_value_reports_empty_and_null_sequences() {
    let sequence = Handle::new(BackReferences::default());
    let iterator = TypedSequenceIterator::new(&sequence, None);
    assert_eq!(
        iterator.checked_value().unwrap_err().exception_type(),
        "Standard_OutOfRange"
    );
    assert_eq!(
        TypedSequenceIterator::empty()
            .checked_value()
            .unwrap_err()
            .exception_type(),
        "Standard_NullObject"
    );
}

#[test]
fn checked_base_iterator_retains_the_defined_null_result() {
    let iterator = martensite_tktobj::object_model::StandardObjectIterator::default();
    let interface: &dyn ObjectIterator = &iterator;
    assert!(interface.checked_value().unwrap().is_null());
}
