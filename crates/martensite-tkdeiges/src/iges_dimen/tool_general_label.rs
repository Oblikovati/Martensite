//! IgesDimenToolGeneralLabel.
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
    iges_dump_entities, iges_new_array1,
};
use crate::iges_dimen::{IgesDimenGeneralLabel, IgesDimenGeneralNote, IgesDimenLeaderArrow};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDimenToolGeneralLabel,
    "The own-parameter work on `IgesDimenGeneralLabel`."
);

impl IgesDimenToolGeneralLabel {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDimenGeneralLabel>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0059!(leaders, nbval, note);
        crate::iges_shared_blocks::iges_block_0070!(ir, nbval, note, pr, st);
        if st && (nbval > 0) {
            leaders = iges_new_array1::<Handle<IgesDimenLeaderArrow>>(1, nbval)?;
        } else {
            pr.add_fail("Number of Leaders: Not Positive", "");
        }
        if !leaders.is_null() {
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
                        "IgesDimenToolGeneralLabel::read_own_params",
                    )?;
                }
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(&note, &leaders)?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDimenGeneralLabel>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut upper: i32 = ent.nb_leaders()?;
        iw.send_entity(&ent.note(), false)?;
        iw.send(upper)?;
        for i in 1..=upper {
            iw.send_entity(&ent.leader(i)?, false)?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a GeneralLabel `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDimenGeneralLabel>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let mut upper: i32 = ent.nb_leaders()?;
        iter.get_one_item(&ent.note().cast());
        for i in 1..=upper {
            iter.get_one_item(&ent.leader(i)?.cast());
        }
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDimenGeneralLabel>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(210, 0);
        crate::iges_shared_blocks::iges_block_0039!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesDimenGeneralLabel>,
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
        another: &Handle<IgesDimenGeneralLabel>,
        ent: &Handle<IgesDimenGeneralLabel>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut note: Handle<IgesDimenGeneralNote> = tc.transferred(&another.note().cast())?.cast();
        let mut nbval: i32 = another.nb_leaders()?;
        let mut leaders: Handle<HArray1<Handle<IgesDimenLeaderArrow>>> =
            iges_new_array1::<Handle<IgesDimenLeaderArrow>>(1, nbval)?;
        for i in 1..=nbval {
            let mut new_ent: Handle<IgesDimenLeaderArrow> =
                tc.transferred(&another.leader(i)?.cast())?.cast();
            iges_array1_set(
                &leaders,
                i,
                new_ent.clone(),
                "IgesDimenToolGeneralLabel::own_copy",
            )?;
        }
        ent.init(&note, &leaders)?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenGeneralLabel>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut sublevel: i32 = if level > 4 { 1 } else { 0 };
        write!(s, "IgesDimenGeneralLabel\nGeneral Note Entity : ")?;
        dumper.dump(&ent.note(), s, sublevel, -1)?;
        write!(s, "\nNumber of Leaders : {}\nLeaders : ", ent.nb_leaders()?)?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_leaders()?, |first| {
            ent.leader(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
