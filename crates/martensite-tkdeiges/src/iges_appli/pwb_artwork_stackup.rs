//! IgesAppliPWBArtworkStackup.
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
/// defines PWBArtworkStackup, Type `406` Form `25`
/// Used to communicate which exchange file levels are to
/// be combined in order to create the artwork for a
/// printed wire board (PWB). This property should be
/// attached to the entity defining the printed wire
/// assembly (PWA) or if no such entity exists, then the
/// property should stand alone in the file.
IgesAppliPWBArtworkStackup, IgesAppliPWBArtworkStackupState, "IgesAppliPWBArtworkStackup", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_artwork_stackup_ident: Handle<HAsciiString>,
the_level_numbers: Handle<HArray1<i32>>,
}
}

impl IgesAppliPWBArtworkStackup {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get identification = the_artwork_stackup_ident: Handle<HAsciiString>;
    }
    /// This method is used to set the fields of the class
    /// PWBArtworkStackup
    /// - nbPropVal    : number of property values
    /// - anArtIdent   : Artwork Stackup Identification
    /// - allLevelNums : Level Numbers
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        nb_prop_val: i32,
        an_art_ident: &Handle<HAsciiString>,
        all_level_nums: &Handle<HArray1<i32>>,
    ) -> Result<(), Failure> {
        if iges_array1_lower(all_level_nums, "IgesAppliPWBArtworkStackup::init")? != 1 {
            return Err(exceptions::dimension_mismatch(
                "IgesAppliPWBArtworkStackup : Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_prop_val;
            state.the_artwork_stackup_ident = an_art_ident.clone();
            state.the_level_numbers = all_level_nums.clone();
        }
        self.init_type_and_form(406, 25);
        Ok(())
    }
    /// returns total number of Level Numbers
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_level_numbers(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_level_numbers,
            "IgesAppliPWBArtworkStackup::nb_level_numbers",
        )
    }
    /// returns Level Number
    /// raises exception if Index at most 0 or Index &gt; NbLevelNumbers
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn level_number(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_level_numbers,
            index,
            "IgesAppliPWBArtworkStackup::level_number",
        )
    }
}
