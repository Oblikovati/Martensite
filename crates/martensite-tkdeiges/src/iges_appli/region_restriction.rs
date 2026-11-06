//! IgesAppliRegionRestriction.
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
/// defines RegionRestriction, Type `406` Form `2`
/// Defines regions to set an application's restriction
/// over a region.
IgesAppliRegionRestriction, IgesAppliRegionRestrictionState, "IgesAppliRegionRestriction", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_elect_vias_restrict: i32,
the_elect_comp_restrict: i32,
the_elect_ckt_restrict: i32,
}
}

impl IgesAppliRegionRestriction {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get electrical_vias_restriction = the_elect_vias_restrict: i32;
    get electrical_component_restriction = the_elect_comp_restrict: i32;
    get electrical_ckt_restriction = the_elect_ckt_restrict: i32;
    }
    /// This method is used to set the fields of the class
    /// RegionRestriction
    /// - nbPropVal  : Number of property values, always = 3
    /// - aViasRest  : Electrical Vias restriction
    /// - aCompoRest : Electrical components restriction
    /// - aCktRest   : Electrical circuitry restriction
    pub fn init(&self, nb_prop_val: i32, a_vias_rest: i32, a_compo_rest: i32, a_ckt_rest: i32) {
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_prop_val;
            state.the_elect_vias_restrict = a_vias_rest;
            state.the_elect_comp_restrict = a_compo_rest;
            state.the_elect_ckt_restrict = a_ckt_rest;
        }
        self.init_type_and_form(406, 2);
    }
}
