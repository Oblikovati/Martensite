//! `StepConstructContextTool`: the global context of writing: the ApplicationProtocolDefinition, one
//! for all products, the context of AP203, the position in the assembly structure and the root
//! entities the Actor needs.
//!
//! :j4 16.03.99 gka S4134
//! abv 18.11.99 renamed from the context tool
use super::{StepConstructAP203Context, StepConstructAssembly, StepConstructPart};
use crate::de_step::DeStepWriteModeStepSchema;
use crate::geom_to_step::GeomToStepMakeAxis2Placement3d;
use crate::step_basic::{StepBasicApplicationContext, StepBasicApplicationProtocolDefinition};
use crate::step_data::{StepDataFactors, StepDataStepModel};
use crate::step_entity::{HSequence, step_live};
use crate::step_geom::StepGeomAxis2Placement3d;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::{AsciiString, HAsciiString};
use martensite_tkxsbase::interface::InterfaceModelEntities;

const WHERE: &str = "StepConstructContextTool";

fn text(value: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(value))
}

/// The year, the status and the schema name of a schema's protocol definition.
fn protocol_fields(schema: i32) -> (i32, &'static str, &'static str) {
    match schema {
        2 => (1998, "draft international standard", "automotive_design"),
        3 => (1994, "international standard", "config_control_design"),
        4 => (2000, "international standard", "automotive_design"),
        5 => (
            2013,
            "international standard",
            "ap242_managed_model_based_3d_engineering",
        ),
        _ => (1997, "committee draft", "automotive_design"),
    }
}

/// The application name a schema writes.
fn application_name(schema: i32) -> &'static str {
    match schema {
        3 => "configuration controlled 3D designs of mechanical parts and assemblies",
        5 => "Managed model based 3d engineering",
        _ => "core data for automotive mechanical design processes",
    }
}

/// ```
/// use martensite_tkdestep::step_construct::StepConstructContextTool;
/// let mut tool = StepConstructContextTool::new();
/// assert!(!tool.is_ap203() && !tool.is_ap214() && !tool.is_ap242());
/// assert_eq!(tool.get_ac_year().unwrap(), 1998);
/// tool.next_level();
/// tool.next_index();
/// assert_eq!((tool.level(), tool.index()), (1, 2));
/// assert_eq!(tool.get_product_name().string().as_bytes(), b"Product 2");
/// ```
#[derive(Clone)]
pub struct StepConstructContextTool {
    my_level: Vec<i32>,
    the_apd: Handle<StepBasicApplicationProtocolDefinition>,
    the_ap203: StepConstructAP203Context,
    my_axis: Handle<StepGeomAxis2Placement3d>,
    my_global_factor: StepDataFactors,
    my_schema: DeStepWriteModeStepSchema,
    my_product_name: AsciiString,
}

impl Default for StepConstructContextTool {
    /// The schema is unset until `set_model`; AP214 CD falls in its `default` branches.
    fn default() -> Self {
        Self {
            my_level: Vec::new(),
            the_apd: Handle::default(),
            the_ap203: StepConstructAP203Context::default(),
            my_axis: Handle::default(),
            my_global_factor: StepDataFactors::default(),
            my_schema: DeStepWriteModeStepSchema::AP214_CD,
            my_product_name: AsciiString::default(),
        }
    }
}

impl StepConstructContextTool {
    /// `StepConstructContextTool()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `StepConstructContextTool(aStepModel)`.
    pub fn with_model(step_model: &Handle<StepDataStepModel>) -> Result<Self, Failure> {
        let mut tool = Self::default();
        tool.set_model(step_model)?;
        Ok(tool)
    }
    /// `set_model`: the protocol definition is the first of that type in the model.
    pub fn set_model(&mut self, step_model: &Handle<StepDataStepModel>) -> Result<(), Failure> {
        self.the_apd = Handle::default();
        let model = step_live(step_model, "StepConstructContextTool::SetModel")?;
        {
            let parameters = model.internal_parameters();
            self.my_schema = parameters.write_schema;
            self.my_product_name = parameters.write_product_name.clone();
        }
        for i in 1..=model.nb_entities() {
            let apd = model
                .value(i)?
                .cast::<StepBasicApplicationProtocolDefinition>();
            if apd.is_some() {
                self.the_apd = apd;
                break;
            }
        }
        Ok(())
    }
    /// `set_global_factor`.
    pub fn set_global_factor(&mut self, global_factor: &StepDataFactors) {
        self.my_global_factor = *global_factor;
    }
    /// `get_apd`.
    pub fn get_apd(&self) -> Handle<StepBasicApplicationProtocolDefinition> {
        self.the_apd.clone()
    }
    /// `add_apd`: fills the protocol definition by the schema, a new one when none or `enforce`.
    pub fn add_apd(&mut self, enforce: bool) -> Result<(), Failure> {
        if self.the_apd.is_null() || enforce {
            self.the_apd = Handle::new(StepBasicApplicationProtocolDefinition::new());
        }
        let schema = self.my_schema.as_raw();
        let apd = step_live(&self.the_apd, WHERE)?;
        let (year, status, schema_name) = protocol_fields(schema); // j4
        apd.set_application_protocol_year(year);
        apd.set_status(&text(status));
        apd.set_application_interpreted_model_schema_name(&text(schema_name));
        if apd.application().is_null() {
            apd.set_application(&Handle::new(StepBasicApplicationContext::new()));
        }
        step_live(&apd.application(), WHERE)?.set_application(&text(application_name(schema)));
        Ok(())
    }
    /// The lowered schema name of the protocol definition equals `name`.
    fn schema_is(&self, name: &str) -> bool {
        let Some(apd) = self.the_apd.get() else {
            return false;
        };
        let schema = apd.application_interpreted_model_schema_name();
        let Some(schema) = schema.get() else {
            return false;
        };
        schema
            .string()
            .as_bytes()
            .eq_ignore_ascii_case(name.as_bytes())
    }
    /// `is_ap203`: the schema name is config_control_design.
    pub fn is_ap203(&self) -> bool {
        self.schema_is("config_control_design")
    }
    /// `is_ap214`: the schema name is automotive_design.
    pub fn is_ap214(&self) -> bool {
        self.schema_is("automotive_design")
    }
    /// `is_ap242`: the schema name is ap242_managed_model_based_3d_engineering.
    pub fn is_ap242(&self) -> bool {
        self.schema_is("ap242_managed_model_based_3d_engineering")
    }
    /// `GetACstatus`.
    pub fn get_ac_status(&self) -> Handle<HAsciiString> {
        self.the_apd
            .get()
            .map_or_else(|| text(""), |apd| apd.status())
    }
    /// `GetACschemaName`.
    pub fn get_ac_schema_name(&self) -> Handle<HAsciiString> {
        self.the_apd.get().map_or_else(
            || text(""),
            |apd| apd.application_interpreted_model_schema_name(),
        )
    }
    /// `GetACyear`.
    pub fn get_ac_year(&self) -> Result<i32, Failure> {
        Ok(self
            .the_apd
            .get()
            .map_or(1998, |apd| apd.application_protocol_year()))
    }
    /// `GetACname`.
    pub fn get_ac_name(&self) -> Handle<HAsciiString> {
        let Some(apd) = self.the_apd.get() else {
            return text("");
        };
        apd.application()
            .get()
            .map_or_else(|| text(""), |context| context.application())
    }
    /// `SetACstatus`.
    pub fn set_ac_status(&self, status: &Handle<HAsciiString>) {
        if let Some(apd) = self.the_apd.get() {
            apd.set_status(status);
        }
    }
    /// `SetACschemaName`.
    pub fn set_ac_schema_name(&self, schema_name: &Handle<HAsciiString>) {
        if let Some(apd) = self.the_apd.get() {
            apd.set_application_interpreted_model_schema_name(schema_name);
        }
    }
    /// `SetACyear`.
    pub fn set_ac_year(&self, year: i32) {
        if let Some(apd) = self.the_apd.get() {
            apd.set_application_protocol_year(year);
        }
    }
    /// `SetACname`.
    pub fn set_ac_name(&self, name: &Handle<HAsciiString>) -> Result<(), Failure> {
        let Some(apd) = self.the_apd.get() else {
            return Ok(());
        };
        if apd.application().is_null() {
            apd.set_application(&Handle::new(StepBasicApplicationContext::new()));
        }
        step_live(&apd.application(), WHERE)?.set_application(name);
        Ok(())
    }
    /// `get_default_axis`: a default axis placement.
    pub fn get_default_axis(&mut self) -> Result<Handle<StepGeomAxis2Placement3d>, Failure> {
        if self.my_axis.is_null() {
            self.my_axis =
                GeomToStepMakeAxis2Placement3d::at_origin(&self.my_global_factor)?.value()?;
        }
        Ok(self.my_axis.clone())
    }
    /// `AP203Context`: the context specific for AP203.
    pub fn ap203_context(&mut self) -> &mut StepConstructAP203Context {
        &mut self.the_ap203
    }
    /// `Level`: the current assembly level.
    pub fn level(&self) -> i32 {
        self.my_level.len() as i32
    }
    /// `next_level`.
    pub fn next_level(&mut self) {
        self.my_level.push(1);
    }
    /// `prev_level`.
    pub fn prev_level(&mut self) {
        self.my_level.pop();
    }
    /// `set_level`.
    pub fn set_level(&mut self, lev: i32) {
        self.my_level.resize(lev.max(0) as usize, 1);
    }
    /// `index`: the index of the assembly component on the current level.
    pub fn index(&self) -> i32 {
        self.my_level.last().copied().unwrap_or(0)
    }
    /// `next_index`.
    pub fn next_index(&mut self) {
        if let Some(last) = self.my_level.last_mut() {
            *last += 1;
        }
    }
    /// `prev_index`.
    pub fn prev_index(&mut self) {
        if let Some(last) = self.my_level.last_mut() {
            *last -= 1;
        }
    }
    /// `set_index`.
    pub fn set_index(&mut self, ind: i32) {
        if let Some(last) = self.my_level.last_mut() {
            *last = ind;
        }
    }
    /// `get_product_name`: from the `WriteProductName` parameter and the position in the assembly.
    pub fn get_product_name(&self) -> Handle<HAsciiString> {
        let mut name = if self.my_product_name.is_empty() {
            String::from("Product")
        } else {
            String::from_utf8_lossy(self.my_product_name.as_bytes()).into_owned()
        };
        for (position, level) in self.my_level.iter().enumerate() {
            name.push_str(if position > 0 { "." } else { " " });
            name.push_str(&level.to_string());
        }
        text(&name)
    }
    /// `get_roots_for_part`: the root entities a part needs, its SDR included.
    pub fn get_roots_for_part(
        &mut self,
        sdr_tool: &StepConstructPart,
    ) -> Result<Handle<HSequence<Handle<StandardTransient>>>, Failure> {
        let seq = Handle::new(HSequence::<Handle<StandardTransient>>::default());
        seq.append(sdr_tool.sdr_value().cast());
        if sdr_tool.prpc().is_some() {
            seq.append(sdr_tool.prpc().cast());
        }
        // for AP203, add required product management data
        if self.my_schema.as_raw() == 3 {
            self.the_ap203.init(sdr_tool)?;
            let context = &self.the_ap203;
            seq.append(context.get_product_category_relationship().cast());
            seq.append(context.get_creator().cast());
            seq.append(context.get_design_owner().cast());
            seq.append(context.get_design_supplier().cast());
            seq.append(context.get_classification_officer().cast());
            seq.append(context.get_security().cast());
            seq.append(context.get_creation_date().cast());
            seq.append(context.get_classification_date().cast());
            seq.append(context.get_approval().cast());
            seq.append(context.get_approver().cast());
            seq.append(context.get_approval_date_time().cast());
        }
        Ok(seq)
    }
    /// `get_roots_for_assembly_link`: the root entities an assembly link needs, NAUO and CDSR included.
    pub fn get_roots_for_assembly_link(
        &mut self,
        assembly: &StepConstructAssembly,
    ) -> Result<Handle<HSequence<Handle<StandardTransient>>>, Failure> {
        let seq = Handle::new(HSequence::<Handle<StandardTransient>>::default());
        seq.append(assembly.item_value());
        // for AP203, write required product management data
        if self.my_schema.as_raw() == 3 {
            self.the_ap203.init_nauo(&assembly.get_nauo()?)?;
            let context = &self.the_ap203;
            seq.append(context.get_security().cast());
            seq.append(context.get_classification_officer().cast());
            seq.append(context.get_classification_date().cast());
            seq.append(context.get_approval().cast());
            seq.append(context.get_approver().cast());
            seq.append(context.get_approval_date_time().cast());
        }
        Ok(seq)
    }
}
