//! IgesDimenCurveDimension.
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
use martensite_tkernel::standard::Handle;

iges_entity! {
/// defines CurveDimension, Type `204` Form `0`
/// Used to dimension curves
/// Consists of one tail segment of nonzero length
/// beginning with an arrowhead and which serves to define
/// the orientation
IgesDimenCurveDimension, IgesDimenCurveDimensionState, "IgesDimenCurveDimension", parent IgesDataIgesEntity, {
the_note: Handle<IgesDimenGeneralNote>,
the_first_curve: Handle<IgesDataIgesEntity>,
the_second_curve: Handle<IgesDataIgesEntity>,
the_first_leader: Handle<IgesDimenLeaderArrow>,
the_second_leader: Handle<IgesDimenLeaderArrow>,
the_first_witness_line: Handle<IgesDimenWitnessLine>,
the_second_witness_line: Handle<IgesDimenWitnessLine>,
}
}

impl IgesDimenCurveDimension {
    iges_methods! {
    get note = the_note: Handle<IgesDimenGeneralNote>;
    get first_curve = the_first_curve: Handle<IgesDataIgesEntity>;
    get second_curve = the_second_curve: Handle<IgesDataIgesEntity>;
    get first_leader = the_first_leader: Handle<IgesDimenLeaderArrow>;
    get second_leader = the_second_leader: Handle<IgesDimenLeaderArrow>;
    get first_witness_line = the_first_witness_line: Handle<IgesDimenWitnessLine>;
    get second_witness_line = the_second_witness_line: Handle<IgesDimenWitnessLine>;
    }
    /// This method is used to set the fields of the class
    /// CurveDimension
    /// - aNote         : General Note Entity
    /// - aCurve        : First Curve Entity
    /// - anotherCurve  : Second Curve Entity or a Null Handle
    /// - aLeader       : First Leader Entity
    /// - anotherLeader : Second Leader Entity
    /// - aLine         : First Witness Line Entity or a Null
    ///   Handle
    /// - anotherLine   : Second Witness Line Entity or a Null
    ///   Handle
    pub fn init(
        &self,
        a_note: &Handle<IgesDimenGeneralNote>,
        a_curve: &Handle<IgesDataIgesEntity>,
        another_curve: &Handle<IgesDataIgesEntity>,
        a_leader: &Handle<IgesDimenLeaderArrow>,
        another_leader: &Handle<IgesDimenLeaderArrow>,
        a_line: &Handle<IgesDimenWitnessLine>,
        another_line: &Handle<IgesDimenWitnessLine>,
    ) {
        {
            let mut state = self.state.write();
            state.the_note = a_note.clone();
            state.the_first_curve = a_curve.clone();
            state.the_second_curve = another_curve.clone();
            state.the_first_leader = a_leader.clone();
            state.the_second_leader = another_leader.clone();
            state.the_first_witness_line = a_line.clone();
            state.the_second_witness_line = another_line.clone();
        }
        self.init_type_and_form(204, 0);
    }
    /// returns False if theSecondCurve is a Null Handle.
    pub fn has_second_curve(&self) -> bool {
        !(self.state.read().the_second_curve.is_null())
    }
    /// returns False if theFirstWitnessLine is a Null Handle.
    pub fn has_first_witness_line(&self) -> bool {
        !(self.state.read().the_first_witness_line.is_null())
    }
    /// returns False if theSecondWitnessLine is a Null Handle.
    pub fn has_second_witness_line(&self) -> bool {
        !(self.state.read().the_second_witness_line.is_null())
    }
}
