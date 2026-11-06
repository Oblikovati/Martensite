//! `IFSelectSignature`: a named signature of entities (an `InterfaceSignType`), with
//! its list of possible values (cases) and a text match.
use crate::interface::{InterfaceInterfaceModel, InterfaceSignType};
use martensite_tkernel::{
    impl_transient_interface,
    standard::{Failure, Handle, SharedState, StandardTransient, exceptions},
    tcollection::AsciiString,
};

/// The fields of `IFSelectSignature`: the name, the integer case and the cases.
pub struct IFSelectSignatureBase {
    /// `thename` (protected).
    pub thename: AsciiString,
    thecasi: SharedState<[i32; 3]>,
    thecasl: SharedState<Option<Vec<AsciiString>>>,
}

impl IFSelectSignatureBase {
    /// The protected constructor `IFSelectSignature(name)`: no case.
    pub fn new(name: &str) -> Self {
        Self {
            thename: AsciiString::new(name),
            thecasi: SharedState::new([0; 3]),
            thecasl: SharedState::new(None),
        }
    }
    /// `MatchValue(val, text, exact)`: `val` equals `text` (exact) or contains it;
    /// `StandardOutOfRange` for an empty `text` not exact (its first character is read).
    pub fn match_value(val: &str, text: &AsciiString, exact: bool) -> Result<bool, Failure> {
        let pattern = text.as_bytes();
        if exact {
            return Ok(pattern == val.as_bytes());
        }
        if pattern.is_empty() {
            return Err(exceptions::out_of_range(
                "IFSelectSignature::MatchValue: empty text; expected a text of 1 character or more",
            ));
        }
        Ok(val.as_bytes().windows(pattern.len()).any(|w| w == pattern))
    }
    /// `IntValue(val)`: the decimal text of an integer.
    pub fn int_value(val: i32) -> String {
        val.to_string()
    }
}

/// `IFSelectSignature`, abstract (`Value` of `InterfaceSignType` is required): its
/// `Name` is [`signature_name`]; `Matches` is virtual.
///
/// ```
/// use martensite_tkxsbase::if_select::{IFSelectSignType, IFSelectSignature};
/// let sign = IFSelectSignType::new(true);
/// assert_eq!(sign.label().as_bytes(), b"Signature : Class Type");
/// ```
pub trait IFSelectSignature: InterfaceSignType {
    /// The embedded fields.
    fn signature_base(&self) -> &IFSelectSignatureBase;
    /// `SetIntCase(hasmin, valmin, hasmax, valmax)`: the values are integers.
    fn set_int_case(&self, hasmin: bool, valmin: i32, hasmax: bool, valmax: i32) {
        let mut casi = self.signature_base().thecasi.write();
        casi[0] = 1;
        if hasmin {
            casi[0] += 2;
            casi[1] = valmin;
        }
        if hasmax {
            casi[0] += 4;
            casi[2] = valmax;
        }
    }
    /// `IsIntCase(hasmin, valmin, hasmax, valmax)`: `Some((hasmin, valmin, hasmax,
    /// valmax))` for integer values, else `None` (natively false, outputs reset).
    fn is_int_case(&self) -> Option<(bool, i32, bool, i32)> {
        let casi = *self.signature_base().thecasi.read();
        if casi[0] == 0 {
            return None;
        }
        let hasmin = casi[0] & 2 != 0;
        let hasmax = casi[0] & 4 != 0;
        Some((
            hasmin,
            if hasmin { casi[1] } else { 0 },
            hasmax,
            if hasmax { casi[2] } else { 0 },
        ))
    }
    /// `AddCase(acase)`: a possible value.
    fn add_case(&self, acase: &str) {
        self.signature_base()
            .thecasl
            .write()
            .get_or_insert_with(Vec::new)
            .push(AsciiString::new(acase));
    }
    /// `CaseList`: the possible values (`None` when not given).
    fn case_list(&self) -> Option<Vec<AsciiString>> {
        self.signature_base().thecasl.read().clone()
    }
    /// `Label`: "Signature : " and the name.
    fn label(&self) -> AsciiString {
        let mut label = AsciiString::new("Signature : ");
        label.assign_cat(&self.signature_base().thename);
        label
    }
    /// `Matches(ent, model, text, exact)`: `MatchValue` of the value of `ent`.
    fn matches(
        &self,
        ent: &Handle<StandardTransient>,
        model: &Handle<dyn InterfaceInterfaceModel>,
        text: &AsciiString,
        exact: bool,
    ) -> Result<bool, Failure> {
        IFSelectSignatureBase::match_value(&self.value(ent, model), text, exact)
    }
}
impl_transient_interface!(
    dyn IFSelectSignature,
    "IFSelectSignature",
    dyn InterfaceSignType
);

/// `IFSelectSignature::Name`: the name given at the construction.
pub fn signature_name<S: IFSelectSignature + ?Sized>(sign: &S) -> String {
    String::from_utf8_lossy(sign.signature_base().thename.as_bytes()).into_owned()
}

/// Implements `MoniToolSignText` for a signature (`Name` of `IFSelectSignature`, `Text`
/// of `InterfaceSignType`).
macro_rules! impl_signature_text {
    ($type:ty) => {
        impl $crate::moni_tool::MoniToolSignText for $type {
            fn name(&self) -> String {
                $crate::if_select::signature_name(self)
            }
            fn text(
                &self,
                ent: &martensite_tkernel::standard::Handle<
                    martensite_tkernel::standard::StandardTransient,
                >,
                context: &martensite_tkernel::standard::Handle<
                    martensite_tkernel::standard::StandardTransient,
                >,
            ) -> martensite_tkernel::tcollection::AsciiString {
                $crate::interface::interface_sign_type_text(self, ent, context)
            }
        }
    };
}
pub(crate) use impl_signature_text;
