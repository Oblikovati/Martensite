//! IgesDimenToolDiameterDimension.
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
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_dump_xy_l,
};
use crate::iges_dimen::{IgesDimenDiameterDimension, IgesDimenGeneralNote, IgesDimenLeaderArrow};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkmath::gp::Xy;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDimenToolDiameterDimension,
    "The own-parameter work on `IgesDimenDiameterDimension`."
);

impl IgesDimenToolDiameterDimension {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDimenDiameterDimension>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut note: Handle<IgesDimenGeneralNote> = Default::default();
        let mut first_leader: Handle<IgesDimenLeaderArrow> = Default::default();
        let mut second_leader: Handle<IgesDimenLeaderArrow> = Default::default();
        let mut center: Xy = Default::default();
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "General Note Entity",
            <IgesDimenGeneralNote>::static_type(),
            &mut note,
            false,
        )?;
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "First Leader Entity",
            <IgesDimenLeaderArrow>::static_type(),
            &mut first_leader,
            false,
        )?;
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "Second Leader Entity",
            <IgesDimenLeaderArrow>::static_type(),
            &mut second_leader,
            true,
        )?;
        pr.read_xy(&pr.current_list(1, 2), "Arc Center Co-ords", &mut center)?;
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(&note, &first_leader, &second_leader, center);
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDimenDiameterDimension>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0064!(ent, iw);
        iw.send(ent.center().x())?;
        iw.send(ent.center().y())?;
        Ok(())
    }
    /// Lists the Entities shared by a DiameterDimension `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDimenDiameterDimension>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0061!(ent, iter);
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(
        &self,
        _unused_0: &Handle<IgesDimenDiameterDimension>,
    ) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(206, 0);
        crate::iges_shared_blocks::iges_block_0039!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesDimenDiameterDimension>,
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
        another: &Handle<IgesDimenDiameterDimension>,
        ent: &Handle<IgesDimenDiameterDimension>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0052!(another, first_leader, note, second_leader, tc);
        let mut center: Xy = another.center().xy();
        ent.init(&note, &first_leader, &second_leader, center);
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenDiameterDimension>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut sublevel: i32 = if level > 4 { 1 } else { 0 };
        write!(s, "IgesDimenDiameterDimension\nGeneral Note Entity : ")?;
        dumper.dump(&ent.note(), s, sublevel, -1)?;
        write!(s, "\nFirst  Leader Entity : ")?;
        dumper.dump(&ent.first_leader(), s, sublevel, -1)?;
        write!(s, "\nSecond Leader Entity : ")?;
        dumper.dump(&ent.second_leader(), s, sublevel, -1)?;
        write!(s, "\nCenter Point : ")?;
        iges_dump_xy_l(s, level, &ent.center().xy(), &ent.location()?)?;
        writeln!(s)?;
        Ok(())
    }
}
