//! IgesAppliLineWidening.
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
/// defines LineWidening, Type `406` Form `5`
/// Defines the characteristics of entities when they are
/// used to define locations of items.
IgesAppliLineWidening, IgesAppliLineWideningState, "IgesAppliLineWidening", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_width: f64,
the_cornering_code: i32,
the_extension_flag: i32,
the_justification_flag: i32,
the_extension_value: f64,
}
}

impl IgesAppliLineWidening {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get width_of_metalization = the_width: f64;
    get cornering_code = the_cornering_code: i32;
    get extension_flag = the_extension_flag: i32;
    get justification_flag = the_justification_flag: i32;
    get extension_value = the_extension_value: f64;
    }
    /// This method is used to set the fields of the class
    /// LineWidening
    /// - nbPropVal   : Number of property values = 5
    /// - aWidth      : Width of metalization
    /// - aCornering  : Cornering codes
    ///   0 = rounded
    ///   1 = squared
    /// - aExtnFlag   : Extension Flag
    ///   0 = No Extension
    ///   1 = One-half width extension
    ///   2 = Extn set by ExtnVal
    /// - aJustifFlag : Justification flag
    ///   0 = Center justified
    ///   1 = left justified
    ///   2 = right justified
    /// - aExtnVal    : Extension value if aExtnFlag = 2
    pub fn init(
        &self,
        nb_prop_val: i32,
        a_width: f64,
        a_cornering: i32,
        a_extn_flag: i32,
        a_justif_flag: i32,
        a_extn_val: f64,
    ) {
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_prop_val;
            state.the_width = a_width;
            state.the_cornering_code = a_cornering;
            state.the_extension_flag = a_extn_flag;
            state.the_justification_flag = a_justif_flag;
            state.the_extension_value = a_extn_val;
        }
        self.init_type_and_form(406, 5);
    }
}
