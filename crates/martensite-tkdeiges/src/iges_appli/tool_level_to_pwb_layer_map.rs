//! IgesAppliToolLevelToPWBLayerMap.
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
use crate::iges_appli::IgesAppliLevelToPWBLayerMap;
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_array1_set,
    iges_dump_string, iges_dump_strings, iges_live, iges_new_array1,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesAppliToolLevelToPWBLayerMap,
    "The own-parameter work on `IgesAppliLevelToPWBLayerMap`."
);

impl IgesAppliToolLevelToPWBLayerMap {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesAppliLevelToPWBLayerMap>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut num: i32 = Default::default();
        let mut i: i32 = Default::default();
        let mut temp_nb_property_values: i32 = Default::default();
        let mut temp_exchange_file_level_number: Handle<HArray1<i32>> = Default::default();
        let mut temp_native_level: Handle<HArray1<Handle<HAsciiString>>> = Default::default();
        let mut temp_physical_layer_number: Handle<HArray1<i32>> = Default::default();
        let mut temp_exchange_file_level_ident: Handle<HArray1<Handle<HAsciiString>>> =
            Default::default();
        pr.read_integer_text(
            &pr.current(),
            "Number of property values",
            &mut temp_nb_property_values,
        )?;
        if !(pr.read_integer_text(&pr.current(), "Number of definitions", &mut num)?) {
            num = 0;
        }
        if num > 0 {
            temp_exchange_file_level_number = iges_new_array1::<i32>(1, num)?;
            temp_native_level = iges_new_array1::<Handle<HAsciiString>>(1, num)?;
            temp_physical_layer_number = iges_new_array1::<i32>(1, num)?;
            temp_exchange_file_level_ident = iges_new_array1::<Handle<HAsciiString>>(1, num)?;
        } else {
            pr.add_fail("Number of definitions: Not Positive", "");
        }
        if (((!temp_exchange_file_level_number.is_null()) && (!temp_native_level.is_null()))
            && (!temp_physical_layer_number.is_null()))
            && (!temp_exchange_file_level_ident.is_null())
        {
            for i in 1..=num {
                let mut temp_efln: i32 = Default::default();
                if pr.read_integer_text(
                    &pr.current(),
                    "Exchange File Level Number",
                    &mut temp_efln,
                )? {
                    iges_array1_set(
                        &temp_exchange_file_level_number,
                        i,
                        temp_efln,
                        "IgesAppliToolLevelToPWBLayerMap::read_own_params",
                    )?;
                }
                let mut temp_nl: Handle<HAsciiString> = Default::default();
                if pr.read_text(&pr.current(), "Native Level Identification", &mut temp_nl)? {
                    iges_array1_set(
                        &temp_native_level,
                        i,
                        temp_nl.clone(),
                        "IgesAppliToolLevelToPWBLayerMap::read_own_params",
                    )?;
                }
                let mut temp_pln: i32 = Default::default();
                if pr.read_integer_text(&pr.current(), "Physical Layer Number", &mut temp_pln)? {
                    iges_array1_set(
                        &temp_physical_layer_number,
                        i,
                        temp_pln,
                        "IgesAppliToolLevelToPWBLayerMap::read_own_params",
                    )?;
                }
                let mut temp_efli: Handle<HAsciiString> = Default::default();
                if pr.read_text(
                    &pr.current(),
                    "Exchange File Level Identification",
                    &mut temp_efli,
                )? {
                    iges_array1_set(
                        &temp_exchange_file_level_ident,
                        i,
                        temp_efli.clone(),
                        "IgesAppliToolLevelToPWBLayerMap::read_own_params",
                    )?;
                }
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            temp_nb_property_values,
            &temp_exchange_file_level_number,
            &temp_native_level,
            &temp_physical_layer_number,
            &temp_exchange_file_level_ident,
        )?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesAppliLevelToPWBLayerMap>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut num: i32 = Default::default();
        iw.send(ent.nb_property_values())?;
        iw.send(ent.nb_level_to_layer_defs()?)?;
        {
            num = ent.nb_level_to_layer_defs()?;
            i = 1;
            while i <= num {
                iw.send(ent.exchange_file_level_number(i)?)?;
                iw.send(ent.native_level(i)?)?;
                iw.send(ent.physical_layer_number(i)?)?;
                iw.send(ent.exchange_file_level_ident(i)?)?;
                i += 1;
            }
        }
        Ok(())
    }
    /// Lists the Entities shared by a LevelToPWBLayerMap `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesAppliLevelToPWBLayerMap>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(
        &self,
        _unused_0: &Handle<IgesAppliLevelToPWBLayerMap>,
    ) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 24);
        crate::iges_shared_blocks::iges_block_0011!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesAppliLevelToPWBLayerMap>,
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
        another: &Handle<IgesAppliLevelToPWBLayerMap>,
        ent: &Handle<IgesAppliLevelToPWBLayerMap>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut temp_nb_property_values: i32 = another.nb_property_values();
        let mut num: i32 = another.nb_level_to_layer_defs()?;
        let mut temp_exchange_file_level_number: Handle<HArray1<i32>> =
            iges_new_array1::<i32>(1, num)?;
        let mut temp_native_level: Handle<HArray1<Handle<HAsciiString>>> =
            iges_new_array1::<Handle<HAsciiString>>(1, num)?;
        let mut temp_physical_layer_number: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, num)?;
        let mut temp_exchange_file_level_ident: Handle<HArray1<Handle<HAsciiString>>> =
            iges_new_array1::<Handle<HAsciiString>>(1, num)?;
        for i in 1..=num {
            iges_array1_set(
                &temp_exchange_file_level_number,
                i,
                another.exchange_file_level_number(i)?,
                "IgesAppliToolLevelToPWBLayerMap::own_copy",
            )?;
            iges_array1_set(
                &temp_native_level,
                i,
                Handle::new(HAsciiString::copied(iges_live(
                    &another.native_level(i)?,
                    "IgesAppliToolLevelToPWBLayerMap::own_copy",
                )?)),
                "IgesAppliToolLevelToPWBLayerMap::own_copy",
            )?;
            iges_array1_set(
                &temp_physical_layer_number,
                i,
                another.physical_layer_number(i)?,
                "IgesAppliToolLevelToPWBLayerMap::own_copy",
            )?;
            iges_array1_set(
                &temp_exchange_file_level_ident,
                i,
                Handle::new(HAsciiString::copied(iges_live(
                    &another.exchange_file_level_ident(i)?,
                    "IgesAppliToolLevelToPWBLayerMap::own_copy",
                )?)),
                "IgesAppliToolLevelToPWBLayerMap::own_copy",
            )?;
        }
        ent.init(
            temp_nb_property_values,
            &temp_exchange_file_level_number,
            &temp_native_level,
            &temp_physical_layer_number,
            &temp_exchange_file_level_ident,
        )?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesAppliLevelToPWBLayerMap>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut i: i32 = Default::default();
        let mut num: i32 = Default::default();
        writeln!(s, "IgesAppliLevelToPWBLayerMap")?;
        writeln!(
            s,
            "Number of property values : {}",
            ent.nb_property_values()
        )?;
        writeln!(s, "Exchange File Level Number :")?;
        writeln!(s, "Native Level Identification :")?;
        writeln!(s, "Physical Layer Number :")?;
        write!(s, "Exchange File Level Identification : ")?;
        iges_dump_strings(s, -level, 1, ent.nb_level_to_layer_defs()?, |first| {
            ent.exchange_file_level_ident(first)
        })?;
        writeln!(s)?;
        if level > 4 {
            {
                num = ent.nb_level_to_layer_defs()?;
                i = 1;
                while i <= num {
                    writeln!(s, "[{}]:", i)?;
                    writeln!(
                        s,
                        "Exchange File Level Number : {}",
                        ent.exchange_file_level_number(i)?
                    )?;
                    write!(s, "Native Level Identification : ")?;
                    iges_dump_string(s, &ent.native_level(i)?)?;
                    writeln!(s)?;
                    writeln!(
                        s,
                        "Physical Layer Number : {}",
                        ent.physical_layer_number(i)?
                    )?;
                    write!(s, "Exchange File Level Identification : ")?;
                    iges_dump_string(s, &ent.exchange_file_level_ident(i)?)?;
                    writeln!(s)?;
                    i += 1;
                }
            }
        }
        Ok(())
    }
}
