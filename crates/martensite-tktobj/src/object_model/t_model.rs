//! `martensite_object_model_ModelAttribute`: the document data framework attribute that stores a ObjectModel model on the main label.
use super::DocumentModel;
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Guid, Handle, SharedState, StandardTransient},
};
use martensite_tklcaf::{
    guid_from_text,
    tdata_std::support::cast_attribute,
    tdf::{TdfAttribute, TdfAttributeBase, TdfAttributeState, TdfRelocationTable},
};

/// `martensite_object_model_ModelAttribute::GetID()`.
const TMODEL_ID: Guid = guid_from_text("bbdab6a6-dca9-11d4-ba37-0060b0ee18ea");

/// `martensite_object_model_ModelAttribute`; `label.add_attribute(&Handle::new(ModelAttribute::new()).cast(), true)`.
#[derive(Default)]
pub struct ModelAttribute {
    transient: StandardTransient,
    base: TdfAttributeBase,
    model: SharedState<Handle<dyn DocumentModel>>,
}
impl_transient!(ModelAttribute, "martensite_object_model_ModelAttribute", dyn TdfAttribute, transient, interfaces: [dyn TdfAttribute]);

impl ModelAttribute {
    /// `GetID`.
    pub fn get_id() -> Guid {
        TMODEL_ID
    }
    /// `martensite_object_model_ModelAttribute()`: no model.
    pub fn new() -> Self {
        Self::default()
    }
    /// `Set(theModel)`: backed up, then set.
    pub fn set(&self, model: &Handle<dyn DocumentModel>) -> Result<(), Failure> {
        self.backup()?;
        *self.model.write() = model.clone();
        Ok(())
    }
    /// `Model`: the stored model.
    pub fn model(&self) -> Handle<dyn DocumentModel> {
        self.model.read().clone()
    }
}

impl TdfAttribute for ModelAttribute {
    fn attribute_base(&self) -> &TdfAttributeBase {
        &self.base
    }
    fn id(&self) -> Guid {
        TMODEL_ID
    }
    fn restore(&self, with: &Handle<dyn TdfAttribute>) -> Result<(), Failure> {
        let other = cast_attribute::<ModelAttribute>(
            with,
            "martensite_object_model_ModelAttribute::Restore",
        )?;
        *self.model.write() = other.model();
        Ok(())
    }
    fn new_empty(&self) -> Handle<dyn TdfAttribute> {
        Handle::new(Self::new()).cast()
    }
    fn paste(
        &self,
        into: &Handle<dyn TdfAttribute>,
        _relocation_table: &Handle<TdfRelocationTable>,
    ) -> Result<(), Failure> {
        let target = cast_attribute::<ModelAttribute>(
            into,
            "martensite_object_model_ModelAttribute::Paste",
        )?;
        target.set(&self.model())
    }
}
