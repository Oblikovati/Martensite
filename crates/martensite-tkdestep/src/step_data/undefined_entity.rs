//! `StepDataUndefinedEntity`: an entity the protocol does not know, its type read from the
//! file and its parameters kept in an `InterfaceUndefinedContent`.
use super::{StepDataStepReaderData, StepDataStepWriter};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, SharedState, StandardTransient},
    tcollection::{AsciiString, HAsciiString},
};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceFileReaderDataParams,
    InterfaceParamType, InterfaceUndefinedContent,
};

#[derive(Default)]
struct UndefinedState {
    thetype: Handle<HAsciiString>,
    thecont: Handle<InterfaceUndefinedContent>,
    thesub: bool,
    thenext: Handle<StepDataUndefinedEntity>,
}

/// ```
/// use martensite_tkdestep::step_data::StepDataUndefinedEntity;
/// let entity = StepDataUndefinedEntity::new(true);
/// assert!(entity.is_sub() && !entity.is_complex() && entity.step_type().is_empty());
/// ```
#[derive(Default)]
pub struct StepDataUndefinedEntity {
    transient: StandardTransient,
    state: SharedState<UndefinedState>,
}
impl_transient!(
    StepDataUndefinedEntity,
    "StepDataUndefinedEntity",
    StandardTransient,
    transient
);

impl StepDataUndefinedEntity {
    /// `StepDataUndefinedEntity(issub = false)`.
    pub fn new(issub: bool) -> Self {
        Self {
            transient: StandardTransient::default(),
            state: SharedState::new(UndefinedState {
                thetype: Handle::default(),
                thecont: Handle::new(InterfaceUndefinedContent::new()),
                thesub: issub,
                thenext: Handle::default(),
            }),
        }
    }
    /// `undefined_content`.
    pub fn undefined_content(&self) -> Handle<InterfaceUndefinedContent> {
        self.state.read().thecont.clone()
    }
    /// `is_sub`.
    pub fn is_sub(&self) -> bool {
        self.state.read().thesub
    }
    /// `is_complex`.
    pub fn is_complex(&self) -> bool {
        self.state.read().thenext.is_some()
    }
    /// `Next`.
    pub fn next(&self) -> Handle<StepDataUndefinedEntity> {
        self.state.read().thenext.clone()
    }
    /// `step_type`.
    pub fn step_type(&self) -> AsciiString {
        let state = self.state.read();
        state
            .thetype
            .get()
            .map(|text| text.string().clone())
            .unwrap_or_default()
    }
    /// `read_record(SR, num, ach)`.
    pub fn read_record(
        &self,
        sr: &StepDataStepReaderData,
        num: i32,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let recorded = sr.record_type(num);
        self.state.write().thetype = Handle::new(HAsciiString::from_string(&recorded));
        let content = self.undefined_content();
        let nb = sr.nb_params(num)?;
        content.reservate(nb, 4)?;
        for rank in 1..=nb {
            read_param(sr, num, rank, ach, &content)?;
        }
        let nextyp = sr.next_for_complex(num);
        if nextyp == 0 {
            return Ok(());
        }
        let next = Self::new(false);
        next.read_record(sr, nextyp, ach)?;
        self.state.write().thenext = Handle::new(next);
        Ok(())
    }
    /// `write_params(SW)`.
    pub fn write_params(&self, sw: &mut StepDataStepWriter) -> Result<(), Failure> {
        if !self.is_sub() {
            sw.start_entity(&self.step_type())?;
        }
        let content = self.undefined_content();
        for rank in 1..=content.nb_params() {
            match content.param_type(rank)? {
                InterfaceParamType::ParamSub => {
                    let sub = content
                        .param_entity(rank)?
                        .cast::<StepDataUndefinedEntity>();
                    let live = sub.get().ok_or_else(null_sub_entity)?;
                    if live.is_sub() {
                        sw.open_typed_sub(live.step_type().as_bytes())?;
                    }
                    live.write_params(sw)?;
                    if live.is_sub() {
                        sw.close_sub()?;
                    }
                }
                InterfaceParamType::ParamIdent => sw.send_entity(&content.param_entity(rank)?)?,
                _ => {
                    let value = content.param_value(rank)?;
                    let text = value.get().map(|v| v.string().clone()).unwrap_or_default();
                    sw.send_string(text.as_bytes())?;
                }
            }
        }
        let next = self.next();
        next.get().map_or(Ok(()), |live| live.write_params(sw))
    }
    /// `get_from_another(other, TC)`.
    pub fn get_from_another(
        &self,
        other: &Handle<StepDataUndefinedEntity>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let live = other.get().ok_or_else(null_sub_entity)?;
        let content = Handle::new(InterfaceUndefinedContent::new());
        content.get_from_another(&live.undefined_content(), tc)?;
        let next = if live.is_complex() {
            tc.transferred(&live.next().cast())?.cast()
        } else {
            Handle::default()
        };
        let mut state = self.state.write();
        state.thetype = Handle::new(HAsciiString::from_string(&live.step_type()));
        state.thecont = content;
        state.thesub = live.is_sub();
        state.thenext = next;
        Ok(())
    }
    /// `fill_shared(list)`.
    pub fn fill_shared(&self, list: &mut InterfaceEntityIterator) -> Result<(), Failure> {
        let content = self.undefined_content();
        for rank in 1..=content.nb_params() {
            match content.param_type(rank)? {
                InterfaceParamType::ParamSub => {
                    let sub = content
                        .param_entity(rank)?
                        .cast::<StepDataUndefinedEntity>();
                    sub.get().ok_or_else(null_sub_entity)?.fill_shared(list)?;
                }
                InterfaceParamType::ParamIdent => list.add_item(&content.param_entity(rank)?),
                _ => {}
            }
        }
        let next = self.next();
        next.get().map_or(Ok(()), |live| live.fill_shared(list))
    }
}

fn null_sub_entity() -> Failure {
    martensite_tkernel::standard::exceptions::null_object(
        "StepDataUndefinedEntity: null entity; expected an undefined entity",
    )
}

/// One parameter of `read_record`.
fn read_param(
    sr: &StepDataStepReaderData,
    num: i32,
    rank: i32,
    ach: &Handle<InterfaceCheck>,
    content: &InterfaceUndefinedContent,
) -> Result<(), Failure> {
    let value = sr.param_c_value(num, rank)?;
    let mut partyp = sr.param_type(num, rank)?;
    let mut anent = Handle::<StandardTransient>::default();
    let mut nume = 0;
    let mut hval = Handle::<HAsciiString>::default();
    match partyp {
        InterfaceParamType::ParamIdent => {
            nume = sr.param_number(num, rank)?;
            if nume > 0 {
                anent = sr.bound_entity(nume);
                if anent.is_null() {
                    nume = 0;
                }
            }
            if nume <= 0 {
                ach.add_fail_text("A reference to another entity is unresolved", "");
                partyp = InterfaceParamType::ParamVoid;
            }
        }
        InterfaceParamType::ParamSub => {
            nume = sr.param_number(num, rank)?;
            let sub = StepDataUndefinedEntity::new(true);
            sub.read_record(sr, nume, ach)?;
            anent = Handle::new(sub).cast();
        }
        InterfaceParamType::ParamText if value.contains(&b'\n') => {
            nume = -1;
            let cleaned: Vec<u8> = value.iter().copied().filter(|b| *b != b'\n').collect();
            hval = Handle::new(HAsciiString::new(cleaned));
        }
        _ => {}
    }
    if nume == 0 {
        hval = Handle::new(HAsciiString::new(&value));
    }
    if nume > 0 {
        return content.add_entity(partyp, &anent);
    }
    content.add_literal(partyp, &hval)
}
