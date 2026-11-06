//! STEPEdit: tools to exploit and edit a set of STEP data: editors, selections.
mod edit_context;
mod edit_sdr;

pub use edit_context::STEPEditEditContext;
pub use edit_sdr::STEPEditEditSDR;

use crate::step_ap214::StepAP214;
use crate::step_data::StepDataStepModel;
use crate::step_entity::step_live;
use crate::step_select::StepSelectStepType;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::if_select::{
    IFSelectSelectDeduct, IFSelectSelectModelEntities, IFSelectSelectModelRoots,
    IFSelectSelectSignature, IFSelectSignature,
};
use martensite_tkxsbase::interface::{InterfaceModelEntities, InterfaceProtocol};
use std::sync::OnceLock;

/// The signature all selections share; a static initialized under a mutex.
static SIGN_TYPE: OnceLock<Handle<StepSelectStepType>> = OnceLock::new();

/// `STEPEdit`: the static members.
///
/// ```
/// use martensite_tkdestep::step_edit::STEPEdit;
/// assert!(STEPEdit::protocol().is_some());
/// let model = STEPEdit::new_model().unwrap();
/// assert!(model.is_some());
/// let selection = STEPEdit::new_select_sdr().unwrap();
/// assert_eq!(selection.signature_text().as_bytes(), b"SHAPE_DEFINITION_REPRESENTATION");
/// ```
#[allow(clippy::upper_case_acronyms)]
pub struct STEPEdit;

impl STEPEdit {
    /// `Protocol`: a protocol fit for STEP.
    pub fn protocol() -> Handle<dyn InterfaceProtocol> {
        StepAP214::protocol().cast()
    }
    /// `new_model`: a new empty model fit for STEP, with its header determined from the protocol.
    pub fn new_model() -> Result<Handle<StepDataStepModel>, Failure> {
        let model = StepDataStepModel::new();
        step_live(&model, "STEPEdit::NewModel")?.set_protocol(&Self::protocol());
        Ok(model)
    }
    /// `sign_type`: a type signature fit for STEP; one for the process.
    pub fn sign_type() -> Result<Handle<dyn IFSelectSignature>, Failure> {
        if let Some(made) = SIGN_TYPE.get() {
            return Ok(made.cast());
        }
        let mut signature = StepSelectStepType::new();
        signature.set_protocol(&Self::protocol())?;
        Ok(SIGN_TYPE.get_or_init(|| Handle::new(signature)).cast())
    }
    /// `new_select_sdr`: a selection of ShapeDefinitionRepresentation; searches among root entities by
    /// default.
    pub fn new_select_sdr() -> Result<Handle<IFSelectSelectSignature>, Failure> {
        let selection = Handle::new(IFSelectSelectSignature::with_signature(
            &Self::sign_type()?,
            "SHAPE_DEFINITION_REPRESENTATION",
            true,
        ));
        step_live(&selection, "STEPEdit::NewSelectSDR")?
            .set_input(&Handle::new(IFSelectSelectModelRoots::new()).cast());
        Ok(selection)
    }
    /// `new_select_placed_item`: a selection of placed items, a MappedItem or a
    /// ContextDependentShapeRepresentation, which itself refers to a RepresentationRelationship with
    /// possible subtypes; in the whole model by default.
    pub fn new_select_placed_item() -> Result<Handle<IFSelectSelectSignature>, Failure> {
        Self::select_in_model("MAPPED_ITEM|CONTEXT_DEPENDENT_SHAPE_REPRESENTATION")
    }
    /// `new_select_shape_repr`: a selection of ShapeRepresentation and its sub-types, plus
    /// ContextDependentShapeRepresentation; in the whole model by default.
    pub fn new_select_shape_repr() -> Result<Handle<IFSelectSelectSignature>, Failure> {
        // REPRESENTATION_RELATIONSHIP passe par CONTEXT_DEPENDENT_SHAPE_REPRESENTATION
        Self::select_in_model("SHAPE_REPRESENTATION")
    }
    /// A selection by a signature text contained in the type, over all the model.
    fn select_in_model(text: &str) -> Result<Handle<IFSelectSelectSignature>, Failure> {
        let selection = Handle::new(IFSelectSelectSignature::with_signature(
            &Self::sign_type()?,
            text,
            false,
        ));
        step_live(&selection, "STEPEdit::NewSelect")?
            .set_input(&Handle::new(IFSelectSelectModelEntities::new()).cast());
        Ok(selection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkernel::standard::RuntimeTyped;
    use martensite_tkxsbase::interface::InterfaceInterfaceModel;

    #[test]
    fn the_sign_type_is_one_for_the_process() {
        let (first, second) = (
            STEPEdit::sign_type().unwrap(),
            STEPEdit::sign_type().unwrap(),
        );
        assert!(first.object_address() == second.object_address());
    }

    #[test]
    fn the_selections_take_their_input_and_text() {
        let placed = STEPEdit::new_select_placed_item().unwrap();
        let placed = placed.get().unwrap();
        assert!(!placed.is_exact());
        assert_eq!(
            placed.signature_text().as_bytes(),
            b"MAPPED_ITEM|CONTEXT_DEPENDENT_SHAPE_REPRESENTATION"
        );
        assert!(
            placed
                .input()
                .cast::<IFSelectSelectModelEntities>()
                .is_some()
        );
        let sdr = STEPEdit::new_select_sdr().unwrap();
        let sdr = sdr.get().unwrap();
        assert!(sdr.is_exact());
        assert!(sdr.input().cast::<IFSelectSelectModelRoots>().is_some());
        let repr = STEPEdit::new_select_shape_repr().unwrap();
        assert_eq!(
            repr.get().unwrap().signature_text().as_bytes(),
            b"SHAPE_REPRESENTATION"
        );
    }

    #[test]
    fn a_new_model_has_the_protocol_fit_for_step() {
        let model = STEPEdit::new_model().unwrap();
        let protocol = model.get().unwrap().protocol();
        assert!(protocol.is_some());
        assert_eq!(
            protocol.dynamic_type().unwrap().name(),
            crate::step_ap214::StepAP214Protocol::static_type().name()
        );
    }
}
