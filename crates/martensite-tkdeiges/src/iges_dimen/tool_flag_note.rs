//! IgesDimenToolFlagNote.
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
    iges_dump_entities, iges_dump_xyz_l, iges_new_array1, iges_real_text,
};
use crate::iges_dimen::{IgesDimenFlagNote, IgesDimenGeneralNote, IgesDimenLeaderArrow};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDimenToolFlagNote,
    "The own-parameter work on `IgesDimenFlagNote`."
);

impl IgesDimenToolFlagNote {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDimenFlagNote>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut lower_left: Xyz = Default::default();
        let mut angle: f64 = Default::default();
        crate::iges_shared_blocks::iges_block_0059!(leaders, nbval, note);
        pr.read_xyz(
            &pr.current_list(1, 3),
            "Lower Left Corner Co-ords",
            &mut lower_left,
        )?;
        pr.read_real_text(&pr.current(), "Rotation Angle", &mut angle)?;
        crate::iges_shared_blocks::iges_block_0070!(ir, nbval, note, pr, st);
        if st && (nbval > 0) {
            leaders = iges_new_array1::<Handle<IgesDimenLeaderArrow>>(1, nbval)?;
            for i in 1..=nbval {
                let mut anentity: Handle<IgesDimenLeaderArrow> = Default::default();
                if pr.read_entity_typed(
                    ir,
                    &pr.current(),
                    "Leaders",
                    <IgesDimenLeaderArrow>::static_type(),
                    &mut anentity,
                    false,
                )? {
                    iges_array1_set(
                        &leaders,
                        i,
                        anentity.clone(),
                        "IgesDimenToolFlagNote::read_own_params",
                    )?;
                }
            }
        } else {
            if nbval < 0 {
                pr.add_fail("Number of Leaders: Less than zero", "");
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(lower_left, angle, &note, &leaders)?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDimenFlagNote>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0062!(ent, iw);
        iw.send(ent.angle())?;
        iw.send_entity(&ent.note(), false)?;
        let mut upper: i32 = ent.nb_leaders()?;
        iw.send(upper)?;
        for i in 1..=upper {
            iw.send_entity(&ent.leader(i)?, false)?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a FlagNote `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDimenFlagNote>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.note().cast());
        let mut upper: i32 = ent.nb_leaders()?;
        for i in 1..=upper {
            iter.get_one_item(&ent.leader(i)?.cast());
        }
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDimenFlagNote>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(208, 0);
        crate::iges_shared_blocks::iges_block_0039!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesDimenFlagNote>,
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
        another: &Handle<IgesDimenFlagNote>,
        ent: &Handle<IgesDimenFlagNote>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut lower_left: Xyz = another.lower_left_corner().xyz();
        let mut angle: f64 = another.angle();
        let mut note: Handle<IgesDimenGeneralNote> = tc.transferred(&another.note().cast())?.cast();
        let mut nbval: i32 = another.nb_leaders()?;
        let mut leaders: Handle<HArray1<Handle<IgesDimenLeaderArrow>>> = Default::default();
        if nbval > 0 {
            leaders = iges_new_array1::<Handle<IgesDimenLeaderArrow>>(1, nbval)?;
            for i in 1..=nbval {
                let mut new_ent: Handle<IgesDimenLeaderArrow> =
                    tc.transferred(&another.leader(i)?.cast())?.cast();
                iges_array1_set(
                    &leaders,
                    i,
                    new_ent.clone(),
                    "IgesDimenToolFlagNote::own_copy",
                )?;
            }
        }
        ent.init(lower_left, angle, &note, &leaders)?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenFlagNote>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut sublevel: i32 = if level > 4 { 1 } else { 0 };
        write!(s, "IgesDimenFlagNote\nLowerLeftCorner : ")?;
        iges_dump_xyz_l(s, level, &ent.lower_left_corner().xyz(), &ent.location()?)?;
        write!(
            s,
            "\nRotation Angle: {}\nGeneral Note Entity : ",
            iges_real_text(ent.angle())
        )?;
        dumper.dump(&ent.note(), s, sublevel, -1)?;
        write!(
            s,
            "\nNumber of Leaders : {}   Leaders : ",
            ent.nb_leaders()?
        )?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_leaders()?, |first| {
            ent.leader(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
