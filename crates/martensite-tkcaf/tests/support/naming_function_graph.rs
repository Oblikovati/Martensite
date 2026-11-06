//! Actual object/function/selection setup from martensite-tkcaf TNamingTool tests.
use super::naming_function_values as values;
use super::naming_selection_driver::NamingSelectionDriver;
use super::support::naming::fill_selection_valid_map;
use martensite_tkbrep::topods::Shape;
use martensite_tkcaf::tnaming::{TNamingNamedShape, TNamingSelector, TNamingTool};
use martensite_tkernel::standard::{Failure, Guid, Handle};
use martensite_tklcaf::{
    guid_from_text,
    tdata_std::{TDataStdTreeNode, TDataStdUAttribute},
    tdf::{
        TdfAttribute, TdfAttributeState, TdfChildIterator, TdfLabel, TdfLabelMap, TdfReference,
        TdfTagSource,
    },
    tdoc_std::TDocStdDocument,
    tfunction::{TFunctionDriver, TFunctionFunction, TFunctionLogbook},
};

/// Add the actual named object and tree node; `add_object(&document)?`.
pub fn add_object(
    document: &Handle<TDocStdDocument>,
) -> Result<Handle<TDataStdUAttribute>, Failure> {
    let main = document.main()?;
    let root = TDataStdTreeNode::set(&main)?;
    let label = TdfTagSource::new_child_of(&main)?;
    let object = TDataStdUAttribute::set(
        &label,
        &guid_from_text("6c6915ab-775f-4475-859e-befd74d26a23"),
    )?;
    root.append(&TDataStdTreeNode::set(&label)?)?;
    Ok(object)
}

fn node_at(label: &TdfLabel) -> Result<Handle<TDataStdTreeNode>, Failure> {
    let mut node = Handle::default();
    label.find_attribute_as(&TDataStdTreeNode::get_default_tree_id(), &mut node)?;
    Ok(node)
}

fn append_function_child(
    label: &TdfLabel,
    node: &Handle<TDataStdTreeNode>,
) -> Result<TdfLabel, Failure> {
    let child = TdfTagSource::new_child_of(label)?;
    node.append(&TDataStdTreeNode::set(&child)?)?;
    Ok(child)
}

/// Append the real argument/result node layout; `add_function(&object, &guid)?`.
pub fn add_function(
    object: &Handle<TDataStdUAttribute>,
    guid: &Guid,
) -> Result<Handle<TFunctionFunction>, Failure> {
    let label = TdfTagSource::new_child_of(&object.label())?;
    let function = TFunctionFunction::set_with_driver(&label, guid)?;
    let node = TDataStdTreeNode::set(&label)?;
    node_at(&object.label())?.append(&node)?;
    append_function_child(&label, &node)?;
    let result = append_function_child(&label, &node)?;
    TdfReference::set(&object.label(), &result)?;
    Ok(function)
}

pub(super) fn object_value(
    object: &Handle<TDataStdUAttribute>,
) -> Result<Handle<TNamingNamedShape>, Failure> {
    if object.is_null() || object.id() != guid_from_text("6c6915ab-775f-4475-859e-befd74d26a23") {
        return Ok(Handle::default());
    }
    let mut reference: Handle<TdfReference> = Handle::default();
    let mut named = Handle::default();
    if object
        .label()
        .find_attribute_as(&TdfReference::get_id(), &mut reference)?
    {
        reference
            .get()
            .unwrap()
            .get()
            .find_attribute_as(&TNamingNamedShape::get_id(), &mut named)?;
    }
    Ok(named)
}

fn name_selection(
    function: &Handle<TFunctionFunction>,
    context: &Handle<TNamingNamedShape>,
    selection: &Shape,
    keep: bool,
) -> Result<(), Failure> {
    if context.is_null() || context.is_empty() {
        return Ok(());
    }
    let selector = TNamingSelector::new(&values::result_label(function)?);
    if !selector.select(selection, &context.get().unwrap().get()?, false, keep)? {
        return Ok(());
    }
    let argument = function.label().find_child(1, true)?.find_child(1, true)?;
    TdfReference::set(&argument, &context.label().father())?;
    Ok(())
}

/// Add an attached selection with the actual object tree; `add_selection(&doc, &object, &face, true)?`.
pub fn add_selection(
    document: &Handle<TDocStdDocument>,
    context: &Handle<TDataStdUAttribute>,
    selection: &Shape,
    keep: bool,
) -> Result<(Handle<TDataStdUAttribute>, Handle<TFunctionFunction>), Failure> {
    let object = add_object(document)?;
    let node = node_at(&object.label())?;
    node.remove()?;
    node_at(&context.label())?.append(&node)?;
    let function = add_function(
        &object,
        &guid_from_text("12e94554-6dbc-11d4-b9c8-0060b0ee281b"),
    )?;
    name_selection(&function, &object_value(context)?, selection, keep)?;
    Ok((object, function))
}

fn all_named_labels(root: &TdfLabel, valid: &mut TdfLabelMap) -> Result<(), Failure> {
    let mut children = TdfChildIterator::new(root, true)?;
    while children.more() {
        let label = children.value();
        let mut named: Handle<TNamingNamedShape> = Handle::default();
        if label.find_attribute_as(&TNamingNamedShape::get_id(), &mut named)? {
            valid.add(label);
        }
        children.next();
    }
    Ok(())
}

/// Execute the real selection driver with the original valid-label setup; `solve_selection(&f, &driver)?`.
pub fn solve_selection(
    function: &Handle<TFunctionFunction>,
    driver: &NamingSelectionDriver,
) -> Result<Shape, Failure> {
    let result = values::result_label(function)?;
    let mut valid = TdfLabelMap::default();
    fill_selection_valid_map(&result, &mut valid);
    all_named_labels(&function.label().root()?, &mut valid)?;
    let mut log = TFunctionLogbook::set(&function.label())?;
    log.clear()?;
    log.set_valid_labels(&valid)?;
    driver.init(&function.label());
    if driver.execute(&mut log)? != 0 {
        return Ok(Shape::default());
    }
    TNamingTool::current_shape(&values::function_result(function)?)
}
