//! IgesAppliPartNumber.
#![allow(
    unused_mut,
    unused_variables,
    unused_assignments,
    clippy::needless_late_init,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::collapsible_else_if,
    clippy::needless_return,
    reason = "the members keep the order and the locals of the record they read"
)]
use crate::iges_data::IgesDataIgesEntity;
use crate::{iges_entity, iges_methods};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::HAsciiString;

iges_entity! {
/// defines PartNumber, Type `406` Form `9`
/// Attaches a set of text strings that define the common
/// part numbers to an entity being used to represent a
/// physical component
IgesAppliPartNumber, IgesAppliPartNumberState, "IgesAppliPartNumber", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_generic_number: Handle<HAsciiString>,
the_military_number: Handle<HAsciiString>,
the_vendor_number: Handle<HAsciiString>,
the_internal_number: Handle<HAsciiString>,
}
}

impl IgesAppliPartNumber {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get generic_number = the_generic_number: Handle<HAsciiString>;
    get military_number = the_military_number: Handle<HAsciiString>;
    get vendor_number = the_vendor_number: Handle<HAsciiString>;
    get internal_number = the_internal_number: Handle<HAsciiString>;
    }
    /// This method is used to set the fields of the class
    /// PartNumber
    /// - nbPropVal : number of property values, always = 4
    /// - aGenName  : Generic part number or name
    /// - aMilName  : Military Standard (MIL-STD) part number
    /// - aVendName : Vendor part number or name
    /// - anIntName : Internal part number
    pub fn init(
        &self,
        nb_prop_val: i32,
        a_gen_name: &Handle<HAsciiString>,
        a_mil_name: &Handle<HAsciiString>,
        a_vend_name: &Handle<HAsciiString>,
        an_int_name: &Handle<HAsciiString>,
    ) {
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_prop_val;
            state.the_generic_number = a_gen_name.clone();
            state.the_military_number = a_mil_name.clone();
            state.the_vendor_number = a_vend_name.clone();
            state.the_internal_number = an_int_name.clone();
        }
        self.init_type_and_form(406, 9);
    }
}
