//! IgesAppliPinNumber.
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
/// defines PinNumber, Type `406` Form `8`
/// Used to attach a text string representing a component
/// pin number to an entity being used to represent an
/// electrical component's pin
IgesAppliPinNumber, IgesAppliPinNumberState, "IgesAppliPinNumber", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_pin_number: Handle<HAsciiString>,
}
}

impl IgesAppliPinNumber {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get pin_number_val = the_pin_number: Handle<HAsciiString>;
    }
    /// This method is used to set the fields of the class
    /// PinNumber
    /// - nbPropVal : Number of property values (always = 1)
    /// - aValue    : Pin Number value
    pub fn init(&self, nb_prop_val: i32, a_value: &Handle<HAsciiString>) {
        {
            let mut state = self.state.write();
            state.the_pin_number = a_value.clone();
            state.the_nb_property_values = nb_prop_val;
        }
        self.init_type_and_form(406, 8);
    }
}
