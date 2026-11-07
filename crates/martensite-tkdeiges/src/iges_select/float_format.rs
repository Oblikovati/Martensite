//! The format of the reals of a written file.
use super::IgesSelectFileModifier;
use crate::iges_data::IgesDataIgesWriter;
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, SharedState, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkxsbase::{
    if_select::{IFSelectContextWrite, IFSelectGeneralModifier, IFSelectGeneralModifierBase},
    interface::InterfaceFloatWriter,
};

struct FloatFormatFields {
    thezerosup: bool,
    themainform: String,
    theformrange: String,
    therangemin: f64,
    therangemax: f64,
}

/// What [`IgesSelectFloatFormat::format`] gives.
#[derive(Clone, Debug, PartialEq)]
pub struct IgesSelectFloatFormatValues {
    /// Whether the trailing zeros are suppressed.
    pub zerosup: bool,
    /// The main format.
    pub mainform: String,
    /// Whether a format is set for a range.
    pub hasrange: bool,
    /// The format in the range.
    pub forminrange: String,
    /// The lower value of the range.
    pub rangemin: f64,
    /// The upper value of the range.
    pub rangemax: f64,
}

/// Sets the format of the reals on the writer.
///
/// ```
/// use martensite_tkdeiges::iges_select::IgesSelectFloatFormat;
/// let format = IgesSelectFloatFormat::new();
/// format.set_default(6);
/// assert_eq!(format.format().mainform, "%8.6E");
/// ```
pub struct IgesSelectFloatFormat {
    transient: StandardTransient,
    base: IFSelectGeneralModifierBase,
    state: SharedState<FloatFormatFields>,
}
impl_transient!(
    IgesSelectFloatFormat,
    "IgesSelectFloatFormat",
    dyn IgesSelectFileModifier,
    transient,
    interfaces: [dyn IFSelectGeneralModifier, dyn IgesSelectFileModifier]
);

impl Default for IgesSelectFloatFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl IgesSelectFloatFormat {
    /// The default format, zeros suppressed.
    pub fn new() -> Self {
        Self {
            transient: StandardTransient::default(),
            base: IFSelectGeneralModifierBase::new(false),
            state: SharedState::new(FloatFormatFields {
                thezerosup: true,
                themainform: "%E".to_owned(),
                theformrange: "%f".to_owned(),
                therangemin: 0.1,
                therangemax: 1000.0,
            }),
        }
    }
    /// The default formats, with a count of digits when positive.
    pub fn set_default(&self, digits: i32) {
        let mut state = self.state.write();
        if digits <= 0 {
            state.themainform = "%E".to_owned();
            state.theformrange = "%f".to_owned();
        } else {
            state.themainform = format!("%{}.{digits}E", digits + 2);
            state.theformrange = format!("%{}.{digits}f", digits + 2);
        }
        state.therangemin = 0.1;
        state.therangemax = 1000.0;
        state.thezerosup = true;
    }
    /// Sets whether the trailing zeros are suppressed.
    pub fn set_zero_suppress(&self, mode: bool) {
        self.state.write().thezerosup = mode;
    }
    /// Sets the main format.
    pub fn set_format(&self, format: &str) {
        self.state.write().themainform = format.to_owned();
    }
    /// Sets the format of a range of values; empty for none.
    pub fn set_format_for_range(&self, format: &str, rmin: f64, rmax: f64) {
        let mut state = self.state.write();
        state.theformrange = format.to_owned();
        state.therangemin = rmin;
        state.therangemax = rmax;
    }
    /// The formats.
    pub fn format(&self) -> IgesSelectFloatFormatValues {
        let state = self.state.read();
        IgesSelectFloatFormatValues {
            zerosup: state.thezerosup,
            mainform: state.themainform.clone(),
            hasrange: !state.theformrange.is_empty(),
            forminrange: state.theformrange.clone(),
            rangemin: state.therangemin,
            rangemax: state.therangemax,
        }
    }
    fn range_value(value: f64, rmin: f64, rmax: f64) -> Vec<u8> {
        let mut text = Vec::new();
        // The range given holds the value, and the plain format cannot fail.
        let converted = InterfaceFloatWriter::convert(
            value,
            &mut text,
            true,
            rmin / 2.0,
            rmax * 2.0,
            "%f",
            "%f",
        );
        if converted.is_err() {
            text.clear();
        }
        text
    }
}

impl IFSelectGeneralModifier for IgesSelectFloatFormat {
    fn general_modifier_base(&self) -> &IFSelectGeneralModifierBase {
        &self.base
    }
    fn label(&self) -> AsciiString {
        let state = self.state.read();
        let mut label = b"Float Format ".to_vec();
        if state.thezerosup {
            label.extend_from_slice(b" ZeroSup ");
        }
        label.extend_from_slice(state.themainform.as_bytes());
        if !state.theformrange.is_empty() {
            let (rmin, rmax) = (state.therangemin, state.therangemax);
            label.extend_from_slice(b", in range ");
            label.extend_from_slice(&Self::range_value(rmin, rmin, rmax));
            label.push(b' ');
            label.extend_from_slice(&Self::range_value(rmax, rmin, rmax));
            label.push(b':');
            label.extend_from_slice(state.theformrange.as_bytes());
        }
        AsciiString::from_bytes(label)
    }
}

impl IgesSelectFileModifier for IgesSelectFloatFormat {
    fn perform(
        &self,
        _ctx: &mut IFSelectContextWrite,
        writer: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let state = self.state.read();
        let float_writer = writer.float_writer();
        float_writer.set_format(&state.themainform, true);
        float_writer.set_zero_suppress(state.thezerosup);
        if !state.theformrange.is_empty() {
            float_writer.set_format_for_range(
                &state.theformrange,
                state.therangemin,
                state.therangemax,
            );
        }
        Ok(())
    }
}
