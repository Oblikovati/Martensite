//! Immediate transient-parent borrowing and concrete destruction obligations.
use crate::object_model_document::{ObjectFixture, TestDocumentModel, serial};
use martensite_tkernel::{
    standard::{Handle, RuntimeTyped, StandardTransient, Transient},
    tcollection::ExtendedString,
};
use martensite_tklcaf::{tdata_std::TDataStdInteger, tdoc_std::TDocStdDocument};
use martensite_tktobj::object_model::{
    DocumentApplication, DocumentApplicationApi, DocumentModel, DocumentModelParent,
    DocumentModelState, ModelIntegrityCheck, ModelObjectParent, ObjectIteratorParent,
    StandardObjectIterator, StandardPartition, TypeRegistry,
};
use std::sync::Arc;

fn assert_transient_parent(parent: &StandardTransient, child: &dyn Transient) {
    assert_eq!(
        StandardTransient::static_type().name(),
        "Standard_Transient"
    );
    assert!(std::ptr::eq(parent, child.transient_base()));
    assert_eq!(parent.ref_count(), child.transient_base().ref_count());
}

#[test]
fn model_parent_alias_borrows_the_shared_reference_count_base() {
    let _serial = serial();
    let model = TestDocumentModel::new();
    let parent: &DocumentModelParent = model.transient_base();
    assert_transient_parent(parent, model.get().unwrap());
}

#[test]
fn object_parent_alias_borrows_the_shared_reference_count_base() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.create_object();
    let parent: &ModelObjectParent = object.transient_base();
    assert_transient_parent(parent, object.get().unwrap());
}

#[test]
fn iterator_parent_alias_borrows_the_shared_reference_count_base() {
    let iterator = StandardObjectIterator::default();
    let parent: &ObjectIteratorParent = iterator.transient_base();
    assert_transient_parent(parent, &iterator);
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
    let shared: Handle<dyn DocumentModel> = model.cast();
    let count = model.ref_count();
    let checker = ModelIntegrityCheck::new(&shared);
    assert_eq!(model.ref_count(), count + 1);
    drop(checker);
    assert_eq!(model.ref_count(), count);
    assert!(!shared.is_null());
    assert!(!model.get_label().is_null());
    assert_eq!(application.nb_documents().unwrap(), opened);
}
