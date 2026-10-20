//! `PCDM_ReferenceIterator`: re-creates in a retrieved document the references stored in
//! its file.
use super::{PcdmReference, native_header};
use crate::cdm::{CdmApplication, CdmDocument, CdmDocumentReferences, CdmMetaData};
use crate::live;
use martensite_tkernel::{
    impl_transient,
    message::Messenger,
    osd::{CadPath, File},
    standard::{Failure, Handle, SharedState, StandardTransient, exceptions},
    tcollection::{AsciiString, ExtendedString},
};
use std::sync::Arc;

#[path = "reference_iterator_hooks.rs"]
mod hooks;
pub use hooks::{PcdmDefaultReferenceIteratorHooks, PcdmReferenceIteratorHooks};

#[derive(Default)]
struct IteratorState {
    references: Vec<PcdmReference>,
    iterator: i32,
}

/// Recreates stored references with overridable initialization and cursor callbacks.
/// Use `PcdmReferenceIterator::new(messenger)` for the default native-header provider.
pub struct PcdmReferenceIterator {
    transient: StandardTransient,
    state: SharedState<IteratorState>,
    message_driver: Option<Arc<Messenger>>,
    hooks: Arc<dyn PcdmReferenceIteratorHooks>,
}
impl_transient!(
    PcdmReferenceIterator,
    "PCDM_ReferenceIterator",
    StandardTransient,
    transient
);

impl PcdmReferenceIterator {
    /// `PCDM_ReferenceIterator(theMessageDriver)`.
    pub fn new(message_driver: &Arc<Messenger>) -> Self {
        Self::with_hooks(message_driver, Arc::new(PcdmDefaultReferenceIteratorHooks))
    }
    /// Supplies a reference provider through the native virtual callback contract.
    ///
    /// ```
    /// use std::sync::Arc;
    /// use martensite_tkcdf::pcdm::{PcdmDefaultReferenceIteratorHooks, PcdmReferenceIterator};
    /// use martensite_tkernel::message::Messenger;
    /// let iterator = PcdmReferenceIterator::with_hooks(
    ///     &Arc::new(Messenger::new()), Arc::new(PcdmDefaultReferenceIteratorHooks));
    /// ```
    pub fn with_hooks(
        message_driver: &Arc<Messenger>,
        hooks: Arc<dyn PcdmReferenceIteratorHooks>,
    ) -> Self {
        Self {
            transient: StandardTransient::default(),
            state: SharedState::default(),
            message_driver: Some(message_driver.clone()),
            hooks,
        }
    }
    /// A default provider with a nullable message sink; no sink suppresses warnings.
    /// Example: `let iterator = PcdmReferenceIterator::with_optional_message_driver(None);`.
    pub fn with_optional_message_driver(message_driver: Option<&Arc<Messenger>>) -> Self {
        Self {
            transient: StandardTransient::default(),
            state: SharedState::default(),
            message_driver: message_driver.cloned(),
            hooks: Arc::new(PcdmDefaultReferenceIteratorHooks),
        }
    }
    /// `LoadReferences`: creates in the document one stored reference per reference of
    /// the meta-data's file, with the stored identifier and version.
    pub fn load_references(
        &self,
        document: &Handle<dyn CdmDocument>,
        meta_data: &Handle<CdmMetaData>,
        application: &Handle<dyn CdmApplication>,
        use_storage_configuration: bool,
    ) -> Result<(), Failure> {
        // The source loop asks the provider first: an empty traversal never uses
        // the document or application, and must not reject their unused handles.
        self.init(meta_data)?;
        while self.more() {
            self.load_current_reference(document, application, use_storage_configuration)?;
            self.next();
        }
        Ok(())
    }
    fn load_current_reference(
        &self,
        document: &Handle<dyn CdmDocument>,
        application: &Handle<dyn CdmApplication>,
        use_storage_configuration: bool,
    ) -> Result<(), Failure> {
        let target = live(document, "PCDM_ReferenceIterator::LoadReferences")?;
        let app = live(application, "PCDM_ReferenceIterator::LoadReferences")?;
        let referenced = {
            let mut table = app.meta_data_look_up_table();
            self.meta_data(&mut table, use_storage_configuration)?
        };
        target.create_reference_with_identifier(
            &referenced,
            self.reference_identifier()?,
            application,
            self.document_version()?,
            use_storage_configuration,
        )
    }
    /// `Init`: clears old references, reads the stored records and restarts after success.
    /// Example: `iterator.init(&metadata)?;` leaves no stale records after a failed read.
    pub fn init(&self, meta_data: &Handle<CdmMetaData>) -> Result<(), Failure> {
        self.hooks.init(self, meta_data)
    }
    fn default_init(&self, meta_data: &Handle<CdmMetaData>) -> Result<(), Failure> {
        // Init clears before dereferencing metadata or reading; failures must not
        // retain references from an earlier file, and only success resets the cursor.
        self.state.write().references.clear();
        let file_name = live(meta_data, "PCDM_ReferenceIterator::Init")?.file_name();
        native_header::references(&file_name, self.message_driver.as_ref(), |reference| {
            // Append after the warning callback, into the current member sequence:
            // a reentrant Init may have changed it while no state guard was held.
            self.state.write().references.push(reference);
        })?;
        self.state.write().iterator = 1;
        Ok(())
    }
    /// Private `More`.
    fn more(&self) -> bool {
        self.hooks.more(self)
    }
    fn default_more(&self) -> bool {
        let state = self.state.read();
        state.iterator <= state.references.len() as i32
    }
    /// Private `Next`.
    fn next(&self) {
        self.hooks.next(self);
    }
    fn default_next(&self) {
        self.state.write().iterator += 1;
    }
    fn current(&self) -> Result<PcdmReference, Failure> {
        let state = self.state.read();
        let index = state.iterator;
        usize::try_from(index - 1)
            .ok()
            .and_then(|position| state.references.get(position).cloned())
            .ok_or_else(|| {
                exceptions::out_of_range(&format!(
                    "PCDM_ReferenceIterator: index {index} outside 1..{}",
                    state.references.len()
                ))
            })
    }
    /// Private `MetaData`: the meta-data of the current reference's file; its folder and
    /// name split as on the pinned (non-Windows) platform, read-only when not writable.
    fn meta_data(
        &self,
        table: &mut crate::cdm::MetaDataLookUpTable,
        _use_storage_configuration: bool,
    ) -> Result<Handle<CdmMetaData>, Failure> {
        self.hooks
            .meta_data(self, table, _use_storage_configuration)
    }
    fn default_meta_data(
        &self,
        table: &mut crate::cdm::MetaDataLookUpTable,
        _use_storage_configuration: bool,
    ) -> Result<Handle<CdmMetaData>, Failure> {
        let file = self.current()?.file_name();
        let path = AsciiString::from_extended(&file, None);
        let (mut folder, name) = CadPath::folder_and_file_from_path(path.as_bytes());
        if !folder.is_empty() {
            folder.truncate(folder.length() - 1)?;
        }
        let read_only = !File::new(path.as_bytes()).is_writeable();
        let folder = ExtendedString::from_ascii_string(&folder, true);
        let name = ExtendedString::from_ascii_string(&name, true);
        Ok(CdmMetaData::look_up(
            table, &folder, &name, &file, &file, read_only,
        ))
    }
    /// Private `ReferenceIdentifier` of the current reference.
    fn reference_identifier(&self) -> Result<i32, Failure> {
        self.hooks.reference_identifier(self)
    }
    fn default_reference_identifier(&self) -> Result<i32, Failure> {
        Ok(self.current()?.reference_identifier())
    }
    /// Private `DocumentVersion` of the current reference.
    fn document_version(&self) -> Result<i32, Failure> {
        self.hooks.document_version(self)
    }
    fn default_document_version(&self) -> Result<i32, Failure> {
        Ok(self.current()?.document_version())
    }
}

#[cfg(test)]
#[path = "reference_iterator_header_tests.rs"]
mod header_tests;
