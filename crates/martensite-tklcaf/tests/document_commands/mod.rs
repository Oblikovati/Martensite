//! The document commands of the regression scenarios, as calls of the Rust API.
//!
//! Each function names the scenario command it stands for and does what that command does;
//! a command that fails returns `Err`, the error the scenario checks.
#![allow(dead_code)] // Each case file uses the commands it needs.
use martensite_tkernel::standard::{Failure, Guid, Handle};
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tklcaf::tdata_std::{TDataStdInteger, TDataStdName, TDataStdReal};
use martensite_tklcaf::tdf::{TdfAttribute, TdfAttributeIterator, TdfLabel, TdfTool};
use martensite_tklcaf::tdoc_std::TDocStdDocument;

/// `NewDocument D`: a document not handled by the application (`new TDocStd_Document("dummy")`).
pub fn new_document() -> Handle<TDocStdDocument> {
    TDocStdDocument::new(&ExtendedString::new("dummy")).expect("dummy document")
}

/// `NewDocument D format`: `A->NewDocument(format, D)` then `TDataStd_Name::Set(root, D)`;
/// a new document of the application never stores here, so its storage format is the only
/// trace of `format`.
pub fn new_named_document(name: &str, format: &str) -> Handle<TDocStdDocument> {
    let document = TDocStdDocument::new(&ExtendedString::new(format)).expect("document");
    let root = document.get_data().get().expect("data").root();
    TDataStdName::set(&root, &ExtendedString::new(name)).expect("root name");
    document
}

/// `DDF::AddLabel` (`Label D entry`): the label of an entry, created when missing.
pub fn add_label(document: &Handle<TDocStdDocument>, entry: &str) -> TdfLabel {
    let mut label = TdfLabel::default();
    TdfTool::label_from_c_entry(&document.get_data(), entry.as_bytes(), &mut label, true)
        .expect("label entry");
    label
}

/// `DDF::FindLabel`: the existing label of an entry; an error for a missing one.
pub fn find_label(document: &Handle<TDocStdDocument>, entry: &str) -> Result<TdfLabel, String> {
    let mut label = TdfLabel::default();
    TdfTool::label_from_c_entry(&document.get_data(), entry.as_bytes(), &mut label, false)
        .map_err(|failure| failure.message().to_owned())?;
    if label.is_null() {
        return Err(format!("label {entry} not found"));
    }
    Ok(label)
}

/// `SetReal D entry value`.
pub fn set_real(document: &Handle<TDocStdDocument>, entry: &str, value: f64) {
    TDataStdReal::set(&add_label(document, entry), value).expect("SetReal");
}

/// `SetInteger D entry value`.
pub fn set_integer(document: &Handle<TDocStdDocument>, entry: &str, value: i32) {
    TDataStdInteger::set(&add_label(document, entry), value).expect("SetInteger");
}

/// The attribute of `id` on an existing label, the error of a Get command otherwise
/// (`attribute not found for entry : <entry>`).
fn find(
    document: &Handle<TDocStdDocument>,
    entry: &str,
    id: &Guid,
) -> Result<Handle<dyn TdfAttribute>, String> {
    let label = find_label(document, entry)?;
    let mut attribute = Handle::default();
    if !label
        .find_attribute(id, &mut attribute)
        .map_err(|f: Failure| f.message().to_owned())?
    {
        return Err(format!("attribute not found for entry : {entry}"));
    }
    Ok(attribute)
}

/// `GetReal D entry`.
pub fn get_real(document: &Handle<TDocStdDocument>, entry: &str) -> Result<f64, String> {
    let attribute = find(document, entry, &TDataStdReal::get_id())?;
    Ok(attribute
        .cast::<TDataStdReal>()
        .get()
        .expect("TDataStd_Real")
        .get())
}

/// `GetInteger D entry`.
pub fn get_integer(document: &Handle<TDocStdDocument>, entry: &str) -> Result<i32, String> {
    let attribute = find(document, entry, &TDataStdInteger::get_id())?;
    Ok(attribute
        .cast::<TDataStdInteger>()
        .get()
        .expect("TDataStd_Integer")
        .get())
}

/// `Attributes D entry`: the dynamic type names of the label's attributes.
pub fn attributes(document: &Handle<TDocStdDocument>, entry: &str) -> Vec<String> {
    let label = find_label(document, entry).expect("Attributes label");
    let mut iterator = TdfAttributeIterator::new(&label, true).expect("iterator");
    let mut names = Vec::new();
    while iterator.more() {
        if let Some(descriptor) = iterator.value().dynamic_type() {
            names.push(descriptor.name().to_owned());
        }
        iterator.next();
    }
    names
}

/// `ForgetAll D entry`: `ForgetAllAttributes()` (children included).
pub fn forget_all(document: &Handle<TDocStdDocument>, entry: &str) {
    find_label(document, entry)
        .expect("ForgetAll label")
        .forget_all_attributes(true)
        .expect("ForgetAll");
}

/// `ForgetAtt D entry guid`.
pub fn forget_attribute(document: &Handle<TDocStdDocument>, entry: &str, id: &Guid) {
    find_label(document, entry)
        .expect("ForgetAtt label")
        .forget_attribute_id(id)
        .expect("ForgetAtt");
}

/// The entry of a label (`Label D entry` returns it).
pub fn entry(label: &TdfLabel) -> String {
    let mut text = martensite_tkernel::tcollection::AsciiString::default();
    TdfTool::entry(label, &mut text);
    String::from_utf8_lossy(text.as_bytes()).into_owned()
}

/// The word list of a command result: the words between white space, a `"..."` word without
/// its quotes (the dumps quote only space-free addresses).
pub fn word_list(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|word| word.trim_matches('"').to_owned())
        .collect()
}

/// The first index of `word` in `list`, -1 when absent.
pub fn word_index(list: &[String], word: &str) -> i64 {
    list.iter()
        .position(|item| item == word)
        .map_or(-1, |index| index as i64)
}

/// The word at `index` in `list`, empty out of range.
pub fn word_at(list: &[String], index: i64) -> String {
    usize::try_from(index)
        .ok()
        .and_then(|index| list.get(index))
        .cloned()
        .unwrap_or_default()
}
