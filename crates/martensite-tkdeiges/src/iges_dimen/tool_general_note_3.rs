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
    IgesDataDumpResult, IgesDataIgesDumper, iges_dump_string, iges_dump_vals, iges_dump_xyz_l,
    iges_real_text,
};
use crate::iges_dimen::{IgesDimenGeneralNote, IgesDimenToolGeneralNote};
use martensite_tkernel::standard::Handle;
use std::io::Write;

impl IgesDimenToolGeneralNote {
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenGeneralNote>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut sublevel: i32 = if level > 4 { 1 } else { 0 };
        let mut upper: i32 = ent.nb_strings()?;
        write!(
            s,
            "IgesDimenGeneralNote\nNumber of Text Strings : {}\nNumber of Characters :\nBox Widths :\nBox Heights :\nFont Codes :\nFont Entities :\nSlant Angles :\nRotation Angles :\nMirror Flags :\nRotate Flags :\nStart Points :\nTexts : ",
            upper
        )?;
        iges_dump_vals(s, -level, 1, ent.nb_strings()?, |first| {
            ent.nb_characters(first)
        })?;
        writeln!(s)?;
        if level > 4 {
            writeln!(s, "Details of each String")?;
            for i in 1..=upper {
                writeln!(
                    s,
                    "[{}]:\nNumber of Characters : {}  Box Width  : {}  Box Height : {}",
                    i,
                    ent.nb_characters(i)?,
                    iges_real_text(ent.box_width(i)?),
                    iges_real_text(ent.box_height(i)?)
                )?;
                if ent.is_font_entity(i)? {
                    write!(s, "Font Entity : ")?;
                    dumper.dump(&ent.font_entity(i)?, s, sublevel, -1)?;
                    writeln!(s)?;
                } else {
                    write!(
                        s,
                        "Font Code : {}\nSlant Angle : {}  Rotation Angle : {}  Mirror Flag : {}  Rotate Flag : {}\nStart Point : ",
                        ent.font_code(i)?,
                        iges_real_text(ent.slant_angle(i)?),
                        iges_real_text(ent.rotation_angle(i)?),
                        ent.mirror_flag(i)?,
                        ent.rotate_flag(i)?
                    )?;
                }
                iges_dump_xyz_l(s, level, &ent.start_point(i)?.xyz(), &ent.location()?)?;
                write!(s, "\nText : ")?;
                iges_dump_string(s, &ent.text(i)?)?;
                writeln!(s)?;
            }
        }
        writeln!(s)?;
        Ok(())
    }
}
