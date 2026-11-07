//! IgesDimenLinearDimension.
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
use crate::iges_dimen::{IgesDimenGeneralNote, IgesDimenLeaderArrow, IgesDimenWitnessLine};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::standard::{Failure, Handle, exceptions};

iges_entity! {
/// defines LinearDimension, Type `216` Form `0`
/// Used for linear dimensioning
IgesDimenLinearDimension, IgesDimenLinearDimensionState, "IgesDimenLinearDimension", parent IgesDataIgesEntity, {
the_note: Handle<IgesDimenGeneralNote>,
the_first_leader: Handle<IgesDimenLeaderArrow>,
the_second_leader: Handle<IgesDimenLeaderArrow>,
the_first_witness: Handle<IgesDimenWitnessLine>,
the_second_witness: Handle<IgesDimenWitnessLine>,
}
}

impl IgesDimenLinearDimension {
    iges_methods! {
    get note = the_note: Handle<IgesDimenGeneralNote>;
    get first_leader = the_first_leader: Handle<IgesDimenLeaderArrow>;
    get second_leader = the_second_leader: Handle<IgesDimenLeaderArrow>;
    get first_witness = the_first_witness: Handle<IgesDimenWitnessLine>;
    get second_witness = the_second_witness: Handle<IgesDimenWitnessLine>;
    }
    /// This method is used to set the fields of the class
    /// LinearDimension
    /// - aNote          : General Note Entity
    /// - aLeader        : First Leader Entity
    /// - anotherLeader  : Second Leader Entity
    /// - aWitness       : First Witness Line Entity or a Null
    ///   Handle
    /// - anotherWitness : Second Witness Line Entity or a Null
    ///   Handle
    pub fn init(
        &self,
        a_note: &Handle<IgesDimenGeneralNote>,
        a_leader: &Handle<IgesDimenLeaderArrow>,
        another_leader: &Handle<IgesDimenLeaderArrow>,
        a_witness: &Handle<IgesDimenWitnessLine>,
        another_witness: &Handle<IgesDimenWitnessLine>,
    ) {
        {
            let mut state = self.state.write();
            state.the_note = a_note.clone();
            state.the_first_leader = a_leader.clone();
            state.the_second_leader = another_leader.clone();
            state.the_first_witness = a_witness.clone();
            state.the_second_witness = another_witness.clone();
        }
        self.init_type_and_form(216, self.form_number());
    }
    /// Changes FormNumber (indicates the Nature of the Dimension
    /// Unspecified, Diameter or Radius)
    /// Error if not in range \[0-2\]
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn set_form_number(&self, fm: i32) -> Result<(), Failure> {
        if (fm < 0) || (fm > 2) {
            return Err(exceptions::out_of_range(
                "IgesDimenLinearDimension : SetFormNumber",
            ));
        }
        self.init_type_and_form(216, fm);
        Ok(())
    }
    /// returns False if no first witness line
    pub fn has_first_witness(&self) -> bool {
        !(self.state.read().the_first_witness.is_null())
    }
    /// returns False if no second witness line
    pub fn has_second_witness(&self) -> bool {
        !(self.state.read().the_second_witness.is_null())
    }
}
