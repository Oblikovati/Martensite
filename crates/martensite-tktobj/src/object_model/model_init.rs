//! The model behavior of the `martensite_object_model_DocumentModel` members that find objects by name and set up a
//! new or loaded model.
use super::ModelObjectBase;
use super::{DocumentModel, DocumentModelState, LabelIterator, ModelObject, NameIndex};
use martensite_tkernel::{
    standard::{Failure, Handle},
    tcollection::HExtendedString,
};

/// The body of `martensite_object_model_DocumentModel::FindObject`: the object of a registered name.
pub(crate) fn find_object<M: DocumentModel + ?Sized>(
    model: &M,
    name: &Handle<HExtendedString>,
    dictionary: &Handle<NameIndex>,
) -> Result<Handle<dyn ModelObject>, Failure> {
    let dictionary = model.dictionary_or_own(dictionary)?;
    let mut result = Handle::default();
    if model.is_registered_name(name, &dictionary)? {
        let label = martensite_tkcdf::live(
            &dictionary,
            "martensite_object_model_DocumentModel::FindObject",
        )?
        .find_label(name)?;
        ModelObjectBase::get_obj(&label, &mut result, false)?;
    }
    Ok(result)
}

/// The body of `martensite_object_model_DocumentModel::initNewModel`.
pub(crate) fn init_new_model<M: DocumentModel + ?Sized>(
    model: &M,
    is_new: bool,
) -> Result<bool, Failure> {
    NameIndex::set(&model.get_label())?;
    if is_new {
        return Ok(true);
    }
    let objects = model.get_objects()?;
    let objects = martensite_tkcdf::live(
        &objects,
        "martensite_object_model_DocumentModel::initNewModel",
    )?;
    while objects.more() {
        if let Some(object) = objects.value().get() {
            object.after_retrieval()?;
        }
        objects.next()?;
    }
    model.update_back_references(&model.get_main_partition()?.cast())?;
    if model.is_to_check() {
        let checker = model.get_checker()?;
        let checker = martensite_tkcdf::live(
            &checker,
            "martensite_object_model_DocumentModel::initNewModel",
        )?;
        checker.perform()?;
        checker.send_messages()?;
        model.set_modified(true)?;
    }
    Ok(true)
}

/// The referred objects of `object`, when its references come as a label iterator.
fn referred_objects(
    object: &dyn ModelObject,
) -> Result<Option<Vec<Handle<dyn ModelObject>>>, Failure> {
    let iterator: Handle<dyn LabelIterator> = object.get_references(None)?.cast();
    let Some(iterator) = iterator.get() else {
        return Ok(None);
    };
    let mut referred = Vec::new();
    while iterator.more() {
        referred.push(iterator.value());
        iterator.next()?;
    }
    Ok(Some(referred))
}

/// The body of `martensite_object_model_DocumentModel::updateBackReferences`: recursive on the children; the old
/// back references to the object are removed first (LH3D15722), then created again.
pub(crate) fn update_back_references<M: DocumentModel + ?Sized>(
    model: &M,
    object: &Handle<dyn ModelObject>,
) -> Result<(), Failure> {
    let Some(live) = object.get() else {
        return Ok(());
    };
    let children = live.get_children(None)?;
    let children = martensite_tkcdf::live(
        &children,
        "martensite_object_model_DocumentModel::updateBackReferences",
    )?;
    while children.more() {
        model.update_back_references(&children.value())?;
        children.next()?;
    }
    let Some(referred) = referred_objects(live)? else {
        return Ok(()); // to avoid exception
    };
    for target in referred.iter().filter_map(Handle::get) {
        target.remove_back_reference(object, false);
    }
    let Some(referred) = referred_objects(live)? else {
        return Ok(());
    };
    for target in referred.iter().filter_map(Handle::get) {
        target.add_back_reference(object);
    }
    Ok(())
}
