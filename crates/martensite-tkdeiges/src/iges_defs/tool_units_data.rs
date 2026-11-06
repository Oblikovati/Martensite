//! IgesDefsToolUnitsData.
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
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_array1_set,
    iges_dump_string, iges_dump_strings, iges_live, iges_new_array1, iges_real_text,
};
use crate::iges_data::{iges_read_real_item, iges_read_text_item};
use crate::iges_defs::IgesDefsUnitsData;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDefsToolUnitsData,
    "The own-parameter work on `IgesDefsUnitsData`."
);

impl IgesDefsToolUnitsData {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDefsUnitsData>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut nbval: i32 = Default::default();
        crate::iges_shared_blocks::iges_block_0054!(unit_scales, unit_types, unit_values);
        let mut st: bool = pr.read_integer_text(&pr.current(), "Number of Units", &mut nbval)?;
        if st && (nbval > 0) {
            unit_types = iges_new_array1::<Handle<HAsciiString>>(1, nbval)?;
            unit_values = iges_new_array1::<Handle<HAsciiString>>(1, nbval)?;
            unit_scales = iges_new_array1::<f64>(1, nbval)?;
        } else {
            pr.add_fail("Number of Units: Less than or Equal or zero", "");
        }
        if !unit_types.is_null() {
            for i in 1..=nbval {
                iges_read_text_item(
                    pr,
                    "Type of Unit",
                    &unit_types,
                    i,
                    "IgesDefsToolUnitsData::read_own_params",
                )?;
                iges_read_text_item(
                    pr,
                    "Value of Unit",
                    &unit_values,
                    i,
                    "IgesDefsToolUnitsData::read_own_params",
                )?;
                iges_read_real_item(
                    pr,
                    "Scale of Unit",
                    &unit_scales,
                    i,
                    "IgesDefsToolUnitsData::read_own_params",
                )?;
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(&unit_types, &unit_values, &unit_scales)?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDefsUnitsData>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut upper: i32 = ent.nb_units()?;
        iw.send(upper)?;
        for i in 1..=upper {
            iw.send(ent.unit_type(i)?)?;
            iw.send(ent.unit_value(i)?)?;
            iw.send(ent.scale_factor(i)?)?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a UnitsData `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesDefsUnitsData>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDefsUnitsData>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(316, 0);
        crate::iges_shared_blocks::iges_block_0014!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesDefsUnitsData>,
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
        another: &Handle<IgesDefsUnitsData>,
        ent: &Handle<IgesDefsUnitsData>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0054!(unit_scales, unit_types, unit_values);
        let mut nbval: i32 = another.nb_units()?;
        unit_types = iges_new_array1::<Handle<HAsciiString>>(1, nbval)?;
        unit_values = iges_new_array1::<Handle<HAsciiString>>(1, nbval)?;
        unit_scales = iges_new_array1::<f64>(1, nbval)?;
        for i in 1..=nbval {
            let mut unit_type: Handle<HAsciiString> = Handle::new(HAsciiString::copied(iges_live(
                &another.unit_type(i)?,
                "IgesDefsToolUnitsData::own_copy",
            )?));
            iges_array1_set(
                &unit_types,
                i,
                unit_type.clone(),
                "IgesDefsToolUnitsData::own_copy",
            )?;
            let mut unit_value: Handle<HAsciiString> = Handle::new(HAsciiString::copied(
                iges_live(&another.unit_value(i)?, "IgesDefsToolUnitsData::own_copy")?,
            ));
            iges_array1_set(
                &unit_values,
                i,
                unit_value.clone(),
                "IgesDefsToolUnitsData::own_copy",
            )?;
            let mut unit_scale: f64 = another.scale_factor(i)?;
            iges_array1_set(
                &unit_scales,
                i,
                unit_scale,
                "IgesDefsToolUnitsData::own_copy",
            )?;
        }
        ent.init(&unit_types, &unit_values, &unit_scales)?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDefsUnitsData>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        writeln!(
            s,
            "IgesDefsUnitsData\nNumber of Units : {}\nType of Unit :\nValue of Unit :\nScale Factor :",
            ent.nb_units()?
        )?;
        iges_dump_strings(s, -level, 1, ent.nb_units()?, |first| ent.unit_type(first))?;
        writeln!(s)?;
        if level > 4 {
            writeln!(s, "Details of the Units")?;
            let mut upper: i32 = ent.nb_units()?;
            for i in 1..=upper {
                write!(s, "[{}] Type  : ", i)?;
                iges_dump_string(s, &ent.unit_type(i)?)?;
                write!(s, "\n     Value : ")?;
                iges_dump_string(s, &ent.unit_value(i)?)?;
                writeln!(
                    s,
                    "\n     ScaleFactor: {}",
                    iges_real_text(ent.scale_factor(i)?)
                )?;
            }
        }
        writeln!(s)?;
        Ok(())
    }
}
