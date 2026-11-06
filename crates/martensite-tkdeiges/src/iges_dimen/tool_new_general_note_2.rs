//! More of IgesDimenToolNewGeneralNote.
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
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesEntity, iges_array1_set, iges_live,
    iges_new_array1,
};
use crate::iges_dimen::{IgesDimenNewGeneralNote, IgesDimenToolNewGeneralNote};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};

impl IgesDimenToolNewGeneralNote {
    /// Lists the Entities shared by a NewGeneralNote `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDimenNewGeneralNote>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let mut nbval: i32 = ent.nb_strings()?;
        for i in 1..=nbval {
            if ent.is_char_set_entity(i)? {
                iter.get_one_item(&ent.char_set_entity(i)?.cast());
            }
        }
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDimenNewGeneralNote>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(213, 0);
        crate::iges_shared_blocks::iges_block_0025!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDimenNewGeneralNote>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let mut jcode: i32 = ent.justify_code();
        if (jcode < 0) || (jcode > 3) {
            ach.add_fail_text("Justify Code != 0, 1, 2, 3", "");
        }
        let mut upper: i32 = ent.nb_strings()?;
        for i in 1..=upper {
            if ent.nb_characters(i)?
                != (iges_live(&ent.text(i)?, "IgesDimenToolNewGeneralNote::own_check")?.length())
            {
                let mut mess = String::new();
                mess = format!("{} : Number of Characters != Length of Text String", i);
                ach.add_fail_text(&mess, "");
            }
            let mut charcode: i32 = ent.char_set_code(i)?;
            if ((charcode >= 0) && (charcode != 1)) && ((charcode < 1001) || (charcode > 1003)) {
                let mut mess = String::new();
                mess = format!("{} : Character Set Code != 1, 1001, 1002, 1003", i);
                ach.add_fail_text(&mess, "");
            }
            let mut chardisp: i32 = ent.character_display(i)?;
            if (chardisp < 0) || (chardisp > 1) {
                let mut mess = String::new();
                mess = format!("{} : Character Display != 0, 1", i);
                ach.add_fail_text(&mess, "");
            }
            let mut mflag: i32 = ent.mirror_flag(i)?;
            if (mflag < 0) || (mflag > 2) {
                let mut mess = String::new();
                mess = format!("{} : Mirror flag != 0, 1, 2", i);
                ach.add_fail_text(&mess, "");
            }
            let mut rflag: i32 = ent.rotate_flag(i)?;
            if (rflag < 0) || (rflag > 1) {
                let mut mess = String::new();
                mess = format!("{} : Rotate flag != 0, 1", i);
                ach.add_fail_text(&mess, "");
            }
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDimenNewGeneralNote>,
        ent: &Handle<IgesDimenNewGeneralNote>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut width: f64 = another.text_width();
        let mut height: f64 = another.text_height();
        let mut justify_code: i32 = another.justify_code();
        let mut area_loc: Xyz = another.area_location().xyz();
        let mut area_rotation_angle: f64 = another.area_rotation_angle();
        let mut base_line_pos: Xyz = another.base_line_position().xyz();
        let mut normal_interline_space: f64 = another.normal_interline_space();
        let mut nbval: i32 = another.nb_strings()?;
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
        char_displays = iges_new_array1::<i32>(1, nbval)?;
        char_widths = iges_new_array1::<f64>(1, nbval)?;
        char_heights = iges_new_array1::<f64>(1, nbval)?;
        inter_char_spaces = iges_new_array1::<f64>(1, nbval)?;
        interline_spaces = iges_new_array1::<f64>(1, nbval)?;
        font_styles = iges_new_array1::<i32>(1, nbval)?;
        char_angles = iges_new_array1::<f64>(1, nbval)?;
        control_code_strings = iges_new_array1::<Handle<HAsciiString>>(1, nbval)?;
        crate::iges_shared_blocks::iges_block_0065!(box_heights, box_widths, nb_chars, nbval);
        char_set_codes = iges_new_array1::<i32>(1, nbval)?;
        char_set_entities = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, nbval)?;
        crate::iges_shared_blocks::iges_block_0023!(
            mirror_flags,
            nbval,
            rotate_flags,
            rotation_angles,
            slant_angles,
            start_points,
            texts
        );
        for i in 1..=nbval {
            iges_array1_set(
                &char_displays,
                i,
                another.character_display(i)?,
                "IgesDimenToolNewGeneralNote::own_copy",
            )?;
            iges_array1_set(
                &char_widths,
                i,
                another.character_width(i)?,
                "IgesDimenToolNewGeneralNote::own_copy",
            )?;
            iges_array1_set(
                &char_heights,
                i,
                another.character_height(i)?,
                "IgesDimenToolNewGeneralNote::own_copy",
            )?;
            iges_array1_set(
                &inter_char_spaces,
                i,
                another.inter_character_space(i)?,
                "IgesDimenToolNewGeneralNote::own_copy",
            )?;
            iges_array1_set(
                &interline_spaces,
                i,
                another.interline_space(i)?,
                "IgesDimenToolNewGeneralNote::own_copy",
            )?;
            iges_array1_set(
                &font_styles,
                i,
                another.font_style(i)?,
                "IgesDimenToolNewGeneralNote::own_copy",
            )?;
            iges_array1_set(
                &char_angles,
                i,
                another.character_angle(i)?,
                "IgesDimenToolNewGeneralNote::own_copy",
            )?;
            iges_array1_set(
                &control_code_strings,
                i,
                Handle::new(HAsciiString::copied(iges_live(
                    &another.control_code_string(i)?,
                    "IgesDimenToolNewGeneralNote::own_copy",
                )?)),
                "IgesDimenToolNewGeneralNote::own_copy",
            )?;
            iges_array1_set(
                &nb_chars,
                i,
                another.nb_characters(i)?,
                "IgesDimenToolNewGeneralNote::own_copy",
            )?;
            iges_array1_set(
                &box_widths,
                i,
                another.box_width(i)?,
                "IgesDimenToolNewGeneralNote::own_copy",
            )?;
            iges_array1_set(
                &box_heights,
                i,
                another.box_height(i)?,
                "IgesDimenToolNewGeneralNote::own_copy",
            )?;
            if another.is_char_set_entity(i)? {
                let mut char_set_entity: Handle<IgesDataIgesEntity> =
                    tc.transferred(&another.char_set_entity(i)?.cast())?.cast();
                iges_array1_set(
                    &char_set_entities,
                    i,
                    char_set_entity.clone(),
                    "IgesDimenToolNewGeneralNote::own_copy",
                )?;
            } else {
                iges_array1_set(
                    &char_set_codes,
                    i,
                    another.char_set_code(i)?,
                    "IgesDimenToolNewGeneralNote::own_copy",
                )?;
            }
            iges_array1_set(
                &slant_angles,
                i,
                another.slant_angle(i)?,
                "IgesDimenToolNewGeneralNote::own_copy",
            )?;
            iges_array1_set(
                &rotation_angles,
                i,
                another.rotation_angle(i)?,
                "IgesDimenToolNewGeneralNote::own_copy",
            )?;
            iges_array1_set(
                &mirror_flags,
                i,
                another.mirror_flag(i)?,
                "IgesDimenToolNewGeneralNote::own_copy",
            )?;
            iges_array1_set(
                &rotate_flags,
                i,
                another.rotate_flag(i)?,
                "IgesDimenToolNewGeneralNote::own_copy",
            )?;
            iges_array1_set(
                &start_points,
                i,
                another.start_point(i)?.xyz(),
                "IgesDimenToolNewGeneralNote::own_copy",
            )?;
            iges_array1_set(
                &texts,
                i,
                Handle::new(HAsciiString::copied(iges_live(
                    &another.text(i)?,
                    "IgesDimenToolNewGeneralNote::own_copy",
                )?)),
                "IgesDimenToolNewGeneralNote::own_copy",
            )?;
        }
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
}
