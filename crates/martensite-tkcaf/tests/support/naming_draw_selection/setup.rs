//! Actual command label names, function lookup, attachment and ComputeFunction dispatch.
use super::super::{
    naming_box_driver::result_label,
    naming_function_graph::{add_function, add_object, object_value},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkcaf::tnaming::TNamingSelector;
use martensite_tkernel::{
    standard::{Failure, Guid, Handle},
    tcollection::ExtendedString,
};
use martensite_tklcaf::{
    guid_from_text,
    tdata_std::{TDataStdName, TDataStdTreeNode, TDataStdUAttribute},
    tdf::{TdfAttributeState, TdfLabel, TdfReference},
    tdoc_std::TDocStdDocument,
    tfunction::{TFunctionDriver, TFunctionDriverTable, TFunctionFunction, TFunctionLogbook},
};

/// Preserve command names absent from the narrower unit test setup; `name_function(&function,"Box_Function")?`.
pub fn name_function(function: &Handle<TFunctionFunction>, name: &str) -> Result<(), Failure> {
    set_name(&function.label(), name)?;
    set_name(&function.label().find_child(1, false)?, "Arguments")?;
    set_name(&result_label(function)?, "Result")
}
fn set_name(label: &TdfLabel, name: &str) -> Result<(), Failure> {
    TDataStdName::set(label, &ExtendedString::new(name))?;
    Ok(())
}
/// Native ComputeFunction passes the existing logbook unchanged; `compute_function(&function,&mut log)?`.
pub fn compute_function(
    table: &Handle<TFunctionDriverTable>,
    function: &Handle<TFunctionFunction>,
    log: &mut Handle<TFunctionLogbook>,
) -> Result<i32, Failure> {
    let mut driver: Handle<dyn TFunctionDriver> = Handle::default();
    if !table.find_driver(&function.get_driver_guid(), &mut driver, 0) {
        return Ok(1);
    }
    driver.init(&function.label());
    driver.execute(log)
}
fn move_under_context(
    object: &Handle<TDataStdUAttribute>,
    context: &Handle<TDataStdUAttribute>,
) -> Result<(), Failure> {
    let mut node: Handle<TDataStdTreeNode> = Handle::default();
    object.find_attribute_as(&TDataStdTreeNode::get_default_tree_id(), &mut node)?;
    node.remove()?;
    let mut parent: Handle<TDataStdTreeNode> = Handle::default();
    if context.find_attribute_as(&TDataStdTreeNode::get_default_tree_id(), &mut parent)? {
        parent.append(&node)?;
    }
    set_name(&object.label(), "Auxiliary_Object")
}
fn context_reference(
    function: &Handle<TFunctionFunction>,
    context: &Handle<TDataStdUAttribute>,
) -> Result<(), Failure> {
    let named = object_value(context)?;
    if named.is_null() {
        return Ok(());
    }
    let parent = named.label().father();
    let mut source: Handle<TFunctionFunction> = Handle::default();
    if parent.find_attribute_as(&TFunctionFunction::get_id(), &mut source)? {
        TdfReference::set(
            &function.label().find_child(1, false)?.find_child(1, true)?,
            &source.label(),
        )?;
    }
    Ok(())
}
/// Native selection result retains caught selection exceptions for diagnostics.
pub struct SelectionAttempt {
    pub object: Handle<TDataStdUAttribute>,
    pub selected: bool,
    pub exception: Option<Failure>,
}
impl SelectionAttempt {
    fn new(object: Handle<TDataStdUAttribute>, selected: bool, exception: Option<Failure>) -> Self {
        Self {
            object,
            selected,
            exception,
        }
    }
}
fn prepare_attachment(
    document: &Handle<TDocStdDocument>,
    context: &Handle<TDataStdUAttribute>,
) -> Result<(Handle<TDataStdUAttribute>, Handle<TFunctionFunction>), Failure> {
    let object = add_object(document)?;
    move_under_context(&object, context)?;
    let function = add_function(
        &object,
        &guid_from_text("12e94554-6dbc-11d4-b9c8-0060b0ee281b"),
    )?;
    name_function(&function, "ISelection")?;
    result_label(&function)?.forget_all_attributes(true)?;
    Ok((object, function))
}
/// Actual MakeSelection returns selection failure separately; `make_selection(&doc,&context,&shape,true)?`.
pub fn make_selection(
    document: &Handle<TDocStdDocument>,
    context: &Handle<TDataStdUAttribute>,
    shape: &Shape,
    keep: bool,
) -> Result<SelectionAttempt, Failure> {
    let (object, function) = prepare_attachment(document, context)?;
    let result = result_label(&function)?;
    let source = object_value(context)?.get().unwrap().get()?;
    let selected = TNamingSelector::new(&result).select(shape, &source, false, keep);
    if matches!(selected, Ok(false)) {
        return Ok(SelectionAttempt::new(object, false, None));
    }
    context_reference(&function, context)?;
    Ok(SelectionAttempt::new(object, true, selected.err()))
}
/// Register the actual selection class for either supported command GUID; `register_selection_driver(&id)`.
pub fn register_selection_driver(table: &Handle<TFunctionDriverTable>, guid: &Guid) {
    let driver: Handle<dyn TFunctionDriver> =
        Handle::new(super::super::naming_selection_driver::NamingSelectionDriver::default()).cast();
    table.add_driver(guid, &driver, 0);
}
