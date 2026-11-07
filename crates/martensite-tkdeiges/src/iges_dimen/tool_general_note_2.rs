//! More of IgesDimenToolGeneralNote.
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
    IgesDataDefType, IgesDataDirChecker, iges_array1_set, iges_live, iges_new_array1,
};
use crate::iges_dimen::{IgesDimenGeneralNote, IgesDimenToolGeneralNote};
use crate::iges_graph::IgesGraphTextFontDef;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};

impl IgesDimenToolGeneralNote {
    /// Lists the Entities shared by a GeneralNote `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDimenGeneralNote>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let mut nbval: i32 = ent.nb_strings()?;
        for i in 1..=nbval {
            if ent.is_font_entity(i)? {
                iter.get_one_item(&ent.font_entity(i)?.cast());
            }
        }
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDimenGeneralNote>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_forms(212, 0, 105);
        crate::iges_shared_blocks::iges_block_0025!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDimenGeneralNote>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if (((ent.form_number() < 0) || (ent.form_number() > 8))
            && ((ent.form_number() < 100) || (ent.form_number() > 102)))
            && (ent.form_number() != 105)
        {
            ach.add_fail_text("Form Number: Not Valid", "");
        }
        let mut upper: i32 = ent.nb_strings()?;
        for i in 1..=upper {
            if ent.nb_characters(i)?
                != (iges_live(&ent.text(i)?, "IgesDimenToolGeneralNote::own_check")?.length())
            {
                let mut mess = String::new();
                mess = format!("{} : Number of Characters != Length of Text String", i);
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
        another: &Handle<IgesDimenGeneralNote>,
        ent: &Handle<IgesDimenGeneralNote>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut nbval: i32 = another.nb_strings()?;
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
        crate::iges_shared_blocks::iges_block_0065!(box_heights, box_widths, nb_chars, nbval);
        font_codes = iges_new_array1::<i32>(1, nbval)?;
        font_entities = iges_new_array1::<Handle<IgesGraphTextFontDef>>(1, nbval)?;
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
            let mut nb_char: i32 = another.nb_characters(i)?;
            iges_array1_set(&nb_chars, i, nb_char, "IgesDimenToolGeneralNote::own_copy")?;
            let mut box_width: f64 = another.box_width(i)?;
            iges_array1_set(
                &box_widths,
                i,
                box_width,
                "IgesDimenToolGeneralNote::own_copy",
            )?;
            let mut box_height: f64 = another.box_height(i)?;
            iges_array1_set(
                &box_heights,
                i,
                box_height,
                "IgesDimenToolGeneralNote::own_copy",
            )?;
            if another.is_font_entity(i)? {
                let mut font_entity: Handle<IgesGraphTextFontDef> =
                    tc.transferred(&another.font_entity(i)?.cast())?.cast();
                iges_array1_set(
                    &font_entities,
                    i,
                    font_entity.clone(),
                    "IgesDimenToolGeneralNote::own_copy",
                )?;
                iges_array1_set(&font_codes, i, -1, "IgesDimenToolGeneralNote::own_copy")?;
            } else {
                let mut font_code: i32 = another.font_code(i)?;
                iges_array1_set(
                    &font_codes,
                    i,
                    font_code,
                    "IgesDimenToolGeneralNote::own_copy",
                )?;
            }
            let mut slant_angle: f64 = another.slant_angle(i)?;
            iges_array1_set(
                &slant_angles,
                i,
                slant_angle,
                "IgesDimenToolGeneralNote::own_copy",
            )?;
            let mut rotation_angle: f64 = another.rotation_angle(i)?;
            iges_array1_set(
                &rotation_angles,
                i,
                rotation_angle,
                "IgesDimenToolGeneralNote::own_copy",
            )?;
            let mut mirror_flag: i32 = another.mirror_flag(i)?;
            iges_array1_set(
                &mirror_flags,
                i,
                mirror_flag,
                "IgesDimenToolGeneralNote::own_copy",
            )?;
            let mut rotate_flag: i32 = another.rotate_flag(i)?;
            iges_array1_set(
                &rotate_flags,
                i,
                rotate_flag,
                "IgesDimenToolGeneralNote::own_copy",
            )?;
            let mut start_point: Xyz = another.start_point(i)?.xyz();
            iges_array1_set(
                &start_points,
                i,
                start_point,
                "IgesDimenToolGeneralNote::own_copy",
            )?;
            let mut text: Handle<HAsciiString> = Handle::new(HAsciiString::copied(iges_live(
                &another.text(i)?,
                "IgesDimenToolGeneralNote::own_copy",
            )?));
            iges_array1_set(
                &texts,
                i,
                text.clone(),
                "IgesDimenToolGeneralNote::own_copy",
            )?;
        }
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
        ent.set_form_number(another.form_number())?;
        Ok(())
    }
}
