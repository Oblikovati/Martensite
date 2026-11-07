//! IgesDimenToolAngularDimension.
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
    iges_real_text,
};
use crate::iges_dimen::{
    IgesDimenAngularDimension, IgesDimenGeneralNote, IgesDimenLeaderArrow, IgesDimenWitnessLine,
};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkmath::gp::Xy;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDimenToolAngularDimension,
    "The own-parameter work on `IgesDimenAngularDimension`."
);

impl IgesDimenToolAngularDimension {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDimenAngularDimension>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut note: Handle<IgesDimenGeneralNote> = Default::default();
        let mut first_witness: Handle<IgesDimenWitnessLine> = Default::default();
        let mut second_witness: Handle<IgesDimenWitnessLine> = Default::default();
        let mut vertex: Xy = Default::default();
        let mut radius: f64 = Default::default();
        let mut first_leader: Handle<IgesDimenLeaderArrow> = Default::default();
        let mut second_leader: Handle<IgesDimenLeaderArrow> = Default::default();
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
            "First Witness Entity",
            <IgesDimenWitnessLine>::static_type(),
            &mut first_witness,
            true,
        )?;
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "Second Witness Entity",
            <IgesDimenWitnessLine>::static_type(),
            &mut second_witness,
            true,
        )?;
        pr.read_xy(&pr.current_list(1, 2), "Vertex Point Co-ords", &mut vertex)?;
        pr.read_real_text(&pr.current(), "Radius of Leader arcs", &mut radius)?;
        crate::iges_shared_blocks::iges_block_0068!(first_leader, ir, pr, second_leader);
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            &note,
            &first_witness,
            &second_witness,
            vertex,
            radius,
            &first_leader,
            &second_leader,
        );
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDimenAngularDimension>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send_entity(&ent.note(), false)?;
        iw.send_entity(&ent.first_witness_line(), false)?;
        iw.send_entity(&ent.second_witness_line(), false)?;
        iw.send(ent.vertex().x())?;
        iw.send(ent.vertex().y())?;
        iw.send(ent.radius())?;
        iw.send_entity(&ent.first_leader(), false)?;
        iw.send_entity(&ent.second_leader(), false)?;
        Ok(())
    }
    /// Lists the Entities shared by a AngularDimension `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDimenAngularDimension>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.note().cast());
        iter.get_one_item(&ent.first_witness_line().cast());
        iter.get_one_item(&ent.second_witness_line().cast());
        iter.get_one_item(&ent.first_leader().cast());
        iter.get_one_item(&ent.second_leader().cast());
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDimenAngularDimension>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(202, 0);
        crate::iges_shared_blocks::iges_block_0039!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesDimenAngularDimension>,
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
        another: &Handle<IgesDimenAngularDimension>,
        ent: &Handle<IgesDimenAngularDimension>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut note: Handle<IgesDimenGeneralNote> = tc.transferred(&another.note().cast())?.cast();
        let mut first_witness: Handle<IgesDimenWitnessLine> =
            tc.transferred(&another.first_witness_line().cast())?.cast();
        let mut second_witness: Handle<IgesDimenWitnessLine> = tc
            .transferred(&another.second_witness_line().cast())?
            .cast();
        let mut vertex: Xy = another.vertex().xy();
        let mut radius: f64 = another.radius();
        crate::iges_shared_blocks::iges_block_0069!(another, first_leader, second_leader, tc);
        ent.init(
            &note,
            &first_witness,
            &second_witness,
            vertex,
            radius,
            &first_leader,
            &second_leader,
        );
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenAngularDimension>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut sublevel: i32 = if level > 4 { 1 } else { 0 };
        write!(s, "IgesDimenAngularDimension\nGeneral Note Entity   : ")?;
        dumper.dump(&ent.note(), s, sublevel, -1)?;
        write!(s, "\nFirst  Witness Entity : ")?;
        crate::iges_shared_blocks::iges_block_0060!(dumper, ent, s, sublevel);
        write!(s, "\nVertex Point Co-ords  : ")?;
        iges_dump_xy_l(s, level, &ent.vertex().xy(), &ent.location()?)?;
        writeln!(s)?;
        write!(
            s,
            "Radius of Leader arcs : {}\nFirst  Leader Entity  : ",
            iges_real_text(ent.radius())
        )?;
        dumper.dump(&ent.first_leader(), s, sublevel, -1)?;
        write!(s, "\nSecond Leader Entity  : ")?;
        dumper.dump(&ent.second_leader(), s, sublevel, -1)?;
        writeln!(s)?;
        Ok(())
    }
}
