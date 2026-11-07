//! IgesDimenDimensionTolerance.
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
/// defines Dimension Tolerance, Type `406`, Form `29`
/// Provides tolerance information for a dimension which
/// can be used by the receiving system to regenerate the
/// dimension.
IgesDimenDimensionTolerance, IgesDimenDimensionToleranceState, "IgesDimenDimensionTolerance", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_secondary_tolerance_flag: i32,
the_tolerance_type: i32,
the_tolerance_placement_flag: i32,
the_upper_tolerance: f64,
the_lower_tolerance: f64,
the_sign_suppression_flag: bool,
the_fraction_flag: i32,
the_precision: i32,
}
}

impl IgesDimenDimensionTolerance {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get secondary_tolerance_flag = the_secondary_tolerance_flag: i32;
    get tolerance_type = the_tolerance_type: i32;
    get tolerance_placement_flag = the_tolerance_placement_flag: i32;
    get upper_tolerance = the_upper_tolerance: f64;
    get lower_tolerance = the_lower_tolerance: f64;
    get sign_suppression_flag = the_sign_suppression_flag: bool;
    get fraction_flag = the_fraction_flag: i32;
    get precision = the_precision: i32;
    }
    /// This method is used to set the fields of the class
    /// DimensionTolerance
    /// - nbPropVal     : Number of property values, default = 8
    /// - aSecTolFlag   : Secondary Tolerance Flag
    ///   0 = Applies to primary dimension
    ///   1 = Applies to secondary dimension
    ///   2 = Display values as fractions
    /// - aTolType      : Tolerance Type
    ///   1  = Bilateral
    ///   2  = Upper/Lower
    ///   3  = Unilateral Upper
    ///   4  = Unilateral Lower
    ///   5  = Range - min before max
    ///   6  = Range - min after max
    ///   7  = Range - min above max
    ///   8  = Range - min below max
    ///   9  = Nominal + Range - min above max
    ///   10 = Nominal + Range - min below max
    /// - aTolPlaceFlag : Tolerance Placement Flag
    ///   1 = Before nominal value
    ///   2 = After nominal value
    ///   3 = Above nominal value
    ///   4 = Below nominal value
    /// - anUpperTol    : Upper Tolerance
    /// - aLowerTol     : Lower Tolerance
    /// - aSignFlag     : Sign Suppression Flag
    /// - aFracFlag     : Fraction Flag
    ///   0 = Display values as decimal numbers
    ///   1 = Display values as mixed fractions
    ///   2 = Display values as fractions
    /// - aPrecision    : Precision Value
    pub fn init(
        &self,
        nb_prop_val: i32,
        a_sec_tol_flag: i32,
        a_tol_type: i32,
        a_tol_place_flag: i32,
        an_upper_tol: f64,
        a_lower_tol: f64,
        a_sign_flag: bool,
        a_frac_flag: i32,
        a_precision: i32,
    ) {
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_prop_val;
            state.the_secondary_tolerance_flag = a_sec_tol_flag;
            state.the_tolerance_type = a_tol_type;
            state.the_tolerance_placement_flag = a_tol_place_flag;
            state.the_upper_tolerance = an_upper_tol;
            state.the_lower_tolerance = a_lower_tol;
            state.the_sign_suppression_flag = a_sign_flag;
            state.the_fraction_flag = a_frac_flag;
            state.the_precision = a_precision;
        }
        self.init_type_and_form(406, 29);
    }
}
