//! IgesAppliReferenceDesignator.
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
/// defines ReferenceDesignator, Type `406` Form `7`
/// Used to attach a text string containing the value of
/// a component reference designator to an entity being
/// used to represent a component.
IgesAppliReferenceDesignator, IgesAppliReferenceDesignatorState, "IgesAppliReferenceDesignator", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_ref_desig_text: Handle<HAsciiString>,
}
}

impl IgesAppliReferenceDesignator {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get ref_designator_text = the_ref_desig_text: Handle<HAsciiString>;
    }
    /// This method is used to set the fields of the class
    /// ReferenceDesignator
    /// - nbPropVal : Number of property values = 1
    /// - aText     : Reference designator text
    pub fn init(&self, nb_prop_val: i32, a_text: &Handle<HAsciiString>) {
        {
            let mut state = self.state.write();
            state.the_ref_desig_text = a_text.clone();
            state.the_nb_property_values = nb_prop_val;
        }
        self.init_type_and_form(406, 7);
    }
}
