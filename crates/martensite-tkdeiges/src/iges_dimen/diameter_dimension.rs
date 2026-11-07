//! IgesDimenDiameterDimension.
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
use crate::iges_dimen::{IgesDimenGeneralNote, IgesDimenLeaderArrow};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Point2, Xy, Xyz};

iges_entity! {
/// defines DiameterDimension, Type `206` Form `0`
/// Used for dimensioning diameters
IgesDimenDiameterDimension, IgesDimenDiameterDimensionState, "IgesDimenDiameterDimension", parent IgesDataIgesEntity, {
the_note: Handle<IgesDimenGeneralNote>,
the_first_leader: Handle<IgesDimenLeaderArrow>,
the_second_leader: Handle<IgesDimenLeaderArrow>,
the_center: Xy,
}
}

impl IgesDimenDiameterDimension {
    iges_methods! {
    get note = the_note: Handle<IgesDimenGeneralNote>;
    get first_leader = the_first_leader: Handle<IgesDimenLeaderArrow>;
    get second_leader = the_second_leader: Handle<IgesDimenLeaderArrow>;
    }
    /// This method is used to set the fields of the class
    /// DiameterDimension
    /// - aNote         : General Note Entity
    /// - aLeader       : First Leader Entity
    /// - anotherLeader : Second Leader Entity or a Null Handle.
    /// - aCenter       : Arc center coordinates
    pub fn init(
        &self,
        a_note: &Handle<IgesDimenGeneralNote>,
        a_leader: &Handle<IgesDimenLeaderArrow>,
        another_leader: &Handle<IgesDimenLeaderArrow>,
        a_center: Xy,
    ) {
        {
            let mut state = self.state.write();
            state.the_note = a_note.clone();
            state.the_first_leader = a_leader.clone();
            state.the_second_leader = another_leader.clone();
            state.the_center = a_center;
        }
        self.init_type_and_form(206, 0);
    }
    /// returns False if theSecondleader is a Null Handle.
    pub fn has_second_leader(&self) -> bool {
        !(self.state.read().the_second_leader.is_null())
    }
    /// returns the Arc Center coordinates as Pnt2d from package gp
    pub fn center(&self) -> Point2 {
        let mut center: Point2 = Point2::from(self.state.read().the_center);
        center
    }
    /// returns the Arc Center coordinates as Pnt2d from package gp
    /// after Transformation. (Z = 0.0 for Transformation)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_center(&self) -> Result<Point2, Failure> {
        let mut center: Xyz = Xyz::new(
            self.state.read().the_center.x(),
            self.state.read().the_center.y(),
            0.0,
        );
        if self.has_transf() {
            self.location()?.transforms(&mut center);
        }
        Ok(Point2::new(center.x(), center.y()))
    }
}
