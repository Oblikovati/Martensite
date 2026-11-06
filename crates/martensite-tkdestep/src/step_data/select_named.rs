//! `StepDataSelectNamed`: a named member of any kind; `StepDataSelectArrReal`: a named
//! member holding an array of reals.
use super::{StepDataField, StepDataFieldKind, StepDataSelectMember};
use martensite_tkernel::{
    impl_transient,
    ncollection::HArray1,
    standard::{Handle, SharedState, StandardTransient},
    tcollection::AsciiString,
};

#[derive(Default)]
struct NamedState {
    thename: AsciiString,
    theval: StepDataField,
}

/// ```
/// use martensite_tkdestep::step_data::{StepDataSelectMember, StepDataSelectNamed};
/// let member = StepDataSelectNamed::default();
/// assert!(member.set_name("LENGTH_MEASURE"));
/// member.set_real(2.0);
/// assert!(member.has_name() && member.real() == 2.0);
/// ```
#[derive(Default)]
pub struct StepDataSelectNamed {
    transient: StandardTransient,
    state: SharedState<NamedState>,
}
impl_transient!(
    StepDataSelectNamed,
    "StepDataSelectNamed",
    dyn StepDataSelectMember,
    transient,
    interfaces: [dyn StepDataSelectMember]
);

impl StepDataSelectNamed {
    /// `Field`: a shallow copy (a reference, which cannot outlive the lock).
    pub fn field(&self) -> StepDataField {
        self.state.read().theval.clone()
    }
    /// `CField`: edits the field (a modifiable reference).
    pub fn change_field<R>(&self, edit: impl FnOnce(&mut StepDataField) -> R) -> R {
        edit(&mut self.state.write().theval)
    }
}

impl StepDataSelectMember for StepDataSelectNamed {
    fn has_name(&self) -> bool {
        self.state.read().thename.length() > 0
    }
    fn name(&self) -> String {
        String::from_utf8_lossy(self.state.read().thename.as_bytes()).into_owned()
    }
    fn set_name(&self, name: &str) -> bool {
        self.state.write().thename = AsciiString::new(name);
        true
    }
    fn kind(&self) -> i32 {
        self.state.read().theval.kind(false).to_int()
    }
    fn set_kind(&self, kind: i32) {
        self.state.write().theval.clear_int(kind);
    }
    fn int(&self) -> i32 {
        self.state.read().theval.int()
    }
    fn set_int(&self, val: i32) {
        self.state.write().theval.set_int(val);
    }
    fn real(&self) -> f64 {
        self.state.read().theval.scalar_real()
    }
    fn set_real(&self, val: f64) {
        self.state.write().theval.set_real(val);
    }
    fn string(&self) -> AsciiString {
        self.state.read().theval.scalar_string()
    }
    fn set_string(&self, val: &[u8]) {
        self.state.write().theval.set_string(val);
    }
}

/// `StepDataSelectArrReal`: its kind is always `any`.
///
/// ```
/// use martensite_tkdestep::step_data::{StepDataFieldKind, StepDataSelectArrReal, StepDataSelectMember};
/// assert_eq!(StepDataSelectArrReal::default().kind(), StepDataFieldKind::ANY.to_int());
/// ```
#[derive(Default)]
pub struct StepDataSelectArrReal {
    named: StepDataSelectNamed,
    arr: SharedState<Handle<HArray1<f64>>>,
}
impl_transient!(
    StepDataSelectArrReal,
    "StepDataSelectArrReal",
    StepDataSelectNamed,
    named,
    interfaces: [dyn StepDataSelectMember]
);

impl StepDataSelectArrReal {
    /// `arr_real`.
    pub fn arr_real(&self) -> Handle<HArray1<f64>> {
        self.arr.read().clone()
    }
    /// `set_arr_real(arr)`.
    pub fn set_arr_real(&self, arr: &Handle<HArray1<f64>>) {
        *self.arr.write() = arr.clone();
    }
}

impl StepDataSelectMember for StepDataSelectArrReal {
    fn has_name(&self) -> bool {
        self.named.has_name()
    }
    fn name(&self) -> String {
        self.named.name()
    }
    fn set_name(&self, name: &str) -> bool {
        self.named.set_name(name)
    }
    fn kind(&self) -> i32 {
        StepDataFieldKind::ANY.to_int()
    }
    fn set_kind(&self, kind: i32) {
        self.named.set_kind(kind);
    }
    fn int(&self) -> i32 {
        self.named.int()
    }
    fn set_int(&self, val: i32) {
        self.named.set_int(val);
    }
    fn real(&self) -> f64 {
        self.named.real()
    }
    fn set_real(&self, val: f64) {
        self.named.set_real(val);
    }
    fn string(&self) -> AsciiString {
        self.named.string()
    }
    fn set_string(&self, val: &[u8]) {
        self.named.set_string(val);
    }
}
