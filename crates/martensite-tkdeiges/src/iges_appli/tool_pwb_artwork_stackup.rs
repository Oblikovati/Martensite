//! IgesAppliToolPWBArtworkStackup.
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
use crate::iges_appli::IgesAppliPWBArtworkStackup;
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_array1_set,
    iges_dump_string, iges_dump_vals, iges_live, iges_new_array1,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesAppliToolPWBArtworkStackup,
    "The own-parameter work on `IgesAppliPWBArtworkStackup`."
);

impl IgesAppliToolPWBArtworkStackup {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesAppliPWBArtworkStackup>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut num: i32 = Default::default();
        let mut temp_nb_property_values: i32 = Default::default();
        let mut temp_artwork_stackup_ident: Handle<HAsciiString> = Default::default();
        let mut temp_level_numbers: Handle<HArray1<i32>> = Default::default();
        pr.read_integer_text(
            &pr.current(),
            "Number of property values",
            &mut temp_nb_property_values,
        )?;
        pr.read_text(
            &pr.current(),
            "Artwork Stackup Identification",
            &mut temp_artwork_stackup_ident,
        )?;
        if !(pr.read_integer_text(&pr.current(), "Number of level numbers", &mut num)?) {
            num = 0;
        }
        if num > 0 {
            temp_level_numbers = iges_new_array1::<i32>(1, num)?;
        } else {
            pr.add_fail("Number of level numbers: Not Positive", "");
        }
        if !temp_level_numbers.is_null() {
            pr.read_ints(
                &pr.current_list(num, 1),
                "Level Numbers",
                &mut temp_level_numbers,
                1,
            )?;
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            temp_nb_property_values,
            &temp_artwork_stackup_ident,
            &temp_level_numbers,
        )?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesAppliPWBArtworkStackup>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut num: i32 = Default::default();
        iw.send(ent.nb_property_values())?;
        iw.send(ent.identification())?;
        iw.send(ent.nb_level_numbers()?)?;
        {
            num = ent.nb_level_numbers()?;
            i = 1;
            while i <= num {
                iw.send(ent.level_number(i)?)?;
                i += 1;
            }
        }
        Ok(())
    }
    /// Lists the Entities shared by a PWBArtworkStackup `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesAppliPWBArtworkStackup>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(
        &self,
        _unused_0: &Handle<IgesAppliPWBArtworkStackup>,
    ) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 25);
        crate::iges_shared_blocks::iges_block_0011!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesAppliPWBArtworkStackup>,
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
        another: &Handle<IgesAppliPWBArtworkStackup>,
        ent: &Handle<IgesAppliPWBArtworkStackup>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut num: i32 = another.nb_level_numbers()?;
        let mut temp_nb_property_values: i32 = another.nb_property_values();
        let mut temp_artwork_stackup_ident: Handle<HAsciiString> =
            Handle::new(HAsciiString::copied(iges_live(
                &another.identification(),
                "IgesAppliToolPWBArtworkStackup::own_copy",
            )?));
        let mut temp_level_numbers: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, num)?;
        for i in 1..=num {
            iges_array1_set(
                &temp_level_numbers,
                i,
                another.level_number(i)?,
                "IgesAppliToolPWBArtworkStackup::own_copy",
            )?;
        }
        ent.init(
            temp_nb_property_values,
            &temp_artwork_stackup_ident,
            &temp_level_numbers,
        )?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesAppliPWBArtworkStackup>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesAppliPWBArtworkStackup")?;
        writeln!(
            s,
            "Number of property values : {}",
            ent.nb_property_values()
        )?;
        write!(s, "Artwork Stackup Identification : ")?;
        iges_dump_string(s, &ent.identification())?;
        writeln!(s)?;
        write!(s, "Level Numbers : ")?;
        iges_dump_vals(s, level, 1, ent.nb_level_numbers()?, |first| {
            ent.level_number(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
