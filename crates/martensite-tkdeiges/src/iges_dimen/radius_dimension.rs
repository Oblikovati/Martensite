//! IgesDimenRadiusDimension.
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
use crate::{iges_entity, iges_methods};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Point2, Point3, Xy, Xyz};

iges_entity! {
/// Defines IGES Radius Dimension, type `222` Form &lt;0, 1&gt;,
/// A Radius Dimension Entity consists of a General Note, a
/// leader, and an arc center point. A second form of this
/// entity accounts for the occasional need to have two
/// leader entities referenced.
IgesDimenRadiusDimension, IgesDimenRadiusDimensionState, "IgesDimenRadiusDimension", parent IgesDataIgesEntity, {
the_note: Handle<IgesDimenGeneralNote>,
the_leader_arrow: Handle<IgesDimenLeaderArrow>,
the_center: Xy,
the_leader2: Handle<IgesDimenLeaderArrow>,
}
}

impl IgesDimenRadiusDimension {
    iges_methods! {
    get note = the_note: Handle<IgesDimenGeneralNote>;
    get leader = the_leader_arrow: Handle<IgesDimenLeaderArrow>;
    get leader2 = the_leader2: Handle<IgesDimenLeaderArrow>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_note: &Handle<IgesDimenGeneralNote>,
        an_arrow: &Handle<IgesDimenLeaderArrow>,
        arc_center: Xy,
        another_arrow: &Handle<IgesDimenLeaderArrow>,
    ) {
        {
            let mut state = self.state.write();
            state.the_note = a_note.clone();
            state.the_leader_arrow = an_arrow.clone();
            state.the_center = arc_center;
            state.the_leader2 = another_arrow.clone();
        }
        if !another_arrow.is_null() {
            self.init_type_and_form(222, 1);
        } else {
            self.init_type_and_form(222, self.form_number());
        }
    }
    /// Allows to change Form Number
    /// (1 admits null arrow)
    pub fn init_form(&self, form: i32) {
        self.init_type_and_form(222, form);
    }
    /// returns the coordinates of the Arc Center
    pub fn center(&self) -> Point2 {
        let mut g: Point2 = Point2::from(self.state.read().the_center);
        g
    }
    /// returns the coordinates of the Arc Center after Transformation
    /// (Z coord taken from ZDepth of Leader Entity)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_center(&self) -> Result<Point3, Failure> {
        let mut tmp_xyz: Xyz = Xyz::new(
            self.state.read().the_center.x(),
            self.state.read().the_center.y(),
            iges_live(
                &self.state.read().the_leader_arrow,
                "IgesDimenRadiusDimension::transformed_center",
            )?
            .z_depth(),
        );
        if self.has_transf() {
            self.location()?.transforms(&mut tmp_xyz);
        }
        Ok(Point3::from(tmp_xyz))
    }
    /// returns True if form is 1, False if 0
    pub fn has_leader2(&self) -> bool {
        !(self.state.read().the_leader2.is_null())
    }
}
