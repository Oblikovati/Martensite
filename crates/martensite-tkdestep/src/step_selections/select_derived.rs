//! `STEPSelectionsSelectDerived`: the step type signature that also matches derived types.
use crate::rw_step_ap214::RwStepAP214GeneralModule;
use crate::step_data::{StepDataProtocolOps, StepDataReadWriteModule, StepDataWriterLib};
use crate::step_entity::step_live;
use crate::step_select::StepSelectStepType;
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkxsbase::if_select::{IFSelectSignature, IFSelectSignatureBase};
use martensite_tkxsbase::interface::{
    InterfaceGeneralModule, InterfaceInterfaceModel, InterfaceProtocol, InterfaceSignType,
};
use martensite_tkxsbase::moni_tool::MoniToolSignText;

/// Select derived.
#[derive(Default)]
pub struct STEPSelectionsSelectDerived {
    base: StepSelectStepType,
    thelib: Option<StepDataWriterLib>,
}
impl_transient!(STEPSelectionsSelectDerived, "STEPSelectionsSelectDerived", StepSelectStepType, base, interfaces: [dyn IFSelectSignature, dyn InterfaceSignType, dyn MoniToolSignText]);

/// `GetStepType`: an entity of the named type, to test against; null when the name is not of the
/// module.
fn entity_of_step_type(
    module: &Handle<dyn StepDataReadWriteModule>,
    step_type: &AsciiString,
) -> Handle<StandardTransient> {
    let Some(live) = module.get() else {
        return Handle::default();
    };
    let number = live.case_step(step_type);
    if number == 0 {
        return Handle::default();
    }
    let mut made = Handle::default();
    RwStepAP214GeneralModule::default().new_void(number, &mut made);
    made
}

impl STEPSelectionsSelectDerived {
    /// `STEPSelectionsSelectDerived()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `set_protocol(proto)`.
    ///
    /// # Errors
    /// `an interface error` when `proto` is not a STEP protocol.
    pub fn set_protocol(&mut self, proto: &Handle<dyn InterfaceProtocol>) -> Result<(), Failure> {
        self.base.set_protocol(proto)?;
        self.thelib = Some(StepDataWriterLib::new(
            &proto.cast::<dyn StepDataProtocolOps>(),
        ));
        Ok(())
    }
}

impl MoniToolSignText for STEPSelectionsSelectDerived {
    fn name(&self) -> String {
        MoniToolSignText::name(&self.base)
    }
    fn text(
        &self,
        ent: &Handle<StandardTransient>,
        context: &Handle<StandardTransient>,
    ) -> AsciiString {
        self.base.text(ent, context)
    }
}

impl InterfaceSignType for STEPSelectionsSelectDerived {
    fn value(
        &self,
        ent: &Handle<StandardTransient>,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> String {
        InterfaceSignType::value(&self.base, ent, model)
    }
}

impl IFSelectSignature for STEPSelectionsSelectDerived {
    fn signature_base(&self) -> &IFSelectSignatureBase {
        self.base.signature_base()
    }
    /// `matches(ent, model, text, exact)`: `ent` is of the named type or derives from it.
    fn matches(
        &self,
        ent: &Handle<StandardTransient>,
        _model: &Handle<dyn InterfaceInterfaceModel>,
        text: &AsciiString,
        _exact: bool,
    ) -> Result<bool, Failure> {
        let Some((module, cn)) = self.thelib.as_ref().and_then(|lib| lib.select(ent)) else {
            return Ok(false);
        };
        let checker = entity_of_step_type(&module, text);
        let Some(checker_type) = checker.dynamic_type() else {
            return Ok(false);
        };
        let live = step_live(&module, "STEPSelectionsSelectDerived")?;
        if !live.is_complex(cn) {
            // The entity may itself be a the runtime type; an entity of a model never is.
            return Ok(ent.is_kind(checker_type));
        }
        let mut names = Vec::new();
        live.complex_type(cn, &mut names);
        for name in &names {
            let member = entity_of_step_type(&module, name);
            // A member the module does not name is the null dereference.
            step_live(
                &member,
                "STEPSelectionsSelectDerived: a member of the complex type",
            )?;
            if member.is_kind(checker_type) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
