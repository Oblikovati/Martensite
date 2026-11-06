//! IgesDimenToolLinearDimension.
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
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader,
};
use crate::iges_dimen::{
    IgesDimenGeneralNote, IgesDimenLeaderArrow, IgesDimenLinearDimension, IgesDimenWitnessLine,
};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDimenToolLinearDimension,
    "The own-parameter work on `IgesDimenLinearDimension`."
);

impl IgesDimenToolLinearDimension {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDimenLinearDimension>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut note: Handle<IgesDimenGeneralNote> = Default::default();
        crate::iges_shared_blocks::iges_block_0029!(
            first_leader,
            first_witness,
            ir,
            note,
            pr,
            second_leader,
            second_witness
        );
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
        ent: &Handle<IgesDimenLinearDimension>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0064!(ent, iw);
        iw.send_entity(&ent.first_witness(), false)?;
        iw.send_entity(&ent.second_witness(), false)?;
        Ok(())
    }
    /// Lists the Entities shared by a LinearDimension `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDimenLinearDimension>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0061!(ent, iter);
        iter.get_one_item(&ent.first_witness().cast());
        iter.get_one_item(&ent.second_witness().cast());
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDimenLinearDimension>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_forms(216, 0, 2);
        crate::iges_shared_blocks::iges_block_0039!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesDimenLinearDimension>,
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
        another: &Handle<IgesDimenLinearDimension>,
        ent: &Handle<IgesDimenLinearDimension>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0052!(another, first_leader, note, second_leader, tc);
        let mut first_witness: Handle<IgesDimenWitnessLine> =
            tc.transferred(&another.first_witness().cast())?.cast();
        let mut second_witness: Handle<IgesDimenWitnessLine> =
            tc.transferred(&another.second_witness().cast())?.cast();
        ent.init(
            &note,
            &first_leader,
            &second_leader,
            &first_witness,
            &second_witness,
        );
        ent.set_form_number(another.form_number())?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenLinearDimension>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut sublevel: i32 = if level > 4 { 1 } else { 0 };
        writeln!(s, "IgesDimenLinearDimension")?;
        if ent.form_number() == 0 {
            writeln!(s, "     (Undetermined Form)")?;
        } else {
            if ent.form_number() == 1 {
                writeln!(s, "     (Diameter Form)")?;
            } else {
                if ent.form_number() == 2 {
                    writeln!(s, "     (Radius Form)")?;
                }
            }
        }
        write!(s, "General Note Entity : ")?;
        dumper.dump(&ent.note(), s, sublevel, -1)?;
        crate::iges_shared_blocks::iges_block_0035!(dumper, ent, s, sublevel);
        dumper.dump(&ent.first_witness(), s, sublevel, -1)?;
        write!(s, "\nSecond Witness Entity : ")?;
        dumper.dump(&ent.second_witness(), s, sublevel, -1)?;
        writeln!(s)?;
        Ok(())
    }
}
