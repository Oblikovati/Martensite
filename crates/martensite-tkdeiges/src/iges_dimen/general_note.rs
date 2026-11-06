//! IgesDimenGeneralNote.
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
use crate::iges_entity;
use crate::iges_graph::IgesGraphTextFontDef;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::{Point3, Xyz};

iges_entity! {
/// defines GeneralNote, Type `212` Form &lt;0-8, 100-200, 105&gt;
/// Used for formatting boxed text in different ways
IgesDimenGeneralNote, IgesDimenGeneralNoteState, "IgesDimenGeneralNote", parent IgesDataIgesEntity, {
the_nb_chars: Handle<HArray1<i32>>,
the_box_widths: Handle<HArray1<f64>>,
the_box_heights: Handle<HArray1<f64>>,
the_font_codes: Handle<HArray1<i32>>,
the_font_entities: Handle<HArray1<Handle<IgesGraphTextFontDef>>>,
the_slant_angles: Handle<HArray1<f64>>,
the_rotation_angles: Handle<HArray1<f64>>,
the_mirror_flags: Handle<HArray1<i32>>,
the_rotate_flags: Handle<HArray1<i32>>,
the_start_points: Handle<HArray1<Xyz>>,
the_texts: Handle<HArray1<Handle<HAsciiString>>>,
}
}

impl IgesDimenGeneralNote {
    /// This method is used to set the fields of the class
    /// GeneralNote
    /// - nNbChars      : number of chars strings
    /// - widths        : Box widths
    /// - heights       : Box heights
    /// - fontCodes     : Font codes, default = 1
    /// - fonts         : Text Font Definition Entities
    /// - slants        : Slant angles in radians
    /// - rotations     : Rotation angles in radians
    /// - mirrorFlags   : Mirror flags
    /// - rotFlags      : Rotation internal text flags
    /// - start         : Text start points
    /// - texts         : Text strings
    ///   raises exception if there is mismatch between the various
    ///   Array Lengths.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        nb_chars: &Handle<HArray1<i32>>,
        widths: &Handle<HArray1<f64>>,
        heights: &Handle<HArray1<f64>>,
        font_codes: &Handle<HArray1<i32>>,
        fonts: &Handle<HArray1<Handle<IgesGraphTextFontDef>>>,
        slants: &Handle<HArray1<f64>>,
        rotations: &Handle<HArray1<f64>>,
        mirror_flags: &Handle<HArray1<i32>>,
        rot_flags: &Handle<HArray1<i32>>,
        start: &Handle<HArray1<Xyz>>,
        texts: &Handle<HArray1<Handle<HAsciiString>>>,
    ) -> Result<(), Failure> {
        let mut num: i32 = iges_array1_length(nb_chars, "IgesDimenGeneralNote::init")?;
        if ((((((((((iges_array1_lower(nb_chars, "IgesDimenGeneralNote::init")? != 1)
            || ((iges_array1_lower(widths, "IgesDimenGeneralNote::init")? != 1)
                || (iges_array1_length(widths, "IgesDimenGeneralNote::init")? != num)))
            || ((iges_array1_lower(heights, "IgesDimenGeneralNote::init")? != 1)
                || (iges_array1_length(heights, "IgesDimenGeneralNote::init")? != num)))
            || ((iges_array1_lower(font_codes, "IgesDimenGeneralNote::init")? != 1)
                || (iges_array1_length(font_codes, "IgesDimenGeneralNote::init")? != num)))
            || ((iges_array1_lower(fonts, "IgesDimenGeneralNote::init")? != 1)
                || (iges_array1_length(fonts, "IgesDimenGeneralNote::init")? != num)))
            || ((iges_array1_lower(slants, "IgesDimenGeneralNote::init")? != 1)
                || (iges_array1_length(slants, "IgesDimenGeneralNote::init")? != num)))
            || ((iges_array1_lower(rotations, "IgesDimenGeneralNote::init")? != 1)
                || (iges_array1_length(rotations, "IgesDimenGeneralNote::init")? != num)))
            || ((iges_array1_lower(mirror_flags, "IgesDimenGeneralNote::init")? != 1)
                || (iges_array1_length(mirror_flags, "IgesDimenGeneralNote::init")? != num)))
            || ((iges_array1_lower(rot_flags, "IgesDimenGeneralNote::init")? != 1)
                || (iges_array1_length(rot_flags, "IgesDimenGeneralNote::init")? != num)))
            || ((iges_array1_lower(start, "IgesDimenGeneralNote::init")? != 1)
                || (iges_array1_length(start, "IgesDimenGeneralNote::init")? != num)))
            || ((iges_array1_lower(texts, "IgesDimenGeneralNote::init")? != 1)
                || (iges_array1_length(texts, "IgesDimenGeneralNote::init")? != num))
        {
            return Err(exceptions::dimension_mismatch(
                "IgesDimenGeneralNote : Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.the_nb_chars = nb_chars.clone();
            state.the_box_widths = widths.clone();
            state.the_box_heights = heights.clone();
            state.the_font_codes = font_codes.clone();
            state.the_font_entities = fonts.clone();
            state.the_slant_angles = slants.clone();
            state.the_rotation_angles = rotations.clone();
            state.the_mirror_flags = mirror_flags.clone();
            state.the_rotate_flags = rot_flags.clone();
            state.the_start_points = start.clone();
            state.the_texts = texts.clone();
        }
        self.init_type_and_form(212, self.form_number());
        Ok(())
    }
    /// Changes FormNumber (indicates Graphical Representation)
    /// Error if not in ranges \[0-8\] or \[100-102\] or 105
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn set_form_number(&self, form: i32) -> Result<(), Failure> {
        if (((form < 0) || (form > 8)) && ((form < 100) || (form > 102))) && (form != 105) {
            return Err(exceptions::out_of_range(
                "IgesDimenGeneralNote : SetFormNumber",
            ));
        }
        self.init_type_and_form(212, form);
        Ok(())
    }
    /// returns number of text strings in General Note
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_strings(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_nb_chars,
            "IgesDimenGeneralNote::nb_strings",
        )
    }
    /// returns number of characters of string or zero
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_characters(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_nb_chars,
            index,
            "IgesDimenGeneralNote::nb_characters",
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
            "IgesDimenGeneralNote::box_width",
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
            "IgesDimenGeneralNote::box_height",
        )
    }
    /// returns False if Value, True if Entity
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn is_font_entity(&self, index: i32) -> Result<bool, Failure> {
        Ok(!(iges_array1_item(
            &self.state.read().the_font_entities,
            index,
            "IgesDimenGeneralNote::is_font_entity",
        )?
        .is_null()))
    }
    /// returns Font code (default = 1) of string
    /// returns 0 if IsFontEntity () is True
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn font_code(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_font_codes,
            index,
            "IgesDimenGeneralNote::font_code",
        )
    }
    /// returns Text Font Definition Entity of string
    /// returns a Null Handle if IsFontEntity () returns False
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn font_entity(&self, index: i32) -> Result<Handle<IgesGraphTextFontDef>, Failure> {
        iges_array1_item(
            &self.state.read().the_font_entities,
            index,
            "IgesDimenGeneralNote::font_entity",
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
            "IgesDimenGeneralNote::slant_angle",
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
            "IgesDimenGeneralNote::rotation_angle",
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
            "IgesDimenGeneralNote::mirror_flag",
        )
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
            "IgesDimenGeneralNote::rotate_flag",
        )
    }
    /// returns text start point of Index'th string
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn start_point(&self, index: i32) -> Result<Point3, Failure> {
        let mut point: Point3 = Point3::from(iges_array1_item(
            &self.state.read().the_start_points,
            index,
            "IgesDimenGeneralNote::start_point",
        )?);
        Ok(point)
    }
    /// returns text start point of Index'th string after Transformation
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_start_point(&self, index: i32) -> Result<Point3, Failure> {
        let mut point: Xyz = iges_array1_item(
            &self.state.read().the_start_points,
            index,
            "IgesDimenGeneralNote::transformed_start_point",
        )?;
        if self.has_transf() {
            self.location()?.transforms(&mut point);
        }
        Ok(Point3::from(point))
    }
    /// returns distance from Start Point plane of string
    /// raises exception if Index at most 0 or Index &gt; NbStrings()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn z_depth_start_point(&self, index: i32) -> Result<f64, Failure> {
        Ok(iges_array1_item(
            &self.state.read().the_start_points,
            index,
            "IgesDimenGeneralNote::z_depth_start_point",
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
            "IgesDimenGeneralNote::text",
        )
    }
}
