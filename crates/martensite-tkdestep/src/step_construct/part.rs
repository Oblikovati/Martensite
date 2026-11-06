//! `StepConstructPart`: the STEP structures of a part (SDR), PRODUCT, PDF and so on, as the current
//! schema requires; also investigates and modifies them.
//!
//! :k8 abv 6 Jan 99: unique names for PRODUCT_DEFINITION_FORMATIONs
//! :k9 abv 6 Jan 99: TR10: eliminating duplicated APPLICATION_CONTEXT entities
//! :j4 gka 16.03.99 S4134
//! abv 20.11.99 renamed from the shape definition tool
use crate::step_basic::{
    StepBasicApplicationContext, StepBasicDesignContext, StepBasicMechanicalContext,
    StepBasicProduct, StepBasicProductContext, StepBasicProductDefinition,
    StepBasicProductDefinitionContext, StepBasicProductDefinitionFormation,
    StepBasicProductDefinitionFormationWithSpecifiedSource, StepBasicProductRelatedProductCategory,
    StepBasicProductType, StepBasicSource,
};
use crate::step_data::{StepDataSelectType, StepDataStepModel};
use crate::step_entity::{step_array1_set, step_live, step_new_array1};
use crate::step_repr::{
    StepReprCharacterizedDefinition, StepReprProductDefinitionShape, StepReprPropertyDefinition,
    StepReprRepresentedDefinition,
};
use crate::step_shape::{StepShapeShapeDefinitionRepresentation, StepShapeShapeRepresentation};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;

const WHERE: &str = "StepConstructPart";

fn text(value: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(value))
}

/// ```
/// use martensite_tkdestep::step_construct::StepConstructPart;
/// let part = StepConstructPart::new();
/// assert!(!part.is_done() && part.sdr_value().is_null() && part.sr_value().unwrap().is_null());
/// assert!(part.pd().is_err());
/// ```
#[derive(Clone, Default)]
pub struct StepConstructPart {
    my_done: bool,
    my_sdr: Handle<StepShapeShapeDefinitionRepresentation>,
    my_prpc: Handle<StepBasicProductRelatedProductCategory>,
}

/// A getter and a setter of a text field of one of the chain's entities.
macro_rules! part_text_fields {
    ($($(#[$meta:meta])* $owner:ident: $get:ident = $field:ident, $set:ident = $setter:ident;)+) => {
        $(
            $(#[$meta])*
            pub fn $get(&self) -> Result<Handle<HAsciiString>, Failure> {
                Ok(step_live(&self.$owner()?, WHERE)?.$field())
            }
            $(#[$meta])*
            pub fn $set(&self, value: &Handle<HAsciiString>) -> Result<(), Failure> {
                step_live(&self.$owner()?, WHERE)?.$setter(value);
                Ok(())
            }
        )+
    };
}

/// The product context of a schema: AP214 CD and AP203 write a mechanical context.
fn new_product_context(schema: i32) -> Handle<StepBasicProductContext> {
    match schema {
        2 | 4 | 5 => Handle::new(StepBasicProductContext::new()),
        _ => Handle::new(StepBasicMechanicalContext::new()).cast(),
    }
}

/// The formation of a schema: AP203 writes the one with a specified source.
fn new_formation(schema: i32) -> Handle<StepBasicProductDefinitionFormation> {
    if schema != 3 {
        return Handle::new(StepBasicProductDefinitionFormation::new());
    }
    let formation = StepBasicProductDefinitionFormationWithSpecifiedSource::new();
    formation.set_make_or_buy(StepBasicSource::SNotKnown);
    Handle::new(formation).cast()
}

/// The definition context and its name for a schema: AP203 writes a design context without name.
fn new_definition_context(
    schema: i32,
) -> (
    Handle<StepBasicProductDefinitionContext>,
    Handle<HAsciiString>,
) {
    if schema == 3 {
        return (Handle::new(StepBasicDesignContext::new()).cast(), text(""));
    }
    (
        Handle::new(StepBasicProductDefinitionContext::new()),
        text("part definition"),
    )
}

/// The category of a part for a schema, and its name.
fn new_category(
    schema: i32,
) -> (
    Handle<StepBasicProductRelatedProductCategory>,
    Handle<HAsciiString>,
) {
    match schema {
        // !!!!! or "assembly"
        3 => (
            Handle::new(StepBasicProductRelatedProductCategory::new()),
            text("detail"),
        ),
        2 | 4 | 5 => (
            Handle::new(StepBasicProductRelatedProductCategory::new()),
            text("part"),
        ),
        _ => (
            Handle::new(StepBasicProductType::new()).cast(),
            text("part"),
        ),
    }
}

/// The product of a new part, with its one context.
fn new_product(
    name: &Handle<HAsciiString>,
    ac: &Handle<StepBasicApplicationContext>,
    schema: i32,
) -> Result<Handle<StepBasicProduct>, Failure> {
    let context = new_product_context(schema);
    live_init(&context, &text(""), ac, &text("mechanical"))?;
    let contexts = step_new_array1::<Handle<StepBasicProductContext>>(1, 1)?;
    step_array1_set(&contexts, 1, context, WHERE)?;
    let product = Handle::new(StepBasicProduct::new());
    product.init(name, name, &text(""), &contexts);
    Ok(product)
}

fn live_init(
    context: &Handle<StepBasicProductContext>,
    name: &Handle<HAsciiString>,
    ac: &Handle<StepBasicApplicationContext>,
    discipline: &Handle<HAsciiString>,
) -> Result<(), Failure> {
    step_live(context, WHERE)?.init(name, ac, discipline);
    Ok(())
}

/// The product definition of a new part, on its formation and context.
fn new_definition(
    product: &Handle<StepBasicProduct>,
    ac: &Handle<StepBasicApplicationContext>,
    schema: i32,
) -> Result<Handle<StepBasicProductDefinition>, Failure> {
    let formation = new_formation(schema);
    step_live(&formation, WHERE)?.init(&text(""), &text(""), product);
    let (context, context_name) = new_definition_context(schema);
    step_live(&context, WHERE)?.init(&context_name, ac, &text("design"));
    let definition = Handle::new(StepBasicProductDefinition::new());
    definition.init(&text("design"), &text(""), &formation, &context);
    Ok(definition)
}

impl StepConstructPart {
    /// `StepConstructPart()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `make_sdr`: a new SDR with its product structure, and its category, for the current schema.
    pub fn make_sdr(
        &mut self,
        sr: &Handle<StepShapeShapeRepresentation>,
        name: &Handle<HAsciiString>,
        ac: &Handle<StepBasicApplicationContext>,
        step_model: &Handle<StepDataStepModel>,
    ) -> Result<(), Failure> {
        // get current schema
        let schema = step_live(step_model, "StepConstructPart::MakeSDR")?
            .internal_parameters()
            .write_schema
            .as_raw();
        let product = new_product(name, ac, schema)?;
        let definition = new_definition(&product, ac, schema)?;
        let mut characterized = StepReprCharacterizedDefinition::default();
        characterized.set_value(&definition.cast())?;
        let pds = Handle::new(StepReprProductDefinitionShape::new());
        pds.init(&text(""), true, &text(""), &characterized);
        // finally, create SDR
        let mut represented = StepReprRepresentedDefinition::default();
        represented.set_value(&pds.cast())?;
        let sdr = Handle::new(StepShapeShapeDefinitionRepresentation::new());
        sdr.init(&represented, &sr.cast());
        self.my_sdr = sdr;
        // and an associated PRPC
        let (category, category_name) = new_category(schema);
        let products = step_new_array1::<Handle<StepBasicProduct>>(1, 1)?;
        step_array1_set(&products, 1, product, WHERE)?;
        step_live(&category, WHERE)?.init(&category_name, false, &Handle::default(), &products);
        self.my_prpc = category;
        self.my_done = true;
        Ok(())
    }
    /// `read_sdr`.
    pub fn read_sdr(&mut self, shape: &Handle<StepShapeShapeDefinitionRepresentation>) {
        self.my_sdr = shape.clone();
        self.my_done = self.my_sdr.is_some();
    }
    /// `is_done`.
    pub fn is_done(&self) -> bool {
        self.my_done
    }
    /// `SDRValue`: the SDR, null if not done.
    pub fn sdr_value(&self) -> Handle<StepShapeShapeDefinitionRepresentation> {
        self.my_sdr.clone()
    }
    /// `SRValue`: `SDR->UsedRepresentation()`, null if not done.
    pub fn sr_value(&self) -> Result<Handle<StepShapeShapeRepresentation>, Failure> {
        if !self.my_done {
            return Ok(Handle::default());
        }
        Ok(step_live(&self.my_sdr, WHERE)?.used_representation().cast())
    }
    /// `SDR->Definition().PropertyDefinition()`.
    fn property(&self) -> Result<Handle<StepReprPropertyDefinition>, Failure> {
        Ok(step_live(&self.my_sdr, WHERE)?
            .definition()
            .property_definition())
    }
    /// `PD`.
    pub fn pd(&self) -> Result<Handle<StepBasicProductDefinition>, Failure> {
        Ok(step_live(&self.property()?, WHERE)?
            .definition()
            .product_definition())
    }
    /// `PDS`.
    pub fn pds(&self) -> Result<Handle<StepReprProductDefinitionShape>, Failure> {
        Ok(self.property()?.cast())
    }
    /// `PDF`.
    pub fn pdf(&self) -> Result<Handle<StepBasicProductDefinitionFormation>, Failure> {
        Ok(step_live(&self.pd()?, WHERE)?.formation())
    }
    /// `PDC`.
    pub fn pdc(&self) -> Result<Handle<StepBasicProductDefinitionContext>, Failure> {
        Ok(step_live(&self.pd()?, WHERE)?.frame_of_reference())
    }
    /// `Product`.
    pub fn product(&self) -> Result<Handle<StepBasicProduct>, Failure> {
        Ok(step_live(&self.pdf()?, WHERE)?.of_product())
    }
    /// `PC`.
    pub fn pc(&self) -> Result<Handle<StepBasicProductContext>, Failure> {
        step_live(&self.product()?, WHERE)?.frame_of_reference_value(1)
    }
    /// `AC`.
    pub fn ac(&self) -> Result<Handle<StepBasicApplicationContext>, Failure> {
        Ok(step_live(&self.pc()?, WHERE)?.frame_of_reference())
    }
    /// `PRPC`.
    pub fn prpc(&self) -> Handle<StepBasicProductRelatedProductCategory> {
        self.my_prpc.clone()
    }
    /// The category as the owner of its text fields.
    fn category(&self) -> Result<Handle<StepBasicProductRelatedProductCategory>, Failure> {
        Ok(self.my_prpc.clone())
    }
    part_text_fields! {
        /// `PCname` and `SetPCname`.
        pc: pc_name = name, set_pc_name = set_name;
        /// `PCdisciplineType` and `SetPCdisciplineType`.
        pc: pc_discipline_type = discipline_type, set_pc_discipline_type = set_discipline_type;
        /// `ACapplication` and `SetACapplication`.
        ac: ac_application = application, set_ac_application = set_application;
        /// `PDCname` and `SetPDCname`.
        pdc: pdc_name = name, set_pdc_name = set_name;
        /// `PDCstage` and `SetPDCstage`.
        pdc: pdc_stage = life_cycle_stage, set_pdc_stage = set_life_cycle_stage;
        /// `Pid` and `SetPid`.
        product: pid = id, set_pid = set_id;
        /// `Pname` and `SetPname`.
        product: pname = name, set_pname = set_name;
        /// `Pdescription` and `SetPdescription`.
        product: pdescription = description, set_pdescription = set_description;
        /// `PDFid` and `SetPDFid`.
        pdf: pdf_id = id, set_pdf_id = set_id;
        /// `PDFdescription` and `SetPDFdescription`.
        pdf: pdf_description = description, set_pdf_description = set_description;
        /// `PDdescription` and `SetPDdescription`.
        pd: pd_description = description, set_pd_description = set_description;
        /// `PDSname` and `SetPDSname`.
        property: pds_name = name, set_pds_name = set_name;
        /// `PDSdescription` and `SetPDSdescription`.
        property: pds_description = description, set_pds_description = set_description;
        /// `PRPCname` and `SetPRPCname`.
        category: prpc_name = name, set_prpc_name = set_name;
        /// `PRPCdescription` and `SetPRPCdescription`.
        category: prpc_description = description, set_prpc_description = set_description;
    }
}
