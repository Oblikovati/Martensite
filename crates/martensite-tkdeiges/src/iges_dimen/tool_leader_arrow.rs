//! IgesDimenToolLeaderArrow.
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
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_array1_set,
    iges_dump_list_xy_lz, iges_dump_xy_lz, iges_new_array1, iges_real_text,
};
use crate::iges_dimen::IgesDimenLeaderArrow;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::Xy;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDimenToolLeaderArrow,
    "The own-parameter work on `IgesDimenLeaderArrow`."
);

impl IgesDimenToolLeaderArrow {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDimenLeaderArrow>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut arrow_head_height: f64 = Default::default();
        let mut arrow_head_width: f64 = Default::default();
        let mut z_depth: f64 = Default::default();
        let mut arrow_head: Xy = Default::default();
        let mut segment_tails: Handle<HArray1<Xy>> = Default::default();
        let mut nbval: i32 = Default::default();
        let mut st: bool = pr.read_integer_text(&pr.current(), "Count of Segments", &mut nbval)?;
        if st && (nbval > 0) {
            segment_tails = iges_new_array1::<Xy>(1, nbval)?;
        } else {
            pr.add_fail("Count of Segments: Not Positive", "");
        }
        pr.read_real_text(&pr.current(), "Arrow Head Height", &mut arrow_head_height)?;
        pr.read_real_text(&pr.current(), "Arrow Head Width", &mut arrow_head_width)?;
        pr.read_real_text(&pr.current(), "Z Depth", &mut z_depth)?;
        pr.read_xy(
            &pr.current_list(1, 2),
            "Arrow Head Position",
            &mut arrow_head,
        )?;
        if !segment_tails.is_null() {
            for i in 1..=nbval {
                let mut temp_xy: Xy = Default::default();
                if pr.read_xy(&pr.current_list(1, 2), "Segment Co-ords.", &mut temp_xy)? {
                    iges_array1_set(
                        &segment_tails,
                        i,
                        temp_xy,
                        "IgesDimenToolLeaderArrow::read_own_params",
                    )?;
                }
            }
            self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
            ent.init(
                arrow_head_height,
                arrow_head_width,
                z_depth,
                arrow_head,
                &segment_tails,
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
        ent: &Handle<IgesDimenLeaderArrow>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut upper: i32 = ent.nb_segments()?;
        iw.send(upper)?;
        iw.send(ent.arrow_head_height())?;
        iw.send(ent.arrow_head_width())?;
        iw.send(ent.z_depth())?;
        iw.send(ent.arrow_head().x())?;
        iw.send(ent.arrow_head().y())?;
        for i in 1..=upper {
            iw.send(ent.segment_tail(i)?.x())?;
            iw.send(ent.segment_tail(i)?.y())?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a LeaderArrow `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesDimenLeaderArrow>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDimenLeaderArrow>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_forms(214, 1, 12);
        crate::iges_shared_blocks::iges_block_0039!(dc);
        dc.hierarchy_status_ignored();
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesDimenLeaderArrow>,
        _unused_1: &InterfaceShareTool,
        _unused_2: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDimenLeaderArrow>,
        ent: &Handle<IgesDimenLeaderArrow>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut nbval: i32 = another.nb_segments()?;
        let mut arrow_head_height: f64 = another.arrow_head_height();
        let mut arrow_head_width: f64 = another.arrow_head_width();
        let mut z_depth: f64 = another.z_depth();
        let mut arrow_head: Xy = another.arrow_head().xy();
        let mut segment_tails: Handle<HArray1<Xy>> = iges_new_array1::<Xy>(1, nbval)?;
        for i in 1..=nbval {
            iges_array1_set(
                &segment_tails,
                i,
                another.segment_tail(i)?.xy(),
                "IgesDimenToolLeaderArrow::own_copy",
            )?;
        }
        ent.init(
            arrow_head_height,
            arrow_head_width,
            z_depth,
            arrow_head,
            &segment_tails,
        )?;
        ent.set_form_number(another.form_number())?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenLeaderArrow>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        write!(
            s,
            "IgesDimenLeaderArrow\nNumber of Segments : {}\nArrowhead Height   : {}\nArrowhead Width    : {}\nZ depth            : {}\nArrowhead co-ords  : ",
            ent.nb_segments()?,
            iges_real_text(ent.arrow_head_height()),
            iges_real_text(ent.arrow_head_width()),
            iges_real_text(ent.z_depth())
        )?;
        iges_dump_xy_lz(
            s,
            level,
            &ent.arrow_head().xy(),
            &ent.location()?,
            ent.z_depth(),
        )?;
        write!(s, "\nSegment Tails : ")?;
        iges_dump_list_xy_lz(
            s,
            level,
            1,
            ent.nb_segments()?,
            |first| Ok(ent.segment_tail(first)?.xy()),
            &ent.location()?,
            ent.z_depth(),
        )?;
        writeln!(s)?;
        Ok(())
    }
}
