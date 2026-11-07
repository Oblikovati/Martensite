//! IgesGraphDrawingSize.
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

iges_entity! {
/// defines IGESDrawingSize, Type `406` Form `16`
///
/// Specifies the drawing size in drawing units. The
/// origin of the drawing is defined to be (0,0) in
/// drawing space
IgesGraphDrawingSize, IgesGraphDrawingSizeState, "IgesGraphDrawingSize", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_x_size: f64,
the_y_size: f64,
}
}

impl IgesGraphDrawingSize {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get x_size = the_x_size: f64;
    get y_size = the_y_size: f64;
    }
    /// This method is used to set the fields of the class
    /// DrawingSize
    /// - nbProps : Number of property values (NP = 2)
    /// - aXSize  : Extent of Drawing along positive XD axis
    /// - aYSize  : Extent of Drawing along positive YD axis
    pub fn init(&self, nb_props: i32, a_x_size: f64, a_y_size: f64) {
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_props;
            state.the_x_size = a_x_size;
            state.the_y_size = a_y_size;
        }
        self.init_type_and_form(406, 16);
    }
}
