//! IgesGraphDefinitionLevel.
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
use crate::iges_data::{IgesDataLevelListEntity, iges_array1_item};
use crate::iges_entity;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};

iges_entity! {
/// defines IGESDefinitionLevel, Type `406` Form `1`
///
/// Indicates the no. of levels on which an entity is
/// defined
IgesGraphDefinitionLevel, IgesGraphDefinitionLevelState, "IgesGraphDefinitionLevel", kind dyn IgesDataLevelListEntity, {
the_level_numbers: Handle<HArray1<i32>>,
}
}

impl IgesDataLevelListEntity for IgesGraphDefinitionLevel {
    fn nb_level_numbers(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_level_numbers,
            "IgesGraphDefinitionLevel::nb_level_numbers",
        )
    }
    fn level_number(&self, level_index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_level_numbers,
            level_index,
            "IgesGraphDefinitionLevel::level_number",
        )
    }
}

impl IgesGraphDefinitionLevel {
    /// This method is used to set the fields of the class
    /// DefinitionLevel
    /// - allLevelNumbers : Values of Level Numbers
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(&self, all_level_numbers: &Handle<HArray1<i32>>) -> Result<(), Failure> {
        if iges_array1_lower(all_level_numbers, "IgesGraphDefinitionLevel::init")? != 1 {
            return Err(exceptions::dimension_mismatch(
                "IgesGraphDefinitionLevel : Init",
            ));
        }
        self.state.write().the_level_numbers = all_level_numbers.clone();
        self.init_type_and_form(406, 1);
        Ok(())
    }
    /// returns the number of property values in `me`
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_property_values(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_level_numbers,
            "IgesGraphDefinitionLevel::nb_property_values",
        )
    }
}
