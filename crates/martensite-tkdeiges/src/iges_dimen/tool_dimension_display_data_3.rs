//! More of IgesDimenToolDimensionDisplayData.
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
    IgesDataDumpResult, IgesDataIgesDumper, iges_dump_string, iges_dump_vals, iges_real_text,
};
use crate::iges_dimen::{IgesDimenDimensionDisplayData, IgesDimenToolDimensionDisplayData};
use martensite_tkernel::standard::Handle;
use std::io::Write;

impl IgesDimenToolDimensionDisplayData {
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenDimensionDisplayData>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        write!(
            s,
            "IgesDimenDimensionDisplayData\nNo. of property values : {}\nDimensionType : {}",
            ent.nb_property_values(),
            ent.dimension_type()
        )?;
        match ent.dimension_type() {
            0 => {
                writeln!(s, " (Ordinary)")?;
            }
            1 => {
                writeln!(s, " (Reference)")?;
            }
            2 => {
                writeln!(s, " (Basic)")?;
            }
            _ => {
                writeln!(s, " (Incorrect Value)")?;
            }
        }
        write!(s, "Preferred Label Position : {}", ent.label_position())?;
        match ent.label_position() {
            0 => {
                writeln!(s, " (Does not exist)")?;
            }
            1 => {
                writeln!(s, " (Before Measurement)")?;
            }
            2 => {
                writeln!(s, " (After Measurement)")?;
            }
            3 => {
                writeln!(s, " (Above Measurement)")?;
            }
            4 => {
                writeln!(s, " (Below Measurement)")?;
            }
            _ => {
                writeln!(s, " (Incorrect Value)")?;
            }
        }
        write!(s, "Character set interpretation : {}", ent.character_set())?;
        match ent.character_set() {
            1 => {
                writeln!(s, " (Standard ASCII)")?;
            }
            1001 => {
                writeln!(s, " (Symbol Font 1)")?;
            }
            1002 => {
                writeln!(s, " (Symbol Font 2)")?;
            }
            1003 => {
                writeln!(s, " (Drafting Font)")?;
            }
            _ => {
                writeln!(s, " (Not meaningful)")?;
            }
        }
        write!(s, "LString : ")?;
        iges_dump_string(s, &ent.l_string())?;
        write!(s, "\nDecimal Symbol : ")?;
        if ent.decimal_symbol() == 0 {
            writeln!(s, "0 (.)")?;
        } else {
            writeln!(s, "1 (,)")?;
        }
        write!(
            s,
            "Witness Line Angle : {}\nText Alignment : ",
            iges_real_text(ent.witness_line_angle())
        )?;
        if ent.text_alignment() == 0 {
            writeln!(s, "0 (Horizontal)")?;
        } else {
            if ent.text_alignment() == 1 {
                writeln!(s, "1 (Parallel)")?;
            } else {
                writeln!(s, "{} (Incorrect Value)", ent.text_alignment())?;
            }
        }
        write!(s, "Text Level : {}", ent.text_level())?;
        match ent.text_level() {
            0 => {
                writeln!(s, " (Neither above nor below)")?;
            }
            1 => {
                writeln!(s, " (Above)")?;
            }
            2 => {
                writeln!(s, " (Below)")?;
            }
            _ => {
                writeln!(s, " (Incorrect Value)")?;
            }
        }
        write!(s, "Preferred Text placement : {}", ent.text_placement())?;
        match ent.text_placement() {
            0 => {
                writeln!(s, " (Between witness lines)")?;
            }
            1 => {
                writeln!(s, " (Outside near the first witness line)")?;
            }
            2 => {
                writeln!(s, " (Outside near second witness line)")?;
            }
            _ => {
                writeln!(s, " (Incorrect Value)")?;
            }
        }
        write!(
            s,
            "Arrow Head Orientation : {}",
            ent.arrow_head_orientation()
        )?;
        if ent.arrow_head_orientation() == 0 {
            writeln!(s, " (In, pointing out)")?;
        } else {
            if ent.arrow_head_orientation() == 1 {
                writeln!(s, " (Out, pointing in)")?;
            } else {
                writeln!(s, " (Incorrect Value)")?;
            }
        }
        let mut nbnotes: i32 = ent.nb_supplementary_notes()?;
        writeln!(
            s,
            " Primary Dimension Value : {}\n Number of Supplementary Notes : {}\nSupplementary Notes ,  Start Index ,  End   Index :",
            iges_real_text(ent.initial_value()),
            nbnotes
        )?;
        iges_dump_vals(s, -level, 1, nbnotes, |first| ent.end_index(first))?;
        writeln!(s)?;
        if level > 4 {
            for i in 1..=nbnotes {
                writeln!(
                    s,
                    "[{}]:\nSupplementary Note : {}, Start Index : {}, End Index : {}",
                    i,
                    ent.supplementary_note(i)?,
                    ent.start_index(i)?,
                    ent.end_index(i)?
                )?;
            }
        }
        writeln!(s)?;
        Ok(())
    }
}
