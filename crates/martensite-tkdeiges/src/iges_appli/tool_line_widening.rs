//! IgesAppliToolLineWidening.
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
use crate::iges_appli::IgesAppliLineWidening;
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefList, IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataLevelListEntity, IgesDataParamReader,
    iges_real_text,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesAppliToolLineWidening,
    "The own-parameter work on `IgesAppliLineWidening`."
);

impl IgesAppliToolLineWidening {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesAppliLineWidening>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_nb_property_values: i32 = Default::default();
        let mut temp_width: f64 = Default::default();
        let mut temp_cornering_code: i32 = Default::default();
        let mut temp_extension_flag: i32 = Default::default();
        let mut temp_justification_flag: i32 = Default::default();
        let mut temp_extension_value: f64 = 0.0;
        pr.read_integer_text(
            &pr.current(),
            "No. of Property values",
            &mut temp_nb_property_values,
        )?;
        pr.read_real_text(&pr.current(), "Width of metalization", &mut temp_width)?;
        pr.read_integer_text(&pr.current(), "Cornering code", &mut temp_cornering_code)?;
        pr.read_integer_text(&pr.current(), "Extension Flag", &mut temp_extension_flag)?;
        pr.read_integer_text(
            &pr.current(),
            "Justification Flag",
            &mut temp_justification_flag,
        )?;
        if pr.is_param_defined(pr.current_number())? {
            pr.read_real_text(&pr.current(), "Extension value", &mut temp_extension_value)?;
        } else {
            if temp_extension_flag == 2 {
                pr.add_fail("Extension Value not defined while Extension Flag = 2", "");
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            temp_nb_property_values,
            temp_width,
            temp_cornering_code,
            temp_extension_flag,
            temp_justification_flag,
            temp_extension_value,
        );
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesAppliLineWidening>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.nb_property_values())?;
        iw.send(ent.width_of_metalization())?;
        iw.send(ent.cornering_code())?;
        iw.send(ent.extension_flag())?;
        iw.send(ent.justification_flag())?;
        iw.send(ent.extension_value())?;
        Ok(())
    }
    /// Lists the Entities shared by a LineWidening `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesAppliLineWidening>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a LineWidening
    /// (NbPropertyValues forced to 5, Level cleared if Subordinate != 0)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesAppliLineWidening>) -> Result<bool, Failure> {
        let mut res: bool = ent.nb_property_values() != 5;
        if res {
            ent.init(
                5,
                ent.width_of_metalization(),
                ent.cornering_code(),
                ent.extension_flag(),
                ent.justification_flag(),
                ent.extension_value(),
            );
        }
        if ent.subordinate_status() != 0 {
            let mut nulevel: Handle<dyn IgesDataLevelListEntity> = Default::default();
            ent.init_level(&nulevel, 0);
            res = true;
        }
        Ok(res)
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesAppliLineWidening>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 5);
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
        ent: &Handle<IgesAppliLineWidening>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.subordinate_status() != 0 {
            if (ent.def_level() == (IgesDataDefList::DefOne))
                || (ent.def_level() == (IgesDataDefList::DefSeveral))
            {
                ach.add_warning_text("Level type: defined while ignored", "");
            }
        }
        if ent.nb_property_values() != 5 {
            ach.add_fail_text("Number of Property Values != 5", "");
        }
        if (ent.cornering_code() != 0) && (ent.cornering_code() != 1) {
            ach.add_fail_text("Cornering Code incorrect", "");
        }
        if (ent.extension_flag() < 0) || (ent.extension_flag() > 2) {
            ach.add_fail_text("Extension Flag value incorrect", "");
        }
        if (ent.justification_flag() < 0) || (ent.justification_flag() > 2) {
            ach.add_fail_text("Justification Flag value incorrect", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesAppliLineWidening>,
        ent: &Handle<IgesAppliLineWidening>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        ent.init(
            5,
            another.width_of_metalization(),
            another.cornering_code(),
            another.extension_flag(),
            another.justification_flag(),
            another.extension_value(),
        );
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesAppliLineWidening>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _unused_3: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesAppliLineWidening")?;
        writeln!(
            s,
            "Number of property values : {}",
            ent.nb_property_values()
        )?;
        writeln!(
            s,
            "Width of metalization : {}",
            iges_real_text(ent.width_of_metalization())
        )?;
        write!(s, "Cornering Code : ")?;
        if ent.cornering_code() == 0 {
            writeln!(s, "0 (rounded)")?;
        } else {
            if ent.cornering_code() == 1 {
                writeln!(s, "1 (squared)")?;
            } else {
                writeln!(s, "incorrect value")?;
            }
        }
        write!(s, "Extension Flag : ")?;
        if ent.extension_flag() == 0 {
            writeln!(s, "0 (No Extension)")?;
        } else {
            if ent.extension_flag() == 1 {
                writeln!(s, "1 (One-half width extension)")?;
            } else {
                if ent.extension_flag() == 2 {
                    writeln!(s, "2 (Extension set by ExtensionValue)")?;
                } else {
                    writeln!(s, "incorrect value")?;
                }
            }
        }
        write!(s, "Justification Flag : ")?;
        if ent.justification_flag() == 0 {
            writeln!(s, "0 (Centre justified)")?;
        } else {
            if ent.justification_flag() == 1 {
                writeln!(s, "1 (left justified)")?;
            } else {
                if ent.justification_flag() == 2 {
                    writeln!(s, "2 (right justified)")?;
                } else {
                    writeln!(s, "incorrect value")?;
                }
            }
        }
        if ent.extension_flag() == 2 {
            writeln!(
                s,
                "Extension Value : {}",
                iges_real_text(ent.extension_value())
            )?;
        } else {
            writeln!(s, "No Extension Value (Extension Flag != 2)")?;
        }
        Ok(())
    }
}
