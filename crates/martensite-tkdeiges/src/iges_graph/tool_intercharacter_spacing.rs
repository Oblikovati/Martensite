//! IgesGraphToolIntercharacterSpacing.
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
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_real_text,
};
use crate::iges_graph::IgesGraphIntercharacterSpacing;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesGraphToolIntercharacterSpacing,
    "The own-parameter work on `IgesGraphIntercharacterSpacing`."
);

impl IgesGraphToolIntercharacterSpacing {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGraphIntercharacterSpacing>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut nb_property_values: i32 = Default::default();
        let mut i_space: f64 = Default::default();
        pr.read_integer_text(
            &pr.current(),
            "No. of property values",
            &mut nb_property_values,
        )?;
        if nb_property_values != 1 {
            pr.add_fail("No. of Property values : Value is not 1", "");
        }
        pr.read_real_text(
            &pr.current(),
            "Intercharacter space in % of text height",
            &mut i_space,
        )?;
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(nb_property_values, i_space);
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGraphIntercharacterSpacing>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.nb_property_values())?;
        iw.send(ent.i_space())?;
        Ok(())
    }
    /// Lists the Entities shared by a IntercharacterSpacing `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesGraphIntercharacterSpacing>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a IntercharacterSpacing
    /// (NbPropertyValues forced to 1)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(
        &self,
        ent: &Handle<IgesGraphIntercharacterSpacing>,
    ) -> Result<bool, Failure> {
        let mut res: bool = ent.nb_property_values() != 1;
        if res {
            ent.init(1, ent.i_space());
        }
        Ok(res)
    }
    /// Returns specific DirChecker
    pub fn dir_checker(
        &self,
        _unused_0: &Handle<IgesGraphIntercharacterSpacing>,
    ) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 18);
        crate::iges_shared_blocks::iges_block_0017!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGraphIntercharacterSpacing>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if (ent.i_space() < 0.0) || (ent.i_space() > 100.0) {
            ach.add_fail_text("Intercharacter Space : Value not in the range [0-100]", "");
        }
        if ent.nb_property_values() != 1 {
            ach.add_fail_text("No. of Property values : Value != 1", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGraphIntercharacterSpacing>,
        ent: &Handle<IgesGraphIntercharacterSpacing>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        ent.init(1, another.i_space());
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGraphIntercharacterSpacing>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _unused_3: i32,
    ) -> IgesDataDumpResult {
        writeln!(
            s,
            "IgesGraphIntercharacterSpacing\nNo. of property values : {}\nIntercharacter space in % of text height : {}\n",
            ent.nb_property_values(),
            iges_real_text(ent.i_space())
        )?;
        Ok(())
    }
}
