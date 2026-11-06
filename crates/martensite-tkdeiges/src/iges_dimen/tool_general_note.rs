//! IgesDimenToolGeneralNote.
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
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_array1_set,
    iges_new_array1,
};
use crate::iges_dimen::IgesDimenGeneralNote;
use crate::iges_graph::IgesGraphTextFontDef;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::Xyz;

iges_tool!(
    IgesDimenToolGeneralNote,
    "The own-parameter work on `IgesDimenGeneralNote`."
);

impl IgesDimenToolGeneralNote {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDimenGeneralNote>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut nbval: i32 = Default::default();
        crate::iges_shared_blocks::iges_block_0001!(
            box_heights,
            box_widths,
            font_codes,
            font_entities,
            mirror_flags,
            nb_chars,
            rotate_flags,
            rotation_angles,
            slant_angles,
            start_points,
            texts
        );
        let mut st: bool =
            pr.read_integer_text(&pr.current(), "Number of Text Strings", &mut nbval)?;
        if st && (nbval > 0) {
            nb_chars = iges_new_array1::<i32>(1, nbval)?;
            box_widths = iges_new_array1::<f64>(1, nbval)?;
            box_heights = iges_new_array1::<f64>(1, nbval)?;
            font_codes = iges_new_array1::<i32>(1, nbval)?;
            font_entities = iges_new_array1::<Handle<IgesGraphTextFontDef>>(1, nbval)?;
            slant_angles = iges_new_array1::<f64>(1, nbval)?;
            rotation_angles = iges_new_array1::<f64>(1, nbval)?;
            mirror_flags = iges_new_array1::<i32>(1, nbval)?;
            rotate_flags = iges_new_array1::<i32>(1, nbval)?;
            start_points = iges_new_array1::<Xyz>(1, nbval)?;
            texts = iges_new_array1::<Handle<HAsciiString>>(1, nbval)?;
        } else {
            pr.add_fail("Number of Text Strings: Not Positive", "");
        }
        if !nb_chars.is_null() {
            for i in 1..=nbval {
                let mut nb_char: i32 = Default::default();
                let mut box_width: f64 = Default::default();
                let mut box_height: f64 = Default::default();
                let mut font_code: i32 = Default::default();
                let mut font_entity: Handle<IgesGraphTextFontDef> = Default::default();
                let mut slant_angle: f64 = Default::default();
                let mut rotation_angle: f64 = Default::default();
                let mut mirror_flag: i32 = Default::default();
                let mut rotate_flag: i32 = Default::default();
                let mut start_point: Xyz = Default::default();
                let mut text: Handle<HAsciiString> = Default::default();
                if pr.read_integer_text(&pr.current(), "Number of Characters", &mut nb_char)? {
                    iges_array1_set(
                        &nb_chars,
                        i,
                        nb_char,
                        "IgesDimenToolGeneralNote::read_own_params",
                    )?;
                }
                if pr.read_real_text(&pr.current(), "Box Width", &mut box_width)? {
                    iges_array1_set(
                        &box_widths,
                        i,
                        box_width,
                        "IgesDimenToolGeneralNote::read_own_params",
                    )?;
                }
                if pr.read_real_text(&pr.current(), "Box Height", &mut box_height)? {
                    iges_array1_set(
                        &box_heights,
                        i,
                        box_height,
                        "IgesDimenToolGeneralNote::read_own_params",
                    )?;
                }
                let mut curnum: i32 = pr.current_number();
                if pr.defined_else_skip()? {
                    pr.read_integer_text(&pr.current(), "Font Code", &mut font_code)?;
                    if font_code < 0 {
                        font_entity = pr.param_entity(ir, curnum)?.cast::<IgesGraphTextFontDef>();
                        if font_entity.is_null() {
                            pr.add_fail("Font Entity : incorrect reference", "");
                        }
                        iges_array1_set(
                            &font_entities,
                            i,
                            font_entity.clone(),
                            "IgesDimenToolGeneralNote::read_own_params",
                        )?;
                        iges_array1_set(
                            &font_codes,
                            i,
                            -1,
                            "IgesDimenToolGeneralNote::read_own_params",
                        )?;
                    } else {
                        iges_array1_set(
                            &font_codes,
                            i,
                            font_code,
                            "IgesDimenToolGeneralNote::read_own_params",
                        )?;
                    }
                } else {
                    iges_array1_set(
                        &font_codes,
                        i,
                        1,
                        "IgesDimenToolGeneralNote::read_own_params",
                    )?;
                }
                if pr.defined_else_skip()? {
                    if pr.read_real_text(&pr.current(), "Slant Angle", &mut slant_angle)? {
                        iges_array1_set(
                            &slant_angles,
                            i,
                            slant_angle,
                            "IgesDimenToolGeneralNote::read_own_params",
                        )?;
                    }
                } else {
                    iges_array1_set(
                        &slant_angles,
                        i,
                        (std::f64::consts::PI) / 2.0,
                        "IgesDimenToolGeneralNote::read_own_params",
                    )?;
                }
                if pr.read_real_text(&pr.current(), "Rotation Angle", &mut rotation_angle)? {
                    iges_array1_set(
                        &rotation_angles,
                        i,
                        rotation_angle,
                        "IgesDimenToolGeneralNote::read_own_params",
                    )?;
                }
                if pr.read_integer_text(&pr.current(), "Mirror Flag", &mut mirror_flag)? {
                    iges_array1_set(
                        &mirror_flags,
                        i,
                        mirror_flag,
                        "IgesDimenToolGeneralNote::read_own_params",
                    )?;
                }
                if pr.read_integer_text(&pr.current(), "Rotate Flag", &mut rotate_flag)? {
                    iges_array1_set(
                        &rotate_flags,
                        i,
                        rotate_flag,
                        "IgesDimenToolGeneralNote::read_own_params",
                    )?;
                }
                if pr.read_xyz(&pr.current_list(1, 3), "Start Point", &mut start_point)? {
                    iges_array1_set(
                        &start_points,
                        i,
                        start_point,
                        "IgesDimenToolGeneralNote::read_own_params",
                    )?;
                }
                if pr.read_text(&pr.current(), "Text String", &mut text)? {
                    iges_array1_set(
                        &texts,
                        i,
                        text.clone(),
                        "IgesDimenToolGeneralNote::read_own_params",
                    )?;
                }
            }
        }
        if !((((((((((nb_chars.is_null() || box_widths.is_null()) || box_heights.is_null())
            || font_codes.is_null())
            || font_entities.is_null())
            || slant_angles.is_null())
            || rotation_angles.is_null())
            || mirror_flags.is_null())
            || rotate_flags.is_null())
            || start_points.is_null())
            || texts.is_null())
        {
            self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
            ent.init(
                &nb_chars,
                &box_widths,
                &box_heights,
                &font_codes,
                &font_entities,
                &slant_angles,
                &rotation_angles,
                &mirror_flags,
                &rotate_flags,
                &start_points,
                &texts,
            )?;
        }
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDimenGeneralNote>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut nbval: i32 = ent.nb_strings()?;
        iw.send(nbval)?;
        for i in 1..=nbval {
            iw.send(ent.nb_characters(i)?)?;
            iw.send(ent.box_width(i)?)?;
            iw.send(ent.box_height(i)?)?;
            if ent.is_font_entity(i)? {
                iw.send_entity(&ent.font_entity(i)?, true)?;
            } else {
                iw.send(ent.font_code(i)?)?;
            }
            iw.send(ent.slant_angle(i)?)?;
            iw.send(ent.rotation_angle(i)?)?;
            iw.send(ent.mirror_flag(i)?)?;
            iw.send(ent.rotate_flag(i)?)?;
            iw.send(ent.start_point(i)?.x())?;
            iw.send(ent.start_point(i)?.y())?;
            iw.send(ent.start_point(i)?.z())?;
            iw.send(ent.text(i)?)?;
        }
        Ok(())
    }
}
