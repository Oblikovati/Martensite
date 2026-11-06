//! IgesDimenAngularDimension.
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
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Point2, Xy, Xyz};

iges_entity! {
/// defines AngularDimension, Type `202` Form `0`
/// Used to dimension angles
IgesDimenAngularDimension, IgesDimenAngularDimensionState, "IgesDimenAngularDimension", parent IgesDataIgesEntity, {
the_note: Handle<IgesDimenGeneralNote>,
the_first_witness_line: Handle<IgesDimenWitnessLine>,
the_second_witness_line: Handle<IgesDimenWitnessLine>,
the_vertex: Xy,
the_radius: f64,
the_first_leader: Handle<IgesDimenLeaderArrow>,
the_second_leader: Handle<IgesDimenLeaderArrow>,
}
}

impl IgesDimenAngularDimension {
    iges_methods! {
    get note = the_note: Handle<IgesDimenGeneralNote>;
    get first_witness_line = the_first_witness_line: Handle<IgesDimenWitnessLine>;
    get second_witness_line = the_second_witness_line: Handle<IgesDimenWitnessLine>;
    get radius = the_radius: f64;
    get first_leader = the_first_leader: Handle<IgesDimenLeaderArrow>;
    get second_leader = the_second_leader: Handle<IgesDimenLeaderArrow>;
    }
    /// This method is used to set the fields of the class
    /// AngularDimension
    /// - aNote         : General Note Entity
    /// - aLine         : First Witness Line Entity or Null
    ///   Handle
    /// - anotherLine   : Second Witness Line Entity or Null
    ///   Handle
    /// - aVertex       : Coordinates of vertex point
    /// - aRadius       : Radius of leader arcs
    /// - aLeader       : First Leader Entity
    /// - anotherLeader : Second Leader Entity
    pub fn init(
        &self,
        a_note: &Handle<IgesDimenGeneralNote>,
        a_line: &Handle<IgesDimenWitnessLine>,
        another_line: &Handle<IgesDimenWitnessLine>,
        a_vertex: Xy,
        a_radius: f64,
        a_leader: &Handle<IgesDimenLeaderArrow>,
        another_leader: &Handle<IgesDimenLeaderArrow>,
    ) {
        {
            let mut state = self.state.write();
            state.the_note = a_note.clone();
            state.the_first_witness_line = a_line.clone();
            state.the_second_witness_line = another_line.clone();
            state.the_vertex = a_vertex;
            state.the_radius = a_radius;
            state.the_first_leader = a_leader.clone();
            state.the_second_leader = another_leader.clone();
        }
        self.init_type_and_form(202, 0);
    }
    /// returns False if theFirstWitnessLine is Null Handle.
    pub fn has_first_witness_line(&self) -> bool {
        !(self.state.read().the_first_witness_line.is_null())
    }
    /// returns False if theSecondWitnessLine is Null Handle.
    pub fn has_second_witness_line(&self) -> bool {
        !(self.state.read().the_second_witness_line.is_null())
    }
    /// returns the coordinates of the Vertex point as Pnt2d from gp.
    pub fn vertex(&self) -> Point2 {
        let mut vertex: Point2 = Point2::from(self.state.read().the_vertex);
        vertex
    }
    /// returns the coordinates of the Vertex point as Pnt2d from gp
    /// after Transformation. (Z = 0.0 for Transformation)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_vertex(&self) -> Result<Point2, Failure> {
        let mut point: Xyz = Xyz::new(
            self.state.read().the_vertex.x(),
            self.state.read().the_vertex.y(),
            0.0,
        );
        if self.has_transf() {
            self.location()?.transforms(&mut point);
        }
        Ok(Point2::new(point.x(), point.y()))
    }
}
