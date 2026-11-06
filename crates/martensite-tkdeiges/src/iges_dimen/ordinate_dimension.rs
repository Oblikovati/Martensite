//! IgesDimenOrdinateDimension.
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
/// defines IGES Ordinate Dimension, Type `218` Form &lt;0, 1&gt;,
/// Note: The ordinate dimension entity is used to
/// indicate dimensions from a common base line.
/// Dimensioning is only permitted along the XT
/// or YT axis.
IgesDimenOrdinateDimension, IgesDimenOrdinateDimensionState, "IgesDimenOrdinateDimension", parent IgesDataIgesEntity, {
the_note: Handle<IgesDimenGeneralNote>,
is_it_line: bool,
the_witness_line: Handle<IgesDimenWitnessLine>,
the_leader: Handle<IgesDimenLeaderArrow>,
}
}

impl IgesDimenOrdinateDimension {
    iges_methods! {
    get is_line = is_it_line: bool;
    get note = the_note: Handle<IgesDimenGeneralNote>;
    get witness_line = the_witness_line: Handle<IgesDimenWitnessLine>;
    get leader = the_leader: Handle<IgesDimenLeaderArrow>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_note: &Handle<IgesDimenGeneralNote>,
        is_line: bool,
        a_line: &Handle<IgesDimenWitnessLine>,
        an_arrow: &Handle<IgesDimenLeaderArrow>,
    ) {
        {
            let mut state = self.state.write();
            state.the_note = a_note.clone();
            state.is_it_line = is_line;
            state.the_witness_line = a_line.clone();
            state.the_leader = an_arrow.clone();
        }
        if a_line.is_null() || an_arrow.is_null() {
            self.init_type_and_form(218, 0);
        } else {
            self.init_type_and_form(218, 1);
        }
    }
    /// returns True if Leader and False if Witness Line (only for Form 0)
    pub fn is_leader(&self) -> bool {
        !(self.state.read().is_it_line)
    }
}
