//! IgesAppliLevelFunction.
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
/// defines LevelFunction, Type `406` Form `3`
/// Used to transfer the meaning or intended use of a level
/// in the sending system
IgesAppliLevelFunction, IgesAppliLevelFunctionState, "IgesAppliLevelFunction", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_func_descrip_code: i32,
the_func_descrip: Handle<HAsciiString>,
}
}

impl IgesAppliLevelFunction {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get func_description_code = the_func_descrip_code: i32;
    get func_description = the_func_descrip: Handle<HAsciiString>;
    }
    /// This method is used to set the fields of the class
    /// LevelFunction
    /// - nbPropVal    : Number of Properties, always = 2
    /// - aCode        : Function Description code
    ///   default = 0
    /// - aFuncDescrip : Function Description
    ///   default = null string
    pub fn init(&self, nb_prop_val: i32, a_code: i32, a_func_descrip: &Handle<HAsciiString>) {
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_prop_val;
            state.the_func_descrip_code = a_code;
            state.the_func_descrip = a_func_descrip.clone();
        }
        self.init_type_and_form(406, 3);
    }
}
