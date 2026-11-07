//! IgesGraphIntercharacterSpacing.
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
/// defines IGESIntercharacterSpacing, Type `406` Form `18`
///
/// Specifies the gap between letters when fixed-pitch
/// spacing is used
IgesGraphIntercharacterSpacing, IgesGraphIntercharacterSpacingState, "IgesGraphIntercharacterSpacing", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_i_space: f64,
}
}

impl IgesGraphIntercharacterSpacing {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get i_space = the_i_space: f64;
    }
    /// This method is used to set the fields of the class
    /// IntercharacterSpacing
    /// - nbProps  : Number of property values (NP = 1)
    /// - anISpace : Intercharacter spacing percentage
    pub fn init(&self, nb_props: i32, an_i_space: f64) {
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_props;
            state.the_i_space = an_i_space;
        }
        self.init_type_and_form(406, 18);
    }
}
