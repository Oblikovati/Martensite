//! IgesAppliToolPartNumber.
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
use crate::iges_appli::IgesAppliPartNumber;
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_dump_string, iges_live,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesAppliToolPartNumber,
    "The own-parameter work on `IgesAppliPartNumber`."
);

impl IgesAppliToolPartNumber {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesAppliPartNumber>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_nb_property_values: i32 = Default::default();
        let mut temp_generic_number: Handle<HAsciiString> = Default::default();
        let mut temp_military_number: Handle<HAsciiString> = Default::default();
        let mut temp_vendor_number: Handle<HAsciiString> = Default::default();
        let mut temp_internal_number: Handle<HAsciiString> = Default::default();
        if pr.defined_else_skip()? {
            pr.read_integer_text(
                &pr.current(),
                "Number of property values",
                &mut temp_nb_property_values,
            )?;
        } else {
            temp_nb_property_values = 4;
        }
        pr.read_text(
            &pr.current(),
            "Generic Number or Name",
            &mut temp_generic_number,
        )?;
        pr.read_text(
            &pr.current(),
            "Military Number or Name",
            &mut temp_military_number,
        )?;
        pr.read_text(
            &pr.current(),
            "Vendor Number or Name",
            &mut temp_vendor_number,
        )?;
        pr.read_text(
            &pr.current(),
            "Internal Number or Name",
            &mut temp_internal_number,
        )?;
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            temp_nb_property_values,
            &temp_generic_number,
            &temp_military_number,
            &temp_vendor_number,
            &temp_internal_number,
        );
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesAppliPartNumber>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.nb_property_values())?;
        iw.send(ent.generic_number())?;
        iw.send(ent.military_number())?;
        iw.send(ent.vendor_number())?;
        iw.send(ent.internal_number())?;
        Ok(())
    }
    /// Lists the Entities shared by a PartNumber `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesAppliPartNumber>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a PartNumber
    /// (NbPropertyValues forced to 4)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesAppliPartNumber>) -> Result<bool, Failure> {
        let mut res: bool = ent.nb_property_values() != 4;
        if res {
            ent.init(
                4,
                &ent.generic_number(),
                &ent.military_number(),
                &ent.vendor_number(),
                &ent.internal_number(),
            );
        }
        Ok(res)
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesAppliPartNumber>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 9);
        crate::iges_shared_blocks::iges_block_0011!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesAppliPartNumber>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.nb_property_values() != 4 {
            ach.add_fail_text("Number of property values != 4", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesAppliPartNumber>,
        ent: &Handle<IgesAppliPartNumber>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut temp_nb_property_values: i32 = another.nb_property_values();
        let mut temp_generic_number: Handle<HAsciiString> =
            Handle::new(HAsciiString::copied(iges_live(
                &another.generic_number(),
                "IgesAppliToolPartNumber::own_copy",
            )?));
        let mut temp_military_number: Handle<HAsciiString> =
            Handle::new(HAsciiString::copied(iges_live(
                &another.military_number(),
                "IgesAppliToolPartNumber::own_copy",
            )?));
        let mut temp_vendor_number: Handle<HAsciiString> =
            Handle::new(HAsciiString::copied(iges_live(
                &another.vendor_number(),
                "IgesAppliToolPartNumber::own_copy",
            )?));
        let mut temp_internal_number: Handle<HAsciiString> =
            Handle::new(HAsciiString::copied(iges_live(
                &another.internal_number(),
                "IgesAppliToolPartNumber::own_copy",
            )?));
        ent.init(
            temp_nb_property_values,
            &temp_generic_number,
            &temp_military_number,
            &temp_vendor_number,
            &temp_internal_number,
        );
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesAppliPartNumber>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _unused_3: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesAppliPartNumber")?;
        writeln!(
            s,
            "Number of property values : {}",
            ent.nb_property_values()
        )?;
        write!(s, "Generic  Number or Name : ")?;
        iges_dump_string(s, &ent.generic_number())?;
        writeln!(s)?;
        write!(s, "Military Number or Name : ")?;
        iges_dump_string(s, &ent.military_number())?;
        writeln!(s)?;
        write!(s, "Vendor   Number or Name : ")?;
        iges_dump_string(s, &ent.vendor_number())?;
        writeln!(s)?;
        write!(s, "Internal Number or Name : ")?;
        iges_dump_string(s, &ent.internal_number())?;
        writeln!(s)?;
        Ok(())
    }
}
