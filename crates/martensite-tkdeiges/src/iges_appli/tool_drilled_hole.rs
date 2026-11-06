//! IgesAppliToolDrilledHole.
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
use crate::iges_appli::IgesAppliDrilledHole;
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
    IgesAppliToolDrilledHole,
    "The own-parameter work on `IgesAppliDrilledHole`."
);

impl IgesAppliToolDrilledHole {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesAppliDrilledHole>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_nb_property_values: i32 = Default::default();
        let mut temp_drill_dia_size: f64 = Default::default();
        let mut temp_finish_dia_size: f64 = Default::default();
        let mut temp_plating_flag: i32 = Default::default();
        let mut temp_nb_lower_layer: i32 = Default::default();
        let mut temp_nb_higher_layer: i32 = Default::default();
        pr.read_integer_text(
            &pr.current(),
            "No. of Property values",
            &mut temp_nb_property_values,
        )?;
        pr.read_real_text(
            &pr.current(),
            "Drill diameter size",
            &mut temp_drill_dia_size,
        )?;
        pr.read_real_text(
            &pr.current(),
            "Finish diameter size",
            &mut temp_finish_dia_size,
        )?;
        pr.read_integer_text(&pr.current(), "Plating Flag", &mut temp_plating_flag)?;
        pr.read_integer_text(
            &pr.current(),
            "Lower numbered layer",
            &mut temp_nb_lower_layer,
        )?;
        pr.read_integer_text(
            &pr.current(),
            "Higher numbered layer",
            &mut temp_nb_higher_layer,
        )?;
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            temp_nb_property_values,
            temp_drill_dia_size,
            temp_finish_dia_size,
            temp_plating_flag,
            temp_nb_lower_layer,
            temp_nb_higher_layer,
        );
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesAppliDrilledHole>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.nb_property_values())?;
        iw.send(ent.drill_dia_size())?;
        iw.send(ent.finish_dia_size())?;
        iw.send_boolean(ent.is_plating())?;
        iw.send(ent.nb_lower_layer())?;
        iw.send(ent.nb_higher_layer())?;
        Ok(())
    }
    /// Lists the Entities shared by a DrilledHole `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesAppliDrilledHole>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a DrilledHole
    /// (NbPropertyValues forced to 5, Level cleared if Subordinate != 0)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesAppliDrilledHole>) -> Result<bool, Failure> {
        let mut res: bool = ent.nb_property_values() != 5;
        if res {
            ent.init(
                5,
                ent.drill_dia_size(),
                ent.finish_dia_size(),
                if ent.is_plating() { 1 } else { 0 },
                ent.nb_lower_layer(),
                ent.nb_higher_layer(),
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
    pub fn dir_checker(&self, _unused_0: &Handle<IgesAppliDrilledHole>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 6);
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
        ent: &Handle<IgesAppliDrilledHole>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.subordinate_status() != 0 {
            if (ent.def_level() != (IgesDataDefList::DefOne))
                && (ent.def_level() != (IgesDataDefList::DefSeveral))
            {
                ach.add_fail_text("Level type : Not value/reference", "");
            }
        }
        if ent.nb_property_values() != 5 {
            ach.add_fail_text("Number of Property Values != 5", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesAppliDrilledHole>,
        ent: &Handle<IgesAppliDrilledHole>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        ent.init(
            5,
            another.drill_dia_size(),
            another.finish_dia_size(),
            if another.is_plating() { 1 } else { 0 },
            another.nb_lower_layer(),
            another.nb_higher_layer(),
        );
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesAppliDrilledHole>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _unused_3: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesAppliDrilledHole")?;
        writeln!(
            s,
            "Number of property values : {}",
            ent.nb_property_values()
        )?;
        write!(
            s,
            "Drill  diameter size :{}  ",
            iges_real_text(ent.drill_dia_size())
        )?;
        writeln!(
            s,
            "Finish diameter size : {}",
            iges_real_text(ent.finish_dia_size())
        )?;
        write!(s, "Plating indication flag : ")?;
        if !ent.is_plating() {
            write!(s, "NO  -  ")?;
        } else {
            write!(s, "YES  -  ")?;
        }
        write!(s, "Lower Numbered Layer  : {}  ", ent.nb_lower_layer())?;
        writeln!(s, "Higher Numbered Layer : {}", ent.nb_higher_layer())?;
        Ok(())
    }
}
