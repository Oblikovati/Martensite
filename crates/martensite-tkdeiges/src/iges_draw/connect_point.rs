//! IgesDrawConnectPoint.
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
use crate::iges_graph::IgesGraphTextDisplayTemplate;
use crate::{iges_entity, iges_methods};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::{Point3, Xyz};

iges_entity! {
/// defines IGESConnectPoint, Type `132` Form Number `0`
///
/// Connect Point Entity describes a point of connection for
/// zero, one or more entities. Its referenced from Composite
/// curve, or Network Subfigure Definition/Instance, or Flow
/// Associative Instance, or it may stand alone.
IgesDrawConnectPoint, IgesDrawConnectPointState, "IgesDrawConnectPoint", parent IgesDataIgesEntity, {
the_point: Xyz,
the_display_symbol: Handle<IgesDataIgesEntity>,
the_type_flag: i32,
the_function_flag: i32,
the_function_identifier: Handle<HAsciiString>,
the_identifier_template: Handle<IgesGraphTextDisplayTemplate>,
the_function_name: Handle<HAsciiString>,
the_function_template: Handle<IgesGraphTextDisplayTemplate>,
the_point_identifier: i32,
the_function_code: i32,
the_swap_flag: bool,
the_owner_subfigure: Handle<IgesDataIgesEntity>,
}
}

impl IgesDrawConnectPoint {
    iges_methods! {
    get display_symbol = the_display_symbol: Handle<IgesDataIgesEntity>;
    get type_flag = the_type_flag: i32;
    get function_flag = the_function_flag: i32;
    get function_identifier = the_function_identifier: Handle<HAsciiString>;
    get identifier_template = the_identifier_template: Handle<IgesGraphTextDisplayTemplate>;
    get function_name = the_function_name: Handle<HAsciiString>;
    get function_template = the_function_template: Handle<IgesGraphTextDisplayTemplate>;
    get point_identifier = the_point_identifier: i32;
    get function_code = the_function_code: i32;
    get swap_flag = the_swap_flag: bool;
    get owner_subfigure = the_owner_subfigure: Handle<IgesDataIgesEntity>;
    }
    /// This method is used to set the fields of the class
    /// ConnectPoint
    /// - aPoint               : A Coordinate point
    /// - aDisplaySymbol       : Display symbol Geometry
    /// - aTypeFlag            : Type of the connection
    /// - aFunctionFlag        : Function flag for the connection
    /// - aFunctionIdentifier  : Connection Point Function Identifier
    /// - anIdentifierTemplate : Connection Point Function Template
    /// - aFunctionName        : Connection Point Function Name
    /// - aFunctionTemplate    : Connection Point Function Template
    /// - aPointIdentifier     : Unique Connect Point Identifier
    /// - aFunctionCode        : Connect Point Function Code
    /// - aSwapFlag            : Connect Point Swap Flag
    /// - anOwnerSubfigure     : Pointer to the "Owner" Entity
    pub fn init(
        &self,
        a_point: Xyz,
        a_display_symbol: &Handle<IgesDataIgesEntity>,
        a_type_flag: i32,
        a_function_flag: i32,
        a_function_identifier: &Handle<HAsciiString>,
        an_identifier_template: &Handle<IgesGraphTextDisplayTemplate>,
        a_function_name: &Handle<HAsciiString>,
        a_function_template: &Handle<IgesGraphTextDisplayTemplate>,
        a_point_identifier: i32,
        a_function_code: i32,
        a_swap_flag: i32,
        an_owner_subfigure: &Handle<IgesDataIgesEntity>,
    ) {
        {
            let mut state = self.state.write();
            state.the_point = a_point;
            state.the_display_symbol = a_display_symbol.clone();
            state.the_type_flag = a_type_flag;
            state.the_function_flag = a_function_flag;
            state.the_function_identifier = a_function_identifier.clone();
            state.the_identifier_template = an_identifier_template.clone();
            state.the_function_name = a_function_name.clone();
            state.the_function_template = a_function_template.clone();
            state.the_point_identifier = a_point_identifier;
            state.the_function_code = a_function_code;
            state.the_swap_flag = a_swap_flag != 0;
            state.the_owner_subfigure = an_owner_subfigure.clone();
        }
        self.init_type_and_form(132, 0);
    }
    /// returns the coordinate of the connection point
    pub fn point(&self) -> Point3 {
        let mut temp_point: Point3 = Point3::from(self.state.read().the_point);
        temp_point
    }
    /// returns the Transformed coordinate of the connection point
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_point(&self) -> Result<Point3, Failure> {
        let mut temp_point: Xyz = self.state.read().the_point;
        if self.has_transf() {
            self.location()?.transforms(&mut temp_point);
        }
        let mut temp_res: Point3 = Point3::from(temp_point);
        Ok(temp_res)
    }
    /// returns True if Display symbol is specified
    /// else returns False
    pub fn has_display_symbol(&self) -> bool {
        !(self.state.read().the_display_symbol.is_null())
    }
    /// returns True if Text Display Template is specified for Identifier
    /// else returns False
    pub fn has_identifier_template(&self) -> bool {
        !(self.state.read().the_identifier_template.is_null())
    }
    /// returns True if Text Display Template is specified for Function Name
    /// else returns False
    pub fn has_function_template(&self) -> bool {
        !(self.state.read().the_function_template.is_null())
    }
    /// returns True if Network Subfigure Instance/Definition Entity
    /// is specified
    /// else returns False
    pub fn has_owner_subfigure(&self) -> bool {
        !(self.state.read().the_owner_subfigure.is_null())
    }
}
