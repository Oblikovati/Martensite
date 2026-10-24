//! The model behavior of the `martensite_object_model_DocumentModel` members that load, save and close the document.
//! A model saves and loads through the storage and retrieval drivers of its application's
//! driver `SaveAs` and `Load` report the driver failure of `martensite_object_model_DocumentApplication`.
use super::messages::{message_text, message_with};
use super::{
    DocumentApplication, DocumentApplicationApi, DocumentModel, DocumentModelState, ModelAssistant,
    ModelAttribute,
};
use martensite_tkernel::{
    message::Gravity,
    standard::{Failure, Handle},
    tcollection::{ExtendedString, HExtendedString},
};
use martensite_tklcaf::{tdf::TdfLabel, tdoc_std::TDocStdDocument};
use std::io;

/// The body of `martensite_object_model_DocumentModel::CloseDocument` with the application to close in: aborts an
/// open command, forgets every attribute and closes the document.
pub(crate) fn close_document_with(
    document: &Handle<TDocStdDocument>,
    application: &Handle<dyn DocumentApplicationApi>,
) -> Result<(), Failure> {
    let live = martensite_tkcdf::live(
        document,
        "martensite_object_model_DocumentModel::CloseDocument",
    )?;
    // An open command would be aborted at document destruction.
    if live.has_open_command() {
        live.abort_command()?;
    }
    live.main()?.root()?.forget_all_attributes(true)?;
    martensite_tkcdf::live(
        application,
        "martensite_object_model_DocumentModel::CloseDocument",
    )?
    .close_tdoc_std_document(document)
}

/// The base `martensite_object_model_DocumentModel::Close` on a label with the static application (what the runtime
/// destructor runs); false without document.
pub(crate) fn close_label(label: &TdfLabel) -> Result<bool, Failure> {
    if label.is_null() {
        return Ok(false);
    }
    let document = TDocStdDocument::get(label)?;
    if document.is_null() {
        return Ok(false);
    }
    close_document_with(&document, &DocumentApplication::get_instance()?)?;
    Ok(true)
}

/// The body of `martensite_object_model_DocumentModel::checkDocumentEmpty`: an empty name, a missing file or a file
/// of zero length; false when the file cannot be opened.
pub(crate) fn check_document_empty(file: &ExtendedString) -> bool {
    if file.is_empty() {
        return true;
    }
    let path = file.to_utf8();
    if std::fs::metadata(&path).is_err() {
        return true;
    }
    match std::fs::File::open(&path) {
        Ok(opened) => opened.metadata().is_ok_and(|metadata| metadata.len() == 0),
        Err(_) => false,
    }
}

/// `Put model in a new attribute on root label`: a `martensite_object_model_ModelAttribute` on the main label set to
/// the model, whose label it becomes.
fn attach_to_new_document<M: DocumentModel + ?Sized>(
    model: &M,
    document: &Handle<TDocStdDocument>,
) -> Result<(), Failure> {
    let label =
        martensite_tkcdf::live(document, "martensite_object_model_DocumentModel::Load")?.main()?;
    let attribute = Handle::new(ModelAttribute::new());
    label.add_attribute(&attribute.cast(), true)?;
    attribute.set(&model.model_base().this()?)?;
    model.set_label(&label);
    Ok(())
}

/// True when the read document holds this model (`aLabel.Data() == aDoc->GetData()`).
fn holds_model<M: DocumentModel + ?Sized>(model: &M, document: &Handle<TDocStdDocument>) -> bool {
    let label = model.get_label();
    match document.get() {
        Some(document) => !label.is_null() && label.data() == document.get_data(),
        None => false,
    }
}

/// Releases a document from the session (when there is one) and the model's label.
fn release<M: DocumentModel + ?Sized>(
    model: &M,
    document: &Handle<TDocStdDocument>,
) -> Result<(), Failure> {
    if !document.is_null() {
        model.close_document(document)?;
    }
    model.set_label(&TdfLabel::default());
    Ok(())
}

/// Sends `ObjectModel_M_WrongFile` (with the file name when there is one) as an alarm.
fn send_wrong_file<M: DocumentModel + ?Sized>(
    model: &M,
    file: Option<&ExtendedString>,
) -> Result<(), Failure> {
    let text = match file {
        Some(file) => message_with("ObjectModel_M_WrongFile", file)?,
        None => message_text("ObjectModel_M_WrongFile")?,
    };
    model.messenger().send_extended(&text, Gravity::Alarm);
    Ok(())
}

/// The common tail of both `Load`: `initNewModel` (a caught failure is a wrong file),
/// then the document is released when it failed.
fn initialize_loaded<M: DocumentModel + ?Sized>(
    model: &M,
    document: &Handle<TDocStdDocument>,
    is_new: bool,
    file: Option<&ExtendedString>,
) -> Result<bool, Failure> {
    let is_init_ok = match model.init_new_model(is_new) {
        Ok(is_init_ok) => is_init_ok,
        Err(_) => {
            send_wrong_file(model, file)?;
            false
        }
    };
    if !is_init_ok {
        release(model, document)?;
    }
    Ok(is_init_ok)
}

/// Sets this model as the current one of the assistant and clears the type map.
fn begin_persistence<M: DocumentModel + ?Sized>(model: &M) -> Result<(), Failure> {
    ModelAssistant::set_current_model(&model.model_base().this()?);
    ModelAssistant::clear_type_map();
    Ok(())
}

/// Unsets the current model of the assistant and clears the type map.
fn end_persistence() {
    ModelAssistant::unset_current_model();
    ModelAssistant::clear_type_map();
}

/// The body of `martensite_object_model_DocumentModel::Load(theFile)`.
pub(crate) fn load_file<M: DocumentModel + ?Sized>(
    model: &M,
    file: &ExtendedString,
) -> Result<bool, Failure> {
    let application = model.get_application()?;
    let application =
        martensite_tkcdf::live(&application, "martensite_object_model_DocumentModel::Load")?;
    begin_persistence(model)?;
    let mut document = Handle::default();
    let is_file_empty = model.check_document_empty(file);
    let mut status = if is_file_empty {
        let created = application.create_new_document(&mut document, &model.get_format())?;
        if created {
            attach_to_new_document(model, &document)?;
        }
        created
    } else {
        read_file(model, application, file, &mut document)?
    };
    if status {
        status = initialize_loaded(model, &document, is_file_empty, Some(file))?;
    }
    end_persistence();
    Ok(status)
}

/// Retrieves the document of a file; a document without this model is a wrong file.
fn read_file<M: DocumentModel + ?Sized>(
    model: &M,
    application: &dyn DocumentApplicationApi,
    file: &ExtendedString,
    document: &mut Handle<TDocStdDocument>,
) -> Result<bool, Failure> {
    let loading = message_with("ObjectModel_M_LoadDocument", file)?;
    model.messenger().send_extended(&loading, Gravity::Info);
    if !application.load_document(file, document)? {
        // No message: LoadDocument has reported the error.
        release(model, document)?;
        return Ok(false);
    }
    if holds_model(model, document) {
        return Ok(true);
    }
    release(model, document)?;
    send_wrong_file(model, Some(file))?;
    Ok(false)
}

/// The body of `martensite_object_model_DocumentModel::Load(theIStream)`: a stream that cannot be read gives an
/// empty model.
pub(crate) fn load_stream<M: DocumentModel + ?Sized>(
    model: &M,
    stream: &mut dyn io::Read,
) -> Result<bool, Failure> {
    let application = model.get_application()?;
    let application =
        martensite_tkcdf::live(&application, "martensite_object_model_DocumentModel::Load")?;
    begin_persistence(model)?;
    let mut document = Handle::default();
    model
        .messenger()
        .send_extended(&message_text("ObjectModel_M_LoadDocument")?, Gravity::Info);
    let (mut status, is_file_loaded) = if application.load_document_stream(stream, &mut document)? {
        let is_valid = holds_model(model, &document);
        if !is_valid {
            release(model, &document)?;
            send_wrong_file(model, None)?;
        }
        (is_valid, is_valid)
    } else {
        release(model, &document)?;
        let created = application.create_new_document(&mut document, &model.get_format())?;
        if created {
            attach_to_new_document(model, &document)?;
        }
        (created, false)
    };
    if status {
        status = initialize_loaded(model, &document, !is_file_loaded, None)?;
    }
    end_persistence();
    Ok(status)
}

/// Calls `BeforeStoring` on every object with the document's transaction mode off.
fn store_transient_fields<M: DocumentModel + ?Sized>(
    model: &M,
    document: &TDocStdDocument,
) -> Result<(), Failure> {
    let transaction_mode = document.modification_mode();
    document.set_modification_mode(false)?;
    let result = prepare_objects(model);
    let restored = document.set_modification_mode(transaction_mode);
    result.and(restored)
}

fn prepare_objects<M: DocumentModel + ?Sized>(model: &M) -> Result<(), Failure> {
    let objects = model.get_objects()?;
    let objects =
        martensite_tkcdf::live(&objects, "martensite_object_model_DocumentModel::SaveAs")?;
    while objects.more() {
        if let Some(object) = objects.value().get() {
            object.before_storing()?;
        }
        objects.next()?;
    }
    Ok(())
}

/// Saves without altering the destination before preparation and storage succeed (#245).
pub(crate) fn save_as_file<M: DocumentModel + ?Sized>(
    model: &M,
    file: &ExtendedString,
) -> Result<bool, Failure> {
    ModelAssistant::clear_type_map();
    let document = TDocStdDocument::get(&model.get_label())?;
    let Some(live) = document.get() else {
        return Ok(false);
    };
    if !destination_writable(file) {
        let text = message_with("ObjectModel_M_NoWriteAccess", file)?;
        model.messenger().send_extended(&text, Gravity::Alarm);
        return Ok(false);
    }
    store_transient_fields(model, live)?;
    let application = model.get_application()?;
    let status = martensite_tkcdf::live(
        &application,
        "martensite_object_model_DocumentModel::SaveAs",
    )?
    .save_document(&document, file)?;
    ModelAssistant::clear_type_map();
    Ok(status)
}

fn destination_writable(file: &ExtendedString) -> bool {
    let path = std::path::PathBuf::from(file.to_utf8());
    match std::fs::OpenOptions::new().write(true).open(&path) {
        Ok(_) => true,
        Err(error) if error.kind() == io::ErrorKind::NotFound => path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or_else(|| std::path::Path::new("."))
            .is_dir(),
        Err(_) => false,
    }
}

/// The body of `martensite_object_model_DocumentModel::SaveAs(theOStream)`.
pub(crate) fn save_as_stream<M: DocumentModel + ?Sized>(
    model: &M,
    stream: &mut dyn io::Write,
) -> Result<bool, Failure> {
    ModelAssistant::clear_type_map();
    let document = TDocStdDocument::get(&model.get_label())?;
    let Some(live) = document.get() else {
        return Ok(false);
    };
    store_transient_fields(model, live)?;
    let application = model.get_application()?;
    let status = martensite_tkcdf::live(
        &application,
        "martensite_object_model_DocumentModel::SaveAs",
    )?
    .save_document_stream(&document, stream)?;
    ModelAssistant::clear_type_map();
    Ok(status)
}

/// The body of `martensite_object_model_DocumentModel::GetFile`: the path of a stored document, null otherwise.
pub(crate) fn model_file<M: DocumentModel + ?Sized>(
    model: &M,
) -> Result<Handle<HExtendedString>, Failure> {
    let document = model.get_document()?;
    let Some(document) = document.get().filter(|document| document.is_saved()) else {
        return Ok(Handle::default());
    };
    let path = document.get_path()?;
    if path.is_empty() {
        return Ok(Handle::default());
    }
    Ok(Handle::new(HExtendedString::from(path)))
}
