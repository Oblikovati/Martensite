//! `StepDataStepModel`: the entities of a STEP file, its header entities, its labels and
//! its units.
use super::{StepData, StepDataStepWriter};
use crate::de_step::DeStepParameters;
use martensite_tkernel::{
    impl_transient,
    resource::FormatType,
    standard::{Failure, Handle, RuntimeType, SharedState, StandardTransient},
    tcollection::HAsciiString,
    units_methods::LengthUnit,
};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceEntityList,
    InterfaceGeneralLib, InterfaceInterfaceModel, InterfaceModelBase, InterfaceModelEntities,
    InterfaceModelReports, InterfaceShareTool,
};
use std::io::{self, Write};
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

#[derive(Default)]
struct StepModelState {
    theheader: InterfaceEntityList,
    theidnums: Option<Vec<i32>>,
    my_write_unit: Option<f64>,
    my_local_length_unit: Option<f64>,
}

/// `StepDataStepModel`. [`StepDataStepModel::new`] gives the bound handle `dump_header` and
/// `get_from_another` need (`this`).
///
/// ```
/// use martensite_tkdestep::step_data::StepDataStepModel;
/// use martensite_tkxsbase::interface::InterfaceModelEntities;
/// let model = StepDataStepModel::new();
/// assert_eq!(model.nb_entities(), 0);
/// assert_eq!(model.local_length_unit(), 1.0);
/// ```
#[derive(Default)]
pub struct StepDataStepModel {
    transient: StandardTransient,
    base: InterfaceModelBase,
    state: SharedState<StepModelState>,
    parameters: SharedState<DeStepParameters>,
}
impl_transient!(
    StepDataStepModel,
    "StepDataStepModel",
    dyn InterfaceInterfaceModel,
    transient,
    interfaces: [dyn InterfaceInterfaceModel]
);

fn label_text(nid: i32, num: i32) -> String {
    if nid > 0 {
        format!("#{nid}")
    } else if num > 0 {
        format!("(#{num})")
    } else {
        "(#0..)".to_owned()
    }
}

/// The millimeters of a `write.step.unit` value; `None` when it is not a unit.
fn write_unit_of(unit: i32) -> Option<f64> {
    const UNITS: [(LengthUnit, f64); 10] = [
        (LengthUnit::Inch, 25.4),
        (LengthUnit::Millimeter, 1.0),
        (LengthUnit::Foot, 304.8),
        (LengthUnit::Mile, 1609344.0),
        (LengthUnit::Meter, 1000.0),
        (LengthUnit::Kilometer, 1000000.0),
        (LengthUnit::Mil, 0.0254),
        (LengthUnit::Micron, 0.001),
        (LengthUnit::Centimeter, 10.0),
        (LengthUnit::Microinch, 0.0000254),
    ];
    UNITS
        .iter()
        .find(|(known, _)| *known as i32 == unit)
        .map(|(_, value)| *value)
}

impl StepDataStepModel {
    /// `StepDataStepModel()`: an empty model with an empty header, as a bound handle.
    pub fn new() -> Handle<Self> {
        let model = Handle::new(Self::default());
        if let Some(live) = model.get() {
            live.base.bind_self(&model.cast());
        }
        model
    }
    /// `entity(num)`.
    pub fn entity(&self, num: i32) -> Result<Handle<StandardTransient>, Failure> {
        self.value(num)
    }
    /// `Header`.
    pub fn header(&self) -> InterfaceEntityIterator {
        let mut iter = InterfaceEntityIterator::new();
        self.state.read().theheader.fill_iterator(&mut iter);
        iter
    }
    /// `has_header_entity(atype)`: exactly one header entity of this type.
    pub fn has_header_entity(&self, atype: &RuntimeType) -> bool {
        self.state.read().theheader.nb_typed_entities(atype) == 1
    }
    /// `header_entity(atype)`.
    pub fn header_entity(&self, atype: &RuntimeType) -> Result<Handle<StandardTransient>, Failure> {
        self.state.read().theheader.typed_entity(atype, 0)
    }
    /// `add_header_entity(ent)`.
    pub fn add_header_entity(&self, ent: &Handle<StandardTransient>) -> Result<(), Failure> {
        self.state.write().theheader.append(ent)
    }
    /// `set_ident_label(ent, ident)`: nothing when `ent` is not in the model.
    pub fn set_ident_label(&self, ent: &Handle<StandardTransient>, ident: i32) {
        let num = self.number(ent);
        if num == 0 {
            return;
        }
        let count = self.nb_entities().max(0) as usize + 1;
        let mut state = self.state.write();
        let labels = state.theidnums.get_or_insert_with(Vec::new);
        if labels.len() < count {
            labels.resize(count, 0);
        }
        labels[num as usize] = ident;
    }
    /// `ident_label(ent)`: 0 when it is not in the model.
    pub fn ident_label(&self, ent: &Handle<StandardTransient>) -> i32 {
        if self.state.read().theidnums.is_none() {
            return 0;
        }
        self.label_of(self.number(ent))
    }
    fn label_of(&self, num: i32) -> i32 {
        let state = self.state.read();
        let labels = state.theidnums.as_ref();
        usize::try_from(num)
            .ok()
            .and_then(|rank| labels.and_then(|labels| labels.get(rank).copied()))
            .unwrap_or(0)
    }
    fn label_of_entity(&self, ent: &Handle<StandardTransient>) -> String {
        let has_labels = self.state.read().theidnums.is_some();
        let num = if has_labels { self.number(ent) } else { 0 };
        label_text(self.label_of(num), num)
    }
    /// The transfer parameters (the public field `internal_parameters`).
    pub fn internal_parameters(&self) -> RwLockReadGuard<'_, DeStepParameters> {
        self.parameters.read()
    }
    /// The transfer parameters, to change them.
    pub fn internal_parameters_mut(&self) -> RwLockWriteGuard<'_, DeStepParameters> {
        self.parameters.write()
    }
    /// `source_code_page`: the encoding of the file, for converting names into Unicode.
    pub fn source_code_page(&self) -> FormatType {
        let code = self.parameters.read().read_code_page;
        FormatType::ALL
            .iter()
            .copied()
            .find(|known| *known as i32 == code)
            .unwrap_or(FormatType::Utf8)
    }
    /// `set_source_code_page(code)`.
    pub fn set_source_code_page(&self, code: FormatType) {
        self.parameters.write().read_code_page = code as i32;
    }
    /// `set_local_length_unit(unit)`.
    pub fn set_local_length_unit(&self, unit: f64) {
        self.state.write().my_local_length_unit = Some(unit);
    }
    /// `local_length_unit`: 1 by default.
    pub fn local_length_unit(&self) -> f64 {
        self.state.read().my_local_length_unit.unwrap_or(1.0)
    }
    /// `set_write_length_unit(unit)`.
    pub fn set_write_length_unit(&self, unit: f64) {
        self.state.write().my_write_unit = Some(unit);
    }
    /// `write_length_unit`: read once from the parameters; an unknown value warns and gives 1.
    pub fn write_length_unit(&self) -> f64 {
        if let Some(unit) = self.state.read().my_write_unit {
            return unit;
        }
        let unit = self.parameters.read().write_unit;
        let Some(value) = write_unit_of(unit) else {
            self.global_check(true)
                .add_warning_text("Incorrect write.step.unit parameter, use default value", "");
            return 1.0;
        };
        self.state.write().my_write_unit = Some(value);
        value
    }
    /// `is_initialized_unit`: the local length unit was set.
    pub fn is_initialized_unit(&self) -> bool {
        self.state.read().my_local_length_unit.is_some()
    }
}

impl InterfaceInterfaceModel for StepDataStepModel {
    fn model_base(&self) -> &InterfaceModelBase {
        &self.base
    }
    fn clear_labels(&self) {
        self.state.write().theidnums = None;
    }
    fn clear_header(&self) {
        self.state.write().theheader.clear();
    }
    fn get_from_another(&self, other: &Handle<dyn InterfaceInterfaceModel>) -> Result<(), Failure> {
        self.clear_header();
        let another = other.cast::<StepDataStepModel>();
        let Some(live) = another.get() else {
            return Ok(());
        };
        let mut tool = InterfaceCopyTool::with_protocol(
            &self.base.this(),
            &StepData::header_protocol().cast(),
        )?;
        for head in live.header().content() {
            let mut copied = Handle::default();
            if tool.copy(&head, &mut copied, false, false)? && copied.is_some() {
                self.add_header_entity(&copied)?;
            }
        }
        Ok(())
    }
    fn new_empty_model(&self) -> Handle<dyn InterfaceInterfaceModel> {
        Self::new().cast()
    }
    fn verify_check(&self, ach: &mut Handle<InterfaceCheck>) {
        let protocol = StepData::header_protocol();
        let lib = InterfaceGeneralLib::new(&protocol.cast());
        let shares = InterfaceShareTool::with_protocol(&self.base.this(), &protocol.cast());
        let Ok(shares) = shares else {
            return;
        };
        for head in self.header().content() {
            if let Some((module, cn)) = lib.select(&head)
                && let Err(failure) = module.check_case(cn, &head, &shares, ach)
            {
                ach.add_fail_text(&failure.to_string(), "");
            }
        }
    }
    fn dump_header(&self, s: &mut dyn Write, _level: i32) -> io::Result<()> {
        let protocol = StepData::header_protocol();
        if protocol.is_null() {
            writeln!(
                s,
                " -- WARNING : StepModel DumpHeader, Protocol not defined"
            )?;
        }
        let header = self.header().content();
        writeln!(s, " --  Step Model Header : {} Entities :", header.len())?;
        for head in &header {
            let name = head.dynamic_type().map_or("", |kind| kind.name());
            writeln!(s, "  {name}")?;
        }
        if protocol.is_null() || header.is_empty() {
            return Ok(());
        }
        writeln!(
            s,
            " --  --        STEP MODEL    HEADER  CONTENT      --  --"
        )?;
        let name = protocol.dynamic_type().map_or("", |kind| kind.name());
        writeln!(s, " --   Dumped with Protocol : {name}   --")?;
        let mut writer = StepDataStepWriter::new(&self.base.this().cast());
        writer
            .send_model(&protocol, true)
            .map_err(|failure| io::Error::other(failure.to_string()))?;
        writer.print(s)?;
        Ok(())
    }
    fn print_label(&self, ent: &Handle<StandardTransient>, s: &mut dyn Write) -> io::Result<()> {
        s.write_all(self.label_of_entity(ent).as_bytes())
    }
    fn string_label(&self, ent: &Handle<StandardTransient>) -> Handle<HAsciiString> {
        Handle::new(HAsciiString::new(self.label_of_entity(ent)))
    }
}
