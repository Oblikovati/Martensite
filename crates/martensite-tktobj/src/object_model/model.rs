//! `martensite_object_model_DocumentModel`: the base of the document data framework-based models. Its virtual members are the trait
//! [`DocumentModel`] (the pure virtual `NewEmpty` is required); its non-virtual members are
//! the extension trait [`super::DocumentModelState`] (object-model.md section 3).
use super::model_io;
use super::{
    DocumentApplication, DocumentApplicationApi, DocumentModelState, ModelIntegrityCheck,
    ModelIterator, ModelObject, NameIndex, ObjectIterator,
};
use martensite_tkernel::{
    impl_transient_interface,
    message::Messenger,
    standard::{
        Failure, Guid, Handle, SharedState, StandardTransient, Transient, WeakHandle, exceptions,
    },
    tcollection::{ExtendedString, HExtendedString},
};
use martensite_tklcaf::{
    guid_from_text,
    tdf::{TdfLabel, TdfRelocationTable},
    tdoc_std::TDocStdDocument,
};
use std::{
    io,
    sync::{Arc, OnceLock},
};

/// `martensite_object_model_DocumentModel::DataTag`: the data of the model on its main partition.
pub struct ModelDataTag;
impl ModelDataTag {
    /// `DataTag_FormatVersion`: `martensite_object_model_ModelPartition::DataTag_Last`.
    pub const FORMAT_VERSION: i32 = super::PartitionDataTag::LAST;
    /// `DataTag_Last`.
    pub const LAST: i32 = Self::FORMAT_VERSION + 1;
}

/// The private fields of `martensite_object_model_DocumentModel` (`myLabel`, `myMessenger`) and the self handle.
pub struct DocumentModelBase {
    this: OnceLock<WeakHandle<dyn DocumentModel>>,
    label: SharedState<TdfLabel>,
    messenger: SharedState<Arc<Messenger>>,
}

impl DocumentModelBase {
    /// `martensite_object_model_DocumentModel()`: the messenger of the ObjectModel application.
    pub fn new() -> Result<Self, Failure> {
        let application = DocumentApplication::get_instance()?;
        let messenger = martensite_tkcdf::live(
            &application,
            "martensite_object_model_DocumentModel::martensite_object_model_DocumentModel",
        )?
        .messenger();
        Ok(Self {
            this: OnceLock::new(),
            label: SharedState::new(TdfLabel::default()),
            messenger: SharedState::new(messenger),
        })
    }
    /// Records the owning handle once; a model constructor that returns a handle calls it.
    pub fn bind_self(&self, model: &Handle<dyn DocumentModel>) {
        let _ = self.this.set(model.downgrade());
    }
    /// The handle `this` converts to by the model; `Standard_NullObject` before `bind_self`.
    pub fn this(&self) -> Result<Handle<dyn DocumentModel>, Failure> {
        let this = self.this.get().map(WeakHandle::upgrade).unwrap_or_default();
        if this.is_null() {
            return Err(exceptions::null_object(
                "martensite_object_model_DocumentModel: the model has no owning handle; expected bind_self",
            ));
        }
        Ok(this)
    }
    /// `myLabel`.
    pub(crate) fn label(&self) -> TdfLabel {
        self.label.read().clone()
    }
    /// Sets `myLabel`.
    pub(crate) fn set_label(&self, label: &TdfLabel) {
        *self.label.write() = label.clone();
    }
    /// `myMessenger`.
    pub(crate) fn messenger(&self) -> Arc<Messenger> {
        self.messenger.read().clone()
    }
    /// Sets `myMessenger`.
    pub(crate) fn set_messenger(&self, messenger: Arc<Messenger>) {
        *self.messenger.write() = messenger;
    }
}

impl Drop for DocumentModelBase {
    /// `~martensite_object_model_DocumentModel`: closes the model (the base `Close`, as in a runtime destructor).
    fn drop(&mut self) {
        let label = self.label();
        if !label.is_null() {
            // A destructor cannot report a failure.
            let _ = model_io::close_label(&label);
        }
    }
}

/// `martensite_object_model_DocumentModel`'s virtual members; a concrete model embeds a [`DocumentModelBase`] and
/// registers `dyn DocumentModel` as an interface of its `impl_transient!`.
pub trait DocumentModel: Transient {
    /// The embedded base with the runtime private fields.
    fn model_base(&self) -> &DocumentModelBase;
    /// `NewEmpty` (pure virtual): a new model of the same type.
    fn new_empty(&self) -> Result<Handle<dyn DocumentModel>, Failure>;
    /// `checkDocumentEmpty(theFile)` (protected): true for an empty name, a missing or an
    /// empty file.
    fn check_document_empty(&self, file: &ExtendedString) -> bool {
        model_io::check_document_empty(file)
    }
    /// `Load(theFile)`: reads the model, or initializes an empty one when the file is
    /// empty or missing.
    fn load(&self, file: &ExtendedString) -> Result<bool, Failure> {
        model_io::load_file(self, file)
    }
    /// `Load(theIStream)`: reads the model, or initializes an empty one on failure.
    fn load_stream(&self, stream: &mut dyn io::Read) -> Result<bool, Failure> {
        model_io::load_stream(self, stream)
    }
    /// `SaveAs(theFile)`.
    fn save_as(&self, file: &ExtendedString) -> Result<bool, Failure> {
        model_io::save_as_file(self, file)
    }
    /// `SaveAs(theOStream)`.
    fn save_as_stream(&self, stream: &mut dyn io::Write) -> Result<bool, Failure> {
        model_io::save_as_stream(self, stream)
    }
    /// `Close`: closes the document; false when there is none.
    fn close(&self) -> Result<bool, Failure> {
        let label = self.get_label();
        if label.is_null() {
            return Ok(false);
        }
        let document = TDocStdDocument::get(&label)?;
        if document.is_null() {
            return Ok(false);
        }
        self.close_document(&document)?;
        self.model_base().set_label(&TdfLabel::default());
        Ok(true)
    }
    /// `GetFile`: the path of the stored document, null when not stored.
    fn get_file(&self) -> Result<Handle<HExtendedString>, Failure> {
        model_io::model_file(self)
    }
    /// `GetObjects`: a `martensite_object_model_ModelIterator` on every object.
    fn get_objects(&self) -> Result<Handle<dyn ObjectIterator>, Failure> {
        Ok(ModelIterator::new(&self.model_base().this()?)?.cast())
    }
    /// `GetChildren`: the children of the main partition, null without it.
    fn get_children(&self) -> Result<Handle<dyn ObjectIterator>, Failure> {
        let partition = self.get_main_partition()?;
        match partition.get() {
            Some(partition) => partition.get_children(None),
            None => Ok(Handle::default()),
        }
    }
    /// `FindObject(theName, theDictionary)`: the object of a registered name, null when
    /// none (a null dictionary is the model's own).
    fn find_object(
        &self,
        name: &Handle<HExtendedString>,
        dictionary: &Handle<NameIndex>,
    ) -> Result<Handle<dyn ModelObject>, Failure> {
        super::model_init::find_object(self, name, dictionary)
    }
    /// `GetChecker`: a new `martensite_object_model_ModelIntegrityCheck` of this model.
    fn get_checker(&self) -> Result<Handle<ModelIntegrityCheck>, Failure> {
        Ok(Handle::new(ModelIntegrityCheck::new(
            &self.model_base().this()?,
        )))
    }
    /// `GetRoot`: the main partition.
    fn get_root(&self) -> Result<Handle<dyn ModelObject>, Failure> {
        Ok(self.get_partition(&self.get_label(), false)?.cast())
    }
    /// `GetModelName`: `ObjectModel` by default.
    fn get_model_name(&self) -> Handle<HExtendedString> {
        Handle::new(HExtendedString::from_c_string(b"ObjectModel"))
    }
    /// `IsModified`: the document changed since it was saved; false without document.
    fn is_modified(&self) -> Result<bool, Failure> {
        let document = self.get_document()?;
        match document.get() {
            Some(document) => document.is_changed(),
            None => Ok(false),
        }
    }
    /// `GetApplication`: the static instance of the ObjectModel application.
    fn get_application(&self) -> Result<Handle<dyn DocumentApplicationApi>, Failure> {
        DocumentApplication::get_instance()
    }
    /// `GetFormat`: `ObjectModelBinary` by default.
    fn get_format(&self) -> ExtendedString {
        ExtendedString::new("ObjectModelBinary")
    }
    /// `Update`: true by default.
    fn update(&self) -> Result<bool, Failure> {
        Ok(true)
    }
    /// `GetGUID`: the interface GUID of `martensite_object_model_DocumentModel`.
    fn get_guid(&self) -> Guid {
        guid_from_text("3bbefb49-e618-11d4-ba38-0060b0ee18ea")
    }
    /// `initNewModel(IsNew)` (protected): sets the names map; for a loaded model, the
    /// objects' `AfterRetrieval`, the back references and a consistency check.
    fn init_new_model(&self, is_new: bool) -> Result<bool, Failure> {
        super::model_init::init_new_model(self, is_new)
    }
    /// `updateBackReferences(theObject)` (protected): recursive.
    fn update_back_references(&self, object: &Handle<dyn ModelObject>) -> Result<(), Failure> {
        super::model_init::update_back_references(self, object)
    }
    /// `isToCheck` (protected): true.
    fn is_to_check(&self) -> bool {
        true
    }
    /// `Paste(theModel, theRelocTable)`: clones the main partition into the model.
    fn paste(
        &self,
        model: &Handle<dyn DocumentModel>,
        relocation_table: &Handle<TdfRelocationTable>,
    ) -> Result<bool, Failure> {
        let Some(target) = model.get() else {
            return Ok(false);
        };
        NameIndex::set(&target.get_label())?;
        let partition = self.get_main_partition()?;
        martensite_tkcdf::live(&partition, "martensite_object_model_DocumentModel::Paste")?
            .clone_object(&target.get_label(), relocation_table)?;
        Ok(true)
    }
}
impl_transient_interface!(
    dyn DocumentModel,
    "martensite_object_model_DocumentModel",
    StandardTransient
);
