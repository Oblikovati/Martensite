//! IgesGraphNominalSize.
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
/// defines IGESNominalSize, Type `406` Form `13`
///
/// Specifies a value, a name, and optionally a
/// reference to an engineering standard
IgesGraphNominalSize, IgesGraphNominalSizeState, "IgesGraphNominalSize", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_nominal_size_value: f64,
the_nominal_size_name: Handle<HAsciiString>,
the_standard_name: Handle<HAsciiString>,
}
}

impl IgesGraphNominalSize {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get nominal_size_value = the_nominal_size_value: f64;
    get nominal_size_name = the_nominal_size_name: Handle<HAsciiString>;
    get standard_name = the_standard_name: Handle<HAsciiString>;
    }
    /// This method is used to set the fields of the class
    /// NominalSize
    /// - nbProps           : Number of property values (2 or 3)
    /// - aNominalSizeValue : NominalSize Value
    /// - aNominalSizeName  : NominalSize Name
    /// - aStandardName     : Name of relevant engineering standard
    pub fn init(
        &self,
        nb_props: i32,
        a_nominal_size_value: f64,
        a_nominal_size_name: &Handle<HAsciiString>,
        a_standard_name: &Handle<HAsciiString>,
    ) {
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_props;
            state.the_nominal_size_value = a_nominal_size_value;
            state.the_nominal_size_name = a_nominal_size_name.clone();
            state.the_standard_name = a_standard_name.clone();
        }
        self.init_type_and_form(406, 13);
    }
    /// returns True if an engineering Standard is defined for `me`
    /// else, returns False
    pub fn has_standard_name(&self) -> bool {
        !(self.state.read().the_standard_name.is_null())
    }
}
