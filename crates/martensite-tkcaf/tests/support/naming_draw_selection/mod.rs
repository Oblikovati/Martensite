//! Actual native TestSingleSelection/TestMultipleSelection and SolveFlatFrom command fixtures.
pub(super) mod diagnostics;
pub(super) mod setup;
mod shapes;
use super::naming_function_graph::object_value;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    ncollection::Map,
    standard::{Failure, Handle},
};
use martensite_tklcaf::{
    tdata_std::TDataStdUAttribute,
    tdf::{TdfAttributeState, TdfChildIterator, TdfLabel, TdfTool},
    tdoc_std::TDocStdDocument,
    tfunction::{TFunctionDriverTable, TFunctionFunction, TFunctionLogbook},
};

/// Every attempted occurrence and native diagnostic is retained; `test_selection(&doc,&object,true,false)?`.
#[derive(Default)]
pub(super) struct SelectionBatch {
    pub first: Handle<TDataStdUAttribute>,
    pub attempted: usize,
    pub diagnostics: Vec<String>,
    pub failed_shapes: Vec<Shape>,
}
fn requested_shapes(root: &Shape, multiple: bool) -> Result<Vec<Shape>, Failure> {
    if multiple {
        return shapes::multiple_shapes(root);
    }
    Ok(shapes::single_shapes(root))
}
fn attempt(
    batch: &mut SelectionBatch,
    document: &Handle<TDocStdDocument>,
    object: &Handle<TDataStdUAttribute>,
    shape: &Shape,
    keep: bool,
    context: Option<&Map<Shape>>,
) -> Result<(), Failure> {
    let selection = setup::make_selection(document, object, shape, keep)?;
    record_attempt(batch, &selection);
    let message = attempt_diagnostic(batch, &selection, shape, keep, context)?;
    diagnostics::publish_diagnostic(&selection.object, &message, context.is_some());
    diagnostics::store_diagnostic(&selection.object, &message, context.is_some())?;
    record_diagnostic(batch, message);
    Ok(())
}
fn record_attempt(batch: &mut SelectionBatch, selection: &setup::SelectionAttempt) {
    if batch.first.is_null() {
        batch.first = selection.object.clone();
    }
    batch.attempted += 1;
    if let Some(exception) = &selection.exception {
        use std::io::Write;
        let report = diagnostics::rendered_selection_failure(exception);
        std::io::stdout().write_all(&report).unwrap();
        record_diagnostic(
            batch,
            format!("EXCEPTION: SELECTION_IMPOSSIBLE: {exception}"),
        );
    }
}
fn record_diagnostic(batch: &mut SelectionBatch, message: String) {
    if message.is_empty() {
        return;
    }
    batch.diagnostics.push(message);
}
fn attempt_diagnostic(
    batch: &mut SelectionBatch,
    selection: &setup::SelectionAttempt,
    shape: &Shape,
    keep: bool,
    context: Option<&Map<Shape>>,
) -> Result<String, Failure> {
    if selection.selected {
        return diagnostics::selected_diagnostic(&selection.object, shape, keep, context);
    }
    batch.failed_shapes.push(shape.clone());
    diagnostics::failed_diagnostic(&selection.object, shape)
}
/// Exact default Geometry=false/Xselection=false commands; `test_selection(&doc,&object,true,false)?`.
pub(super) fn test_selection(
    document: &Handle<TDocStdDocument>,
    object: &Handle<TDataStdUAttribute>,
    keep: bool,
    multiple: bool,
) -> Result<SelectionBatch, Failure> {
    let root = selection_root(object)?;
    let context = multiple.then(|| shapes::oriented_context(&root));
    let mut batch = SelectionBatch::default();
    for shape in requested_shapes(&root, multiple)? {
        if shapes::eligible(&shape)? {
            attempt(&mut batch, document, object, &shape, keep, context.as_ref())?;
        }
    }
    diagnostics::persist_failed(document, &batch.failed_shapes)?;
    Ok(batch)
}
fn selection_root(object: &Handle<TDataStdUAttribute>) -> Result<Shape, Failure> {
    let named = object_value(object)?;
    if named.is_null() || named.is_empty() {
        return Err(Failure::new(
            "selection object value is empty; expected nonempty named shape",
        ));
    }
    named.get().unwrap().get()
}
fn solve_auxiliary(
    table: &Handle<TFunctionDriverTable>,
    label: &martensite_tklcaf::tdf::TdfLabel,
    log: &mut Handle<TFunctionLogbook>,
) -> Result<Option<i32>, Failure> {
    let function_label = label.find_child(1, true)?;
    let mut function: Handle<TFunctionFunction> = Handle::default();
    if !function_label.find_attribute_as(&TFunctionFunction::get_id(), &mut function)? {
        return Ok(None);
    }
    log.clear()?;
    setup::compute_function(table, &function, log).map(Some)
}
fn selection_father(
    first: &Handle<TDataStdUAttribute>,
) -> Result<martensite_tklcaf::tdf::TdfLabel, Failure> {
    let father = first.label().father();
    if father.is_null() {
        return Err(Failure::new(
            "first auxiliary has null father; expected document-main parent",
        ));
    }
    Ok(father)
}
fn following_siblings(
    first: &Handle<TDataStdUAttribute>,
) -> Result<Vec<martensite_tklcaf::tdf::TdfLabel>, Failure> {
    let father = selection_father(first)?;
    let mut children = TdfChildIterator::new(&father, false)?;
    let mut found = false;
    let mut labels = Vec::new();
    while children.more() {
        let label = children.value();
        children.next();
        found |= label == first.label();
        if found {
            labels.push(label);
        }
    }
    Ok(labels)
}
/// Actual sibling traversal starts at first auxiliary and clears each logbook; `solve_flat_from(&first)?`.
pub(super) fn solve_flat_from(
    table: &Handle<TFunctionDriverTable>,
    first: &Handle<TDataStdUAttribute>,
) -> Result<Vec<String>, Failure> {
    let mut log = TFunctionLogbook::set(&first.label().father())?;
    let mut diagnostics = Vec::new();
    for label in following_siblings(first)? {
        solve_case_step(table, &label, &mut log, &mut diagnostics)?;
    }
    Ok(diagnostics)
}

fn record_solve_report(reports: &mut Vec<String>, message: String) {
    println!("{message}");
    reports.push(message);
}

fn function_entry(label: &TdfLabel) -> Result<String, Failure> {
    let function = label.find_child(1, true)?;
    let mut entry = martensite_tkernel::tcollection::AsciiString::default();
    TdfTool::entry(&function, &mut entry);
    Ok(String::from_utf8_lossy(entry.as_bytes()).into_owned())
}
const NULL_SOLVE_REPORT: &str = "DNamingSolveFlatFrom:: Null function is found!";
fn exception_solve_report(label: &TdfLabel) -> Result<String, Failure> {
    let entry = function_entry(label)?;
    Ok(format!(
        "DNamingSolveFlatFrom : Exception computing function at label {entry}"
    ))
}
fn solve_case_step(
    table: &Handle<TFunctionDriverTable>,
    label: &TdfLabel,
    log: &mut Handle<TFunctionLogbook>,
    reports: &mut Vec<String>,
) -> Result<(), Failure> {
    match solve_auxiliary(table, label, log) {
        Ok(None) => record_solve_report(reports, NULL_SOLVE_REPORT.into()),
        Ok(Some(0)) => (),
        Ok(Some(code)) => return solve_driver_failure(label, code),
        Err(_) => record_solve_report(reports, exception_solve_report(label)?),
    }
    Ok(())
}
fn solve_driver_failure(label: &TdfLabel, code: i32) -> Result<(), Failure> {
    let entry = function_entry(label)?;
    println!("DNamingSolveFlatFrom: Driver failed at label = {entry}");
    Err(Failure::new(&format!(
        "DNamingSolveFlatFrom driver returned {code}; expected 0 at label {label:?}"
    )))
}
