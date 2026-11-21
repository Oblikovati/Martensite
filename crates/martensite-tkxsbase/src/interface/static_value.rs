//! `InterfaceStatic`: a typed value recorded by name in the process-wide map of static
//! values (`MoniToolTypedValue::Stats`), with its family, a wild-card and an update
//! status. The static functions on the map are in `static_registry`.
use super::{InterfaceParamType, InterfaceTypedValue, InterfaceTypedValueOps};
use crate::moni_tool::{MoniToolTypedValueOps, MoniToolValue};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, SharedState},
    tcollection::{AsciiString, HAsciiString},
};
use std::io::{self, Write};
use std::ops::Deref;

/// `InterfaceStaticSatisfies`: an additional check of a static value.
pub type InterfaceStaticSatisfies = fn(&Handle<HAsciiString>) -> bool;

/// The own fields of a static value.
struct StaticState {
    thefamily: AsciiString,
    thewild: Handle<InterfaceStatic>,
    theupdate: bool,
}

/// A process-wide named parameter with its family, wildcard and update status.
///
/// ```
/// use martensite_tkxsbase::interface::{InterfaceParamType, InterfaceStatic};
/// let value = InterfaceStatic::new("XSTEP", "doc.static", InterfaceParamType::ParamReal, "0.5").unwrap();
/// assert_eq!(value.family().as_bytes(), b"XSTEP");
/// assert_eq!(value.real_value(), 0.5);
/// ```
pub struct InterfaceStatic {
    value: MoniToolValue,
    state: SharedState<StaticState>,
}
impl_transient!(
    InterfaceStatic,
    "InterfaceStatic",
    dyn InterfaceTypedValueOps,
    interfaces: [dyn MoniToolTypedValueOps, dyn InterfaceTypedValueOps]
);

impl Deref for InterfaceStatic {
    type Target = MoniToolValue;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MoniToolTypedValueOps for InterfaceStatic {}
impl InterfaceTypedValueOps for InterfaceStatic {}

fn parameter_value(
    name: &str,
    kind: InterfaceParamType,
    initial: &str,
) -> Result<MoniToolValue, Failure> {
    MoniToolValue::new(
        name,
        InterfaceTypedValue::param_type_to_value_type(kind),
        initial,
    )
}

impl InterfaceStatic {
    /// `InterfaceStatic(family, name, type = ParamText, init = "")`: up to date.
    pub fn new(
        family: &str,
        name: &str,
        kind: InterfaceParamType,
        init: &str,
    ) -> Result<Self, Failure> {
        Ok(Self::with_base(family, parameter_value(name, kind, init)?))
    }
    /// `InterfaceStatic(family, name, other)`: the specification of `other` (limits,
    /// unit, enumeration, object type) and its value. Native quirk kept: the interpret
    /// and satisfies functions and the label are not copied.
    pub fn from_other(family: &str, name: &str, other: &InterfaceStatic) -> Result<Self, Failure> {
        let created = Self::with_base(family, parameter_value(name, other.param_type(), "")?);
        created.copy_specification(other)?;
        if other.is_set_value() {
            let text = String::from_utf8_lossy(other.c_string_value().as_bytes()).into_owned();
            created.set_c_string_value(&text)?;
        }
        Ok(created)
    }
    fn with_base(family: &str, value: MoniToolValue) -> Self {
        Self {
            value,
            state: SharedState::new(StaticState {
                thefamily: AsciiString::new(family),
                thewild: Handle::default(),
                theupdate: true,
            }),
        }
    }
    /// The specification part of the copy constructor, by type.
    fn copy_specification(&self, other: &InterfaceStatic) -> Result<(), Failure> {
        match self.param_type() {
            InterfaceParamType::ParamInteger => {
                for max in [true, false] {
                    if let (true, lim) = other.integer_limit(max) {
                        self.set_integer_limit(max, lim)?;
                    }
                }
            }
            InterfaceParamType::ParamReal => {
                for max in [true, false] {
                    if let (true, lim) = other.real_limit(max) {
                        self.set_real_limit(max, lim)?;
                    }
                }
                self.set_unit_def(&String::from_utf8_lossy(other.unit_def().as_bytes()));
            }
            InterfaceParamType::ParamEnum => self.copy_enumeration(other)?,
            InterfaceParamType::ParamIdent => self.set_object_type(Some(other.object_type()))?,
            _ => {}
        }
        Ok(())
    }
    fn copy_enumeration(&self, other: &InterfaceStatic) -> Result<(), Failure> {
        let (e0, e1, matched) = other.enum_def().unwrap_or((0, -1, false));
        self.start_enum(e0, matched)?;
        for case in e0..=e1 {
            let text = String::from_utf8_lossy(other.enum_val(case).as_bytes()).into_owned();
            self.add_enum(&[&text])?;
        }
        Ok(())
    }
    /// Read the exchange parameter type, e.g. `value.param_type()`.
    pub fn param_type(&self) -> InterfaceParamType {
        InterfaceTypedValueOps::param_type(self)
    }

    /// `PrintStatic(S)`: name, family, the typed value print and the wild-card. Native
    /// quirks kept: the value is not printed after "Value : ", and the satisfies line of
    /// the static's own (never set) function is not printed.
    pub fn print_static(&self, s: &mut dyn Write) -> io::Result<()> {
        s.write_all(b"--- Static Value : ")?;
        s.write_all(self.name().as_bytes())?;
        s.write_all(b"  Family:")?;
        s.write_all(self.family().as_bytes())?;
        self.print(s)?;
        let (wild, update) = {
            let state = self.state.read();
            (state.thewild.clone(), state.theupdate)
        };
        if let Some(card) = wild.get() {
            s.write_all(b" -- Attached to wild-card : ")?;
            s.write_all(card.name().as_bytes())?;
            s.write_all(b"\n")?;
        }
        write!(
            s,
            "--- Actual status : {}  Value : ",
            if update { "" } else { "original" }
        )
    }
    /// `Family`.
    pub fn family(&self) -> AsciiString {
        self.state.read().thefamily.clone()
    }
    /// `SetWild(wildcard)`.
    pub fn set_wild(&self, wildcard: &Handle<InterfaceStatic>) {
        self.state.write().thewild = wildcard.clone();
    }
    /// `Wild`.
    pub fn wild(&self) -> Handle<InterfaceStatic> {
        self.state.read().thewild.clone()
    }
    /// `SetUptodate`.
    pub fn set_uptodate(&self) {
        self.state.write().theupdate = true;
    }
    /// `UpdatedStatus`.
    pub fn updated_status(&self) -> bool {
        self.state.read().theupdate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::moni_tool::MoniToolTypedValue;
    use martensite_tkernel::standard::RuntimeTyped;

    #[test]
    fn copies_the_specification() {
        let source = InterfaceStatic::new("F", "src", InterfaceParamType::ParamEnum, "").unwrap();
        source.start_enum(1, true).unwrap();
        source.add_enum(&["One", "Two"]).unwrap();
        source.set_c_string_value("Two").unwrap();
        let copy = InterfaceStatic::from_other("G", "dst", &source).unwrap();
        assert_eq!(copy.enum_def(), Some((1, 2, true)));
        assert_eq!(copy.integer_value(), 2);
        let real = InterfaceStatic::new("F", "r", InterfaceParamType::ParamReal, "").unwrap();
        real.set_real_limit(true, 4.0).unwrap();
        real.set_unit_def("mm");
        let real_copy = InterfaceStatic::from_other("F", "r2", &real).unwrap();
        assert_eq!(real_copy.real_limit(true), (true, 4.0));
        assert_eq!(real_copy.unit_def().as_bytes(), b"mm");
        let int = InterfaceStatic::new("F", "i", InterfaceParamType::ParamInteger, "").unwrap();
        int.set_integer_limit(false, -1).unwrap();
        assert_eq!(
            InterfaceStatic::from_other("F", "i2", &int)
                .unwrap()
                .integer_limit(false),
            (true, -1)
        );
        let object = InterfaceStatic::new("F", "o", InterfaceParamType::ParamIdent, "").unwrap();
        assert_eq!(
            InterfaceStatic::from_other("F", "o2", &object)
                .unwrap()
                .object_type()
                .name(),
            "Standard_Transient"
        );
    }

    #[test]
    fn wild_card_update_and_print() {
        let value = Handle::new(
            InterfaceStatic::new("F", "p", InterfaceParamType::ParamText, "v").unwrap(),
        );
        let wild =
            Handle::new(InterfaceStatic::new("F", "w", InterfaceParamType::ParamText, "").unwrap());
        value.set_wild(&wild);
        assert!(value.wild() == wild);
        value.set_uptodate();
        assert!(value.updated_status());
        let mut out = Vec::new();
        value.print_static(&mut out).unwrap();
        let text = String::from_utf8(out).unwrap();
        assert!(
            text.starts_with("--- Static Value : p  Family:F--- Typed Value : p\n"),
            "{text}"
        );
        assert!(
            text.ends_with(" -- Attached to wild-card : w\n--- Actual status :   Value : "),
            "{text}"
        );
        assert!(value.is_kind(MoniToolTypedValue::static_type()));
        let view = value.view::<dyn InterfaceTypedValueOps>();
        assert_eq!(view.object_address(), value.object_address());
        assert_eq!(view.get().unwrap().param_type(), value.param_type());
    }
}
