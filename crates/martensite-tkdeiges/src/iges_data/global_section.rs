//! The global section of an IGES file: its parameters, how they are read and written back.
use super::IgesDataBasicEditor;
use martensite_tkernel::{
    message::Msg,
    standard::{Failure, Handle, c_string::atof_bytes},
    tcollection::{AsciiString, HAsciiString},
};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::{
    interface::{InterfaceCheck, InterfaceFloatWriter, InterfaceParamSet, InterfaceParamType},
    xs_algo::{XSAlgoShapeProcessor, xs_algo_units},
};

use super::global_section_dates::system_date;
use super::param_reader::{iges_atoi, iges_msg};

/// The global section of an IGES file.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataGlobalSection;
/// let section = IgesDataGlobalSection::new();
/// assert_eq!((section.separator(), section.end_mark()), (b',', b';'));
/// assert_eq!(section.iges_version(), 11);
/// ```
#[derive(Clone)]
pub struct IgesDataGlobalSection {
    pub(super) the_separator: u8,
    pub(super) the_end_mark: u8,
    pub(super) the_send_name: Handle<HAsciiString>,
    pub(super) the_file_name: Handle<HAsciiString>,
    pub(super) the_system_id: Handle<HAsciiString>,
    pub(super) the_interface_version: Handle<HAsciiString>,
    pub(super) the_integer_bits: i32,
    pub(super) the_max_power10_single: i32,
    pub(super) the_max_digits_single: i32,
    pub(super) the_max_power10_double: i32,
    pub(super) the_max_digits_double: i32,
    pub(super) the_receive_name: Handle<HAsciiString>,
    pub(super) the_scale: f64,
    pub(super) the_cascade_unit: f64,
    pub(super) the_unit_flag: i32,
    pub(super) the_unit_name: Handle<HAsciiString>,
    pub(super) the_line_weight_grad: i32,
    pub(super) the_max_line_weight: f64,
    pub(super) the_date: Handle<HAsciiString>,
    pub(super) the_resolution: f64,
    pub(super) the_max_coord: f64,
    pub(super) has_max_coord: bool,
    pub(super) the_author_name: Handle<HAsciiString>,
    pub(super) the_company_name: Handle<HAsciiString>,
    pub(super) the_iges_version: i32,
    pub(super) the_drafting_standard: i32,
    pub(super) the_last_change_date: Handle<HAsciiString>,
    pub(super) the_appli_protocol: Handle<HAsciiString>,
}

impl Default for IgesDataGlobalSection {
    fn default() -> Self {
        Self {
            the_separator: b',',
            the_end_mark: b';',
            the_send_name: Handle::default(),
            the_file_name: Handle::default(),
            the_system_id: Handle::default(),
            the_interface_version: Handle::default(),
            the_integer_bits: 32,
            the_max_power10_single: 38,
            the_max_digits_single: 6,
            the_max_power10_double: 308,
            the_max_digits_double: 15,
            the_receive_name: Handle::default(),
            the_scale: 1.0,
            the_cascade_unit: 1.0,
            the_unit_flag: 0,
            the_unit_name: Handle::default(),
            the_line_weight_grad: 1,
            the_max_line_weight: 0.0,
            the_date: Handle::default(),
            the_resolution: 0.0,
            the_max_coord: 0.0,
            has_max_coord: false,
            the_author_name: Handle::default(),
            the_company_name: Handle::default(),
            the_iges_version: 11,
            the_drafting_standard: 0,
            the_last_change_date: Handle::default(),
            the_appli_protocol: Handle::default(),
        }
    }
}

/// The new copy of a string, null for a null one.
fn copied_string(astr: &Handle<HAsciiString>) -> Handle<HAsciiString> {
    astr.get().map_or_else(Handle::default, |text| {
        Handle::new(HAsciiString::new(text.string().as_bytes()))
    })
}

/// The counted form `nHtext` of a string, empty for a null or empty one.
fn make_hollerith(astr: &Handle<HAsciiString>) -> Vec<u8> {
    let Some(text) = astr.get() else {
        return Vec::new();
    };
    let content = text.string();
    if content.is_empty() {
        return Vec::new();
    }
    let mut counted = format!("{}H", content.len()).into_bytes();
    counted.extend_from_slice(content.as_bytes());
    counted
}

/// The text of a real parameter, with a `D` exponent written `e`.
fn real_of(val: &[u8]) -> f64 {
    let text: Vec<u8> = val
        .iter()
        .take(50)
        .map(|byte| {
            if matches!(*byte, b'D' | b'd') {
                b'e'
            } else {
                *byte
            }
        })
        .collect();
    atof_bytes(&text)
}

/// The content of a counted text, the text itself when it is not counted.
fn text_of(val: &[u8]) -> Handle<HAsciiString> {
    let whole = AsciiString::from_bytes(val);
    let nhol = whole.search("H");
    if nhol > 1 && nhol < whole.length() && whole.token("H", 1).is_integer_value() {
        return Handle::new(HAsciiString::new(&val[nhol as usize..]));
    }
    Handle::new(HAsciiString::new(val))
}

/// The delimiter of a parameter of the file, 0 when it gives none.
fn delimiter_of(val: &[u8]) -> u8 {
    let first = val.first().copied().unwrap_or(0);
    if val.get(1) == Some(&b'H') {
        return val.get(2).copied().unwrap_or(0);
    }
    first
}

impl IgesDataGlobalSection {
    /// A section with the defaults: IGES 5.3, the usual delimiters.
    pub fn new() -> Self {
        Self::default()
    }
    /// Reads the section from its parameters; what is wrong goes to `ach`.
    ///
    /// # Errors
    /// A failure of the parameter set.
    pub fn init(
        &mut self,
        params: &Handle<InterfaceParamSet>,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        XSAlgoShapeProcessor::prepare_for_transfer();
        *self = Self {
            the_cascade_unit: xs_algo_units().get_cascade_length_unit_default(),
            ..Self::default()
        };
        let set = super::iges_live(params, "IGES global section")?;
        let nbp = set.nb_params();
        for i in 1..=nbp {
            let parameter = set.param(i)?;
            if parameter.param_type() != InterfaceParamType::ParamVoid {
                self.take_param(i, parameter.param_type(), parameter.c_value());
            }
        }
        self.check_count(nbp, ach)?;
        self.correct_unit(ach);
        Ok(())
    }
    fn take_param(&mut self, i: i32, fpt: InterfaceParamType, val: &[u8]) {
        let is_integer = fpt == InterfaceParamType::ParamInteger;
        let is_real = matches!(
            fpt,
            InterfaceParamType::ParamReal | InterfaceParamType::ParamEnum
        );
        let is_real_field = matches!(i, 13 | 17 | 19 | 20);
        let intval = if is_integer { iges_atoi(val) } else { 0 };
        let realval = if is_real {
            real_of(val)
        } else if is_integer && is_real_field {
            atof_bytes(val)
        } else {
            0.0
        };
        let strval = if fpt == InterfaceParamType::ParamText {
            text_of(val)
        } else {
            Handle::default()
        };
        self.store_param(i, intval, realval, strval, delimiter_of(val));
    }
    fn store_param(
        &mut self,
        i: i32,
        intval: i32,
        realval: f64,
        strval: Handle<HAsciiString>,
        sepend: u8,
    ) {
        match i {
            1 if sepend != 0 => self.the_separator = sepend,
            2 if sepend != 0 => self.the_end_mark = sepend,
            3 => self.the_send_name = strval,
            4 => self.the_file_name = strval,
            5 => self.the_system_id = strval,
            6 => self.the_interface_version = strval,
            7 => self.the_integer_bits = intval,
            8 => self.the_max_power10_single = intval,
            9 => self.the_max_digits_single = intval,
            10 => self.the_max_power10_double = intval,
            11 => self.the_max_digits_double = intval,
            12 => self.the_receive_name = strval,
            13 => self.the_scale = realval,
            14 => self.the_unit_flag = intval,
            15 => self.the_unit_name = strval,
            16 => self.the_line_weight_grad = intval,
            17 => self.the_max_line_weight = realval,
            18 => self.the_date = strval,
            19 => self.the_resolution = realval,
            20 => {
                self.the_max_coord = realval;
                self.has_max_coord = true;
            }
            21 => self.the_author_name = strval,
            22 => self.the_company_name = strval,
            23 => self.the_iges_version = intval,
            24 => self.the_drafting_standard = intval,
            25 => self.the_last_change_date = strval,
            26 => self.the_appli_protocol = strval,
            _ => {}
        }
    }
    fn check_count(&self, nbp: i32, ach: &Handle<InterfaceCheck>) -> Result<(), Failure> {
        let (low, high) = if self.the_iges_version < 11 {
            (24, 25)
        } else {
            (25, 26)
        };
        if nbp >= low && nbp <= high {
            return Ok(());
        }
        let mut msg: Msg = iges_msg("XSTEP_39");
        msg.arg_integer(low)?;
        msg.arg_integer(high)?;
        if let Some(check) = ach.get() {
            if nbp < low {
                check.send_fail(&msg);
            } else {
                check.send_warning(&msg);
            }
        }
        Ok(())
    }
    /// A unit flag that is not given is restored from the unit name, inches without one.
    fn correct_unit(&mut self, ach: &Handle<InterfaceCheck>) {
        if self.the_unit_flag != 0 && self.the_unit_flag != 3 {
            return;
        }
        let corrected = self.the_unit_name.get().map_or(1, |name| {
            IgesDataBasicEditor::unit_name_flag(name.string().as_bytes())
        });
        if corrected > 0 {
            self.the_unit_flag = corrected;
            return;
        }
        let Some(check) = ach.get() else {
            return;
        };
        if self.the_unit_flag == 3 {
            check.send_warning(&iges_msg("XSTEP_49"));
        } else {
            check.send_fail(&iges_msg("XSTEP_48"));
        }
    }
    /// Makes every string its own copy.
    pub fn copy_refs(&mut self) {
        for field in [
            &mut self.the_send_name,
            &mut self.the_file_name,
            &mut self.the_system_id,
            &mut self.the_interface_version,
            &mut self.the_receive_name,
            &mut self.the_unit_name,
            &mut self.the_date,
            &mut self.the_author_name,
            &mut self.the_company_name,
            &mut self.the_last_change_date,
            &mut self.the_appli_protocol,
        ] {
            *field = copied_string(field);
        }
    }
    /// The section as the parameters to write, texts in their counted form.
    ///
    /// # Errors
    /// A failure of the parameter set or of the formatting of a real.
    pub fn params(&self) -> Result<Handle<InterfaceParamSet>, Failure> {
        let res = Handle::new(InterfaceParamSet::new(26, 1));
        let set = super::iges_live(&res, "IGES global section")?;
        append_delimiter(set, self.the_separator, b',')?;
        append_delimiter(set, self.the_end_mark, b';')?;
        append_text(set, &self.the_send_name)?;
        append_text(set, &self.the_file_name)?;
        append_text(set, &self.the_system_id)?;
        append_text(set, &self.the_interface_version)?;
        append_integer(set, self.the_integer_bits)?;
        append_integer(set, self.the_max_power10_single)?;
        append_integer(set, self.the_max_digits_single)?;
        append_integer(set, self.the_max_power10_double)?;
        append_integer(set, self.the_max_digits_double)?;
        append_text(set, &self.the_receive_name)?;
        append_real(set, self.the_scale, "%f")?;
        append_integer(set, self.the_unit_flag)?;
        append_text(set, &self.the_unit_name)?;
        append_integer(set, self.the_line_weight_grad)?;
        append_real(set, self.the_max_line_weight, "%f")?;
        append_text(set, &self.the_date)?;
        append_real(set, self.the_resolution, "%g")?;
        if self.has_max_coord {
            append_real(set, self.the_max_coord, "%f")?;
        } else {
            set.append(b"", 0, InterfaceParamType::ParamReal, 0)?;
        }
        append_text(set, &self.the_author_name)?;
        append_text(set, &self.the_company_name)?;
        append_integer(set, self.the_iges_version)?;
        append_integer(set, self.the_drafting_standard)?;
        if self.the_last_change_date.is_some() {
            append_text(set, &self.the_last_change_date)?;
        }
        if self.the_appli_protocol.is_some() {
            append_text(set, &self.the_appli_protocol)?;
        }
        Ok(res.clone())
    }
    /// The content of a counted text; a text not counted stays as it is, null for null.
    pub fn translated_from_hollerith(&self, astr: &Handle<HAsciiString>) -> Handle<HAsciiString> {
        astr.get()
            .map_or_else(Handle::default, |text| text_of(text.string().as_bytes()))
    }
    /// The last change date from the system date, unless it is already set.
    pub fn set_last_change_date_now(&mut self) {
        if self.has_last_change_date() {
            return;
        }
        let (year, month, day, hour, minute, second) = system_date();
        let mode = if year < 2000 { 0 } else { -1 };
        self.the_last_change_date =
            Self::new_date_string(year, month, day, hour, minute, second, mode);
    }
    /// The value of the unit, in the length unit of the session.
    pub fn unit_value(&self) -> f64 {
        IgesDataBasicEditor::unit_flag_value(self.the_unit_flag) / self.the_cascade_unit
    }
    /// Raises the maximum coordinate to the magnitude of `val`.
    pub fn max_max_coord(&mut self, val: f64) {
        let aval = val.abs();
        if !self.has_max_coord {
            self.set_max_coord(aval);
        } else if aval > self.the_max_coord {
            self.the_max_coord = aval;
        }
    }
    /// Raises the maximum coordinate to the coordinates of `xyz`.
    pub fn max_max_coords(&mut self, xyz: &Xyz) {
        self.max_max_coord(xyz.x());
        self.max_max_coord(xyz.y());
        self.max_max_coord(xyz.z());
    }
    /// Sets the maximum coordinate; a value that is not positive marks it as not given.
    pub fn set_max_coord(&mut self, val: f64) {
        self.has_max_coord = val > 0.0;
        self.the_max_coord = if self.has_max_coord { val } else { 0.0 };
    }
}

fn append_delimiter(set: &InterfaceParamSet, value: u8, default: u8) -> Result<(), Failure> {
    if value == default {
        set.append(b"", 0, InterfaceParamType::ParamVoid, 0)?;
    } else {
        set.append(&[value], 1, InterfaceParamType::ParamMisc, 0)?;
    }
    Ok(())
}

fn append_text(set: &InterfaceParamSet, value: &Handle<HAsciiString>) -> Result<(), Failure> {
    let text = make_hollerith(value);
    set.append(&text, text.len() as i32, InterfaceParamType::ParamText, 0)?;
    Ok(())
}

fn append_integer(set: &InterfaceParamSet, value: i32) -> Result<(), Failure> {
    let text = value.to_string();
    set.append(
        text.as_bytes(),
        text.len() as i32,
        InterfaceParamType::ParamInteger,
        0,
    )?;
    Ok(())
}

fn append_real(set: &InterfaceParamSet, value: f64, form: &str) -> Result<(), Failure> {
    let mut text = Vec::new();
    InterfaceFloatWriter::convert(value, &mut text, true, 0.0, 0.0, form, form)?;
    set.append(&text, text.len() as i32, InterfaceParamType::ParamReal, 0)?;
    Ok(())
}
