//! IgesAppliDrilledHole.
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
/// defines DrilledHole, Type `406` Form `6`
/// Identifies an entity representing a drilled hole
/// through a printed circuit board.
IgesAppliDrilledHole, IgesAppliDrilledHoleState, "IgesAppliDrilledHole", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_drill_dia_size: f64,
the_finish_dia_size: f64,
the_plating_flag: i32,
the_nb_lower_layer: i32,
the_nb_higher_layer: i32,
}
}

impl IgesAppliDrilledHole {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get drill_dia_size = the_drill_dia_size: f64;
    get finish_dia_size = the_finish_dia_size: f64;
    get nb_lower_layer = the_nb_lower_layer: i32;
    get nb_higher_layer = the_nb_higher_layer: i32;
    }
    /// This method is used to set the fields of the class
    /// DrilledHole
    /// - nbPropVal    : Number of property values = 5
    /// - aSize        : Drill diameter size
    /// - anotherSize  : Finish diameter size
    /// - aPlating     : Plating indication flag
    ///   False = not plating
    ///   True  = is plating
    /// - aLayer       : Lower numbered layer
    /// - anotherLayer : Higher numbered layer
    pub fn init(
        &self,
        nb_prop_val: i32,
        a_size: f64,
        another_size: f64,
        a_plating: i32,
        a_layer: i32,
        another_layer: i32,
    ) {
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_prop_val;
            state.the_drill_dia_size = a_size;
            state.the_finish_dia_size = another_size;
            state.the_plating_flag = a_plating;
            state.the_nb_lower_layer = a_layer;
            state.the_nb_higher_layer = another_layer;
        }
        self.init_type_and_form(406, 6);
    }
    /// Returns Plating Status:
    /// False = not plating / True = is plating
    pub fn is_plating(&self) -> bool {
        (self.state.read().the_plating_flag) != 0
    }
}
