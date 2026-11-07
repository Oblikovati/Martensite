//! IgesAppliToolLevelFunction.
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
use crate::iges_appli::IgesAppliLevelFunction;
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
    IgesAppliToolLevelFunction,
    "The own-parameter work on `IgesAppliLevelFunction`."
);

impl IgesAppliToolLevelFunction {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesAppliLevelFunction>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_nb_property_values: i32 = Default::default();
        let mut temp_func_descrip_code: i32 = Default::default();
        let mut temp_func_descrip: Handle<HAsciiString> = Default::default();
        pr.read_integer_text(
            &pr.current(),
            "No. of Property values",
            &mut temp_nb_property_values,
        )?;
        if pr.defined_else_skip()? {
            pr.read_integer_text(
                &pr.current(),
                "Function description code",
                &mut temp_func_descrip_code,
            )?;
        } else {
            temp_func_descrip_code = 0;
        }
        if pr.defined_else_skip()? {
            pr.read_text(
                &pr.current(),
                "Function description",
                &mut temp_func_descrip,
            )?;
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            temp_nb_property_values,
            temp_func_descrip_code,
            &temp_func_descrip,
        );
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesAppliLevelFunction>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.nb_property_values())?;
        iw.send(ent.func_description_code())?;
        if ent.func_description().is_null() {
            iw.send_void()?;
        } else {
            iw.send(ent.func_description())?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a LevelFunction `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesAppliLevelFunction>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a LevelFunction
    /// (NbPropertyValues forced to 2)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesAppliLevelFunction>) -> Result<bool, Failure> {
        let mut res: bool = ent.nb_property_values() != 2;
        if res {
            ent.init(2, ent.func_description_code(), &ent.func_description());
        }
        Ok(res)
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesAppliLevelFunction>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 3);
        dc.structure(IgesDataDefType::DefVoid);
        dc.graphics_ignored(-1);
        dc.blank_status_ignored();
        dc.use_flag_ignored();
        dc.hierarchy_status_ignored();
        dc.subordinate_status_required(0);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesAppliLevelFunction>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.nb_property_values() != 2 {
            ach.add_fail_text("Number of Property Values != 2", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesAppliLevelFunction>,
        ent: &Handle<IgesAppliLevelFunction>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut a_nb_property_values: i32 = Default::default();
        let mut code: i32 = Default::default();
        let mut descrip: Handle<HAsciiString> = Default::default();
        if !(another.func_description().is_null()) {
            descrip = Handle::new(HAsciiString::copied(iges_live(
                &another.func_description(),
                "IgesAppliToolLevelFunction::own_copy",
            )?));
        }
        code = another.func_description_code();
        a_nb_property_values = another.nb_property_values();
        ent.init(a_nb_property_values, code, &descrip);
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesAppliLevelFunction>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _unused_3: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesAppliLevelFunction")?;
        writeln!(
            s,
            "Number of property values : {}",
            ent.nb_property_values()
        )?;
        writeln!(
            s,
            "Function Description code : {}",
            ent.func_description_code()
        )?;
        write!(s, "Function Description : ")?;
        iges_dump_string(s, &ent.func_description())?;
        writeln!(s)?;
        Ok(())
    }
}
