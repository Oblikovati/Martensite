//! IgesGraphToolDefinitionLevel.
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
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataLevelListEntity, IgesDataParamReader,
    iges_array1_set, iges_dump_vals, iges_new_array1,
};
use crate::iges_graph::IgesGraphDefinitionLevel;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesGraphToolDefinitionLevel,
    "The own-parameter work on `IgesGraphDefinitionLevel`."
);

impl IgesGraphToolDefinitionLevel {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGraphDefinitionLevel>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut nbval: i32 = Default::default();
        let mut level_numbers: Handle<HArray1<i32>> = Default::default();
        let mut st: bool =
            pr.read_integer_text(&pr.current(), "No. of Property Values", &mut nbval)?;
        if st && (nbval > 0) {
            pr.read_ints(
                &pr.current_list(nbval, 1),
                "array levelNumbers",
                &mut level_numbers,
                1,
            )?;
        } else {
            pr.add_fail("No. of Property Values : Not Positive", "");
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(&level_numbers)?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGraphDefinitionLevel>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut up: i32 = ent.nb_property_values()?;
        iw.send(up)?;
        for i in 1..=up {
            iw.send(ent.level_number(i)?)?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a DefinitionLevel `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesGraphDefinitionLevel>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesGraphDefinitionLevel>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 1);
        crate::iges_shared_blocks::iges_block_0017!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesGraphDefinitionLevel>,
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
        another: &Handle<IgesGraphDefinitionLevel>,
        ent: &Handle<IgesGraphDefinitionLevel>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut nbval: i32 = Default::default();
        let mut level_numbers: Handle<HArray1<i32>> = Default::default();
        nbval = another.nb_property_values()?;
        level_numbers = iges_new_array1::<i32>(1, nbval)?;
        for i in 1..=nbval {
            iges_array1_set(
                &level_numbers,
                i,
                another.level_number(i)?,
                "IgesGraphToolDefinitionLevel::own_copy",
            )?;
        }
        ent.init(&level_numbers)?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGraphDefinitionLevel>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        write!(s, "IgesGraphDefinitionLevel\nLevel Numbers : ")?;
        iges_dump_vals(s, level, 1, ent.nb_property_values()?, |first| {
            ent.level_number(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
