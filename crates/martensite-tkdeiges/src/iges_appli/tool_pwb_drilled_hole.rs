//! IgesAppliToolPWBDrilledHole.
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
use crate::iges_appli::IgesAppliPWBDrilledHole;
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_real_text,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesAppliToolPWBDrilledHole,
    "The own-parameter work on `IgesAppliPWBDrilledHole`."
);

impl IgesAppliToolPWBDrilledHole {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesAppliPWBDrilledHole>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_nb_property_values: i32 = Default::default();
        let mut temp_drill_diameter: f64 = Default::default();
        let mut temp_finish_diameter: f64 = Default::default();
        let mut temp_function_code: i32 = Default::default();
        if pr.defined_else_skip()? {
            pr.read_integer_text(
                &pr.current(),
                "Number of property values",
                &mut temp_nb_property_values,
            )?;
        } else {
            temp_nb_property_values = 3;
        }
        pr.read_real_text(
            &pr.current(),
            "Drill Diameter Size",
            &mut temp_drill_diameter,
        )?;
        pr.read_real_text(
            &pr.current(),
            "Finish Diameter Size",
            &mut temp_finish_diameter,
        )?;
        pr.read_integer_text(
            &pr.current(),
            "Drilled Hole Function Code",
            &mut temp_function_code,
        )?;
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            temp_nb_property_values,
            temp_drill_diameter,
            temp_finish_diameter,
            temp_function_code,
        );
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesAppliPWBDrilledHole>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.nb_property_values())?;
        iw.send(ent.drill_diameter_size())?;
        iw.send(ent.finish_diameter_size())?;
        iw.send(ent.function_code())?;
        Ok(())
    }
    /// Lists the Entities shared by a PWBDrilledHole `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesAppliPWBDrilledHole>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a PWBDrilledHole
    /// (NbPropertyValues forced to 3)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesAppliPWBDrilledHole>) -> Result<bool, Failure> {
        let mut res: bool = ent.nb_property_values() != 3;
        if res {
            ent.init(
                3,
                ent.drill_diameter_size(),
                ent.finish_diameter_size(),
                ent.function_code(),
            );
        }
        Ok(res)
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesAppliPWBDrilledHole>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 26);
        crate::iges_shared_blocks::iges_block_0011!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesAppliPWBDrilledHole>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.nb_property_values() != 3 {
            ach.add_fail_text("Number of property values != 3", "");
        }
        if ((ent.function_code() < 1)
            || ((ent.function_code() > 5) && (ent.function_code() < 5001)))
            || (ent.function_code() > 9999)
        {
            ach.add_fail_text("Drilled Hole Function Code != 1-5,5001-9999", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesAppliPWBDrilledHole>,
        ent: &Handle<IgesAppliPWBDrilledHole>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        ent.init(
            3,
            another.drill_diameter_size(),
            another.finish_diameter_size(),
            another.function_code(),
        );
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesAppliPWBDrilledHole>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _unused_3: i32,
    ) -> IgesDataDumpResult {
        writeln!(
            s,
            "IgesAppliPWBDrilledHole\nNumber of property values : {}\nDrill Diameter Size  : {}\nFinish Diameter Size : {}\nDrilled Hole Function Code : {}",
            ent.nb_property_values(),
            iges_real_text(ent.drill_diameter_size()),
            iges_real_text(ent.finish_diameter_size()),
            ent.function_code()
        )?;
        Ok(())
    }
}
