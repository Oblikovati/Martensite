//! IgesDimenGeneralLabel.
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
use crate::iges_dimen::{IgesDimenGeneralNote, IgesDimenLeaderArrow};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};

iges_entity! {
/// defines GeneralLabel, Type `210` Form `0`
/// Used for general labeling with leaders
IgesDimenGeneralLabel, IgesDimenGeneralLabelState, "IgesDimenGeneralLabel", parent IgesDataIgesEntity, {
the_note: Handle<IgesDimenGeneralNote>,
the_leaders: Handle<HArray1<Handle<IgesDimenLeaderArrow>>>,
}
}

impl IgesDimenGeneralLabel {
    iges_methods! {
    get note = the_note: Handle<IgesDimenGeneralNote>;
    }
    /// This method is used to set the fields of the class
    /// GeneralLabel
    /// - aNote       : General Note Entity
    /// - someLeaders : Associated Leader Entities
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        a_note: &Handle<IgesDimenGeneralNote>,
        some_leaders: &Handle<HArray1<Handle<IgesDimenLeaderArrow>>>,
    ) -> Result<(), Failure> {
        if iges_array1_lower(some_leaders, "IgesDimenGeneralLabel::init")? != 1 {
            return Err(exceptions::dimension_mismatch(
                "IgesDimenGeneralLabel : Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.the_note = a_note.clone();
            state.the_leaders = some_leaders.clone();
        }
        self.init_type_and_form(210, 0);
        Ok(())
    }
    /// returns Number of Leaders
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_leaders(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_leaders,
            "IgesDimenGeneralLabel::nb_leaders",
        )
    }
    /// returns Leader Entity
    /// raises exception if Index at most 0 or Index &gt; NbLeaders()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn leader(&self, index: i32) -> Result<Handle<IgesDimenLeaderArrow>, Failure> {
        iges_array1_item(
            &self.state.read().the_leaders,
            index,
            "IgesDimenGeneralLabel::leader",
        )
    }
}
