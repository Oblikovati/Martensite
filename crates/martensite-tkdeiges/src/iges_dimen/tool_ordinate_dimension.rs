//! IgesDimenToolOrdinateDimension.
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
    IgesDataIgesEntity, IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_live,
};
use crate::iges_dimen::{
    IgesDimenGeneralNote, IgesDimenLeaderArrow, IgesDimenOrdinateDimension, IgesDimenWitnessLine,
};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDimenToolOrdinateDimension,
    "The own-parameter work on `IgesDimenOrdinateDimension`."
);

impl IgesDimenToolOrdinateDimension {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        the_ent: &Handle<IgesDimenOrdinateDimension>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_note: Handle<IgesDimenGeneralNote> = Default::default();
        let mut wit_line: Handle<IgesDimenWitnessLine> = Default::default();
        let mut lead_arr: Handle<IgesDimenLeaderArrow> = Default::default();
        let mut is_line: bool = false;
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "General Note",
            <IgesDimenGeneralNote>::static_type(),
            &mut temp_note,
            false,
        )?;
        if (iges_live(the_ent, "IgesDimenToolOrdinateDimension::read_own_params")?.form_number())
            == 0
        {
            let mut ent: Handle<IgesDataIgesEntity> = Default::default();
            if !(pr.read_entity(ir, &pr.current(), "Line or Leader", &mut ent, false)?) {
            } else {
                if ent.is_kind(<IgesDimenWitnessLine>::static_type()) {
                    wit_line = ent.cast::<IgesDimenWitnessLine>();
                    is_line = true;
                } else {
                    if ent.is_kind(<IgesDimenLeaderArrow>::static_type()) {
                        lead_arr = ent.cast::<IgesDimenLeaderArrow>();
                        is_line = false;
                    } else {
                        pr.add_fail("Line or Leader : Type is incorrect", "");
                    }
                }
            }
        } else {
            pr.read_entity_typed(
                ir,
                &pr.current(),
                "Line",
                <IgesDimenWitnessLine>::static_type(),
                &mut wit_line,
                false,
            )?;
            pr.read_entity_typed(
                ir,
                &pr.current(),
                "Leader",
                <IgesDimenLeaderArrow>::static_type(),
                &mut lead_arr,
                false,
            )?;
        }
        self.dir_checker(the_ent)
            .check_type_and_form(pr.c_check(), the_ent);
        iges_live(the_ent, "IgesDimenToolOrdinateDimension::read_own_params")?
            .init(&temp_note, is_line, &wit_line, &lead_arr);
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDimenOrdinateDimension>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send_entity(&ent.note(), false)?;
        if ent.form_number() == 0 {
            if ent.is_line() {
                iw.send_entity(&ent.witness_line(), false)?;
            } else {
                iw.send_entity(&ent.leader(), false)?;
            }
        } else {
            iw.send_entity(&ent.witness_line(), false)?;
            iw.send_entity(&ent.leader(), false)?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a OrdinateDimension `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDimenOrdinateDimension>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.note().cast());
        iter.get_one_item(&ent.witness_line().cast());
        iter.get_one_item(&ent.leader().cast());
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(
        &self,
        _unused_0: &Handle<IgesDimenOrdinateDimension>,
    ) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_forms(218, 0, 1);
        crate::iges_shared_blocks::iges_block_0039!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDimenOrdinateDimension>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let mut nowitnes: bool = ent.witness_line().is_null();
        let mut noleader: bool = ent.leader().is_null();
        if nowitnes && noleader {
            ach.add_fail_text("Neither WitnessLine nor LeaderArrow is defined", "");
        } else {
            if ent.form_number() == 0 {
                if (!nowitnes) && (!noleader) {
                    ach.add_fail_text("Form 0 cannot afford both WitnessLine and LeaderArrow", "");
                }
            } else {
                if nowitnes || noleader {
                    ach.add_fail_text("Form 1 requires both WtnessLine and LeaderArrow", "");
                }
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
        another: &Handle<IgesDimenOrdinateDimension>,
        ent: &Handle<IgesDimenOrdinateDimension>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut temp_note: Handle<IgesDimenGeneralNote> =
            tc.transferred(&another.note().cast())?.cast();
        let mut wit_line: Handle<IgesDimenWitnessLine> =
            tc.transferred(&another.witness_line().cast())?.cast();
        let mut lead_arr: Handle<IgesDimenLeaderArrow> =
            tc.transferred(&another.leader().cast())?.cast();
        ent.init(&temp_note, another.is_line(), &wit_line, &lead_arr);
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenOrdinateDimension>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesDimenOrdinateDimension")?;
        crate::iges_shared_blocks::iges_block_0063!(dumper, ent, level, s, sublevel);
        writeln!(s)?;
        let mut wit_line: Handle<IgesDimenWitnessLine> = ent.witness_line();
        let mut lead_arr: Handle<IgesDimenLeaderArrow> = ent.leader();
        if !wit_line.is_null() {
            write!(s, "Witness line : ")?;
            dumper.dump(&wit_line, s, sublevel, -1)?;
            writeln!(s)?;
        }
        if !lead_arr.is_null() {
            write!(s, "Leader arrow :")?;
            dumper.dump(&lead_arr, s, sublevel, -1)?;
            writeln!(s)?;
        }
        Ok(())
    }
}
