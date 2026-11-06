//! `StepDataPDescr`: the description of a parameter (type, arity, select members,
//! enumeration values), used by the late-binding reader and writer.
use super::{StepDataEDescr, StepDataEnumTool, StepDataField, StepDataFieldKind as Kind};
use martensite_tkernel::{
    impl_transient,
    standard::{Handle, RuntimeType, SharedState, StandardTransient, WeakHandle},
    tcollection::AsciiString,
};
use martensite_tkxsbase::interface::InterfaceCheck;
use std::sync::OnceLock;

#[derive(Default)]
struct PDescrState {
    thename: String,
    thesel: i32,
    thesnam: String,
    thenext: Handle<StepDataPDescr>,
    thekind: Kind,
    theenum: StepDataEnumTool,
    thetype: Option<&'static RuntimeType>,
    thednam: String,
    thearit: i32,
    thefrom: Handle<StepDataPDescr>,
    theopt: bool,
    theder: bool,
    thefnam: String,
    thefnum: i32,
}

/// `StepDataPDescr`. [`StepDataPDescr::new`] gives the handle the members returning `this`
/// need (ocaf-design.md section 2); an unbound value gives null there.
///
/// ```
/// use martensite_tkdestep::step_data::StepDataPDescr;
/// let descr = StepDataPDescr::new();
/// descr.set_name("radius");
/// descr.set_real();
/// assert!(descr.is_real() && descr.name() == "radius");
/// ```
#[derive(Default)]
pub struct StepDataPDescr {
    transient: StandardTransient,
    state: SharedState<PDescrState>,
    this: OnceLock<WeakHandle<StepDataPDescr>>,
}
impl_transient!(
    StepDataPDescr,
    "StepDataPDescr",
    StandardTransient,
    transient
);

impl StepDataPDescr {
    /// `StepDataPDescr()`, as a bound handle.
    pub fn new() -> Handle<Self> {
        let descr = Handle::new(Self::default());
        if let Some(live) = descr.get() {
            let _ = live.this.set(descr.downgrade());
        }
        descr
    }
    fn this(&self) -> Handle<Self> {
        self.this.get().map(WeakHandle::upgrade).unwrap_or_default()
    }
    /// `set_name(name)`.
    pub fn set_name(&self, name: &str) {
        self.state.write().thename = name.to_owned();
    }
    /// `name`.
    pub fn name(&self) -> String {
        self.state.read().thename.clone()
    }
    fn kind(&self) -> Kind {
        self.state.read().thekind
    }
    /// `set_select`.
    pub fn set_select(&self) {
        self.state.write().thesel = 4;
    }
    /// `add_member(member)`: chains it and updates the select state.
    pub fn add_member(&self, member: &Handle<StepDataPDescr>) {
        if member.is_null() {
            return;
        }
        let next = self.state.read().thenext.clone();
        let chained = if let Some(live) = next.get() {
            live.add_member(member);
            next.clone()
        } else {
            self.state.write().thenext = member.clone();
            member.clone()
        };
        let next_kind = chained.get().map_or(Kind::UNDEFINED, Self::kind);
        let mut state = self.state.write();
        if state.thesel == 3 {
            return;
        }
        let immediate = state.thekind < Kind::ENTITY;
        if immediate && (next_kind >= Kind::ENTITY || state.thesel == 2 || state.thesel == 4) {
            state.thesel = 3;
        } else if state.thekind >= Kind::ENTITY && (state.thesel == 1 || state.thesel == 4) {
            state.thesel = 2;
        }
    }
    /// `set_member_name(memname)`.
    pub fn set_member_name(&self, memname: &str) {
        self.state.write().thesnam = memname.to_owned();
    }
    /// `set_integer`.
    pub fn set_integer(&self) {
        self.state.write().thekind = Kind::INTEGER;
    }
    /// `set_real`.
    pub fn set_real(&self) {
        self.state.write().thekind = Kind::REAL;
    }
    /// `set_string`.
    pub fn set_string(&self) {
        self.state.write().thekind = Kind::STRING;
    }
    /// `set_boolean`.
    pub fn set_boolean(&self) {
        self.state.write().thekind = Kind::BOOLEAN;
    }
    /// `set_logical`.
    pub fn set_logical(&self) {
        self.state.write().thekind = Kind::LOGICAL;
    }
    /// `set_enum`.
    pub fn set_enum(&self) {
        self.state.write().thekind = Kind::ENUM;
    }
    /// `add_enum_def(enumdef)`.
    pub fn add_enum_def(&self, enumdef: &str) {
        self.state.write().theenum.add_definition(enumdef);
    }
    /// `set_type(atype)`.
    pub fn set_type(&self, atype: &'static RuntimeType) {
        let mut state = self.state.write();
        state.thekind = Kind::ENTITY;
        state.thetype = Some(atype);
        state.thednam.clear();
    }
    /// `set_descr(dscnam)`.
    pub fn set_descr(&self, dscnam: &str) {
        let mut state = self.state.write();
        state.thekind = Kind::ENTITY;
        state.thetype = None;
        state.thednam = dscnam.to_owned();
    }
    /// `add_arity(arity = 1)`.
    pub fn add_arity(&self, arity: i32) {
        self.state.write().thearit += arity;
    }
    /// `set_arity(arity = 1)`.
    pub fn set_arity(&self, arity: i32) {
        self.state.write().thearit = arity;
    }
    /// `set_from(other)`: takes its definition; the select types are not copied.
    pub fn set_from(&self, other: &Handle<StepDataPDescr>) {
        let Some(live) = other.get() else {
            return;
        };
        for rank in 0..=live.enum_max() {
            let text = live.enum_text(rank);
            self.add_enum_def(&String::from_utf8_lossy(text.as_bytes()));
        }
        let source = live.state.read();
        let mut state = self.state.write();
        state.thekind = source.thekind;
        state.thetype = source.thetype;
        state.thearit = source.thearit;
        state.thefrom = other.clone();
        state.theopt = source.theopt;
        state.theder = source.theder;
        state.thefnam = source.thefnam.clone();
        state.thefnum = source.thefnum;
    }
    /// `set_optional(opt = true)`.
    pub fn set_optional(&self, opt: bool) {
        self.state.write().theopt = opt;
    }
    /// `set_derived(der = true)`.
    pub fn set_derived(&self, der: bool) {
        self.state.write().theder = der;
    }
    /// `set_field(name, rank)`.
    pub fn set_field(&self, name: &str, rank: i32) {
        let mut state = self.state.write();
        state.thefnam = name.to_owned();
        state.thefnum = rank;
    }
    /// `is_select`.
    pub fn is_select(&self) -> bool {
        let from = self.state.read().thefrom.clone();
        match from.get() {
            Some(live) => live.is_select(),
            None => self.state.read().thesel > 0,
        }
    }
    /// `member(name)`: this description or the chained one with this member name.
    pub fn member(&self, name: &str) -> Handle<StepDataPDescr> {
        let from = self.state.read().thefrom.clone();
        if let Some(live) = from.get() {
            return live.member(name);
        }
        if self.state.read().thesnam == name {
            return self.this();
        }
        let next = self.state.read().thenext.clone();
        next.get()
            .map_or_else(Handle::default, |live| live.member(name))
    }
    /// `is_integer`.
    pub fn is_integer(&self) -> bool {
        self.kind() == Kind::INTEGER
    }
    /// `is_real`.
    pub fn is_real(&self) -> bool {
        self.kind() == Kind::REAL
    }
    /// `is_string`.
    pub fn is_string(&self) -> bool {
        self.kind() == Kind::STRING
    }
    /// `is_boolean`.
    pub fn is_boolean(&self) -> bool {
        matches!(self.kind(), Kind::BOOLEAN | Kind::LOGICAL)
    }
    /// `is_logical`.
    pub fn is_logical(&self) -> bool {
        self.kind() == Kind::LOGICAL
    }
    /// `is_enum`.
    pub fn is_enum(&self) -> bool {
        self.kind() == Kind::ENUM
    }
    /// `enum_max`.
    pub fn enum_max(&self) -> i32 {
        self.state.read().theenum.max_value()
    }
    /// `enum_value(name)`.
    pub fn enum_value(&self, name: &[u8]) -> i32 {
        self.state.read().theenum.value(name)
    }
    /// `enum_text(val)`.
    pub fn enum_text(&self, val: i32) -> AsciiString {
        self.state.read().theenum.text(val)
    }
    /// `is_entity`.
    pub fn is_entity(&self) -> bool {
        self.kind() == Kind::ENTITY
    }
    /// `is_type(atype)`: a sub-type of its type, or of a chained or source description's.
    pub fn is_type(&self, atype: &RuntimeType) -> bool {
        let (own, next, from) = {
            let state = self.state.read();
            (state.thetype, state.thenext.clone(), state.thefrom.clone())
        };
        if own.is_some_and(|kind| atype.is_subtype(kind)) {
            return true;
        }
        if let Some(live) = next.get() {
            return live.is_type(atype);
        }
        from.get().is_some_and(|live| live.is_type(atype))
    }
    /// `Type`.
    pub fn descr_type(&self) -> Option<&'static RuntimeType> {
        self.state.read().thetype
    }
    /// `is_descr(descr)`.
    pub fn is_descr(&self, descr: &Handle<dyn StepDataEDescr>) -> bool {
        let Some(wanted) = descr.get() else {
            return false;
        };
        let (name, next, from) = {
            let state = self.state.read();
            (
                state.thednam.clone(),
                state.thenext.clone(),
                state.thefrom.clone(),
            )
        };
        if !name.is_empty() && wanted.matches(&name) {
            return true;
        }
        if let Some(live) = next.get() {
            return live.is_descr(descr);
        }
        from.get().is_some_and(|live| live.is_descr(descr))
    }
    /// `descr_name`.
    pub fn descr_name(&self) -> String {
        self.state.read().thednam.clone()
    }
    /// `arity`.
    pub fn arity(&self) -> i32 {
        self.state.read().thearit
    }
    /// `simple`: its source when it adds an arity, else itself.
    pub fn simple(&self) -> Handle<StepDataPDescr> {
        let state = self.state.read();
        if state.thearit == 0 || state.thefrom.is_null() {
            return self.this();
        }
        state.thefrom.clone()
    }
    /// `is_optional`.
    pub fn is_optional(&self) -> bool {
        self.state.read().theopt
    }
    /// `is_derived`.
    pub fn is_derived(&self) -> bool {
        self.state.read().theder
    }
    /// `is_field`.
    pub fn is_field(&self) -> bool {
        self.state.read().thefnum > 0
    }
    /// `field_name`.
    pub fn field_name(&self) -> String {
        self.state.read().thefnam.clone()
    }
    /// `field_rank`.
    pub fn field_rank(&self) -> i32 {
        self.state.read().thefnum
    }
    /// `Check(afild, ach)`: nothing.
    pub fn check(&self, _afild: &StepDataField, _ach: &Handle<InterfaceCheck>) {}
}
