//! IgesAppliPWBDrilledHole.
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

iges_entity! {
/// defines PWBDrilledHole, Type `406` Form `26`
/// Used to identify an entity that locates a drilled hole
/// and to specify the characteristics of the drilled hole
IgesAppliPWBDrilledHole, IgesAppliPWBDrilledHoleState, "IgesAppliPWBDrilledHole", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_drill_diameter: f64,
the_finish_diameter: f64,
the_function_code: i32,
}
}

impl IgesAppliPWBDrilledHole {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get drill_diameter_size = the_drill_diameter: f64;
    get finish_diameter_size = the_finish_diameter: f64;
    get function_code = the_function_code: i32;
    }
    /// This method is used to set the fields of the class
    /// PWBDrilledHole
    /// - nbPropVal  : number of property values, always = 3
    /// - aDrillDia  : Drill diameter size
    /// - aFinishDia : Finish diameter size
    /// - aCode      : Function code for drilled hole
    pub fn init(&self, nb_prop_val: i32, a_drill_dia: f64, a_finish_dia: f64, a_code: i32) {
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_prop_val;
            state.the_drill_diameter = a_drill_dia;
            state.the_finish_diameter = a_finish_dia;
            state.the_function_code = a_code;
        }
        self.init_type_and_form(406, 26);
    }
}
