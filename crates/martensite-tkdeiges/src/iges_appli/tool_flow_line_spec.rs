//! IgesAppliToolFlowLineSpec.
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
use crate::iges_appli::IgesAppliFlowLineSpec;
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_array1_set,
    iges_dump_strings, iges_live, iges_new_array1,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesAppliToolFlowLineSpec,
    "The own-parameter work on `IgesAppliFlowLineSpec`."
);

impl IgesAppliToolFlowLineSpec {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesAppliFlowLineSpec>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut num: i32 = Default::default();
        let mut temp_name_and_modifiers: Handle<HArray1<Handle<HAsciiString>>> = Default::default();
        if !(pr.read_integer_text(&pr.current(), "Number of property values", &mut num)?) {
            num = 0;
        }
        if num > 0 {
            temp_name_and_modifiers = iges_new_array1::<Handle<HAsciiString>>(1, num)?;
        } else {
            pr.add_fail("Number of property values: Not Positive", "");
        }
        if !temp_name_and_modifiers.is_null() {
            pr.read_texts(
                &pr.current_list(num, 1),
                "Name and Modifiers",
                &mut temp_name_and_modifiers,
                1,
            )?;
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(&temp_name_and_modifiers)?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesAppliFlowLineSpec>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut num: i32 = Default::default();
        iw.send(ent.nb_property_values()?)?;
        {
            num = ent.nb_property_values()?;
            i = 1;
            while i <= num {
                iw.send(ent.modifier(i)?)?;
                i += 1;
            }
        }
        Ok(())
    }
    /// Lists the Entities shared by a FlowLineSpec `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesAppliFlowLineSpec>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesAppliFlowLineSpec>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 14);
        crate::iges_shared_blocks::iges_block_0011!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesAppliFlowLineSpec>,
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
        another: &Handle<IgesAppliFlowLineSpec>,
        ent: &Handle<IgesAppliFlowLineSpec>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut num: i32 = another.nb_property_values()?;
        let mut temp_name_and_modifiers: Handle<HArray1<Handle<HAsciiString>>> =
            iges_new_array1::<Handle<HAsciiString>>(1, num)?;
        for i in 1..=num {
            iges_array1_set(
                &temp_name_and_modifiers,
                i,
                Handle::new(HAsciiString::copied(iges_live(
                    &another.modifier(i)?,
                    "IgesAppliToolFlowLineSpec::own_copy",
                )?)),
                "IgesAppliToolFlowLineSpec::own_copy",
            )?;
        }
        ent.init(&temp_name_and_modifiers)?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesAppliFlowLineSpec>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesAppliFlowLineSpec")?;
        write!(s, "Name and Modifiers : ")?;
        iges_dump_strings(s, level, 1, ent.nb_property_values()?, |first| {
            ent.modifier(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
