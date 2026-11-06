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
    IgesDataDumpResult, IgesDataIgesDumper, iges_dump_string, iges_dump_vals, iges_dump_xyz_l,
    iges_real_text,
};
use crate::iges_dimen::{IgesDimenNewGeneralNote, IgesDimenToolNewGeneralNote};
use martensite_tkernel::standard::Handle;
use std::io::Write;

impl IgesDimenToolNewGeneralNote {
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenNewGeneralNote>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut sublevel: i32 = if level > 4 { 1 } else { 0 };
        let mut nbval: i32 = ent.nb_strings()?;
        write!(
            s,
            "IgesDimenNewGeneralNote\nText Area  : Width : {}  Height : {}  Justification Code  : {}\nText Area Location Point : ",
            iges_real_text(ent.text_width()),
            iges_real_text(ent.text_height()),
            ent.justify_code()
        )?;
        iges_dump_xyz_l(s, level, &ent.area_location().xyz(), &ent.location()?)?;
        write!(
            s,
            "Rotation Angle of Text : {}\nBase Line Position : ",
            iges_real_text(ent.area_rotation_angle())
        )?;
        iges_dump_xyz_l(s, level, &ent.base_line_position().xyz(), &ent.location()?)?;
        writeln!(
            s,
            "Normal Interline Spacing : {}\nNumber of Text Strings : {}",
            iges_real_text(ent.normal_interline_space()),
            nbval
        )?;
        write!(
            s,
            "Character Display :\nCharacter Width :\nCharacter Height :\nInter Character Spacing :\nInterline Spacing :\nFont Styles :\nCharacter Angle :\nControl Code String :\nNumber of Characters :\nBox Widths :\nBox Heights :\nCharacter Set Codes :\nCharacter Set Entities :\nSlant Angles :\nRotation Angles :\nMirror Flags :\nRotate Flags :\nStart Points :\nTexts : "
        )?;
        iges_dump_vals(s, -level, 1, nbval, |first| ent.nb_characters(first))?;
        writeln!(s)?;
        if level > 4 {
            writeln!(s, "Details of each String")?;
            for i in 1..=nbval {
                write!(
                    s,
                    "[{}]:\nCharacter Display : {}  Character Width   : {}  Character Height  : {}\nInter Character Spacing : {}  Interline Spacing : {}\nFont Styles       : {}\nCharacter Angle   : {}\nControl Code String : ",
                    i,
                    ent.character_display(i)?,
                    iges_real_text(ent.character_width(i)?),
                    iges_real_text(ent.character_height(i)?),
                    iges_real_text(ent.inter_character_space(i)?),
                    iges_real_text(ent.interline_space(i)?),
                    ent.font_style(i)?,
                    iges_real_text(ent.character_angle(i)?)
                )?;
                iges_dump_string(s, &ent.control_code_string(i)?)?;
                writeln!(
                    s,
                    "\nNumber of Characters : {}  Box Width  : {}  Box Height : {}",
                    ent.nb_characters(i)?,
                    iges_real_text(ent.box_width(i)?),
                    iges_real_text(ent.box_height(i)?)
                )?;
                if ent.is_char_set_entity(i)? {
                    write!(s, "Character Set Entity : ")?;
                    dumper.dump(&ent.char_set_entity(i)?, s, sublevel, -1)?;
                    writeln!(s)?;
                } else {
                    write!(
                        s,
                        "Character Set Code : {}\nSlant Angle : {}  Rotation Angle : {}  Mirror Flag : {}  Rotate Flag : {}\nStart Point : ",
                        ent.char_set_code(i)?,
                        iges_real_text(ent.slant_angle(i)?),
                        iges_real_text(ent.rotation_angle(i)?),
                        ent.mirror_flag(i)?,
                        ent.rotate_flag(i)?
                    )?;
                }
                iges_dump_xyz_l(s, level, &ent.start_point(i)?.xyz(), &ent.location()?)?;
                write!(s, "Text : ")?;
                iges_dump_string(s, &ent.text(i)?)?;
                writeln!(s)?;
            }
        } else {
            writeln!(s)?;
        }
        Ok(())
    }
}
