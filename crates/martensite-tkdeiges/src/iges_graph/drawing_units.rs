//! IgesGraphDrawingUnits.
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
/// defines IGESDrawingUnits, Type `406` Form `17`
///
/// Specifies the drawing space units as outlined
/// in the Drawing entity
IgesGraphDrawingUnits, IgesGraphDrawingUnitsState, "IgesGraphDrawingUnits", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_flag: i32,
the_unit: Handle<HAsciiString>,
}
}

impl IgesGraphDrawingUnits {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get flag = the_flag: i32;
    get unit = the_unit: Handle<HAsciiString>;
    }
    /// This method is used to set the fields of the class
    /// DrawingUnits
    /// - nbProps : Number of property values (NP = 2)
    /// - aFlag   : DrawingUnits Flag
    /// - aUnit   : DrawingUnits Name
    pub fn init(&self, nb_props: i32, a_flag: i32, an_unit: &Handle<HAsciiString>) {
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_props;
            state.the_flag = a_flag;
            state.the_unit = an_unit.clone();
        }
        self.init_type_and_form(406, 17);
    }
    /// Computes the value of the unit, in meters, according Flag
    pub fn unit_value(&self) -> f64 {
        let switch_subject = self.state.read().the_flag;
        match switch_subject {
            1 => {
                return 0.0254;
            }
            2 => {
                return 0.001;
            }
            3 => {
                return 1.0;
            }
            4 => {
                return 0.3048;
            }
            5 => {
                return 1609.27;
            }
            6 => {
                return 1.0;
            }
            7 => {
                return 1000.0;
            }
            8 => {
                return 0.0000254;
            }
            9 => {
                return 0.000001;
            }
            10 => {
                return 0.01;
            }
            11 => {
                return 0.0000000254;
            }
            _ => {}
        }
        return 1.0;
    }
}
