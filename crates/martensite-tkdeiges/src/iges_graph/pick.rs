//! IgesGraphPick.
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
/// defines IGESPick, Type `406` Form `21`
///
/// Attaches information that an entity may be picked
/// by whatever pick device is used in the receiving
/// system
IgesGraphPick, IgesGraphPickState, "IgesGraphPick", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_pick: i32,
}
}

impl IgesGraphPick {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get pick_flag = the_pick: i32;
    }
    /// This method is used to set the fields of the class Pick
    /// - nbProps     : Number of property values (NP = 1)
    /// - aPickStatus : Pick Flag
    pub fn init(&self, nb_props: i32, a_pick_status: i32) {
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_props;
            state.the_pick = a_pick_status;
        }
        self.init_type_and_form(406, 21);
    }
    /// returns True if thePick is 0.
    pub fn is_pickable(&self) -> bool {
        (self.state.read().the_pick) == 0
    }
}
