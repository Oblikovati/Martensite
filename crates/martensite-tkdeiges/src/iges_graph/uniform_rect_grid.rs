//! IgesGraphUniformRectGrid.
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
use martensite_tkmath::gp::{Point2, Vector2, Xy};

iges_entity! {
/// defines IGESUniformRectGrid, Type `406` Form `22`
///
/// Stores sufficient information for the creation of
/// a uniform rectangular grid within a drawing
IgesGraphUniformRectGrid, IgesGraphUniformRectGridState, "IgesGraphUniformRectGrid", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
is_it_finite: i32,
is_it_line: i32,
is_it_weighted: i32,
the_grid_point: Xy,
the_grid_spacing: Xy,
the_nb_points_x: i32,
the_nb_points_y: i32,
}
}

impl IgesGraphUniformRectGrid {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get nb_points_x = the_nb_points_x: i32;
    get nb_points_y = the_nb_points_y: i32;
    }
    /// This method is used to set the fields of the class
    /// UniformRectGrid
    /// - nbProps      : Number of property values (NP = 9)
    /// - finite       : Finite/Infinite grid flag
    /// - line         : Line/Point grid flag
    /// - weighted     : Weighted/Unweighted grid flag
    /// - aGridPoint   : Point on the grid
    /// - aGridSpacing : Grid spacing
    /// - pointsX      : No. of points/lines in X Direction
    /// - pointsY      : No. of points/lines in Y Direction
    pub fn init(
        &self,
        nb_props: i32,
        finite: i32,
        line: i32,
        weighted: i32,
        a_grid_point: Xy,
        a_grid_spacing: Xy,
        points_x: i32,
        points_y: i32,
    ) {
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_props;
            state.is_it_finite = finite;
            state.is_it_line = line;
            state.is_it_weighted = weighted;
            state.the_grid_point = a_grid_point;
            state.the_grid_spacing = a_grid_spacing;
            state.the_nb_points_x = points_x;
            state.the_nb_points_y = points_y;
        }
        self.init_type_and_form(406, 22);
    }
    /// returns False if `me` is an infinite grid,
    /// True if `me` is a finite grid.
    pub fn is_finite(&self) -> bool {
        (self.state.read().is_it_finite) == 1
    }
    /// returns False if `me` is a Point grid,
    /// True if `me` is a Line grid.
    pub fn is_line(&self) -> bool {
        (self.state.read().is_it_line) == 1
    }
    /// returns False if `me` is a Weighted grid,
    /// True if `me` is not a Weighted grid.
    pub fn is_weighted(&self) -> bool {
        (self.state.read().is_it_weighted) == 0
    }
    /// returns coordinates of lower left corner,
    /// if `me` is a finite grid,
    /// coordinates of an arbitrary point,
    /// if `me` is an infinite grid.
    pub fn grid_point(&self) -> Point2 {
        Point2::from(self.state.read().the_grid_point)
    }
    /// returns the grid-spacing in drawing coordinates.
    pub fn grid_spacing(&self) -> Vector2 {
        Vector2::from(self.state.read().the_grid_spacing)
    }
}
