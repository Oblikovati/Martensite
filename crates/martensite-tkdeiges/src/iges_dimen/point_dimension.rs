//! IgesDimenPointDimension.
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
use crate::iges_data::{IgesDataIgesEntity, iges_live};
use crate::iges_dimen::{IgesDimenGeneralNote, IgesDimenLeaderArrow};
use crate::iges_geom::{IgesGeomCircularArc, IgesGeomCompositeCurve};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::standard::{Failure, Handle};

iges_entity! {
/// defines IGES Point Dimension, Type `220` Form `0`,
/// A Point Dimension Entity consists of a leader, text, and
/// an optional circle or hexagon enclosing the text
/// IGES specs for this entity mention SimpleClosedPlanarCurve
/// Entity(106/63)which is not listed in LIST.Text In the sequel
/// we have ignored this & considered only the other two entity
/// for representing the hexagon or circle enclosing the text.
IgesDimenPointDimension, IgesDimenPointDimensionState, "IgesDimenPointDimension", parent IgesDataIgesEntity, {
the_note: Handle<IgesDimenGeneralNote>,
the_leader: Handle<IgesDimenLeaderArrow>,
the_geom: Handle<IgesDataIgesEntity>,
}
}

impl IgesDimenPointDimension {
    iges_methods! {
    get note = the_note: Handle<IgesDimenGeneralNote>;
    get leader_arrow = the_leader: Handle<IgesDimenLeaderArrow>;
    get geom = the_geom: Handle<IgesDataIgesEntity>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_note: &Handle<IgesDimenGeneralNote>,
        an_arrow: &Handle<IgesDimenLeaderArrow>,
        a_geom: &Handle<IgesDataIgesEntity>,
    ) {
        {
            let mut state = self.state.write();
            state.the_note = a_note.clone();
            state.the_leader = an_arrow.clone();
            state.the_geom = a_geom.clone();
        }
        self.init_type_and_form(220, 0);
    }
    /// returns the type of geometric entity.
    /// 0 if no hexagon or circle encloses the text
    /// 1 if CircularArc
    /// 2 if CompositeCurve
    /// 3 otherwise
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn geom_case(&self) -> Result<i32, Failure> {
        if self.state.read().the_geom.is_null() {
            return Ok(0);
        } else {
            if (iges_live(
                &self.state.read().the_geom,
                "IgesDimenPointDimension::geom_case",
            )?
            .type_number())
                == 100
            {
                return Ok(1);
            } else {
                if (iges_live(
                    &self.state.read().the_geom,
                    "IgesDimenPointDimension::geom_case",
                )?
                .type_number())
                    == 102
                {
                    return Ok(2);
                } else {
                    return Ok(3);
                }
            }
        }
    }
    /// returns Null handle if GeomCase(me) .ne. 1
    pub fn circular_arc(&self) -> Handle<IgesGeomCircularArc> {
        self.state.read().the_geom.cast::<IgesGeomCircularArc>()
    }
    /// returns Null handle if GeomCase(me) .ne. 2
    pub fn composite_curve(&self) -> Handle<IgesGeomCompositeCurve> {
        self.state.read().the_geom.cast::<IgesGeomCompositeCurve>()
    }
}
