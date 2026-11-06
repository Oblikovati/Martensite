//! IgesDimenToolPointDimension.
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
use crate::iges_dimen::{IgesDimenGeneralNote, IgesDimenLeaderArrow, IgesDimenPointDimension};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDimenToolPointDimension,
    "The own-parameter work on `IgesDimenPointDimension`."
);

impl IgesDimenToolPointDimension {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDimenPointDimension>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_note: Handle<IgesDimenGeneralNote> = Default::default();
        let mut lead_arr: Handle<IgesDimenLeaderArrow> = Default::default();
        let mut temp_geom: Handle<IgesDataIgesEntity> = Default::default();
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "General Note",
            <IgesDimenGeneralNote>::static_type(),
            &mut temp_note,
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
        if pr.is_param_entity(pr.current_number())? {
            pr.read_entity(ir, &pr.current(), "Enclosing entity", &mut temp_geom, false)?;
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(&temp_note, &lead_arr, &temp_geom);
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDimenPointDimension>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send_entity(&ent.note(), false)?;
        iw.send_entity(&ent.leader_arrow(), false)?;
        iw.send_entity(&ent.geom(), false)?;
        Ok(())
    }
    /// Lists the Entities shared by a PointDimension `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDimenPointDimension>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.note().cast());
        iter.get_one_item(&ent.leader_arrow().cast());
        iter.get_one_item(&ent.geom().cast());
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDimenPointDimension>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(220, 0);
        crate::iges_shared_blocks::iges_block_0039!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesDimenPointDimension>,
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
        another: &Handle<IgesDimenPointDimension>,
        ent: &Handle<IgesDimenPointDimension>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut temp_note: Handle<IgesDimenGeneralNote> =
            tc.transferred(&another.note().cast())?.cast();
        let mut temp_arrow: Handle<IgesDimenLeaderArrow> =
            tc.transferred(&another.leader_arrow().cast())?.cast();
        let mut temp_geom: Handle<IgesDataIgesEntity> =
            tc.transferred(&another.geom().cast())?.cast();
        ent.init(&temp_note, &temp_arrow, &temp_geom);
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenPointDimension>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesDimenPointDimension")?;
        crate::iges_shared_blocks::iges_block_0063!(dumper, ent, level, s, sublevel);
        write!(s, "\nLeader Arrow : ")?;
        dumper.dump(&ent.leader_arrow(), s, sublevel, -1)?;
        writeln!(s)?;
        if !(ent.geom().is_null()) {
            write!(s, "Enclosing Entity : ")?;
            dumper.dump(&ent.geom(), s, sublevel, -1)?;
            writeln!(s)?;
        }
        Ok(())
    }
}
