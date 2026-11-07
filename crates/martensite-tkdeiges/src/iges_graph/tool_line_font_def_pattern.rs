//! IgesGraphToolLineFontDefPattern.
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
use crate::iges_data::iges_read_real_item;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_array1_set,
    iges_dump_string, iges_dump_vals, iges_live, iges_new_array1,
};
use crate::iges_graph::IgesGraphLineFontDefPattern;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesGraphToolLineFontDefPattern,
    "The own-parameter work on `IgesGraphLineFontDefPattern`."
);

impl IgesGraphToolLineFontDefPattern {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGraphLineFontDefPattern>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_nb_seg: i32 = Default::default();
        let mut temp_display_pattern: Handle<HAsciiString> = Default::default();
        let mut temp_segment_lengths: Handle<HArray1<f64>> = Default::default();
        if pr.read_integer_text(
            &pr.current(),
            "Number of Visible-Blank Segments",
            &mut temp_nb_seg,
        )? {
            if temp_nb_seg <= 0 {
                pr.add_fail("Number of Visible-Blank Segments : Not Positive", "");
            } else {
                temp_segment_lengths = iges_new_array1::<f64>(1, temp_nb_seg)?;
            }
        }
        if !temp_segment_lengths.is_null() {
            let mut i: i32 = Default::default();
            for i in 1..=temp_nb_seg {
                iges_read_real_item(
                    pr,
                    "Length of Segment",
                    &temp_segment_lengths,
                    i,
                    "IgesGraphToolLineFontDefPattern::read_own_params",
                )?;
            }
        }
        pr.read_text(
            &pr.current(),
            "Visible-Blank Display Pattern",
            &mut temp_display_pattern,
        )?;
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(&temp_segment_lengths, &temp_display_pattern)?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGraphLineFontDefPattern>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut up: i32 = ent.nb_segments()?;
        iw.send(up)?;
        let mut i: i32 = Default::default();
        for i in 1..=up {
            iw.send(ent.length(i)?)?;
        }
        iw.send(ent.display_pattern())?;
        Ok(())
    }
    /// Lists the Entities shared by a LineFontDefPattern `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesGraphLineFontDefPattern>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(
        &self,
        _unused_0: &Handle<IgesGraphLineFontDefPattern>,
    ) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(304, 2);
        crate::iges_shared_blocks::iges_block_0012!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGraphLineFontDefPattern>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.rank_line_font() == 0 {
            ach.add_warning_text("Line Font Rank is zero", "");
        } else {
            if (ent.rank_line_font() < 1) || (ent.rank_line_font() > 5) {
                ach.add_warning_text("Invalid Value As Line Font Rank(Valid Range 1 to 5)", "");
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
        another: &Handle<IgesGraphLineFontDefPattern>,
        ent: &Handle<IgesGraphLineFontDefPattern>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut temp_segment_lengths: Handle<HArray1<f64>> =
            iges_new_array1::<f64>(1, another.nb_segments()?)?;
        let mut i: i32 = Default::default();
        let mut up: i32 = another.nb_segments()?;
        for i in 1..=up {
            iges_array1_set(
                &temp_segment_lengths,
                i,
                another.length(i)?,
                "IgesGraphToolLineFontDefPattern::own_copy",
            )?;
        }
        let mut temp_display_pattern: Handle<HAsciiString> =
            Handle::new(HAsciiString::copied(iges_live(
                &another.display_pattern(),
                "IgesGraphToolLineFontDefPattern::own_copy",
            )?));
        ent.init(&temp_segment_lengths, &temp_display_pattern)?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGraphLineFontDefPattern>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        write!(s, "IgesGraphLineFontDefPattern\nVisible-Blank Segments : ")?;
        let mut nb: i32 = ent.nb_segments()?;
        iges_dump_vals(s, level, 1, nb, |first| ent.length(first))?;
        write!(s, "\nDisplay Pattern : ")?;
        iges_dump_string(s, &ent.display_pattern())?;
        writeln!(s)?;
        if level > 4 {
            writeln!(s, " -> Which Segments are Visible (the others are Blank) :")?;
            for i in 1..=nb {
                if ent.is_visible(i)? {
                    write!(s, "  {}", i)?;
                }
            }
            writeln!(s)?;
        }
        Ok(())
    }
}
