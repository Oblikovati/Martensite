//! IgesAppliLevelToPWBLayerMap.
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
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkernel::tcollection::HAsciiString;

iges_entity! {
/// defines LevelToPWBLayerMap, Type `406` Form `24`
/// Used to correlate an exchange file level number with
/// its corresponding native level identifier, physical PWB
/// layer number and predefined functional level
/// identification
IgesAppliLevelToPWBLayerMap, IgesAppliLevelToPWBLayerMapState, "IgesAppliLevelToPWBLayerMap", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_exchange_file_level_number: Handle<HArray1<i32>>,
the_native_level: Handle<HArray1<Handle<HAsciiString>>>,
the_physical_layer_number: Handle<HArray1<i32>>,
the_exchange_file_level_ident: Handle<HArray1<Handle<HAsciiString>>>,
}
}

impl IgesAppliLevelToPWBLayerMap {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    }
    /// This method is used to set the fields of the class
    /// LevelToPWBLayerMap
    /// - nbPropVal       : Number of property values
    /// - allExchLevels   : Exchange File Level Numbers
    /// - allNativeLevels : Native Level Identifications
    /// - allPhysLevels   : Physical Layer Numbers
    /// - allExchIdents   : Exchange File Level Identifications
    ///   raises exception if allExchLevels, allNativeLevels, allPhysLevels
    ///   and all ExchIdents are not of same dimensions
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        nb_prop_val: i32,
        all_exch_levels: &Handle<HArray1<i32>>,
        all_native_levels: &Handle<HArray1<Handle<HAsciiString>>>,
        all_phys_levels: &Handle<HArray1<i32>>,
        all_exch_idents: &Handle<HArray1<Handle<HAsciiString>>>,
    ) -> Result<(), Failure> {
        let mut num: i32 =
            iges_array1_length(all_exch_levels, "IgesAppliLevelToPWBLayerMap::init")?;
        if (((iges_array1_lower(all_exch_levels, "IgesAppliLevelToPWBLayerMap::init")? != 1)
            || ((iges_array1_lower(all_native_levels, "IgesAppliLevelToPWBLayerMap::init")? != 1)
                || (iges_array1_length(all_native_levels, "IgesAppliLevelToPWBLayerMap::init")?
                    != num)))
            || ((iges_array1_lower(all_phys_levels, "IgesAppliLevelToPWBLayerMap::init")? != 1)
                || (iges_array1_length(all_phys_levels, "IgesAppliLevelToPWBLayerMap::init")?
                    != num)))
            || ((iges_array1_lower(all_exch_idents, "IgesAppliLevelToPWBLayerMap::init")? != 1)
                || (iges_array1_length(all_exch_idents, "IgesAppliLevelToPWBLayerMap::init")?
                    != num))
        {
            return Err(exceptions::dimension_mismatch(
                "IgesAppliLevelToPWBLayerMap: Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_prop_val;
            state.the_exchange_file_level_number = all_exch_levels.clone();
            state.the_native_level = all_native_levels.clone();
            state.the_physical_layer_number = all_phys_levels.clone();
            state.the_exchange_file_level_ident = all_exch_idents.clone();
        }
        self.init_type_and_form(406, 24);
        Ok(())
    }
    /// returns number of level to layer definitions
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_level_to_layer_defs(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_exchange_file_level_number,
            "IgesAppliLevelToPWBLayerMap::nb_level_to_layer_defs",
        )
    }
    /// returns Exchange File Level Number
    /// raises exception if Index at most 0 or Index &gt; NbLevelToLayerDefs
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn exchange_file_level_number(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_exchange_file_level_number,
            index,
            "IgesAppliLevelToPWBLayerMap::exchange_file_level_number",
        )
    }
    /// returns Native Level Identification
    /// raises exception if Index at most 0 or Index &gt; NbLevelToLayerDefs
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn native_level(&self, index: i32) -> Result<Handle<HAsciiString>, Failure> {
        iges_array1_item(
            &self.state.read().the_native_level,
            index,
            "IgesAppliLevelToPWBLayerMap::native_level",
        )
    }
    /// returns Physical Layer Number
    /// raises exception if Index at most 0 or Index &gt; NbLevelToLayerDefs
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn physical_layer_number(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_physical_layer_number,
            index,
            "IgesAppliLevelToPWBLayerMap::physical_layer_number",
        )
    }
    /// `exchange_file_level_ident`.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn exchange_file_level_ident(&self, index: i32) -> Result<Handle<HAsciiString>, Failure> {
        iges_array1_item(
            &self.state.read().the_exchange_file_level_ident,
            index,
            "IgesAppliLevelToPWBLayerMap::exchange_file_level_ident",
        )
    }
}
