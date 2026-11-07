//! IgesAppliFlowLineSpec.
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
use crate::iges_basic::entity_shape::{iges_array1_length, iges_array1_lower};
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item};
use crate::iges_entity;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkernel::tcollection::HAsciiString;

iges_entity! {
/// defines FlowLineSpec, Type `406` Form `14`
/// Attaches one or more text strings to entities being
/// used to represent a flow line
IgesAppliFlowLineSpec, IgesAppliFlowLineSpecState, "IgesAppliFlowLineSpec", parent IgesDataIgesEntity, {
the_name_and_modifiers: Handle<HArray1<Handle<HAsciiString>>>,
}
}

impl IgesAppliFlowLineSpec {
    /// This method is used to set the fields of the class
    /// FlowLineSpec
    /// - allProperties : primary flow line specification and modifiers
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        all_properties: &Handle<HArray1<Handle<HAsciiString>>>,
    ) -> Result<(), Failure> {
        if iges_array1_lower(all_properties, "IgesAppliFlowLineSpec::init")? != 1 {
            return Err(exceptions::dimension_mismatch(
                "IgesAppliFlowLineSpec : Init",
            ));
        }
        self.state.write().the_name_and_modifiers = all_properties.clone();
        self.init_type_and_form(406, 14);
        Ok(())
    }
    /// returns the number of property values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_property_values(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_name_and_modifiers,
            "IgesAppliFlowLineSpec::nb_property_values",
        )
    }
    /// returns primary flow line specification name
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn flow_line_name(&self) -> Result<Handle<HAsciiString>, Failure> {
        iges_array1_item(
            &self.state.read().the_name_and_modifiers,
            1,
            "IgesAppliFlowLineSpec::flow_line_name",
        )
    }
    /// returns specified modifier element
    /// raises exception if Index at most 1 or Index &gt; NbPropertyValues
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn modifier(&self, index: i32) -> Result<Handle<HAsciiString>, Failure> {
        iges_array1_item(
            &self.state.read().the_name_and_modifiers,
            index,
            "IgesAppliFlowLineSpec::modifier",
        )
    }
}
