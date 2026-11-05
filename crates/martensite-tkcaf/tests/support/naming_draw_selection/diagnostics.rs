//! Original selection diagnostics and failed-shape persistence, without suppressing failures.
use super::super::naming_function_graph::object_value;
use super::shapes;
use martensite_tkbrep::topods::Shape;
use martensite_tkcaf::tnaming::{TNamingBuilder, TNamingNameType, TNamingNaming};
use martensite_tkernel::{
    ncollection::Map,
    standard::{Failure, Handle},
    tcollection::ExtendedString,
};
use martensite_tkg3d::top_abs::{ShapeEnum, shape_type_to_string};
use martensite_tklcaf::{
    tdata_std::{TDataStdName, TDataStdUAttribute},
    tdf::{TdfAttributeState, TdfChildIdIterator, TdfTagSource, TdfTool},
    tdoc_std::TDocStdDocument,
};
fn entry(object: &Handle<TDataStdUAttribute>) -> String {
    let mut text = martensite_tkernel::tcollection::AsciiString::default();
    TdfTool::entry(&object.label(), &mut text);
    String::from_utf8_lossy(text.as_bytes()).into_owned()
}
fn unknown_names(object: &Handle<TDataStdUAttribute>, kind: ShapeEnum) -> Result<String, Failure> {
    let mut result = String::new();
    let mut names = TdfChildIdIterator::new(&object.label(), &TNamingNaming::get_id(), true)?;
    while names.more() {
        let naming = names.value().cast::<TNamingNaming>();
        if !naming.is_null() && naming.get_name().name_type() == TNamingNameType::Unknown {
            result.push_str(&format!(
                " Selection at label = {} has UNKNOWN name type, shape type = {}",
                entry(object),
                shape_type_to_string(kind)
            ));
        }
        names.next()?;
    }
    Ok(result)
}
/// Native initial-selection comparison and UNKNOWN-name checks; `selected_diagnostic(...)`.
pub(super) fn selected_diagnostic(
    object: &Handle<TDataStdUAttribute>,
    original: &Shape,
    keep: bool,
    context: Option<&Map<Shape>>,
) -> Result<String, Failure> {
    let named = object_value(object)?;
    if named.is_null() || named.is_empty() {
        return Ok(String::new());
    }
    let selected = named.get().unwrap().get()?;
    let mut message = match context {
        Some(shapes) => shapes::compare_multiple(&selected, shapes)?,
        None => shapes::compare_single(original, &selected, keep)?,
    };
    message.push_str(&unknown_names(object, original.shape_type()?)?);
    Ok(message)
}
/// Actual failed-selection diagnostic includes label and source shape type; `failed_diagnostic(...)`.
pub(super) fn failed_diagnostic(
    object: &Handle<TDataStdUAttribute>,
    original: &Shape,
) -> Result<String, Failure> {
    Ok(format!(
        " Selection at label = {} failed, shape type = {}",
        entry(object),
        shape_type_to_string(original.shape_type()?)
    ))
}
/// Single selection stores its diagnostic name; multiple selection only emits it; `store_diagnostic(...)`.
pub(super) fn store_diagnostic(
    object: &Handle<TDataStdUAttribute>,
    message: &str,
    multiple: bool,
) -> Result<(), Failure> {
    if multiple || message.is_empty() {
        return Ok(());
    }
    TDataStdName::set(&object.label(), &ExtendedString::new(message))?;
    Ok(())
}
/// Failed shapes are generated at fresh document-main children; `persist_failed(&document,&shapes)?`.
pub(super) fn persist_failed(
    document: &Handle<TDocStdDocument>,
    failed: &[Shape],
) -> Result<(), Failure> {
    for shape in failed {
        TNamingBuilder::new(&TdfTagSource::new_child_of(&document.main()?)?)?.generated(shape)?;
    }
    Ok(())
}

/// Keep source single/multiple prefixes and selection entry; `rendered_diagnostic(" Warning", "0:1:2", false)`.
pub(in super::super) fn rendered_diagnostic(message: &str, label: &str, multiple: bool) -> String {
    if multiple {
        return format!("Failed units: {message}");
    }
    if message.contains("Warning") {
        return format!("{message} at {label}");
    }
    format!("Failed units: {message} at {label}")
}
/// Emit actual command text without changing the stored diagnostic; `publish_diagnostic(&object, text, false)`.
pub(super) fn publish_diagnostic(
    object: &Handle<TDataStdUAttribute>,
    message: &str,
    multiple: bool,
) {
    if message.is_empty() {
        return;
    }
    println!("{}", rendered_diagnostic(message, &entry(object), multiple));
}

/// Retain the same typed-exception prefix in both modes and exact what bytes; `rendered_selection_failure(&failure)`.
pub(in super::super) fn rendered_selection_failure(exception: &Failure) -> Vec<u8> {
    let mut report = b"%%%INFO:Error: ::TestSingleSelection failed :".to_vec();
    report.extend_from_slice(exception.what());
    report.push(b'\n');
    report
}
