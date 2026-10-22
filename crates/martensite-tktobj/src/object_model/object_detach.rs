//! The model behavior of the `martensite_object_model_ModelObject` members that delete objects and unlink the
//! objects that refer to them.
use super::{DeletionMode, ModelObject, ModelObjectState};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tklcaf::tdf::TdfData;

/// The body of `martensite_object_model_ModelObject::RemoveBackReference`: removes the first (or every) entry of
/// the object; an emptied sequence is released.
pub(crate) fn object_remove_back_reference<T: ModelObject + ?Sized>(
    object: &T,
    referring: &Handle<dyn ModelObject>,
    single_only: bool,
) {
    let base = object.object_base();
    let sequence = base.back_references();
    let Some(entries) = sequence.get() else {
        return;
    };
    let is_empty = entries.change_sequence(|items| {
        let mut index = 1;
        while index <= items.len() {
            if items.value(index).is_ok_and(|item| item == referring) {
                let _ = items.remove(index);
                if single_only {
                    break;
                }
                continue;
            }
            index += 1;
        }
        items.is_empty()
    });
    if is_empty {
        base.set_back_references(Handle::default());
    }
}

/// The referring objects of `object` that are alive.
fn live_back_references<T: ModelObject + ?Sized>(
    object: &T,
) -> Result<Vec<Handle<dyn ModelObject>>, Failure> {
    let references = object.get_back_references(None);
    let mut objects = Vec::new();
    let Some(references) = references.get() else {
        return Ok(objects);
    };
    while references.more() {
        objects.push(references.value());
        references.next()?;
    }
    Ok(objects)
}

/// The body of `martensite_object_model_ModelObject::CanDetach`.
pub(crate) fn object_can_detach<T: ModelObject + ?Sized>(
    object: &T,
    mode: DeletionMode,
) -> Result<bool, Failure> {
    if !object.is_alive()? {
        return Ok(false);
    }
    let references = live_back_references(object)?;
    // A free object can be deleted in any mode.
    if references.is_empty() || mode == DeletionMode::Forced {
        return Ok(true);
    }
    if mode == DeletionMode::FreeOnly {
        return Ok(false);
    }
    let me = object.object_base().this()?;
    for referring in &references {
        if !martensite_tkcdf::live(referring, "martensite_object_model_ModelObject::CanDetach")?
            .can_remove_reference(&me)?
        {
            return Ok(false); // one of objects could not be unlinked
        }
    }
    Ok(true)
}

/// The body of `martensite_object_model_ModelObject::Detach`: unlinks the referring objects, detaches the
/// children, clears the data and the name, then forgets the object.
pub(crate) fn object_detach<T: ModelObject + ?Sized>(
    object: &T,
    mode: DeletionMode,
) -> Result<bool, Failure> {
    if !object.is_alive()? || !object.remove_back_references(mode)? {
        return Ok(false);
    }
    let old_name = object.get_name()?;
    let children = object.get_children(None)?;
    let children =
        martensite_tkcdf::live(&children, "martensite_object_model_ModelObject::Detach")?;
    while children.more() {
        martensite_tkcdf::live(
            &children.value(),
            "martensite_object_model_ModelObject::Detach",
        )?
        .detach(mode)?;
        children.next()?;
    }
    object.get_reference_label()?.forget_all_attributes(true)?;
    object.clear_back_references();
    object.get_data_label()?.forget_all_attributes(true)?;
    unregister_own_name(object, &old_name)?;
    object.get_label().forget_all_attributes(true)?;
    Ok(true)
}

/// Unregisters the name only when the dictionary maps it to this object's label.
fn unregister_own_name<T: ModelObject + ?Sized>(
    object: &T,
    old_name: &Handle<martensite_tkernel::tcollection::HExtendedString>,
) -> Result<(), Failure> {
    if old_name.is_null() {
        return Ok(());
    }
    let dictionary = object.get_dictionary()?;
    let Some(dictionary) = dictionary.get() else {
        return Ok(());
    };
    if !dictionary.is_registered(old_name) {
        return Ok(());
    }
    let registered = dictionary.find_label(old_name)?;
    if !registered.is_null() && registered == object.get_label() {
        dictionary.remove_name(old_name)?;
    }
    Ok(())
}

/// The body of `martensite_object_model_ModelObject::RemoveBackReferences`: the referring containers drop their
/// reference, the strong referrers are detached (not in `KeepDepending` mode).
pub(crate) fn object_remove_back_references<T: ModelObject + ?Sized>(
    object: &T,
    mode: DeletionMode,
) -> Result<bool, Failure> {
    let references = live_back_references(object)?;
    if references.is_empty() {
        return Ok(true); // a free object can be deleted in any mode
    }
    if mode == DeletionMode::FreeOnly {
        return Ok(false);
    }
    let me = object.object_base().this()?;
    let (containers, strongs) = sort_referring(&references, &me)?;
    if mode == DeletionMode::KeepDepending && !strongs.is_empty() {
        return Ok(false);
    }
    let own_data = object.get_label().data();
    for container in &containers {
        with_modification_allowed(container, &own_data, |referring| {
            referring.remove_reference(&me)
        })?;
    }
    for strong in &strongs {
        with_modification_allowed(strong, &own_data, |referring| {
            referring.detach(mode).map(|_| ())
        })?;
    }
    Ok(true)
}

/// Splits the alive referring objects into those that can drop their reference
/// (containers) and those that cannot (strong relations).
type ReferringSplit = (Vec<Handle<dyn ModelObject>>, Vec<Handle<dyn ModelObject>>);
fn sort_referring(
    references: &[Handle<dyn ModelObject>],
    me: &Handle<dyn ModelObject>,
) -> Result<ReferringSplit, Failure> {
    let (mut containers, mut strongs) = (Vec::new(), Vec::new());
    for referring in references {
        let Some(object) = referring.get() else {
            continue;
        };
        if !object.is_alive()? {
            continue;
        }
        if object.can_remove_reference(me)? {
            containers.push(referring.clone());
        } else {
            strongs.push(referring.clone());
        }
    }
    Ok((containers, strongs))
}

/// Runs `edit` on a referring object, allowing modifications of its framework while
/// it is not the own one (an object of another document), as by the model.
fn with_modification_allowed(
    referring: &Handle<dyn ModelObject>,
    own_data: &Handle<TdfData>,
    edit: impl FnOnce(&dyn ModelObject) -> Result<(), Failure>,
) -> Result<(), Failure> {
    let Some(object) = referring.get() else {
        return Ok(());
    };
    let label = object.get_label();
    if label.is_null() {
        return Ok(()); // undead object on dead label
    }
    let data = label.data();
    let data = martensite_tkcdf::live(
        &data,
        "martensite_object_model_ModelObject::RemoveBackReferences",
    )?;
    let modification_mode = data.is_modification_allowed();
    let foreign = label.data() != *own_data;
    if foreign {
        data.allow_modification(true);
    }
    edit(object)?;
    if foreign {
        data.allow_modification(modification_mode);
    }
    Ok(())
}
