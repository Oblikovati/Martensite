//! `martensite_object_model_ReferenceAttribute`: the document data framework attribute of a reference from one ObjectModel object (the
//! master) to another; each reference registers a back reference in the referred object.
use super::ModelObjectBase;
use super::{ModelObject, ModelObjectState, ObjectAttribute};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Guid, Handle, RuntimeTyped, SharedState, StandardTransient},
};
use martensite_tklcaf::{
    guid_from_text,
    tdata_std::support::cast_attribute,
    tdf::{
        TdfAttribute, TdfAttributeBase, TdfAttributeDelta, TdfAttributeState, TdfDeltaOnAddition,
        TdfDeltaOnRemoval, TdfLabel, TdfRelocationTable,
    },
};

/// `martensite_object_model_ReferenceAttribute::GetID()`.
const REFERENCE_ID: Guid = guid_from_text("3bbefb44-e618-11d4-ba38-0060b0ee18ea");

/// The referred label (`myLabel`) and the master label (`myMasterLabel`).
#[derive(Clone, Default)]
struct ReferenceLabels {
    referred: TdfLabel,
    master: TdfLabel,
}

/// The master object and the referred object of a reference.
type MasterAndReferred = (Handle<dyn ModelObject>, Handle<dyn ModelObject>);

/// `martensite_object_model_ReferenceAttribute`; `ReferenceAttribute::set(&label, &referred, &master)`.
#[derive(Default)]
pub struct ReferenceAttribute {
    transient: StandardTransient,
    base: TdfAttributeBase,
    labels: SharedState<ReferenceLabels>,
}
impl_transient!(ReferenceAttribute, "martensite_object_model_ReferenceAttribute", dyn TdfAttribute, transient,
    interfaces: [dyn TdfAttribute]);

/// The object stored on `label` by a `martensite_object_model_ObjectAttribute`, null when none.
fn object_on(label: &TdfLabel) -> Result<Handle<dyn ModelObject>, Failure> {
    let mut attribute: Handle<ObjectAttribute> = Handle::default();
    if label.is_null() || !label.find_attribute_as(&ObjectAttribute::get_id(), &mut attribute)? {
        return Ok(Handle::default());
    }
    Ok(attribute.get_object())
}

impl ReferenceAttribute {
    /// `GetID`.
    pub fn get_id() -> Guid {
        REFERENCE_ID
    }
    /// `martensite_object_model_ReferenceAttribute()`: null labels.
    pub fn new() -> Self {
        Self::default()
    }
    /// `Set(theLabel, theObject, theMaster)` (static): the reference of the label (the old
    /// referred object loses its back reference) pointing to `object`, which gets a back
    /// reference to `master`.
    pub fn set(
        label: &TdfLabel,
        object: &Handle<dyn ModelObject>,
        master: &Handle<dyn ModelObject>,
    ) -> Result<Handle<ReferenceAttribute>, Failure> {
        let mut reference: Handle<ReferenceAttribute> = Handle::default();
        if !label.find_attribute_as(&REFERENCE_ID, &mut reference)? {
            reference = Handle::new(ReferenceAttribute::new());
            label.add_attribute(&reference.cast(), true)?;
        } else if let Some(old) = reference.get_object()?.get() {
            old.remove_back_reference(master, true);
        }
        let master_label =
            martensite_tkcdf::live(master, "martensite_object_model_ReferenceAttribute::Set")?
                .get_label();
        reference.set_object(object, &master_label)?;
        if let Some(referred) = object.get() {
            referred.add_back_reference(master);
        }
        Ok(reference)
    }
    /// `Set(theObject, theMasterLabel)`: backed up, then the object's label and the master.
    pub fn set_object(
        &self,
        object: &Handle<dyn ModelObject>,
        master_label: &TdfLabel,
    ) -> Result<(), Failure> {
        let referred = object
            .get()
            .map(|object| object.get_label())
            .unwrap_or_default();
        self.set_label(&referred, master_label)
    }
    /// `Set(theLabel, theMasterLabel)`: backed up, then both labels (for persistence).
    pub fn set_label(&self, label: &TdfLabel, master_label: &TdfLabel) -> Result<(), Failure> {
        self.backup()?;
        *self.labels.write() = ReferenceLabels {
            referred: label.clone(),
            master: master_label.clone(),
        };
        Ok(())
    }
    /// `Get`: the object on the referred label, null when none.
    pub fn get_object(&self) -> Result<Handle<dyn ModelObject>, Failure> {
        object_on(&self.get_label())
    }
    /// `GetMasterLabel`.
    pub fn get_master_label(&self) -> TdfLabel {
        self.labels.read().master.clone()
    }
    /// `GetLabel`: the referred label.
    pub fn get_label(&self) -> TdfLabel {
        self.labels.read().referred.clone()
    }
    /// The master object (`myMasterLabel`'s `martensite_object_model_ObjectAttribute`) and the referred object, when
    /// the master label holds an attribute; the shared prologue of the callbacks.
    fn master_and_referred(&self) -> Result<Option<MasterAndReferred>, Failure> {
        let master_label = self.get_master_label();
        let mut master: Handle<ObjectAttribute> = Handle::default();
        if master_label.is_null()
            || !master_label.find_attribute_as(&ObjectAttribute::get_id(), &mut master)?
        {
            return Ok(None);
        }
        Ok(Some((master.get_object(), self.get_object()?)))
    }
    /// Adds a back reference from the referred object to the master, when both exist.
    fn restore_back_reference(&self) -> Result<(), Failure> {
        if let Some((master, referred)) = self.master_and_referred()?
            && let Some(referred) = referred.get()
        {
            referred.add_back_reference(&master);
        }
        Ok(())
    }
}

impl TdfAttribute for ReferenceAttribute {
    fn attribute_base(&self) -> &TdfAttributeBase {
        &self.base
    }
    fn id(&self) -> Guid {
        REFERENCE_ID
    }
    fn restore(&self, with: &Handle<dyn TdfAttribute>) -> Result<(), Failure> {
        let other = cast_attribute::<ReferenceAttribute>(
            with,
            "martensite_object_model_ReferenceAttribute::Restore",
        )?;
        *self.labels.write() = other.labels.read().clone();
        Ok(())
    }
    fn new_empty(&self) -> Handle<dyn TdfAttribute> {
        Handle::new(Self::new()).cast()
    }
    /// `Paste`: the relocated referred object, with the master found above the target.
    fn paste(
        &self,
        into: &Handle<dyn TdfAttribute>,
        relocation_table: &Handle<TdfRelocationTable>,
    ) -> Result<(), Failure> {
        let target = cast_attribute::<ReferenceAttribute>(
            into,
            "martensite_object_model_ReferenceAttribute::Paste",
        )?;
        let referred = self.get_label();
        if referred.is_null() {
            target.labels.write().referred = TdfLabel::default();
            return Ok(());
        }
        let table = martensite_tkcdf::live(
            relocation_table,
            "martensite_object_model_ReferenceAttribute::Paste",
        )?;
        let mut relocated = referred.clone();
        if !table.has_label_relocation(&referred, &mut relocated) {
            relocated = referred;
        }
        paste_into(&target, &object_on(&relocated)?)
    }
    /// `BeforeForget`: the referred object loses its back reference to the master.
    fn before_forget(&self) -> Result<(), Failure> {
        let Some((master, referred)) = self.master_and_referred()? else {
            return Ok(());
        };
        let Some(referred) = referred.get() else {
            return Ok(());
        };
        martensite_tkcdf::live(
            &master,
            "martensite_object_model_ReferenceAttribute::BeforeForget",
        )?
        .before_forget_reference(&self.get_label())?;
        referred.remove_back_reference(&master, true);
        Ok(())
    }
    /// `BeforeUndo`: undoing an addition removes the back reference.
    fn before_undo(
        &self,
        delta: &Handle<dyn TdfAttributeDelta>,
        _force_it: bool,
    ) -> Result<bool, Failure> {
        if !delta.is_kind(TdfDeltaOnAddition::static_type()) {
            return Ok(true);
        }
        if let Some((master, referred)) = self.master_and_referred()?
            && let Some(referred) = referred.get()
        {
            referred.remove_back_reference(&master, true);
        }
        Ok(true)
    }
    /// `AfterUndo`: undoing a removal restores the back reference.
    fn after_undo(
        &self,
        delta: &Handle<dyn TdfAttributeDelta>,
        _force_it: bool,
    ) -> Result<bool, Failure> {
        if delta.is_kind(<dyn TdfDeltaOnRemoval>::static_type()) {
            self.restore_back_reference()?;
        }
        Ok(true)
    }
    /// `AfterResume`: restores the back reference.
    fn after_resume(&self) -> Result<(), Failure> {
        self.restore_back_reference()
    }
    /// `AfterRetrieval`: restores the back reference; false when the master label holds
    /// no object attribute.
    fn after_retrieval(&self, _force_it: bool) -> Result<bool, Failure> {
        if self.get_master_label().is_null() {
            return Ok(true);
        }
        if self.master_and_referred()?.is_none() {
            return Ok(false);
        }
        self.restore_back_reference()?;
        Ok(true)
    }
}

/// The tail of `Paste`: the master is the object above the target attribute's label;
/// nothing is set when it is missing.
fn paste_into(
    target: &ReferenceAttribute,
    referred: &Handle<dyn ModelObject>,
) -> Result<(), Failure> {
    let mut master = Handle::default();
    ModelObjectBase::get_obj(&target.label(), &mut master, true)?;
    let master_label = master
        .get()
        .map(|master| master.get_label())
        .unwrap_or_default();
    let mut master_attribute: Handle<ObjectAttribute> = Handle::default();
    if master_label.is_null()
        || !master_label.find_attribute_as(&ObjectAttribute::get_id(), &mut master_attribute)?
    {
        return Ok(());
    }
    target.set_object(referred, &master_label)?;
    if let Some(referred) = referred.get() {
        referred.add_back_reference(&master_attribute.get_object());
    }
    Ok(())
}
