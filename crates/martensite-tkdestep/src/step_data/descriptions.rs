//! The entity descriptions of the late-binding mechanism: `StepDataEDescr` (abstract),
//! `StepDataESDescr` (a simple type) and `StepDataECDescr` (a complex type).
use super::{StepDataDescribed, StepDataPDescr, StepDataPlex, StepDataSimple};
use martensite_tkernel::{
    impl_transient, impl_transient_interface,
    ncollection::DataMap,
    standard::{Handle, SharedState, StandardTransient, Transient, WeakHandle},
    tcollection::AsciiString,
};
use std::sync::OnceLock;

/// `StepDataEDescr`, abstract.
///
/// ```
/// use martensite_tkdestep::step_data::{StepDataESDescr, StepDataEDescr};
/// assert!(StepDataESDescr::new("POINT").get().unwrap().matches("POINT"));
/// ```
pub trait StepDataEDescr: Transient {
    /// `matches(steptype)`.
    fn matches(&self, steptype: &str) -> bool;
    /// `is_complex`.
    fn is_complex(&self) -> bool;
    /// `new_entity`.
    fn new_entity(&self) -> Handle<dyn StepDataDescribed>;
}
impl_transient_interface!(dyn StepDataEDescr, "StepDataEDescr", StandardTransient);

#[derive(Default)]
struct SimpleDescrState {
    thedescr: Option<Vec<Handle<StepDataPDescr>>>,
    thenames: DataMap<AsciiString, i32>,
    thebase: Handle<StepDataESDescr>,
    thesuper: Handle<StepDataESDescr>,
}

/// `StepDataESDescr`: built as a bound handle, because `new_entity` and `is_sub` use `this`.
///
/// ```
/// use martensite_tkdestep::step_data::StepDataESDescr;
/// let descr = StepDataESDescr::new("POINT");
/// let live = descr.get().unwrap();
/// live.set_nb_fields(1);
/// assert_eq!(live.nb_fields(), 1);
/// ```
pub struct StepDataESDescr {
    transient: StandardTransient,
    thenom: String,
    state: SharedState<SimpleDescrState>,
    this: OnceLock<WeakHandle<StepDataESDescr>>,
}
impl_transient!(
    StepDataESDescr,
    "StepDataESDescr",
    dyn StepDataEDescr,
    transient,
    interfaces: [dyn StepDataEDescr]
);

impl StepDataESDescr {
    /// `StepDataESDescr(name)`.
    pub fn new(name: &str) -> Handle<Self> {
        let descr = Handle::new(Self {
            transient: StandardTransient::default(),
            thenom: name.to_owned(),
            state: SharedState::new(SimpleDescrState::default()),
            this: OnceLock::new(),
        });
        if let Some(live) = descr.get() {
            let _ = live.this.set(descr.downgrade());
        }
        descr
    }
    fn this(&self) -> Handle<Self> {
        self.this.get().map(WeakHandle::upgrade).unwrap_or_default()
    }
    /// `set_nb_fields(nb)`: keeps the existing descriptions, rebuilding the names.
    pub fn set_nb_fields(&self, nb: i32) {
        let mut state = self.state.write();
        state.thenames.clear(false);
        if nb == 0 {
            state.thedescr = None;
            return;
        }
        let mut list = vec![Handle::default(); nb.max(0) as usize];
        if let Some(old) = state.thedescr.take() {
            for (rank, descr) in old.into_iter().enumerate().take(list.len()) {
                if let Some(live) = descr.get() {
                    let name = AsciiString::new(&live.name());
                    state.thenames.bind(name, rank as i32 + 1);
                }
                list[rank] = descr;
            }
        }
        state.thedescr = Some(list);
    }
    /// `set_field(num, name, descr)`: a named copy of `descr`; ignored out of range.
    pub fn set_field(&self, num: i32, name: &str, descr: &Handle<StepDataPDescr>) {
        if num < 1 || num > self.nb_fields() {
            return;
        }
        let field = StepDataPDescr::new();
        if let Some(live) = field.get() {
            live.set_from(descr);
            live.set_name(name);
        }
        let mut state = self.state.write();
        if let Some(list) = state.thedescr.as_mut() {
            list[num as usize - 1] = field;
        }
        state.thenames.bind(AsciiString::new(name), num);
    }
    /// `set_base(base)`.
    pub fn set_base(&self, base: &Handle<StepDataESDescr>) {
        self.state.write().thebase = base.clone();
    }
    /// `set_super(super)`.
    pub fn set_super(&self, parent: &Handle<StepDataESDescr>) {
        let Some(live) = parent.get() else {
            return;
        };
        let mut sup = live.base();
        if sup.is_null() {
            sup = parent.clone();
        }
        let base = self.base();
        match base.get() {
            Some(own) => own.set_super(&sup),
            None => self.state.write().thesuper = sup,
        }
    }
    /// `type_name`.
    pub fn type_name(&self) -> &str {
        &self.thenom
    }
    /// `step_type`.
    pub fn step_type(&self) -> AsciiString {
        AsciiString::new(&self.thenom)
    }
    /// `Base`.
    pub fn base(&self) -> Handle<StepDataESDescr> {
        self.state.read().thebase.clone()
    }
    /// `Super`.
    pub fn parent(&self) -> Handle<StepDataESDescr> {
        self.state.read().thesuper.clone()
    }
    /// `is_sub(other)`.
    pub fn is_sub(&self, other: &Handle<StepDataESDescr>) -> bool {
        let Some(live) = other.get() else {
            return false;
        };
        let mut oth = live.base();
        if oth.is_null() {
            oth = other.clone();
        }
        let base = self.base();
        if let Some(own) = base.get() {
            return own.is_sub(&oth);
        }
        if oth == self.this() {
            return true;
        }
        let parent = self.parent();
        if oth == parent {
            return true;
        }
        parent.get().is_some_and(|own| own.is_sub(&oth))
    }
    /// `nb_fields`.
    pub fn nb_fields(&self) -> i32 {
        self.state
            .read()
            .thedescr
            .as_ref()
            .map_or(0, |list| list.len() as i32)
    }
    /// `rank(name)`: 0 when absent.
    pub fn rank(&self, name: &str) -> i32 {
        let state = self.state.read();
        state
            .thenames
            .find(&AsciiString::new(name))
            .copied()
            .unwrap_or(0)
    }
    /// `name(num)`: empty out of range.
    pub fn name(&self, num: i32) -> String {
        if num < 1 || num > self.nb_fields() {
            return String::new();
        }
        self.field(num)
            .get()
            .map(StepDataPDescr::name)
            .unwrap_or_default()
    }
    /// `Field(num)`: null out of range (an out of range failure, which no caller
    /// reaches: they iterate up to `nb_fields`).
    pub fn field(&self, num: i32) -> Handle<StepDataPDescr> {
        let state = self.state.read();
        let list = state.thedescr.as_ref();
        usize::try_from(num - 1)
            .ok()
            .and_then(|rank| list.and_then(|list| list.get(rank).cloned()))
            .unwrap_or_default()
    }
    /// `named_field(name)`.
    pub fn named_field(&self, name: &str) -> Handle<StepDataPDescr> {
        let rank = self.rank(name);
        if rank > 0 {
            return self.field(rank);
        }
        Handle::default()
    }
}

impl StepDataEDescr for StepDataESDescr {
    fn matches(&self, steptype: &str) -> bool {
        if self.thenom == steptype {
            return true;
        }
        let parent = self.parent();
        parent.get().is_some_and(|live| live.matches(steptype))
    }
    fn is_complex(&self) -> bool {
        false
    }
    fn new_entity(&self) -> Handle<dyn StepDataDescribed> {
        Handle::new(StepDataSimple::new(&self.this())).cast()
    }
}

/// `StepDataECDescr`: its members are sorted by type name.
///
/// ```
/// use martensite_tkdestep::step_data::{StepDataECDescr, StepDataESDescr};
/// let descr = StepDataECDescr::new();
/// let live = descr.get().unwrap();
/// live.add(&StepDataESDescr::new("B"));
/// live.add(&StepDataESDescr::new("A"));
/// assert_eq!(live.type_list()[0].as_bytes(), b"A");
/// ```
#[derive(Default)]
pub struct StepDataECDescr {
    transient: StandardTransient,
    thelist: SharedState<Vec<Handle<StepDataESDescr>>>,
    this: OnceLock<WeakHandle<StepDataECDescr>>,
}
impl_transient!(
    StepDataECDescr,
    "StepDataECDescr",
    dyn StepDataEDescr,
    transient,
    interfaces: [dyn StepDataEDescr]
);

impl StepDataECDescr {
    /// `StepDataECDescr()`.
    pub fn new() -> Handle<Self> {
        let descr = Handle::new(Self::default());
        if let Some(live) = descr.get() {
            let _ = live.this.set(descr.downgrade());
        }
        descr
    }
    /// `add(member)`. Order kept: it inserts before the last member (scanning from
    /// the end) whose name is above the new one.
    pub fn add(&self, member: &Handle<StepDataESDescr>) {
        let Some(live) = member.get() else {
            return;
        };
        let name = live.type_name().to_owned();
        let mut list = self.thelist.write();
        for rank in (0..list.len()).rev() {
            let above = list[rank]
                .get()
                .is_some_and(|known| name.as_str() < known.type_name());
            if above {
                list.insert(rank, member.clone());
                return;
            }
        }
        list.push(member.clone());
    }
    /// `nb_members`.
    pub fn nb_members(&self) -> i32 {
        self.thelist.read().len() as i32
    }
    /// `member(num)`: null out of range.
    pub fn member(&self, num: i32) -> Handle<StepDataESDescr> {
        let list = self.thelist.read();
        usize::try_from(num - 1)
            .ok()
            .and_then(|rank| list.get(rank).cloned())
            .unwrap_or_default()
    }
    /// `type_list`.
    pub fn type_list(&self) -> Vec<AsciiString> {
        let list = self.thelist.read();
        list.iter()
            .filter_map(|member| member.get().map(StepDataESDescr::step_type))
            .collect()
    }
}

impl StepDataEDescr for StepDataECDescr {
    fn matches(&self, steptype: &str) -> bool {
        let list = self.thelist.read();
        list.iter()
            .any(|member| member.get().is_some_and(|m| m.matches(steptype)))
    }
    fn is_complex(&self) -> bool {
        true
    }
    fn new_entity(&self) -> Handle<dyn StepDataDescribed> {
        let this = self.this.get().map(WeakHandle::upgrade).unwrap_or_default();
        let plex = StepDataPlex::new(&this);
        for member in self.thelist.read().iter() {
            let entity = member.get().map(|m| m.new_entity()).unwrap_or_default();
            let simple = entity.cast::<StepDataSimple>();
            if simple.is_some() {
                plex.add(&simple);
            }
        }
        Handle::new(plex).cast()
    }
}
