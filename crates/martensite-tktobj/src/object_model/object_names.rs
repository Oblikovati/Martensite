//! The model behavior of the `martensite_object_model_ModelObject` members on the model, the dictionary and names.
use super::{
    DocumentModel, DocumentModelState, ModelAttribute, ModelObject, ModelObjectState, NameIndex,
};
use martensite_tkernel::{
    standard::{Failure, Handle},
    tcollection::HExtendedString,
};
use martensite_tklcaf::{
    tdata_std::{TDataStdGenericExtString, TDataStdName},
    tdf::TdfLabel,
    tdoc_std::{TDocStdDocument, TDocStdOwner},
};

/// The body of `martensite_object_model_ModelObject::GetModel`: the document is found through the
/// `TDocStd_Owner` of the root, the model through the `martensite_object_model_ModelAttribute` of its main label.
pub(crate) fn model_of_label(label: &TdfLabel) -> Result<Handle<dyn DocumentModel>, Failure> {
    let document = document_of_label(label)?;
    let Some(document) = document.get() else {
        return Ok(Handle::default());
    };
    let main = document.main()?;
    let mut attribute: Handle<ModelAttribute> = Handle::default();
    if !main.is_null() && main.find_attribute_as(&ModelAttribute::get_id(), &mut attribute)? {
        return Ok(attribute.model());
    }
    Ok(Handle::default())
}

/// The document of the label's framework through the owner attribute of the root; null
/// when the label, its data or the owner is missing.
fn document_of_label(label: &TdfLabel) -> Result<Handle<TDocStdDocument>, Failure> {
    let data = label.data();
    let Some(data) = data.get() else {
        return Ok(Handle::default());
    };
    let root = data.root();
    let mut owner: Handle<TDocStdOwner> = Handle::default();
    if root.is_null() || !root.find_attribute_as(&TDocStdOwner::get_id(), &mut owner)? {
        return Ok(Handle::default());
    }
    Ok(owner.get_document())
}

/// The body of `martensite_object_model_ModelObject::GetDictionary`: the model's dictionary, null without model.
pub(crate) fn object_get_dictionary<T: ModelObject + ?Sized>(
    object: &T,
) -> Result<Handle<NameIndex>, Failure> {
    let model = object.get_model()?;
    match model.get() {
        Some(model) => model.get_dictionary(),
        None => Ok(Handle::default()),
    }
}

/// The body of `martensite_object_model_ModelObject::GetName`: the `TDataStd_Name`, else an empty string.
pub(crate) fn object_get_name<T: ModelObject + ?Sized>(
    object: &T,
) -> Result<Handle<HExtendedString>, Failure> {
    let mut name: Handle<TDataStdName> = Handle::default();
    if object
        .get_label()
        .find_attribute_as(&TDataStdName::get_id(), &mut name)?
    {
        return Ok(Handle::new(HExtendedString::from(
            TDataStdGenericExtString::get(&*name),
        )));
    }
    Ok(Handle::new(HExtendedString::from_c_string(b"")))
}

/// The body of `martensite_object_model_ModelObject::SetName`: false when the name is registered already, else
/// the old name is unregistered and the new one registered and stored.
pub(crate) fn object_set_name<T: ModelObject + ?Sized>(
    object: &T,
    name: &Handle<HExtendedString>,
) -> Result<bool, Failure> {
    let old_name = object.get_name()?;
    if same_name(&old_name, name)? {
        return Ok(true);
    }
    let dictionary = object.get_dictionary()?;
    let model = object.get_model()?;
    let model = martensite_tkcdf::live(&model, "martensite_object_model_ModelObject::SetName")?;
    if model.is_registered_name(name, &dictionary)? {
        return Ok(false);
    }
    if !old_name.is_null() {
        model.unregister_name(&old_name, &dictionary)?;
    }
    let Some(text) = name.get() else {
        object
            .get_label()
            .forget_attribute_id(&TDataStdName::get_id())?;
        return Ok(true);
    };
    model.register_name(name, &object.get_label(), &dictionary)?;
    TDataStdName::set(&object.get_label(), &text.string())?;
    Ok(true)
}

/// `!anOldName.IsNull() && theName->String().IsEqual(anOldName->String())`; a null new
/// name is `Standard_NullObject` there, as the runtime dereference.
pub(crate) fn same_name(
    old_name: &Handle<HExtendedString>,
    name: &Handle<HExtendedString>,
) -> Result<bool, Failure> {
    let Some(old_name) = old_name.get() else {
        return Ok(false);
    };
    let name = martensite_tkcdf::live(name, "martensite_object_model_ModelObject::SetName")?;
    Ok(name.string().is_equal(&*old_name.string()))
}

/// The body of `martensite_object_model_ModelObject::AfterRetrieval`: registers the name in the model.
pub(crate) fn object_after_retrieval<T: ModelObject + ?Sized>(object: &T) -> Result<(), Failure> {
    let model = object.get_model()?;
    let Some(model) = model.get() else {
        return Ok(());
    };
    model.register_name(
        &object.get_name()?,
        &object.get_label(),
        &object.get_dictionary()?,
    )
}
