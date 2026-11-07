//! The plain queries and updates of the fields of the global section.
use super::IgesDataGlobalSection;
use martensite_tkernel::{standard::Handle, tcollection::HAsciiString};

/// The plain queries and updates of the section's fields.
macro_rules! global_section_fields {
    ($(value $get:ident, $set:ident, $field:ident: $ty:ty;)* $(text $tget:ident, $tset:ident, $tfield:ident;)*) => {
        impl IgesDataGlobalSection {
            $(
                #[doc = concat!("The `", stringify!($field), "` parameter.")]
                pub fn $get(&self) -> $ty {
                    self.$field
                }
                #[doc = concat!("Sets the `", stringify!($field), "` parameter.")]
                pub fn $set(&mut self, val: $ty) {
                    self.$field = val;
                }
            )*
            $(
                #[doc = concat!("The `", stringify!($tfield), "` parameter, null when not given.")]
                pub fn $tget(&self) -> Handle<HAsciiString> {
                    self.$tfield.clone()
                }
                #[doc = concat!("Sets the `", stringify!($tfield), "` parameter; a counted text is taken by its content.")]
                pub fn $tset(&mut self, val: &Handle<HAsciiString>) {
                    self.$tfield = self.translated_from_hollerith(val);
                }
            )*
        }
    };
}
global_section_fields! {
    value separator, set_separator, the_separator: u8;
    value end_mark, set_end_mark, the_end_mark: u8;
    value integer_bits, set_integer_bits, the_integer_bits: i32;
    value max_power10_single, set_max_power10_single, the_max_power10_single: i32;
    value max_digits_single, set_max_digits_single, the_max_digits_single: i32;
    value max_power10_double, set_max_power10_double, the_max_power10_double: i32;
    value max_digits_double, set_max_digits_double, the_max_digits_double: i32;
    value scale, set_scale, the_scale: f64;
    value cascade_unit, set_cascade_unit, the_cascade_unit: f64;
    value unit_flag, set_unit_flag, the_unit_flag: i32;
    value line_weight_grad, set_line_weight_grad, the_line_weight_grad: i32;
    value max_line_weight, set_max_line_weight, the_max_line_weight: f64;
    value resolution, set_resolution, the_resolution: f64;
    value iges_version, set_iges_version, the_iges_version: i32;
    value drafting_standard, set_drafting_standard, the_drafting_standard: i32;
    text send_name, set_send_name, the_send_name;
    text file_name, set_file_name, the_file_name;
    text system_id, set_system_id, the_system_id;
    text interface_version, set_interface_version, the_interface_version;
    text receive_name, set_receive_name, the_receive_name;
    text unit_name, set_unit_name, the_unit_name;
    text date, set_date, the_date;
    text author_name, set_author_name, the_author_name;
    text company_name, set_company_name, the_company_name;
    text last_change_date, set_last_change_date, the_last_change_date;
    text application_protocol, set_application_protocol, the_appli_protocol;
}

impl IgesDataGlobalSection {
    /// The maximum coordinate.
    pub fn max_coord(&self) -> f64 {
        self.the_max_coord
    }
    /// Whether the maximum coordinate is given.
    pub fn has_max_coord(&self) -> bool {
        self.has_max_coord
    }
    /// Whether the last change date is given.
    pub fn has_last_change_date(&self) -> bool {
        self.the_last_change_date.is_some()
    }
    /// Whether the application protocol is given.
    pub fn has_application_protocol(&self) -> bool {
        self.the_appli_protocol.is_some()
    }
}
