//! `martensite_object_model_DocumentApplication`: the document data framework application of ObjectModel models (resources `ObjectModel`).
use super::application_errors::{reader_error_key, store_error_key};
use super::messages::{load_object_model_messages, message_with};
use martensite_tkcdf::{
    cdf::CdfApplication,
    cdm::CdmApplication,
    json_dump::{base_class, class_begin},
    pcdm::{PcdmReaderStatus, PcdmStoreStatus},
};
use martensite_tkernel::{
    impl_transient_interface,
    message::{Gravity, Messenger, ProgressRange},
    standard::{Failure, Handle, SharedState, StandardTransient, dump::DumpStream},
    tcollection::ExtendedString,
};
use martensite_tklcaf::tdoc_std::{
    TDocStdApplication, TDocStdApplicationBase, TDocStdApplicationIo, TDocStdDocument,
};
use std::{
    io,
    sync::{Arc, OnceLock},
};

/// The fields of `martensite_object_model_DocumentApplication` (`myIsError`, `myIsVerbose`, `myMessenger`) over the
/// `TDocStd_Application` ones.
pub struct DocumentApplicationBase {
    tdoc_std: TDocStdApplicationBase,
    is_error: SharedState<bool>,
    is_verbose: SharedState<bool>,
    messenger: SharedState<Arc<Messenger>>,
}

impl DocumentApplicationBase {
    /// `martensite_object_model_DocumentApplication()`: loads the ObjectModel messages once (`Standard_ProgramError` when
    /// they are invalid), a new messenger, not verbose.
    pub fn new() -> Result<Self, Failure> {
        load_object_model_messages()?;
        Ok(Self {
            tdoc_std: TDocStdApplicationBase::default(),
            is_error: SharedState::new(false),
            is_verbose: SharedState::new(false),
            messenger: SharedState::new(Arc::new(Messenger::new())),
        })
    }
    /// The embedded `TDocStd_Application` fields.
    pub fn tdoc_std(&self) -> &TDocStdApplicationBase {
        &self.tdoc_std
    }
}

/// `martensite_object_model_DocumentApplication`'s members over the embedded base; a concrete application fills its
/// `CdmApplication`, `CdfApplication` and `TDocStdApplication` impls like
/// [`DocumentApplication`].
pub trait DocumentApplicationApi: TDocStdApplication {
    /// The embedded base with the runtime fields.
    fn document_application_base(&self) -> &DocumentApplicationBase;
    /// `SaveDocument(theSourceDoc, theTargetFile)`: `SaveAs`; an error is reported.
    fn save_document(
        &self,
        document: &Handle<TDocStdDocument>,
        target_file: &ExtendedString,
    ) -> Result<bool, Failure> {
        let status = self.save_as(document, target_file, &ProgressRange::default())?;
        self.store_result(status, target_file)
    }
    /// `SaveDocument(theSourceDoc, theOStream)`.
    fn save_document_stream(
        &self,
        document: &Handle<TDocStdDocument>,
        stream: &mut dyn io::Write,
    ) -> Result<bool, Failure> {
        let status = self.save_as_stream(document, stream, &ProgressRange::default());
        self.store_result(status, &ExtendedString::default())
    }
    /// `LoadDocument(theSourceFile, theTargetDoc)`: `Open`; a caught failure is
    /// `PCDM_RS_ReaderException`, an error is reported.
    fn load_document(
        &self,
        source_file: &ExtendedString,
        document: &mut Handle<TDocStdDocument>,
    ) -> Result<bool, Failure> {
        let status = self
            .open_path_without_filter(source_file, document, &ProgressRange::default())
            .unwrap_or(PcdmReaderStatus::ReaderException);
        self.reader_result(status, source_file)
    }
    /// `LoadDocument(theIStream, theTargetDoc)`.
    fn load_document_stream(
        &self,
        stream: &mut dyn io::Read,
        document: &mut Handle<TDocStdDocument>,
    ) -> Result<bool, Failure> {
        let status = self.open_stream_without_filter(stream, document, &ProgressRange::default());
        self.reader_result(status, &ExtendedString::default())
    }
    /// `CreateNewDocument(theDoc, theFormat)`: `NewDocument`.
    fn create_new_document(
        &self,
        document: &mut Handle<TDocStdDocument>,
        format: &ExtendedString,
    ) -> Result<bool, Failure> {
        *self.document_application_base().is_error.write() = false;
        self.new_tdoc_std_document(format, document)?;
        Ok(!*self.document_application_base().is_error.read())
    }
    /// `ErrorMessage(theMsg, theLevel)`: sends the message to the messenger.
    fn error_message(&self, message: &ExtendedString, level: Gravity) {
        self.messenger().send_extended(message, level);
    }
    /// `ErrorMessage(theMsg)`: with `Message_Alarm`.
    fn error_message_alarm(&self, message: &ExtendedString) {
        self.error_message(message, Gravity::Alarm);
    }
    /// `Messenger`: the messenger handle.
    fn messenger(&self) -> Arc<Messenger> {
        self.document_application_base().messenger.read().clone()
    }
    /// Assigns the messenger (the runtime `Messenger()` returns a modifiable reference).
    fn set_messenger(&self, messenger: Arc<Messenger>) {
        *self.document_application_base().messenger.write() = messenger;
    }
    /// `SetVerbose(isVerbose)`.
    fn set_verbose(&self, is_verbose: bool) {
        *self.document_application_base().is_verbose.write() = is_verbose;
    }
    /// `IsVerbose`.
    fn is_verbose(&self) -> bool {
        *self.document_application_base().is_verbose.read()
    }
    /// `DumpJson`: the class, then `TDocStd_Application` as the base.
    fn object_model_dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        class_begin(stream, "martensite_object_model_DocumentApplication")?;
        base_class(stream, depth, &|s, d| {
            TDocStdApplication::dump_json(self, s, d)
        })
    }
    /// `SetError(PCDM_StoreStatus, theInfo)` (protected).
    fn set_store_error(
        &self,
        status: PcdmStoreStatus,
        info: &ExtendedString,
    ) -> Result<(), Failure> {
        self.error_message_alarm(&message_with(store_error_key(status), info)?);
        Ok(())
    }
    /// `SetError(PCDM_ReaderStatus, theInfo)` (protected).
    fn set_reader_error(
        &self,
        status: PcdmReaderStatus,
        info: &ExtendedString,
    ) -> Result<(), Failure> {
        self.error_message_alarm(&message_with(reader_error_key(status), info)?);
        Ok(())
    }
    /// Records the error flag of a store status and reports it (then `Standard::Purge`).
    fn store_result(
        &self,
        status: PcdmStoreStatus,
        info: &ExtendedString,
    ) -> Result<bool, Failure> {
        let is_error = status != PcdmStoreStatus::Ok;
        *self.document_application_base().is_error.write() = is_error;
        if is_error {
            self.set_store_error(status, info)?;
        }
        martensite_tkernel::standard::memory::purge();
        Ok(!is_error)
    }
    /// Records the error flag of a reader status and reports it (then `Standard::Purge`).
    fn reader_result(
        &self,
        status: PcdmReaderStatus,
        info: &ExtendedString,
    ) -> Result<bool, Failure> {
        let is_error = status != PcdmReaderStatus::Ok;
        *self.document_application_base().is_error.write() = is_error;
        if is_error {
            self.set_reader_error(status, info)?;
        }
        martensite_tkernel::standard::memory::purge();
        Ok(!is_error)
    }
}
impl_transient_interface!(
    dyn DocumentApplicationApi,
    "martensite_object_model_DocumentApplication",
    dyn TDocStdApplication
);

/// `martensite_object_model_DocumentApplication` itself; [`DocumentApplication::get_instance`] is the process
/// instance.
pub struct DocumentApplication {
    transient: StandardTransient,
    base: DocumentApplicationBase,
}
super::canonical_runtime_type::canonical_object_model_type!(DocumentApplication, dyn DocumentApplicationApi, transient, [dyn CdmApplication, dyn CdfApplication, dyn TDocStdApplication, dyn DocumentApplicationApi]);

/// The process instance of `GetInstance` (a function-local static by the model).
static INSTANCE: OnceLock<Result<Handle<DocumentApplication>, Failure>> = OnceLock::new();

impl DocumentApplication {
    /// `martensite_object_model_DocumentApplication()`, already in its bound handle.
    pub fn new() -> Result<Handle<DocumentApplication>, Failure> {
        let application = Handle::new(Self {
            transient: StandardTransient::default(),
            base: DocumentApplicationBase::new()?,
        });
        if let Some(own) = application.get() {
            CdmApplication::application_base(own).bind_self(&application.cast());
        }
        Ok(application)
    }
    /// `GetInstance`: the static instance of the application.
    ///
    /// ```
    /// use martensite_tktobj::object_model::DocumentApplication;
    /// use martensite_tklcaf::tdoc_std::TDocStdApplication;
    /// let application = DocumentApplication::get_instance().unwrap();
    /// assert_eq!(application.resources_name(), "ObjectModel");
    /// ```
    pub fn get_instance() -> Result<Handle<dyn DocumentApplicationApi>, Failure> {
        let instance = INSTANCE.get_or_init(Self::new);
        instance
            .as_ref()
            .map(|application| application.cast())
            .map_err(Clone::clone)
    }
}

impl CdmApplication for DocumentApplication {
    martensite_tklcaf::tdoc_std_application_cdm_members!();
}
impl CdfApplication for DocumentApplication {
    martensite_tklcaf::tdoc_std_application_cdf_members!();
}
impl TDocStdApplication for DocumentApplication {
    fn tdoc_std_application_base(&self) -> &TDocStdApplicationBase {
        self.base.tdoc_std()
    }
    /// `ResourcesName`: `ObjectModel`.
    fn resources_name(&self) -> String {
        String::from("ObjectModel")
    }
}
impl DocumentApplicationApi for DocumentApplication {
    fn document_application_base(&self) -> &DocumentApplicationBase {
        &self.base
    }
}
