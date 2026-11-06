//! More of IgesDimenNewGeneralNote.
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
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item};
use crate::iges_dimen::IgesDimenNewGeneralNote;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::{Point3, Xyz};

impl IgesDimenNewGeneralNote {
    /// returns Character Width of string
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn character_width(&self, index: i32) -> Result<f64, Failure> {
        iges_array1_item(
            &self.state.read().the_char_widths,
            index,
            "IgesDimenNewGeneralNote::character_width",
        )
    }
    /// returns Character Height of string
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn character_height(&self, index: i32) -> Result<f64, Failure> {
        iges_array1_item(
            &self.state.read().the_char_heights,
            index,
            "IgesDimenNewGeneralNote::character_height",
        )
    }
    /// returns Inter-character spacing of string
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn inter_character_space(&self, index: i32) -> Result<f64, Failure> {
        iges_array1_item(
            &self.state.read().the_inter_char_spaces,
            index,
            "IgesDimenNewGeneralNote::inter_character_space",
        )
    }
    /// returns Interline spacing of string
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn interline_space(&self, index: i32) -> Result<f64, Failure> {
        iges_array1_item(
            &self.state.read().the_interline_spaces,
            index,
            "IgesDimenNewGeneralNote::interline_space",
        )
    }
    /// returns FontStyle of string
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn font_style(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_font_styles,
            index,
            "IgesDimenNewGeneralNote::font_style",
        )
    }
    /// returns CharacterAngle of string
    /// Angle returned will be between 0 and 2PI
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn character_angle(&self, index: i32) -> Result<f64, Failure> {
        iges_array1_item(
            &self.state.read().the_char_angles,
            index,
            "IgesDimenNewGeneralNote::character_angle",
        )
    }
    /// returns ControlCodeString of string
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn control_code_string(&self, index: i32) -> Result<Handle<HAsciiString>, Failure> {
        iges_array1_item(
            &self.state.read().the_control_code_strings,
            index,
            "IgesDimenNewGeneralNote::control_code_string",
        )
    }
    /// returns number of characters in string or zero
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_characters(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_nb_chars,
            index,
            "IgesDimenNewGeneralNote::nb_characters",
        )
    }
    /// returns Box width of string
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn box_width(&self, index: i32) -> Result<f64, Failure> {
        iges_array1_item(
            &self.state.read().the_box_widths,
            index,
            "IgesDimenNewGeneralNote::box_width",
        )
    }
    /// returns Box height of string
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn box_height(&self, index: i32) -> Result<f64, Failure> {
        iges_array1_item(
            &self.state.read().the_box_heights,
            index,
            "IgesDimenNewGeneralNote::box_height",
        )
    }
    /// returns False if Value, True if Pointer (Entity)
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn is_char_set_entity(&self, index: i32) -> Result<bool, Failure> {
        Ok(!(iges_array1_item(
            &self.state.read().the_char_set_entities,
            index,
            "IgesDimenNewGeneralNote::is_char_set_entity",
        )?
        .is_null()))
    }
    /// returns Character Set Interpretation (default = 1) of string
    /// returns 0 if IsCharSetEntity () is True
    /// 1 = Standard ASCII
    /// 1001 = Symbol Font1
    /// 1002 = Symbol Font2
    /// 1003 = Symbol Font3
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn char_set_code(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_char_set_codes,
            index,
            "IgesDimenNewGeneralNote::char_set_code",
        )
    }
    /// returns Character Set Interpretation of string
    /// returns a Null Handle if IsCharSetEntity () is False
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn char_set_entity(&self, index: i32) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_char_set_entities,
            index,
            "IgesDimenNewGeneralNote::char_set_entity",
        )
    }
    /// returns Slant angle of string in radians
    /// default value = PI/2
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn slant_angle(&self, index: i32) -> Result<f64, Failure> {
        iges_array1_item(
            &self.state.read().the_slant_angles,
            index,
            "IgesDimenNewGeneralNote::slant_angle",
        )
    }
    /// returns Rotation angle of string in radians
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn rotation_angle(&self, index: i32) -> Result<f64, Failure> {
        iges_array1_item(
            &self.state.read().the_rotation_angles,
            index,
            "IgesDimenNewGeneralNote::rotation_angle",
        )
    }
    /// returns Mirror Flag of string
    /// 0 = no mirroring
    /// 1 = mirror axis is perpendicular to the text base line
    /// 2 = mirror axis is text base line
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn mirror_flag(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_mirror_flags,
            index,
            "IgesDimenNewGeneralNote::mirror_flag",
        )
    }
    /// returns False if MirrorFlag = 0. ie. no mirroring
    /// else returns True
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn is_mirrored(&self, index: i32) -> Result<bool, Failure> {
        Ok((iges_array1_item(
            &self.state.read().the_mirror_flags,
            index,
            "IgesDimenNewGeneralNote::is_mirrored",
        )?) != 0)
    }
    /// returns Rotate internal text Flag of string
    /// 0 = text horizontal
    /// 1 = text vertical
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn rotate_flag(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_rotate_flags,
            index,
            "IgesDimenNewGeneralNote::rotate_flag",
        )
    }
    /// returns text start point of string
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn start_point(&self, index: i32) -> Result<Point3, Failure> {
        Ok(Point3::from(iges_array1_item(
            &self.state.read().the_start_points,
            index,
            "IgesDimenNewGeneralNote::start_point",
        )?))
    }
    /// returns text start point of string after Transformation
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_start_point(&self, index: i32) -> Result<Point3, Failure> {
        let mut temp_xyz: Xyz = iges_array1_item(
            &self.state.read().the_start_points,
            index,
            "IgesDimenNewGeneralNote::transformed_start_point",
        )?;
        if self.has_transf() {
            self.location()?.transforms(&mut temp_xyz);
        }
        Ok(Point3::from(temp_xyz))
    }
    /// returns distance from the start point plane
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn z_depth_start_point(&self, index: i32) -> Result<f64, Failure> {
        Ok(iges_array1_item(
            &self.state.read().the_start_points,
            index,
            "IgesDimenNewGeneralNote::z_depth_start_point",
        )?
        .z())
    }
    /// returns text string
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn text(&self, index: i32) -> Result<Handle<HAsciiString>, Failure> {
        iges_array1_item(
            &self.state.read().the_texts,
            index,
            "IgesDimenNewGeneralNote::text",
        )
    }
}
