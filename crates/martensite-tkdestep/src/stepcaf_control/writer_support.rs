//! What the parts of `STEPCAFControlWriter` share.
use super::writer::WHERE;
use crate::step_construct::StepConstruct;
use crate::step_entity::step_live;
use crate::step_repr::StepReprRepresentationItem;
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkernel::tcollection::{AsciiString, HAsciiString};
use martensite_tklcaf::tdata_std::{TDataStdGenericExtString, TDataStdName};
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkmath::toploc::Location;
use martensite_tkxsbase::interface::InterfaceInterfaceModel;
use martensite_tkxsbase::transfer::{TransferFinderProcess, TransferTransientListBinder};
use martensite_tkxsbase::transfer_brep::TransferBRep;
use martensite_tkxsbase::xs_control::XSControlWorkSession;

/// The model of a session.
pub(super) fn model_of(
    ws: &Handle<XSControlWorkSession>,
) -> Result<Handle<dyn InterfaceInterfaceModel>, Failure> {
    Ok(step_live(ws, WHERE)?.model())
}

/// The process that records what the shapes were translated into.
pub(super) fn finder_process_of(
    ws: &Handle<XSControlWorkSession>,
) -> Result<Handle<TransferFinderProcess>, Failure> {
    let writer = step_live(ws, WHERE)?.transfer_writer();
    Ok(step_live(&writer, WHERE)?.finder_process())
}

/// `get_label_name`: appends the name of a label to `text`, without the spaces around it; false
/// without a name.
pub(super) fn get_label_name(
    label: &TdfLabel,
    text: &Handle<HAsciiString>,
) -> Result<bool, Failure> {
    let mut attribute: Handle<TDataStdName> = Handle::default();
    if !label.find_attribute_as(&TDataStdName::get_id(), &mut attribute)? {
        return Ok(false);
    }
    let name = step_live(&attribute, WHERE)?.get();
    if name.is_empty() {
        return Ok(false);
    }
    // set name, removing spaces around it
    let mut buffer = AsciiString::from_extended(&name, None);
    buffer.left_adjust();
    buffer.right_adjust();
    step_live(text, WHERE)?.assign_cat(buffer.as_bytes());
    Ok(true)
}

/// The typed result of the transfer of a shape, null when there is none.
pub(super) fn typed_result_of<T: RuntimeTyped>(
    fp: &Handle<TransferFinderProcess>,
    shape: &Shape,
) -> Result<Handle<T>, Failure> {
    let mapper = TransferBRep::shape_mapper(fp, shape)?;
    let mut found: Handle<T> = Handle::default();
    step_live(fp, WHERE)?.find_typed_transient_as(&mapper.cast(), T::static_type(), &mut found);
    Ok(found)
}

/// `find_entities`: appends the representation items of a shape, also when the shape was split during
/// shape processing; returns how many.
pub(super) fn find_entities(
    fp: &Handle<TransferFinderProcess>,
    shape: &Shape,
    location: &mut Location,
    items: &mut Vec<Handle<StandardTransient>>,
) -> Result<i32, Failure> {
    let item = StepConstruct::find_entity_located(fp, shape, location)?;
    if item.is_some() {
        items.push(item.cast());
        return Ok(1);
    }
    // may be S was split during shape processing
    let mapper = TransferBRep::shape_mapper(fp, shape)?;
    let binder = step_live(fp, WHERE)?.find(&mapper.cast());
    if binder.is_null() {
        return Ok(0);
    }
    let list: Handle<TransferTransientListBinder> = binder.cast();
    let Some(list) = list.get() else {
        return entities_of_children(fp, shape, location, items);
    };
    let mut count = 0;
    for index in 1..=list.nb_transients() {
        let item: Handle<StepReprRepresentationItem> = list.transient(index)?.cast();
        if item.is_some() {
            count += 1;
            items.push(item.cast());
        }
    }
    Ok(count)
}

fn entities_of_children(
    fp: &Handle<TransferFinderProcess>,
    shape: &Shape,
    location: &mut Location,
    items: &mut Vec<Handle<StandardTransient>>,
) -> Result<i32, Failure> {
    let mut count = 0;
    let mut children = ShapeIterator::new(shape, true, true);
    while children.more() {
        let item = StepConstruct::find_entity_located(fp, children.value()?, location)?;
        children.next();
        if item.is_some() {
            count += 1;
            items.push(item.cast());
        }
    }
    Ok(count)
}
