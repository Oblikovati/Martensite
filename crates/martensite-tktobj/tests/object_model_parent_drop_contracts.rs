//! Immediate transient-parent borrowing and concrete destruction obligations.
use crate::object_model_document::{ObjectFixture, TestDocumentModel, serial};
use martensite_tkernel::{
    standard::{Handle, RuntimeTyped, Transient},
    tcollection::ExtendedString,
};
use martensite_tklcaf::{tdata_std::TDataStdInteger, tdoc_std::TDocStdDocument};
use martensite_tktobj::object_model::{
    DocumentApplication, DocumentApplicationApi, DocumentModel, DocumentModelParent,
    DocumentModelState, ModelIntegrityCheck, ModelObjectParent, ObjectIteratorParent,
    StandardObjectIterator, StandardPartition, TypeRegistry,
};
use std::sync::Arc;

fn assert_transient_parent<T: Transient>(parent: &Handle<dyn Transient>, child: &Handle<T>) {
    assert!(parent.is_kind(<dyn Transient>::static_type()));
    assert_eq!(parent.object_address(), child.object_address());
    assert_eq!(parent.ref_count(), child.ref_count());
    assert!(std::ptr::eq(
        parent.dynamic_type().unwrap(),
        child.dynamic_type().unwrap()
    ));
}

#[test]
fn model_parent_alias_shares_the_model_owner() {
    let _serial = serial();
    let model = TestDocumentModel::new();
    assert_transient_parent(&model.upcast::<DocumentModelParent>(), &model);
}

#[test]
fn object_parent_alias_shares_the_object_owner() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.create_object();
    assert_transient_parent(&object.upcast::<ModelObjectParent>(), &object);
}

#[test]
fn iterator_weak_parent_expires_with_the_iterator() {
    let iterator = Handle::new(StandardObjectIterator::default());
    let parent = iterator.upcast::<ObjectIteratorParent>();
    assert_transient_parent(&parent, &iterator);
    let weak = parent.downgrade();
    drop(iterator);
    assert_eq!(weak.ref_count(), 1);
    drop(parent);
    assert!(weak.upgrade().is_null());
}

#[test]
fn dropping_a_persistence_registration_unregisters_its_exact_type_name() {
    let fixture = ObjectFixture::set_up();
    let label = fixture.command(|| fixture.my_partition.new_label().unwrap());
    let name = "ObjectModel_DropRegistrationContract";
    let registration = TypeRegistry::new(name, Arc::new(StandardPartition::persistence_new));
    assert!(
        !TypeRegistry::create_new_object(name, &label)
            .unwrap()
            .is_null()
    );
    drop(registration);
    assert!(
        TypeRegistry::create_new_object(name, &label)
            .unwrap()
            .is_null()
    );
}

type RegisteredModelDocument = (
    Handle<TestDocumentModel>,
    Handle<dyn DocumentApplicationApi>,
    Handle<TDocStdDocument>,
);

fn model_with_registered_document() -> RegisteredModelDocument {
    let model = TestDocumentModel::new();
    let application = DocumentApplication::get_instance().unwrap();
    let mut document = Handle::default();
    application
        .new_tdoc_std_document(
            &ExtendedString::new("binary document format"),
            &mut document,
        )
        .unwrap();
    model.set_label(&document.main().unwrap());
    (model, application, document)
}

#[test]
fn dropping_a_model_aborts_its_command_forgets_attributes_and_closes_document() {
    let _serial = serial();
    let (model, application, document) = model_with_registered_document();
    let opened = application.nb_documents().unwrap();
    document.set_undo_limit(1).unwrap();
    let child = document.main().unwrap().find_child(1, true).unwrap();
    TDataStdInteger::set(&child, 10).unwrap();
    document.new_command().unwrap();
    TDataStdInteger::set(&child, 20).unwrap();
    drop(model);
    assert!(!document.has_open_command());
    assert_eq!(application.nb_documents().unwrap(), opened - 1);
    let mut integer = Handle::<TDataStdInteger>::default();
    assert!(
        !child
            .find_attribute_as(&TDataStdInteger::get_id(), &mut integer)
            .unwrap()
    );
}

#[test]
fn default_checker_drop_releases_its_model_handle_without_closing_the_model() {
    let _serial = serial();
    let (model, application, _document) = model_with_registered_document();
    let opened = application.nb_documents().unwrap();
    let shared: Handle<dyn DocumentModel> = model.upcast();
    let count = model.ref_count();
    let checker = ModelIntegrityCheck::new(&shared);
    assert_eq!(model.ref_count(), count + 1);
    drop(checker);
    assert_eq!(model.ref_count(), count);
    assert!(!shared.is_null());
    assert!(!model.get_label().is_null());
    assert_eq!(application.nb_documents().unwrap(), opened);
}
