//! IgesGraphHighLight.
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
/// defines IGESHighLight, Type `406` Form `20`
///
/// Attaches information that an entity is to be
/// displayed in some system dependent manner
IgesGraphHighLight, IgesGraphHighLightState, "IgesGraphHighLight", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_high_light: i32,
}
}

impl IgesGraphHighLight {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get high_light_status = the_high_light: i32;
    }
    /// This method is used to set the fields of the class
    /// HighLight
    /// - nbProps          : Number of property values (NP = 1)
    /// - aHighLightStatus : HighLight Flag
    pub fn init(&self, nb_props: i32, a_high_light_status: i32) {
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_props;
            state.the_high_light = a_high_light_status;
        }
        self.init_type_and_form(406, 20);
    }
    /// returns True if entity is highlighted
    pub fn is_high_lighted(&self) -> bool {
        (self.state.read().the_high_light) != 0
    }
}
