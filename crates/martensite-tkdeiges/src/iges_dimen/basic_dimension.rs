//! IgesDimenBasicDimension.
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
use crate::{iges_entity, iges_methods};
use martensite_tkmath::gp::{Point2, Xy};

iges_entity! {
/// Defines IGES Basic Dimension, Type 406, Form 31,
/// The basic Dimension Property indicates that the referencing
/// dimension entity is to be displayed with a box around text.
IgesDimenBasicDimension, IgesDimenBasicDimensionState, "IgesDimenBasicDimension", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_lower_left: Xy,
the_lower_right: Xy,
the_upper_right: Xy,
the_upper_left: Xy,
}
}

impl IgesDimenBasicDimension {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    }
    /// `init`.
    pub fn init(&self, nb_prop_val: i32, thell: Xy, thelr: Xy, theur: Xy, theul: Xy) {
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_prop_val;
            state.the_lower_left = thell;
            state.the_lower_right = thelr;
            state.the_upper_right = theur;
            state.the_upper_left = theul;
        }
        self.init_type_and_form(406, 31);
    }
    /// returns coordinates of lower left corner
    pub fn lower_left(&self) -> Point2 {
        let mut g: Point2 = Point2::from(self.state.read().the_lower_left);
        g
    }
    /// returns coordinates of lower right corner
    pub fn lower_right(&self) -> Point2 {
        let mut g: Point2 = Point2::from(self.state.read().the_lower_right);
        g
    }
    /// returns coordinates of upper right corner
    pub fn upper_right(&self) -> Point2 {
        let mut g: Point2 = Point2::from(self.state.read().the_upper_right);
        g
    }
    /// returns coordinates of upper left corner
    pub fn upper_left(&self) -> Point2 {
        let mut g: Point2 = Point2::from(self.state.read().the_upper_left);
        g
    }
}
