//! IgesDimenToolRadiusDimension.
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
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_dump_xy_lz, iges_live,
};
use crate::iges_dimen::{IgesDimenGeneralNote, IgesDimenLeaderArrow, IgesDimenRadiusDimension};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkmath::gp::Xy;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDimenToolRadiusDimension,
    "The own-parameter work on `IgesDimenRadiusDimension`."
);

impl IgesDimenToolRadiusDimension {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDimenRadiusDimension>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_note: Handle<IgesDimenGeneralNote> = Default::default();
        let mut lead_arr: Handle<IgesDimenLeaderArrow> = Default::default();
        let mut arc_center: Xy = Default::default();
        let mut lead_arr2: Handle<IgesDimenLeaderArrow> = Default::default();
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
            "Leader arrow",
            <IgesDimenLeaderArrow>::static_type(),
            &mut lead_arr,
            false,
        )?;
        pr.read_xy(&pr.current_list(1, 2), "Arc center", &mut arc_center)?;
        if ent.form_number() == 1 {
            pr.read_entity_typed(
                ir,
                &pr.current(),
                "Leader arrow 2",
                <IgesDimenLeaderArrow>::static_type(),
                &mut lead_arr2,
                true,
            )?;
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(&temp_note, &lead_arr, arc_center, &lead_arr2);
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDimenRadiusDimension>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send_entity(&ent.note(), false)?;
        iw.send_entity(&ent.leader(), false)?;
        iw.send(ent.center().x())?;
        iw.send(ent.center().y())?;
        if ent.has_leader2() || (ent.form_number() == 1) {
            iw.send_entity(&ent.leader2(), false)?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a RadiusDimension `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDimenRadiusDimension>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.note().cast());
        iter.get_one_item(&ent.leader().cast());
        iter.get_one_item(&ent.leader2().cast());
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDimenRadiusDimension>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_forms(222, 0, 1);
        crate::iges_shared_blocks::iges_block_0039!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDimenRadiusDimension>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.has_leader2() && (ent.form_number() == 0) {
            ach.add_fail_text(
                "Value of Form Number not consistent with presence of Leader2",
                "",
            );
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDimenRadiusDimension>,
        ent: &Handle<IgesDimenRadiusDimension>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut temp_note: Handle<IgesDimenGeneralNote> =
            tc.transferred(&another.note().cast())?.cast();
        let mut lead_arr: Handle<IgesDimenLeaderArrow> =
            tc.transferred(&another.leader().cast())?.cast();
        let mut arc_center: Xy = another.center().xy();
        let mut lead_arr2: Handle<IgesDimenLeaderArrow> = Default::default();
        if another.has_leader2() {
            lead_arr2 = tc
                .transferred(&another.leader2().cast())?
                .cast::<IgesDimenLeaderArrow>();
        }
        ent.init(&temp_note, &lead_arr, arc_center, &lead_arr2);
        ent.init_form(another.form_number());
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenRadiusDimension>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesDimenRadiusDimension")?;
        let mut sublevel: i32 = if level <= 4 { 0 } else { 1 };
        write!(s, "General note : ")?;
        dumper.dump(&ent.note(), s, sublevel, -1)?;
        write!(s, "\nLeader arrow : ")?;
        dumper.dump(&ent.leader(), s, sublevel, -1)?;
        write!(s, "\nArc center : ")?;
        iges_dump_xy_lz(
            s,
            level,
            &ent.center().xy(),
            &ent.location()?,
            iges_live(&ent.leader(), "IgesDimenToolRadiusDimension::own_dump")?.z_depth(),
        )?;
        if ent.has_leader2() {
            write!(s, "\nLeader arrow 2 : ")?;
            dumper.dump(&ent.leader2(), s, sublevel, -1)?;
            writeln!(s)?;
        }
        writeln!(s)?;
        Ok(())
    }
}
