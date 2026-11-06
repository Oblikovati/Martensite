//! `StepDataSelectMember`: a value of a basic type with an optional name, inside a
//! SELECT. The class is only ever used through its subclasses, so it is the trait;
//! `StepDataSelectInt` and `StepDataSelectReal` are its unnamed storages. A specific member
//! embeds one of them, registers it as RTTI parent and overrides what it names.
use super::{StepDataFieldKind, StepDataLogical};
use martensite_tkernel::{
    impl_transient, impl_transient_interface,
    standard::{SharedState, StandardTransient, Transient},
    tcollection::AsciiString,
};
use martensite_tkxsbase::interface::InterfaceParamType;

/// `StepDataSelectMember`. Every member takes `&self` (geom-design.md section 3). The
/// virtual members are provided with the default; the non-virtual ones are
/// provided members no implementation overrides.
///
/// ```
/// use martensite_tkdestep::step_data::{StepDataSelectInt, StepDataSelectMember};
/// let member = StepDataSelectInt::default();
/// member.set_boolean(true);
/// assert!(member.boolean());
/// ```
pub trait StepDataSelectMember: Transient {
    /// `has_name`.
    fn has_name(&self) -> bool {
        false
    }
    /// `name`.
    fn name(&self) -> String {
        String::new()
    }
    /// `set_name(name)`: true when done, false when no name is allowed.
    fn set_name(&self, _name: &str) -> bool {
        false
    }
    /// `matches(name)`: compares with `name`.
    fn matches(&self, name: &str) -> bool {
        name == self.name()
    }
    /// `Kind`: the encoded [`StepDataFieldKind`].
    fn kind(&self) -> i32 {
        0
    }
    /// `set_kind(kind)`.
    fn set_kind(&self, _kind: i32) {}
    /// `int`.
    fn int(&self) -> i32 {
        0
    }
    /// `set_int(val)`.
    fn set_int(&self, _val: i32) {}
    /// `real`.
    fn real(&self) -> f64 {
        0.0
    }
    /// `set_real(val)`.
    fn set_real(&self, _val: f64) {}
    /// `string`.
    fn string(&self) -> AsciiString {
        AsciiString::default()
    }
    /// `set_string(val)`.
    fn set_string(&self, _val: &[u8]) {}
    /// `enum_text`.
    fn enum_text(&self) -> AsciiString {
        self.string()
    }
    /// `set_enum_text(val, text)`.
    fn set_enum_text(&self, _val: i32, text: &[u8]) {
        self.set_string(text);
    }
    /// `param_type`: the kind as an `InterfaceParamType`.
    fn param_type(&self) -> InterfaceParamType {
        let kind = self.kind();
        if !(0..=0xFF).contains(&kind) {
            return InterfaceParamType::ParamMisc;
        }
        match StepDataFieldKind::from_int(kind) {
            StepDataFieldKind::UNDEFINED => InterfaceParamType::ParamVoid,
            StepDataFieldKind::INTEGER => InterfaceParamType::ParamInteger,
            StepDataFieldKind::BOOLEAN | StepDataFieldKind::LOGICAL => {
                InterfaceParamType::ParamLogical
            }
            StepDataFieldKind::ENUM => InterfaceParamType::ParamEnum,
            StepDataFieldKind::REAL => InterfaceParamType::ParamReal,
            StepDataFieldKind::STRING => InterfaceParamType::ParamText,
            _ => InterfaceParamType::ParamMisc,
        }
    }
    /// `integer`.
    fn integer(&self) -> i32 {
        self.int()
    }
    /// `set_integer(val)`.
    fn set_integer(&self, val: i32) {
        self.set_kind(StepDataFieldKind::INTEGER.to_int());
        self.set_int(val);
    }
    /// `boolean`.
    fn boolean(&self) -> bool {
        self.int() > 0
    }
    /// `set_boolean(val)`.
    fn set_boolean(&self, val: bool) {
        self.set_kind(StepDataFieldKind::BOOLEAN.to_int());
        self.set_int(i32::from(val));
    }
    /// `logical`.
    fn logical(&self) -> StepDataLogical {
        StepDataLogical::from_int(self.int())
    }
    /// `set_logical(val)`.
    fn set_logical(&self, val: StepDataLogical) {
        self.set_kind(StepDataFieldKind::LOGICAL.to_int());
        self.set_int(val as i32);
    }
    /// `Enum`.
    fn enum_value(&self) -> i32 {
        self.int()
    }
    /// `set_enum(val, text = "")`.
    fn set_enum(&self, val: i32, text: &[u8]) {
        self.set_kind(StepDataFieldKind::ENUM.to_int());
        self.set_int(val);
        if !text.is_empty() {
            self.set_enum_text(val, text);
        }
    }
}
impl_transient_interface!(
    dyn StepDataSelectMember,
    "StepDataSelectMember",
    StandardTransient
);

/// `StepDataSelectInt`: an integer kind with no name.
///
/// ```
/// use martensite_tkdestep::step_data::{StepDataSelectInt, StepDataSelectMember};
/// let member = StepDataSelectInt::default();
/// member.set_integer(4);
/// assert_eq!(member.integer(), 4);
/// ```
#[derive(Default)]
pub struct StepDataSelectInt {
    transient: StandardTransient,
    state: SharedState<(i32, i32)>,
}
impl_transient!(
    StepDataSelectInt,
    "StepDataSelectInt",
    dyn StepDataSelectMember,
    transient,
    interfaces: [dyn StepDataSelectMember]
);

impl StepDataSelectMember for StepDataSelectInt {
    fn kind(&self) -> i32 {
        self.state.read().0
    }
    fn set_kind(&self, kind: i32) {
        self.state.write().0 = kind;
    }
    fn int(&self) -> i32 {
        self.state.read().1
    }
    fn set_int(&self, val: i32) {
        self.state.write().1 = val;
    }
}

/// `StepDataSelectReal`: a real with no name.
///
/// ```
/// use martensite_tkdestep::step_data::{StepDataSelectMember, StepDataSelectReal};
/// let member = StepDataSelectReal::default();
/// member.set_real(1.5);
/// assert_eq!(member.real(), 1.5);
/// ```
#[derive(Default)]
pub struct StepDataSelectReal {
    transient: StandardTransient,
    state: SharedState<f64>,
}
impl_transient!(
    StepDataSelectReal,
    "StepDataSelectReal",
    dyn StepDataSelectMember,
    transient,
    interfaces: [dyn StepDataSelectMember]
);

impl StepDataSelectMember for StepDataSelectReal {
    fn kind(&self) -> i32 {
        StepDataFieldKind::REAL.to_int()
    }
    fn real(&self) -> f64 {
        *self.state.read()
    }
    fn set_real(&self, val: f64) {
        *self.state.write() = val;
    }
}
