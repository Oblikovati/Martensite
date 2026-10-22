//! `martensite_object_model_ModelIntegrityCheck`: the consistency check of a ObjectModel model (references against back
//! references), with an optional fix mode.
use super::{DocumentModel, DocumentModelState, ModelObject};
use martensite_tkernel::{
    impl_transient,
    message::{Algorithm, Gravity, Status},
    standard::{Failure, Handle, RuntimeTyped, SharedState},
};

/// `martensite_object_model_ModelIntegrityCheck`; `ModelIntegrityCheck::new(&model).perform()`.
pub struct ModelIntegrityCheck {
    // C++ destroys the derived model member before its Message_Algorithm base.
    // Keep the inherited messenger alive while a last-owned model is destroyed.
    model: Handle<dyn DocumentModel>,
    algorithm: Algorithm,
    to_fix: SharedState<bool>,
}
impl_transient!(
    ModelIntegrityCheck,
    "martensite_object_model_ModelIntegrityCheck",
    Algorithm,
    algorithm
);

impl ModelIntegrityCheck {
    /// `martensite_object_model_ModelIntegrityCheck(theModel)`: the fix mode is off.
    pub fn new(model: &Handle<dyn DocumentModel>) -> Self {
        Self {
            algorithm: Algorithm::with_runtime_type(Self::static_type()),
            model: model.clone(),
            to_fix: SharedState::new(false),
        }
    }
    /// The embedded `Message_Algorithm` (statuses and messages).
    pub fn algorithm(&self) -> &Algorithm {
        &self.algorithm
    }
    /// `SetToFix(theToFix)`.
    pub fn set_to_fix(&self, to_fix: bool) {
        *self.to_fix.write() = to_fix;
    }
    /// `IsToFix`.
    pub fn is_to_fix(&self) -> bool {
        *self.to_fix.read()
    }
    /// `GetModel`.
    pub fn get_model(&self) -> Handle<dyn DocumentModel> {
        self.model.clone()
    }
    /// `Perform`: `Message_Fail1` without a model or label, else the reference check.
    pub fn perform(&self) -> Result<bool, Failure> {
        self.algorithm.clear_status();
        let Some(model) = self
            .model
            .get()
            .filter(|model| !model.get_label().is_null())
        else {
            self.algorithm.set_status(Status::FAIL1);
            return Ok(false);
        };
        self.check_references(model)
    }
    /// `SendMessages` of the algorithm (as `martensite_object_model_DocumentModel::initNewModel` calls it).
    pub fn send_messages(&self) -> Result<(), Failure> {
        self.algorithm.send_messages(Gravity::Warning, 20)
    }
    /// `checkReferences`: every object's references have a back reference and every back
    /// reference a reference; the fix mode adds or removes the missing ones.
    fn check_references(&self, model: &dyn DocumentModel) -> Result<bool, Failure> {
        let objects = model.get_objects()?;
        let objects = martensite_tkcdf::live(
            &objects,
            "martensite_object_model_ModelIntegrityCheck::checkReferences",
        )?;
        while objects.more() {
            let object = objects.value();
            match object.get() {
                Some(live) => {
                    self.check_forward(&object, live)?;
                    self.check_backward(&object, live)?;
                }
                None => self.set_status_text(Status::ALARM1, objects.dynamic_type().name()),
            }
            objects.next()?;
        }
        let status = self.algorithm.get_status();
        Ok(!status.is_alarm() && !status.is_fail())
    }
    /// Raises a status with the object's name.
    fn set_status_name(&self, status: Status, object: &dyn ModelObject) -> Result<(), Failure> {
        self.algorithm
            .set_status_with_h_extended(status, &object.get_name()?, true);
        Ok(())
    }
    /// Raises a status with a C text.
    fn set_status_text(&self, status: Status, text: &str) {
        self.algorithm
            .set_status_with_c_str(status, text.as_bytes(), true);
    }
    /// The references of the object: each referred object must be alive and have a back
    /// reference to it.
    fn check_forward(
        &self,
        object: &Handle<dyn ModelObject>,
        live: &dyn ModelObject,
    ) -> Result<(), Failure> {
        let references = live.get_references(None)?;
        let references = martensite_tkcdf::live(
            &references,
            "martensite_object_model_ModelIntegrityCheck::checkReferences",
        )?;
        while references.more() {
            let referred = references.value();
            match alive(&referred)? {
                None => self.set_status_name(Status::ALARM2, live)?,
                Some(referred) => self.check_back_reference(object, live, referred)?,
            }
            references.next()?;
        }
        Ok(())
    }
    /// The referred object must have a back reference to the object.
    fn check_back_reference(
        &self,
        object: &Handle<dyn ModelObject>,
        live: &dyn ModelObject,
        referred: &dyn ModelObject,
    ) -> Result<(), Failure> {
        if contains(&*referred.get_back_references(None), object)? {
            return Ok(()); // ok, back reference found
        }
        if self.is_to_fix() {
            self.set_status_name(Status::WARN1, live)?;
            referred.add_back_reference(object);
        } else {
            self.set_status_name(Status::ALARM4, live)?;
        }
        Ok(())
    }
    /// The back references of the object: each referring object must be alive and refer
    /// to it; the fix mode removes the others.
    fn check_backward(
        &self,
        object: &Handle<dyn ModelObject>,
        live: &dyn ModelObject,
    ) -> Result<(), Failure> {
        let back_references = live.get_back_references(None);
        let mut bad = Vec::new();
        while back_references.more() {
            let referring = back_references.value();
            match alive(&referring)? {
                None => self.set_status_name(Status::ALARM3, live)?,
                Some(referring_live) => {
                    if !contains(&*referring_live.get_references(None)?, object)? {
                        if self.is_to_fix() {
                            self.set_status_name(Status::WARN2, live)?;
                            bad.push(referring.clone());
                        } else {
                            self.set_status_name(Status::ALARM5, live)?;
                        }
                    }
                }
            }
            back_references.next()?;
        }
        for referring in &bad {
            live.remove_back_reference(referring, true);
        }
        Ok(())
    }
}

/// The object when it is not null and alive.
fn alive(object: &Handle<dyn ModelObject>) -> Result<Option<&dyn ModelObject>, Failure> {
    match object.get() {
        Some(live) if live.is_alive()? => Ok(Some(live)),
        _ => Ok(None),
    }
}

/// True when the iterator meets the object before its end (it stops there).
fn contains(
    iterator: &dyn super::ObjectIterator,
    object: &Handle<dyn ModelObject>,
) -> Result<bool, Failure> {
    while iterator.more() {
        if iterator.value() == *object {
            return Ok(true);
        }
        iterator.next()?;
    }
    Ok(false)
}
