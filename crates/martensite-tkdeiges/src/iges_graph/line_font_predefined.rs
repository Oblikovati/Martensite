//! IgesGraphLineFontPredefined.
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
/// defines IGESLineFontPredefined, Type `406` Form `19`
///
/// Provides the ability to specify a line font pattern
/// from a predefined list rather than from
/// Directory Entry Field 4
IgesGraphLineFontPredefined, IgesGraphLineFontPredefinedState, "IgesGraphLineFontPredefined", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_line_font_pattern_code: i32,
}
}

impl IgesGraphLineFontPredefined {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get line_font_pattern_code = the_line_font_pattern_code: i32;
    }
    /// This method is used to set the fields of the class
    /// LineFontPredefined
    /// - nbProps              : Number of property values (NP = 1)
    /// - aLineFontPatternCode : Line Font Pattern Code
    pub fn init(&self, nb_props: i32, a_line_font_pattern_code: i32) {
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_props;
            state.the_line_font_pattern_code = a_line_font_pattern_code;
        }
        self.init_type_and_form(406, 19);
    }
}
