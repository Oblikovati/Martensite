//! IgesDimenNewGeneralNote.
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
use crate::iges_basic::entity_shape::{iges_array1_length, iges_array1_lower};
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::{Point3, Xyz};

iges_entity! {
/// defines NewGeneralNote, Type `213` Form `0`
/// Further attributes for formatting text strings
IgesDimenNewGeneralNote, IgesDimenNewGeneralNoteState, "IgesDimenNewGeneralNote", parent IgesDataIgesEntity, {
the_width: f64,
the_height: f64,
the_justify_code: i32,
the_area_loc: Xyz,
the_area_rotation_angle: f64,
the_base_line_pos: Xyz,
the_normal_interline_space: f64,
the_char_displays: Handle<HArray1<i32>>,
the_char_widths: Handle<HArray1<f64>>,
the_char_heights: Handle<HArray1<f64>>,
the_inter_char_spaces: Handle<HArray1<f64>>,
the_interline_spaces: Handle<HArray1<f64>>,
the_font_styles: Handle<HArray1<i32>>,
the_char_angles: Handle<HArray1<f64>>,
the_control_code_strings: Handle<HArray1<Handle<HAsciiString>>>,
the_nb_chars: Handle<HArray1<i32>>,
the_box_widths: Handle<HArray1<f64>>,
the_box_heights: Handle<HArray1<f64>>,
the_char_set_codes: Handle<HArray1<i32>>,
the_char_set_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>>,
the_slant_angles: Handle<HArray1<f64>>,
the_rotation_angles: Handle<HArray1<f64>>,
the_mirror_flags: Handle<HArray1<i32>>,
the_rotate_flags: Handle<HArray1<i32>>,
the_start_points: Handle<HArray1<Xyz>>,
the_texts: Handle<HArray1<Handle<HAsciiString>>>,
}
}

impl IgesDimenNewGeneralNote {
    iges_methods! {
    get text_width = the_width: f64;
    get text_height = the_height: f64;
    get justify_code = the_justify_code: i32;
    get area_rotation_angle = the_area_rotation_angle: f64;
    get normal_interline_space = the_normal_interline_space: f64;
    }
    /// This method is used to set the fields of the class
    /// NewGeneralNote
    /// - width                : Width of text containment area
    /// - height               : Height of text containment area
    /// - justifyCode          : Justification code
    /// - areaLoc              : Text containment area location
    /// - areaRotationAngle    : Text containment area rotation
    /// - baseLinePos          : Base line position
    /// - normalInterlineSpace : Normal interline spacing
    /// - charDisplays         : Character display type
    /// - charWidths           : Character width
    /// - charHeights          : Character height
    /// - interCharSpc         : Intercharacter spacing
    /// - interLineSpc         : Interline spacing
    /// - fontStyles           : Font style
    /// - charAngles           : Character angle
    /// - controlCodeStrings   : Control Code string
    /// - nbChars              : Number of characters in string
    /// - boxWidths            : Box width
    /// - boxHeights           : Box height
    /// - charSetCodes         : Character Set Interpretation
    /// - charSetEntities      : Character Set Font
    /// - slAngles             : Slant angle of text in radians
    /// - rotAngles            : Rotation angle of text in radians
    /// - mirrorFlags          : Type of mirroring
    /// - rotateFlags          : Rotate internal text flag
    /// - startPoints          : Text start point
    /// - texts                : Text strings
    ///   raises exception if there is mismatch between the various
    ///   Array Lengths.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        width: f64,
        height: f64,
        justify_code: i32,
        area_loc: Xyz,
        area_rotation_angle: f64,
        base_line_pos: Xyz,
        normal_interline_space: f64,
        char_displays: &Handle<HArray1<i32>>,
        char_widths: &Handle<HArray1<f64>>,
        char_heights: &Handle<HArray1<f64>>,
        inter_char_spc: &Handle<HArray1<f64>>,
        inter_line_spc: &Handle<HArray1<f64>>,
        font_styles: &Handle<HArray1<i32>>,
        char_angles: &Handle<HArray1<f64>>,
        control_code_strings: &Handle<HArray1<Handle<HAsciiString>>>,
        nb_chars: &Handle<HArray1<i32>>,
        box_widths: &Handle<HArray1<f64>>,
        box_heights: &Handle<HArray1<f64>>,
        char_set_codes: &Handle<HArray1<i32>>,
        char_set_entities: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
        sl_angles: &Handle<HArray1<f64>>,
        rot_angles: &Handle<HArray1<f64>>,
        mirror_flags: &Handle<HArray1<i32>>,
        rotate_flags: &Handle<HArray1<i32>>,
        start_points: &Handle<HArray1<Xyz>>,
        texts: &Handle<HArray1<Handle<HAsciiString>>>,
    ) -> Result<(), Failure> {
        let mut num: i32 = iges_array1_length(nb_chars, "IgesDimenNewGeneralNote::init")?;
        if ((((((((((((((((((iges_array1_lower(
            nb_chars,
            "IgesDimenNewGeneralNote::init",
        )? != 1)
            || ((iges_array1_lower(
                char_displays,
                "IgesDimenNewGeneralNote::init",
            )? != 1)
                || (iges_array1_length(
                    char_displays,
                    "IgesDimenNewGeneralNote::init",
                )? != num)))
            || ((iges_array1_lower(char_widths, "IgesDimenNewGeneralNote::init")?
                != 1)
                || (iges_array1_length(
                    char_widths,
                    "IgesDimenNewGeneralNote::init",
                )? != num)))
            || ((iges_array1_lower(char_heights, "IgesDimenNewGeneralNote::init")?
                != 1)
                || (iges_array1_length(
                    char_heights,
                    "IgesDimenNewGeneralNote::init",
                )? != num)))
            || ((iges_array1_lower(inter_char_spc, "IgesDimenNewGeneralNote::init")?
                != 1)
                || (iges_array1_length(
                    inter_char_spc,
                    "IgesDimenNewGeneralNote::init",
                )? != num)))
            || ((iges_array1_lower(inter_line_spc, "IgesDimenNewGeneralNote::init")?
                != 1)
                || (iges_array1_length(
                    inter_line_spc,
                    "IgesDimenNewGeneralNote::init",
                )? != num)))
            || ((iges_array1_lower(font_styles, "IgesDimenNewGeneralNote::init")? != 1)
                || (iges_array1_length(font_styles, "IgesDimenNewGeneralNote::init")?
                    != num)))
            || ((iges_array1_lower(char_angles, "IgesDimenNewGeneralNote::init")? != 1)
                || (iges_array1_length(char_angles, "IgesDimenNewGeneralNote::init")?
                    != num)))
            || ((iges_array1_lower(
                control_code_strings,
                "IgesDimenNewGeneralNote::init",
            )? != 1)
                || (iges_array1_length(
                    control_code_strings,
                    "IgesDimenNewGeneralNote::init",
                )? != num)))
            || ((iges_array1_lower(box_widths, "IgesDimenNewGeneralNote::init")? != 1)
                || (iges_array1_length(box_widths, "IgesDimenNewGeneralNote::init")?
                    != num)))
            || ((iges_array1_lower(box_heights, "IgesDimenNewGeneralNote::init")? != 1)
                || (iges_array1_length(box_heights, "IgesDimenNewGeneralNote::init")?
                    != num)))
            || ((iges_array1_lower(char_set_codes, "IgesDimenNewGeneralNote::init")? != 1)
                || (iges_array1_length(char_set_codes, "IgesDimenNewGeneralNote::init")?
                    != num)))
            || ((iges_array1_lower(char_set_entities, "IgesDimenNewGeneralNote::init")? != 1)
                || (iges_array1_length(char_set_entities, "IgesDimenNewGeneralNote::init")?
                    != num)))
            || ((iges_array1_lower(sl_angles, "IgesDimenNewGeneralNote::init")? != 1)
                || (iges_array1_length(sl_angles, "IgesDimenNewGeneralNote::init")? != num)))
            || ((iges_array1_lower(rot_angles, "IgesDimenNewGeneralNote::init")? != 1)
                || (iges_array1_length(rot_angles, "IgesDimenNewGeneralNote::init")? != num)))
            || ((iges_array1_lower(mirror_flags, "IgesDimenNewGeneralNote::init")? != 1)
                || (iges_array1_length(mirror_flags, "IgesDimenNewGeneralNote::init")? != num)))
            || ((iges_array1_lower(rotate_flags, "IgesDimenNewGeneralNote::init")? != 1)
                || (iges_array1_length(rotate_flags, "IgesDimenNewGeneralNote::init")? != num)))
            || ((iges_array1_lower(start_points, "IgesDimenNewGeneralNote::init")? != 1)
                || (iges_array1_length(start_points, "IgesDimenNewGeneralNote::init")? != num)))
            || ((iges_array1_lower(texts, "IgesDimenNewGeneralNote::init")? != 1)
                || (iges_array1_length(texts, "IgesDimenNewGeneralNote::init")? != num))
        {
            return Err(exceptions::dimension_mismatch(
                "IgesDimenGeneralNote : Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.the_width = width;
            state.the_height = height;
            state.the_justify_code = justify_code;
            state.the_area_loc = area_loc;
            state.the_area_rotation_angle = area_rotation_angle;
            state.the_base_line_pos = base_line_pos;
            state.the_normal_interline_space = normal_interline_space;
            state.the_char_displays = char_displays.clone();
            state.the_char_widths = char_widths.clone();
            state.the_char_heights = char_heights.clone();
            state.the_inter_char_spaces = inter_char_spc.clone();
            state.the_interline_spaces = inter_line_spc.clone();
            state.the_font_styles = font_styles.clone();
            state.the_char_angles = char_angles.clone();
            state.the_control_code_strings = control_code_strings.clone();
            state.the_nb_chars = nb_chars.clone();
            state.the_box_widths = box_widths.clone();
            state.the_box_heights = box_heights.clone();
            state.the_char_set_codes = char_set_codes.clone();
            state.the_char_set_entities = char_set_entities.clone();
            state.the_slant_angles = sl_angles.clone();
            state.the_rotation_angles = rot_angles.clone();
            state.the_mirror_flags = mirror_flags.clone();
            state.the_rotate_flags = rotate_flags.clone();
            state.the_start_points = start_points.clone();
            state.the_texts = texts.clone();
        }
        self.init_type_and_form(213, 0);
        Ok(())
    }
    /// returns Text containment area Location point
    pub fn area_location(&self) -> Point3 {
        let mut loc: Point3 = Point3::from(self.state.read().the_area_loc);
        loc
    }
    /// returns Text containment area Location point after Transformation
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_area_location(&self) -> Result<Point3, Failure> {
        let mut temp_xyz: Xyz = self.state.read().the_area_loc;
        if self.has_transf() {
            self.location()?.transforms(&mut temp_xyz);
        }
        Ok(Point3::from(temp_xyz))
    }
    /// returns distance from the containment area plane
    pub fn z_depth_area_location(&self) -> f64 {
        self.state.read().the_area_loc.z()
    }
    /// returns position of first base line
    pub fn base_line_position(&self) -> Point3 {
        let mut pos: Point3 = Point3::from(self.state.read().the_base_line_pos);
        pos
    }
    /// returns position of first base line after Transformation
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_base_line_position(&self) -> Result<Point3, Failure> {
        let mut temp_xyz: Xyz = self.state.read().the_base_line_pos;
        if self.has_transf() {
            self.location()?.transforms(&mut temp_xyz);
        }
        Ok(Point3::from(temp_xyz))
    }
    /// returns distance from the Base line position plane
    pub fn z_depth_base_line_position(&self) -> f64 {
        self.state.read().the_base_line_pos.z()
    }
    /// returns number of text HAsciiStrings
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_strings(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_char_displays,
            "IgesDimenNewGeneralNote::nb_strings",
        )
    }
    /// returns Fixed/Variable width character display of string
    /// 0 = Fixed
    /// 1 = Variable
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn character_display(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_char_displays,
            index,
            "IgesDimenNewGeneralNote::character_display",
        )
    }
    /// returns False if Character display width is Fixed
    /// optional method, if required
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn is_variable(&self, index: i32) -> Result<bool, Failure> {
        Ok((iges_array1_item(
            &self.state.read().the_char_displays,
            index,
            "IgesDimenNewGeneralNote::is_variable",
        )?) == 1)
    }
}
