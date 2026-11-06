//! IgesDimenToolCurveDimension.
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
    IgesDataIgesEntity, IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader,
};
use crate::iges_dimen::{
    IgesDimenCurveDimension, IgesDimenGeneralNote, IgesDimenLeaderArrow, IgesDimenWitnessLine,
};
use crate::iges_geom::IgesGeomLine;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDimenToolCurveDimension,
    "The own-parameter work on `IgesDimenCurveDimension`."
);

impl IgesDimenToolCurveDimension {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDimenCurveDimension>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut note: Handle<IgesDimenGeneralNote> = Default::default();
        let mut first_curve: Handle<IgesDataIgesEntity> = Default::default();
        let mut second_curve: Handle<IgesDataIgesEntity> = Default::default();
        crate::iges_shared_blocks::iges_block_0029!(
            first_leader,
            first_witness,
            ir,
            note,
            pr,
            second_leader,
            second_witness
        );
        pr.read_entity(
            ir,
            &pr.current(),
            "First Curve Entity",
            &mut first_curve,
            false,
        )?;
        pr.read_entity(
            ir,
            &pr.current(),
            "Second Curve Entity",
            &mut second_curve,
            true,
        )?;
        crate::iges_shared_blocks::iges_block_0028!(
            ent,
            first_leader,
            first_witness,
            ir,
            pr,
            second_leader,
            second_witness,
            self
        );
        ent.init(
            &note,
            &first_curve,
            &second_curve,
            &first_leader,
            &second_leader,
            &first_witness,
            &second_witness,
        );
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDimenCurveDimension>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send_entity(&ent.note(), false)?;
        iw.send_entity(&ent.first_curve(), false)?;
        iw.send_entity(&ent.second_curve(), false)?;
        iw.send_entity(&ent.first_leader(), false)?;
        iw.send_entity(&ent.second_leader(), false)?;
        iw.send_entity(&ent.first_witness_line(), false)?;
        iw.send_entity(&ent.second_witness_line(), false)?;
        Ok(())
    }
    /// Lists the Entities shared by a CurveDimension `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDimenCurveDimension>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.note().cast());
        iter.get_one_item(&ent.first_curve().cast());
        iter.get_one_item(&ent.second_curve().cast());
        iter.get_one_item(&ent.first_leader().cast());
        iter.get_one_item(&ent.second_leader().cast());
        iter.get_one_item(&ent.first_witness_line().cast());
        iter.get_one_item(&ent.second_witness_line().cast());
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDimenCurveDimension>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(204, 0);
        crate::iges_shared_blocks::iges_block_0039!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDimenCurveDimension>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.has_second_curve() {
            if ent.first_curve().is_kind(<IgesGeomLine>::static_type()) {
                if ent.second_curve().is_kind(<IgesGeomLine>::static_type()) {
                    ach.add_warning_text("Both curves are IgesGeomLine Entities", "");
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
        another: &Handle<IgesDimenCurveDimension>,
        ent: &Handle<IgesDimenCurveDimension>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut note: Handle<IgesDimenGeneralNote> = tc.transferred(&another.note().cast())?.cast();
        let mut first_curve: Handle<IgesDataIgesEntity> =
            tc.transferred(&another.first_curve().cast())?.cast();
        let mut second_curve: Handle<IgesDataIgesEntity> =
            tc.transferred(&another.second_curve().cast())?.cast();
        crate::iges_shared_blocks::iges_block_0069!(another, first_leader, second_leader, tc);
        let mut first_witness: Handle<IgesDimenWitnessLine> =
            tc.transferred(&another.first_witness_line().cast())?.cast();
        let mut second_witness: Handle<IgesDimenWitnessLine> = tc
            .transferred(&another.second_witness_line().cast())?
            .cast();
        ent.init(
            &note,
            &first_curve,
            &second_curve,
            &first_leader,
            &second_leader,
            &first_witness,
            &second_witness,
        );
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenCurveDimension>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut sublevel: i32 = if level > 4 { 1 } else { 0 };
        write!(s, "IgesDimenCurveDimension\nGeneral Note Entity   : ")?;
        dumper.dump(&ent.note(), s, sublevel, -1)?;
        write!(s, "\nFirst  Curve   Entity : ")?;
        dumper.dump(&ent.first_curve(), s, sublevel, -1)?;
        write!(s, "\nSecond Curve   Entity : ")?;
        dumper.dump(&ent.second_curve(), s, sublevel, -1)?;
        crate::iges_shared_blocks::iges_block_0035!(dumper, ent, s, sublevel);
        crate::iges_shared_blocks::iges_block_0060!(dumper, ent, s, sublevel);
        writeln!(s)?;
        Ok(())
    }
}
