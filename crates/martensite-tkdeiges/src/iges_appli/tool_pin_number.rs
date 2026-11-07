//! IgesAppliToolPinNumber.
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
use crate::iges_appli::IgesAppliPinNumber;
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefList, IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataLevelListEntity, IgesDataParamReader,
    iges_dump_string, iges_live,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesAppliToolPinNumber,
    "The own-parameter work on `IgesAppliPinNumber`."
);

impl IgesAppliToolPinNumber {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesAppliPinNumber>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_nb_property_values: i32 = Default::default();
        let mut temp_pin_number: Handle<HAsciiString> = Default::default();
        pr.read_integer_text(
            &pr.current(),
            "Number of property values",
            &mut temp_nb_property_values,
        )?;
        pr.read_text(&pr.current(), "PinNumber", &mut temp_pin_number)?;
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(temp_nb_property_values, &temp_pin_number);
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesAppliPinNumber>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.nb_property_values())?;
        iw.send(ent.pin_number_val())?;
        Ok(())
    }
    /// Lists the Entities shared by a PinNumber `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesAppliPinNumber>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a PinNumber
    /// (Level cleared in D.E. if Subordinate != 0)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesAppliPinNumber>) -> Result<bool, Failure> {
        let mut res: bool = ent.subordinate_status() != 0;
        if res {
            let mut nulevel: Handle<dyn IgesDataLevelListEntity> = Default::default();
            ent.init_level(&nulevel, 0);
        }
        Ok(res)
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesAppliPinNumber>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 8);
        dc.structure(IgesDataDefType::DefVoid);
        dc.graphics_ignored(-1);
        dc.blank_status_ignored();
        dc.use_flag_ignored();
        dc.hierarchy_status_ignored();
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesAppliPinNumber>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.subordinate_status() != 0 {
            if (ent.def_level() != (IgesDataDefList::DefOne))
                && (ent.def_level() != (IgesDataDefList::DefSeveral))
            {
                ach.add_fail_text("Level type: Incorrect", "");
            }
        }
        if ent.nb_property_values() != 1 {
            ach.add_fail_text("Number of Property Values != 1", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesAppliPinNumber>,
        ent: &Handle<IgesAppliPinNumber>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut a_nb_property_values: i32 = Default::default();
        let mut a_pin_number: Handle<HAsciiString> = Handle::new(HAsciiString::copied(iges_live(
            &another.pin_number_val(),
            "IgesAppliToolPinNumber::own_copy",
        )?));
        a_nb_property_values = another.nb_property_values();
        ent.init(a_nb_property_values, &a_pin_number);
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesAppliPinNumber>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _unused_3: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesAppliPinNumber")?;
        writeln!(
            s,
            "Number of Property Values : {}",
            ent.nb_property_values()
        )?;
        write!(s, "PinNumber : ")?;
        iges_dump_string(s, &ent.pin_number_val())?;
        writeln!(s)?;
        Ok(())
    }
}
