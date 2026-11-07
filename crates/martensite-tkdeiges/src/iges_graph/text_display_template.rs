//! IgesGraphTextDisplayTemplate.
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
use crate::iges_graph::IgesGraphTextFontDef;
use crate::{iges_entity, iges_methods};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Point3, Xyz};

iges_entity! {
/// defines IGES TextDisplayTemplate Entity,
///
/// Used to set parameters for display of information
/// which has been logically included in another entity
/// as a parameter value
IgesGraphTextDisplayTemplate, IgesGraphTextDisplayTemplateState, "IgesGraphTextDisplayTemplate", parent IgesDataIgesEntity, {
the_box_width: f64,
the_box_height: f64,
the_font_code: i32,
the_font_entity: Handle<IgesGraphTextFontDef>,
the_slant_angle: f64,
the_rotation_angle: f64,
the_mirror_flag: i32,
the_rotate_flag: i32,
the_corner: Xyz,
}
}

impl IgesGraphTextDisplayTemplate {
    iges_methods! {
    get box_width = the_box_width: f64;
    get box_height = the_box_height: f64;
    get font_code = the_font_code: i32;
    get font_entity = the_font_entity: Handle<IgesGraphTextFontDef>;
    get slant_angle = the_slant_angle: f64;
    get rotation_angle = the_rotation_angle: f64;
    get mirror_flag = the_mirror_flag: i32;
    get rotate_flag = the_rotate_flag: i32;
    }
    /// This method is used to set the fields of the class
    /// TextDisplayTemplate
    /// - aWidth         : Character box width
    /// - aHeight        : Character box height
    /// - afontCode      : Font code
    /// - aFontEntity    : Text Font Definition Entity
    /// - aSlantAngle    : Slant angle
    /// - aRotationAngle : Rotation angle
    /// - aMirrorFlag    : Mirror Flag
    /// - aRotationFlag  : Rotate internal text flag
    /// - aCorner        : Lower left corner coordinates(Form No. 0),
    ///   Increments from coordinates (Form No. 1)
    pub fn init(
        &self,
        a_width: f64,
        a_height: f64,
        a_font_code: i32,
        a_font_entity: &Handle<IgesGraphTextFontDef>,
        a_slant_angle: f64,
        a_rotation_angle: f64,
        a_mirror_flag: i32,
        a_rotation_flag: i32,
        a_corner: Xyz,
    ) {
        {
            let mut state = self.state.write();
            state.the_box_width = a_width;
            state.the_box_height = a_height;
            state.the_font_code = a_font_code;
            state.the_font_entity = a_font_entity.clone();
            state.the_slant_angle = a_slant_angle;
            state.the_rotation_angle = a_rotation_angle;
            state.the_mirror_flag = a_mirror_flag;
            state.the_rotate_flag = a_rotation_flag;
            state.the_corner = a_corner;
        }
        self.init_type_and_form(312, self.form_number());
    }
    /// Sets `me` to be Incremental (Form 1) if `mode` is True,
    /// or Basolute (Form 0) else
    pub fn set_incremental(&self, f: bool) {
        self.init_type_and_form(312, if f { 1 } else { 0 });
    }
    /// returns True if entity is Incremental (Form 1).
    /// False if entity is Absolute (Form 0).
    pub fn is_incremental(&self) -> bool {
        self.form_number() == 1
    }
    /// returns False if theFontEntity is Null, True otherwise.
    pub fn is_font_entity(&self) -> bool {
        !(self.state.read().the_font_entity.is_null())
    }
    /// If IsIncremental() returns False,
    /// gets coordinates of lower left corner
    /// of first character box.
    /// If IsIncremental() returns True,
    /// gets increments from X, Y, Z coordinates
    /// found in parent entity.
    pub fn starting_corner(&self) -> Point3 {
        Point3::from(self.state.read().the_corner)
    }
    /// If IsIncremental() returns False,
    /// gets coordinates of lower left corner
    /// of first character box.
    /// If IsIncremental() returns True,
    /// gets increments from X, Y, Z coordinates
    /// found in parent entity.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_starting_corner(&self) -> Result<Point3, Failure> {
        let mut temp_xyz: Xyz = self.state.read().the_corner;
        if self.has_transf() {
            self.location()?.transforms(&mut temp_xyz);
        }
        Ok(Point3::from(temp_xyz))
    }
}
