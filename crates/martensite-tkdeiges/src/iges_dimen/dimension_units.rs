//! IgesDimenDimensionUnits.
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
/// defines Dimension Units, Type `406`, Form `28`
/// Describes the units and formatting details of the
/// nominal value of a dimension.
IgesDimenDimensionUnits, IgesDimenDimensionUnitsState, "IgesDimenDimensionUnits", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_secondary_dimen_position: i32,
the_units_indicator: i32,
the_character_set: i32,
the_format_string: Handle<HAsciiString>,
the_fraction_flag: i32,
the_precision: i32,
}
}

impl IgesDimenDimensionUnits {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get secondary_dimen_position = the_secondary_dimen_position: i32;
    get units_indicator = the_units_indicator: i32;
    get character_set = the_character_set: i32;
    get format_string = the_format_string: Handle<HAsciiString>;
    get fraction_flag = the_fraction_flag: i32;
    get precision_or_denominator = the_precision: i32;
    }
    /// This method is used to set the fields of the class
    /// DimensionUnits
    /// - nbPropVal  : Number of property values, always = 6
    /// - aSecondPos : Secondary Dimension Position
    ///   0 = This is the main text
    ///   1 = Before primary dimension
    ///   2 = After primary dimension
    ///   3 = Above primary dimension
    ///   4 = Below primary dimension
    /// - aUnitsInd  : Units Indicator
    /// - aCharSet   : Character Set used
    /// - aFormat    : Format HAsciiString
    ///   1 = Standard ASCII
    ///   1001 = Symbol Font 1
    ///   1002 = Symbol Font 2
    ///   1003 = Drafting Font
    /// - aFracFlag  : Fraction Flag
    ///   0 = Display values as decimal numbers
    ///   1 = Display values as fractions
    /// - aPrecision : Precision Value
    pub fn init(
        &self,
        nb_prop_val: i32,
        a_second_pos: i32,
        a_units_ind: i32,
        a_char_set: i32,
        a_format: &Handle<HAsciiString>,
        a_frac_flag: i32,
        a_precision: i32,
    ) {
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_prop_val;
            state.the_secondary_dimen_position = a_second_pos;
            state.the_units_indicator = a_units_ind;
            state.the_character_set = a_char_set;
            state.the_format_string = a_format.clone();
            state.the_fraction_flag = a_frac_flag;
            state.the_precision = a_precision;
        }
        self.init_type_and_form(406, 28);
    }
}
