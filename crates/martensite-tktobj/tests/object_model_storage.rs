//! Document storage behavior uses the configured application driver.
//! A model without a driver reports the driver failure instead of writing an implicit format.
#![allow(non_snake_case)]
use crate::object_model_document::{ModelFixture, serial};
use martensite_tkernel::{
    impl_transient,
    message::{Gravity, Messenger, Printer, PrinterBase, PrinterType},
    standard::{Failure, Handle, RuntimeType, RuntimeTyped, StandardTransient, dump::DumpStream},
    tcollection::{AsciiString, ExtendedString},
};
use martensite_tklcaf::tdoc_std::{TDocStdApplication, TDocStdDocument};
use martensite_tktobj::object_model::{
    DocumentApplication, DocumentApplicationApi, DocumentModel, DocumentModelBase,
    DocumentModelState, ModelAssistant,
};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex, PoisonError},
};

/// A printer that records every message it receives, for the assertions.
pub struct RecordingPrinter {
    base: PrinterBase,
    messages: Mutex<Vec<(String, Gravity)>>,
}
impl RecordingPrinter {
    /// A messenger that writes into a new recording printer.
    pub fn messenger() -> (Arc<Messenger>, Arc<RecordingPrinter>) {
        let printer = Arc::new(Self {
            base: PrinterBase::new(Gravity::Trace),
            messages: Mutex::new(Vec::new()),
        });
        (Arc::new(Messenger::with_printer(printer.clone())), printer)
    }
    /// The recorded messages.
    pub fn messages(&self) -> Vec<(String, Gravity)> {
        self.messages
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}
impl Printer for RecordingPrinter {
    fn printer_base(&self) -> &PrinterBase {
        &self.base
    }
    fn dynamic_type(&self) -> &'static RuntimeType {
        PrinterType::static_type()
    }
    fn send(&self, text: &AsciiString, gravity: Gravity) {
        let text = String::from_utf8_lossy(text.as_bytes()).into_owned();
        self.messages
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push((text, gravity));
    }
}

/// A model with its own application (`GetApplication` is virtual), so that the messages
/// of one test do not reach the process-wide application.
pub struct PrivateModel {
    transient: StandardTransient,
    base: DocumentModelBase,
    application: Handle<DocumentApplication>,
}
impl_transient!(PrivateModel, "ObjectModel_PrivateModel", dyn DocumentModel, transient,
    interfaces: [dyn DocumentModel]);
impl DocumentModel for PrivateModel {
    fn model_base(&self) -> &DocumentModelBase {
        &self.base
    }
    fn new_empty(&self) -> Result<Handle<dyn DocumentModel>, Failure> {
        Ok(PrivateModel::new().cast())
    }
    fn get_application(&self) -> Result<Handle<dyn DocumentApplicationApi>, Failure> {
        Ok(self.application.cast())
    }
}
impl PrivateModel {
    /// A new model over a new application.
    pub fn new() -> Handle<PrivateModel> {
        let model = Handle::new(Self {
            transient: StandardTransient::default(),
            base: DocumentModelBase::new().unwrap(),
            application: DocumentApplication::new().unwrap(),
        });
        model.model_base().bind_self(&model.cast());
        model
    }
}

/// A model with recording messengers on itself and on its application.
struct Recorded {
    model: Handle<PrivateModel>,
    model_messages: Arc<RecordingPrinter>,
    application_messages: Arc<RecordingPrinter>,
}
fn recorded_model() -> Recorded {
    let model = PrivateModel::new();
    let (messenger, model_messages) = RecordingPrinter::messenger();
    model.set_messenger(messenger);
    let (messenger, application_messages) = RecordingPrinter::messenger();
    model.application.set_messenger(messenger);
    Recorded {
        model,
        model_messages,
        application_messages,
    }
}

/// A directory of this test binary for the files of one test.
fn scratch(name: &str) -> PathBuf {
    let directory = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join("tkobject_model")
        .join(name);
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).unwrap();
    directory
}

fn extended(path: &std::path::Path) -> ExtendedString {
    ExtendedString::new(path.to_str().unwrap())
}

#[test]
fn an_empty_or_missing_file_is_an_empty_document() {
    let directory = scratch("empty_document");
    let model = PrivateModel::new();
    assert!(model.check_document_empty(&ExtendedString::default()));
    assert!(model.check_document_empty(&extended(&directory.join("missing"))));
    let empty = directory.join("empty");
    std::fs::write(&empty, b"").unwrap();
    assert!(model.check_document_empty(&extended(&empty)));
    let full = directory.join("full");
    std::fs::write(&full, b"x").unwrap();
    assert!(!model.check_document_empty(&extended(&full)));
}

#[test]
fn saving_without_a_storage_driver_reports_the_driver_failure() {
    let _serial = serial();
    let directory = scratch("save_without_driver");
    let recorded = recorded_model();
    assert!(recorded.model.load(&ExtendedString::default()).unwrap());
    let file = directory.join("model.object_model");
    assert!(!recorded.model.save_as(&extended(&file)).unwrap());
    assert!(!file.exists());
    let expected = format!(
        "Error saving document {} : driver failure. Check presence of resource files.",
        file.display()
    );
    let messages = recorded.application_messages.messages();
    assert!(
        messages.contains(&(expected, Gravity::Alarm)),
        "{messages:?}"
    );
    let mut stream = Vec::new();
    assert!(!recorded.model.save_as_stream(&mut stream).unwrap());
    assert!(stream.is_empty());
    let expected = "Error saving document  : driver failure. Check presence of resource files.";
    assert!(
        recorded
            .application_messages
            .messages()
            .contains(&(expected.to_owned(), Gravity::Alarm))
    );
    assert!(recorded.model.close().unwrap());
}

#[test]
fn failed_save_preserves_existing_destination() {
    let _serial = serial();
    let directory = scratch("save_preserves_destination");
    let recorded = recorded_model();
    assert!(recorded.model.load(&ExtendedString::default()).unwrap());
    let file = directory.join("model.object_model");
    std::fs::write(&file, b"existing document sentinel").unwrap();
    assert!(!recorded.model.save_as(&extended(&file)).unwrap());
    assert_eq!(std::fs::read(&file).unwrap(), b"existing document sentinel");
    assert!(recorded.model.close().unwrap());
}

#[test]
fn saving_to_a_file_that_cannot_be_written_is_refused() {
    let _serial = serial();
    let directory = scratch("save_without_access");
    let recorded = recorded_model();
    assert!(recorded.model.load(&ExtendedString::default()).unwrap());
    let file = directory
        .join("no_such_directory")
        .join("model.object_model");
    assert!(!recorded.model.save_as(&extended(&file)).unwrap());
    let expected = format!(
        "Cannot write the file {}, check permissions",
        file.display()
    );
    assert_eq!(
        recorded.model_messages.messages(),
        [(expected, Gravity::Alarm)]
    );
    assert!(recorded.application_messages.messages().is_empty());
    assert!(recorded.model.close().unwrap());
}

#[test]
fn loading_an_unreadable_file_fails_and_leaves_no_model() {
    let _serial = serial();
    let directory = scratch("load_unreadable");
    let file = directory.join("garbage.object_model");
    std::fs::write(&file, b"not a document").unwrap();
    let recorded = recorded_model();
    assert!(!recorded.model.load(&extended(&file)).unwrap());
    assert!(recorded.model.get_label().is_null());
    let loading = format!("+++++ Load document {}", file.display());
    assert_eq!(
        recorded.model_messages.messages(),
        [(loading, Gravity::Info)]
    );
    let errors = recorded.application_messages.messages();
    let prefix = format!("Error loading document {} : ", file.display());
    assert!(
        errors
            .iter()
            .any(|(text, gravity)| text.starts_with(&prefix) && *gravity == Gravity::Alarm)
    );
}

#[test]
fn loading_an_unreadable_stream_initializes_an_empty_model() {
    let _serial = serial();
    let recorded = recorded_model();
    let mut stream: &[u8] = b"not a document";
    assert!(recorded.model.load_stream(&mut stream).unwrap());
    assert!(!recorded.model.get_label().is_null());
    assert!(!recorded.model.get_main_partition().unwrap().is_null());
    let messages = recorded.model_messages.messages();
    assert!(messages[0].0.contains("+++++ Load document") && messages[0].1 == Gravity::Info);
    assert!(recorded.model.close().unwrap());
}

#[test]
fn an_unsaved_model_has_no_file_and_cannot_be_saved_again() {
    let fixture = ModelFixture::set_up();
    assert!(fixture.my_model.get_file().unwrap().is_null());
    let failure = fixture.my_model.save().unwrap_err();
    assert!(failure.is_kind("Standard_NoSuchObject"));
}

#[test]
fn the_format_version_is_read_by_the_assistant() {
    let fixture = ModelFixture::set_up();
    let model = &fixture.my_model;
    model.open_command().unwrap();
    model.set_format_version(7).unwrap();
    model.commit_command().unwrap();
    assert_eq!(model.get_format_version().unwrap(), 7);
    ModelAssistant::set_current_model(&model.clone().cast());
    assert_eq!(ModelAssistant::get_app_version().unwrap(), 7);
    assert!(ModelAssistant::get_current_model() == *model);
    ModelAssistant::unset_current_model();
    assert!(ModelAssistant::get_current_model().is_null());
}

#[test]
fn the_assistant_binds_types_and_models() {
    let fixture = ModelFixture::set_up();
    ModelAssistant::clear_type_map();
    let partition = DocumentApplication::static_type();
    let document = TDocStdDocument::static_type();
    assert_eq!(ModelAssistant::bind_type(Some(partition)), 1);
    assert_eq!(ModelAssistant::bind_type(Some(document)), 2);
    assert_eq!(ModelAssistant::bind_type(Some(partition)), 1);
    assert_eq!(ModelAssistant::bind_type(None), 3);
    assert_eq!(ModelAssistant::bind_type(None), 4);
    assert!(ModelAssistant::find_type(2).is_some_and(|kind| std::ptr::eq(kind, document)));
    assert!(ModelAssistant::find_type(3).is_none() && ModelAssistant::find_type(5).is_none());
    assert_eq!(ModelAssistant::find_type_index(Some(document)), 2);
    assert_eq!(ModelAssistant::find_type_index(None), 0);
    ModelAssistant::clear_type_map();
    assert_eq!(ModelAssistant::find_type_index(Some(document)), 0);
    ModelAssistant::clear_model_map();
    ModelAssistant::bind_model(&fixture.my_model.clone().cast());
    assert!(ModelAssistant::find_model("ObjectModel").unwrap() == fixture.my_model);
    assert!(ModelAssistant::find_model("Other").unwrap().is_null());
    ModelAssistant::clear_model_map();
    assert!(ModelAssistant::find_model("ObjectModel").unwrap().is_null());
}

#[test]
fn the_application_is_one_instance_named_object_model() {
    let _serial = serial();
    let first = DocumentApplication::get_instance().unwrap();
    let second = DocumentApplication::get_instance().unwrap();
    assert!(first == second);
    let application = DocumentApplication::new().unwrap();
    assert_eq!(application.resources_name(), "ObjectModel");
    assert!(!application.is_verbose());
    application.set_verbose(true);
    assert!(application.is_verbose());
    let (messenger, printer) = RecordingPrinter::messenger();
    application.set_messenger(messenger);
    application.error_message_alarm(&ExtendedString::new("alarm"));
    application.error_message(&ExtendedString::new("info"), Gravity::Info);
    assert_eq!(
        printer.messages(),
        [
            ("alarm".to_owned(), Gravity::Alarm),
            ("info".to_owned(), Gravity::Info)
        ]
    );
    let mut document = Handle::default();
    assert!(
        application
            .create_new_document(&mut document, &ExtendedString::new("ObjectModelBinary"))
            .unwrap()
    );
    assert!(!document.is_null());
    assert_eq!(application.nb_documents().unwrap(), 1);
    let mut stream = DumpStream::default();
    application.object_model_dump_json(&mut stream, -1).unwrap();
    assert!(
        stream
            .text()
            .starts_with("\"className\": \"martensite_object_model_DocumentApplication\"")
    );
    application.close_tdoc_std_document(&document).unwrap();
}

#[path = "object_model_save_transactions.rs"]
mod save_transactions;
