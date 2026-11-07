//! IgesDimenToolNewGeneralNote.
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
    IgesDataIgesEntity, IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader,
    iges_array1_set, iges_new_array1,
};
use crate::iges_data::{iges_read_integer_item, iges_read_real_item, iges_read_text_item};
use crate::iges_dimen::IgesDimenNewGeneralNote;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::Xyz;

iges_tool!(
    IgesDimenToolNewGeneralNote,
    "The own-parameter work on `IgesDimenNewGeneralNote`."
);

impl IgesDimenToolNewGeneralNote {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDimenNewGeneralNote>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut nbval: i32 = Default::default();
        let mut width: f64 = Default::default();
        let mut height: f64 = Default::default();
        let mut justify_code: i32 = Default::default();
        let mut area_loc: Xyz = Default::default();
        let mut area_rotation_angle: f64 = Default::default();
        let mut base_line_pos: Xyz = Default::default();
        let mut normal_interline_space: f64 = Default::default();
        crate::iges_shared_blocks::iges_block_0074!(
            box_heights,
            box_widths,
            char_angles,
            char_displays,
            char_heights,
            char_set_codes,
            char_set_entities,
            char_widths,
            control_code_strings,
            font_styles,
            inter_char_spaces,
            interline_spaces,
            mirror_flags,
            nb_chars,
            rotate_flags,
            rotation_angles,
            slant_angles,
            start_points,
            texts
        );
        pr.read_real_text(&pr.current(), "Text Width", &mut width)?;
        pr.read_real_text(&pr.current(), "Text Height", &mut height)?;
        pr.read_integer_text(&pr.current(), "Justification Code", &mut justify_code)?;
        pr.read_xyz(&pr.current_list(1, 3), "Area Location Point", &mut area_loc)?;
        pr.read_real_text(
            &pr.current(),
            "Area Rotation Angle",
            &mut area_rotation_angle,
        )?;
        pr.read_xyz(
            &pr.current_list(1, 3),
            "Base Line Position",
            &mut base_line_pos,
        )?;
        pr.read_real_text(
            &pr.current(),
            "NormalInterline Spacing",
            &mut normal_interline_space,
        )?;
        let mut st: bool =
            pr.read_integer_text(&pr.current(), "Number of Text Strings", &mut nbval)?;
        if st && (nbval > 0) {
            char_displays = iges_new_array1::<i32>(1, nbval)?;
            char_widths = iges_new_array1::<f64>(1, nbval)?;
            char_heights = iges_new_array1::<f64>(1, nbval)?;
            inter_char_spaces = iges_new_array1::<f64>(1, nbval)?;
            interline_spaces = iges_new_array1::<f64>(1, nbval)?;
            font_styles = iges_new_array1::<i32>(1, nbval)?;
            char_angles = iges_new_array1::<f64>(1, nbval)?;
            control_code_strings = iges_new_array1::<Handle<HAsciiString>>(1, nbval)?;
            nb_chars = iges_new_array1::<i32>(1, nbval)?;
            box_widths = iges_new_array1::<f64>(1, nbval)?;
            box_heights = iges_new_array1::<f64>(1, nbval)?;
            char_set_codes = iges_new_array1::<i32>(1, nbval)?;
            char_set_entities = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, nbval)?;
            slant_angles = iges_new_array1::<f64>(1, nbval)?;
            rotation_angles = iges_new_array1::<f64>(1, nbval)?;
            mirror_flags = iges_new_array1::<i32>(1, nbval)?;
            rotate_flags = iges_new_array1::<i32>(1, nbval)?;
            start_points = iges_new_array1::<Xyz>(1, nbval)?;
            texts = iges_new_array1::<Handle<HAsciiString>>(1, nbval)?;
        } else {
            pr.add_fail("Number of Text Strings: Not Positive", "");
        }
        if !char_displays.is_null() {
            for i in 1..=nbval {
                let mut char_set_code: i32 = Default::default();
                let mut char_set_entity: Handle<IgesDataIgesEntity> = Default::default();
                let mut start_point: Xyz = Default::default();
                iges_read_integer_item(
                    pr,
                    "Character Display",
                    &char_displays,
                    i,
                    "IgesDimenToolNewGeneralNote::read_own_params",
                )?;
                iges_read_real_item(
                    pr,
                    "Character Width",
                    &char_widths,
                    i,
                    "IgesDimenToolNewGeneralNote::read_own_params",
                )?;
                iges_read_real_item(
                    pr,
                    "Character Height",
                    &char_heights,
                    i,
                    "IgesDimenToolNewGeneralNote::read_own_params",
                )?;
                iges_read_real_item(
                    pr,
                    "Inter-character space",
                    &inter_char_spaces,
                    i,
                    "IgesDimenToolNewGeneralNote::read_own_params",
                )?;
                iges_read_real_item(
                    pr,
                    "Interline space",
                    &interline_spaces,
                    i,
                    "IgesDimenToolNewGeneralNote::read_own_params",
                )?;
                iges_read_integer_item(
                    pr,
                    "Font Style",
                    &font_styles,
                    i,
                    "IgesDimenToolNewGeneralNote::read_own_params",
                )?;
                iges_read_real_item(
                    pr,
                    "Character Angle",
                    &char_angles,
                    i,
                    "IgesDimenToolNewGeneralNote::read_own_params",
                )?;
                iges_read_text_item(
                    pr,
                    "Control Code String",
                    &control_code_strings,
                    i,
                    "IgesDimenToolNewGeneralNote::read_own_params",
                )?;
                iges_read_integer_item(
                    pr,
                    "Number of Characters",
                    &nb_chars,
                    i,
                    "IgesDimenToolNewGeneralNote::read_own_params",
                )?;
                iges_read_real_item(
                    pr,
                    "Box Width",
                    &box_widths,
                    i,
                    "IgesDimenToolNewGeneralNote::read_own_params",
                )?;
                iges_read_real_item(
                    pr,
                    "Box Height",
                    &box_heights,
                    i,
                    "IgesDimenToolNewGeneralNote::read_own_params",
                )?;
                let mut curnum: i32 = pr.current_number();
                if pr.defined_else_skip()? {
                    pr.read_integer_text(
                        &pr.current(),
                        "Character Set Interpretation Code",
                        &mut char_set_code,
                    )?;
                    if char_set_code < 0 {
                        char_set_entity = pr.param_entity(ir, curnum)?;
                        if char_set_entity.is_null() {
                            pr.add_fail("Char Set Interpretation Entity : incorrect reference", "");
                        }
                        iges_array1_set(
                            &char_set_entities,
                            i,
                            char_set_entity.clone(),
                            "IgesDimenToolNewGeneralNote::read_own_params",
                        )?;
                        iges_array1_set(
                            &char_set_codes,
                            i,
                            -1,
                            "IgesDimenToolNewGeneralNote::read_own_params",
                        )?;
                    } else {
                        iges_array1_set(
                            &char_set_codes,
                            i,
                            char_set_code,
                            "IgesDimenToolNewGeneralNote::read_own_params",
                        )?;
                    }
                } else {
                    iges_array1_set(
                        &char_set_codes,
                        i,
                        1,
                        "IgesDimenToolNewGeneralNote::read_own_params",
                    )?;
                }
                if pr.defined_else_skip()? {
                    iges_read_real_item(
                        pr,
                        "Slant Angle",
                        &slant_angles,
                        i,
                        "IgesDimenToolNewGeneralNote::read_own_params",
                    )?;
                } else {
                    iges_array1_set(
                        &slant_angles,
                        i,
                        (std::f64::consts::PI) / 2.0,
                        "IgesDimenToolNewGeneralNote::read_own_params",
                    )?;
                }
                iges_read_real_item(
                    pr,
                    "Rotation Angle",
                    &rotation_angles,
                    i,
                    "IgesDimenToolNewGeneralNote::read_own_params",
                )?;
                iges_read_integer_item(
                    pr,
                    "Mirror Flag",
                    &mirror_flags,
                    i,
                    "IgesDimenToolNewGeneralNote::read_own_params",
                )?;
                iges_read_integer_item(
                    pr,
                    "Rotate Flag",
                    &rotate_flags,
                    i,
                    "IgesDimenToolNewGeneralNote::read_own_params",
                )?;
                if pr.read_xyz(&pr.current_list(1, 3), "Start Point", &mut start_point)? {
                    iges_array1_set(
                        &start_points,
                        i,
                        start_point,
                        "IgesDimenToolNewGeneralNote::read_own_params",
                    )?;
                }
                iges_read_text_item(
                    pr,
                    "Text String",
                    &texts,
                    i,
                    "IgesDimenToolNewGeneralNote::read_own_params",
                )?;
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        crate::iges_shared_blocks::iges_block_0072!(
            area_loc,
            area_rotation_angle,
            base_line_pos,
            box_heights,
            box_widths,
            char_angles,
            char_displays,
            char_heights,
            char_set_codes,
            char_set_entities,
            char_widths,
            control_code_strings,
            ent,
            font_styles,
            height,
            inter_char_spaces,
            interline_spaces,
            justify_code,
            mirror_flags,
            nb_chars,
            normal_interline_space,
            rotate_flags,
            rotation_angles,
            slant_angles,
            start_points,
            texts,
            width
        );
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDimenNewGeneralNote>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.text_width())?;
        iw.send(ent.text_height())?;
        iw.send(ent.justify_code())?;
        iw.send(ent.area_location().x())?;
        iw.send(ent.area_location().y())?;
        iw.send(ent.area_location().z())?;
        iw.send(ent.area_rotation_angle())?;
        iw.send(ent.base_line_position().x())?;
        iw.send(ent.base_line_position().y())?;
        iw.send(ent.base_line_position().z())?;
        iw.send(ent.normal_interline_space())?;
        let mut nbval: i32 = ent.nb_strings()?;
        iw.send(nbval)?;
        for i in 1..=nbval {
            iw.send(ent.character_display(i)?)?;
            iw.send(ent.character_width(i)?)?;
            iw.send(ent.character_height(i)?)?;
            iw.send(ent.inter_character_space(i)?)?;
            iw.send(ent.interline_space(i)?)?;
            iw.send(ent.font_style(i)?)?;
            iw.send(ent.character_angle(i)?)?;
            iw.send(ent.control_code_string(i)?)?;
            iw.send(ent.nb_characters(i)?)?;
            iw.send(ent.box_width(i)?)?;
            iw.send(ent.box_height(i)?)?;
            if ent.is_char_set_entity(i)? {
                iw.send_entity(&ent.char_set_entity(i)?, true)?;
            } else {
                iw.send(ent.char_set_code(i)?)?;
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
