//! IgesDefsToolGenericData.
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
    IgesDataIgesEntity, IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader,
    iges_array1_set, iges_dump_string, iges_live, iges_new_array1, iges_real_text,
};
use crate::iges_defs::IgesDefsGenericData;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDefsToolGenericData,
    "The own-parameter work on `IgesDefsGenericData`."
);

impl IgesDefsToolGenericData {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDefsGenericData>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut num: i32 = Default::default();
        let mut temp_nb_prop_val: i32 = Default::default();
        let mut temp_name: Handle<HAsciiString> = Default::default();
        let mut temp_types: Handle<HArray1<i32>> = Default::default();
        let mut temp_values: Handle<HArray1<Handle<StandardTransient>>> = Default::default();
        pr.read_integer_text(
            &pr.current(),
            "Number of property values",
            &mut temp_nb_prop_val,
        )?;
        pr.read_text(&pr.current(), "Property Name", &mut temp_name)?;
        let mut st: bool =
            pr.read_integer_text(&pr.current(), "Number of TYPE/VALUEs", &mut num)?;
        if st && (num > 0) {
            temp_types = iges_new_array1::<i32>(1, num)?;
            temp_values = iges_new_array1::<Handle<StandardTransient>>(1, num)?;
        } else {
            pr.add_fail("Number of TYPE/VALUEs: Not Positive", "");
        }
        if (!temp_types.is_null()) && (!temp_values.is_null()) {
            for i in 1..=num {
                let mut temp_typ: i32 = Default::default();
                pr.read_integer_text(&pr.current(), "Type code", &mut temp_typ)?;
                iges_array1_set(
                    &temp_types,
                    i,
                    temp_typ,
                    "IgesDefsToolGenericData::read_own_params",
                )?;
                match temp_typ {
                    0 => {
                        pr.set_current_number(pr.current_number() + 1);
                    }
                    1 => {
                        let mut temp_obj: Handle<HArray1<i32>> = Default::default();
                        if pr.read_ints(
                            &pr.current_list(1, 1),
                            "Integer value",
                            &mut temp_obj,
                            1,
                        )? {
                            iges_array1_set(
                                &temp_values,
                                i,
                                temp_obj.cast(),
                                "IgesDefsToolGenericData::read_own_params",
                            )?;
                        }
                    }
                    2 => {
                        let mut temp_obj: Handle<HArray1<f64>> = Default::default();
                        if pr.read_reals(&pr.current_list(1, 1), "Real value", &mut temp_obj, 1)? {
                            iges_array1_set(
                                &temp_values,
                                i,
                                temp_obj.cast(),
                                "IgesDefsToolGenericData::read_own_params",
                            )?;
                        }
                    }
                    3 => {
                        let mut temp_obj: Handle<HAsciiString> = Default::default();
                        if pr.read_text(&pr.current(), "String value", &mut temp_obj)? {
                            iges_array1_set(
                                &temp_values,
                                i,
                                temp_obj.cast(),
                                "IgesDefsToolGenericData::read_own_params",
                            )?;
                        }
                    }
                    4 => {
                        let mut temp_entity: Handle<IgesDataIgesEntity> = Default::default();
                        if pr.read_entity(
                            ir,
                            &pr.current(),
                            "Entity value",
                            &mut temp_entity,
                            false,
                        )? {
                            iges_array1_set(
                                &temp_values,
                                i,
                                temp_entity.cast(),
                                "IgesDefsToolGenericData::read_own_params",
                            )?;
                        }
                    }
                    5 => {
                        pr.set_current_number(pr.current_number() + 1);
                    }
                    6 => {
                        let mut temp_obj: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, 1)?;
                        let mut temp_bool: bool = Default::default();
                        if pr.read_boolean(&pr.current(), "Boolean value", &mut temp_bool, true)? {
                            iges_array1_set(
                                &temp_obj,
                                1,
                                if temp_bool { 1 } else { 0 },
                                "IgesDefsToolGenericData::read_own_params",
                            )?;
                            iges_array1_set(
                                &temp_values,
                                i,
                                temp_obj.cast(),
                                "IgesDefsToolGenericData::read_own_params",
                            )?;
                        }
                    }
                    _ => {}
                }
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(temp_nb_prop_val, &temp_name, &temp_types, &temp_values)?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDefsGenericData>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut num: i32 = Default::default();
        iw.send(ent.nb_property_values())?;
        iw.send(ent.name())?;
        iw.send(ent.nb_type_value_pairs()?)?;
        {
            num = ent.nb_type_value_pairs()?;
            i = 1;
            while i <= num {
                iw.send(ent.type_(i)?)?;
                match ent.type_(i)? {
                    0 => {
                        iw.send_void()?;
                    }
                    1 => {
                        iw.send(ent.value_as_integer(i)?)?;
                    }
                    2 => {
                        iw.send(ent.value_as_real(i)?)?;
                    }
                    3 => {
                        iw.send(ent.value_as_string(i)?)?;
                    }
                    4 => {
                        iw.send_entity(&ent.value_as_entity(i)?, false)?;
                    }
                    5 => {
                        iw.send_void()?;
                    }
                    6 => {
                        iw.send_boolean(ent.value_as_logical(i)?)?;
                    }
                    _ => {}
                }
                i += 1;
            }
        }
        Ok(())
    }
    /// Lists the Entities shared by a GenericData `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDefsGenericData>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut num: i32 = Default::default();
        {
            num = ent.nb_type_value_pairs()?;
            i = 1;
            while i <= num {
                if ent.type_(i)? == 4 {
                    iter.get_one_item(&ent.value_as_entity(i)?.cast());
                }
                i += 1;
            }
        }
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDefsGenericData>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 27);
        crate::iges_shared_blocks::iges_block_0006!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDefsGenericData>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.nb_property_values() != ((ent.nb_type_value_pairs()? * 2) + 2) {
            ach.add_fail_text(
                "Nb. of Property Values not consistent with Nb. of Type/value Pairs",
                "",
            );
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDefsGenericData>,
        ent: &Handle<IgesDefsGenericData>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut num: i32 = another.nb_type_value_pairs()?;
        let mut temp_nb_prop_val: i32 = another.nb_property_values();
        let mut temp_name: Handle<HAsciiString> = Handle::new(HAsciiString::copied(iges_live(
            &another.name(),
            "IgesDefsToolGenericData::own_copy",
        )?));
        let mut temp_types: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, num)?;
        let mut temp_values: Handle<HArray1<Handle<StandardTransient>>> =
            iges_new_array1::<Handle<StandardTransient>>(1, num)?;
        for i in 1..=num {
            iges_array1_set(
                &temp_types,
                i,
                another.type_(i)?,
                "IgesDefsToolGenericData::own_copy",
            )?;
            match another.type_(i)? {
                0 => {}
                1 => {
                    let mut temp_obj: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, 1)?;
                    iges_array1_set(
                        &temp_obj,
                        1,
                        another.value_as_integer(i)?,
                        "IgesDefsToolGenericData::own_copy",
                    )?;
                    iges_array1_set(
                        &temp_values,
                        i,
                        temp_obj.cast(),
                        "IgesDefsToolGenericData::own_copy",
                    )?;
                }
                2 => {
                    let mut temp_obj: Handle<HArray1<f64>> = iges_new_array1::<f64>(1, 1)?;
                    iges_array1_set(
                        &temp_obj,
                        1,
                        another.value_as_real(i)?,
                        "IgesDefsToolGenericData::own_copy",
                    )?;
                    iges_array1_set(
                        &temp_values,
                        i,
                        temp_obj.cast(),
                        "IgesDefsToolGenericData::own_copy",
                    )?;
                }
                3 => {
                    iges_array1_set(
                        &temp_values,
                        i,
                        Handle::new(HAsciiString::copied(iges_live(
                            &another.value_as_string(i)?,
                            "IgesDefsToolGenericData::own_copy",
                        )?))
                        .cast(),
                        "IgesDefsToolGenericData::own_copy",
                    )?;
                }
                4 => {
                    let mut temp_obj: Handle<IgesDataIgesEntity> =
                        tc.transferred(&another.value_as_entity(i)?.cast())?.cast();
                    iges_array1_set(
                        &temp_values,
                        i,
                        temp_obj.cast(),
                        "IgesDefsToolGenericData::own_copy",
                    )?;
                }
                5 => {}
                6 => {
                    let mut temp_obj: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, 1)?;
                    iges_array1_set(
                        &temp_obj,
                        1,
                        if another.value_as_logical(i)? { 1 } else { 0 },
                        "IgesDefsToolGenericData::own_copy",
                    )?;
                    iges_array1_set(
                        &temp_values,
                        i,
                        temp_obj.cast(),
                        "IgesDefsToolGenericData::own_copy",
                    )?;
                }
                _ => {}
            }
        }
        ent.init(temp_nb_prop_val, &temp_name, &temp_types, &temp_values)?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDefsGenericData>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        write!(
            s,
            "IgesDefsGenericData\nNumber of property values : {}\nProperty Name : ",
            ent.nb_property_values()
        )?;
        iges_dump_string(s, &ent.name())?;
        writeln!(s)?;
        match level {
            4 => {
                writeln!(s, "Types  :")?;
                writeln!(s, "Values : Count = {}", ent.nb_type_value_pairs()?)?;
                writeln!(s, "      [ as level > 4 for content ]")?;
            }
            5 | 6 => {
                let mut i: i32 = Default::default();
                let mut num: i32 = Default::default();
                writeln!(s, "Types & Values : ")?;
                {
                    num = ent.nb_type_value_pairs()?;
                    i = 1;
                    while i <= num {
                        write!(s, "[{}]: ", i)?;
                        write!(s, "Type : {}", ent.type_(i)?)?;
                        match ent.type_(i)? {
                            0 => {
                                write!(s, "  (Void)")?;
                            }
                            1 => {
                                write!(s, "  Integer, Value : {}", ent.value_as_integer(i)?)?;
                            }
                            2 => {
                                write!(
                                    s,
                                    "  Real   , Value : {}",
                                    iges_real_text(ent.value_as_real(i)?)
                                )?;
                            }
                            3 => {
                                write!(s, "  String , Value : ")?;
                                iges_dump_string(s, &ent.value_as_string(i)?)?;
                            }
                            4 => {
                                write!(s, "  Entity , Value : ")?;
                                dumper.dump(&ent.value_as_entity(i)?, s, level - 1, -1)?;
                            }
                            5 => {
                                write!(s, " (Not used)")?;
                            }
                            6 => {
                                write!(
                                    s,
                                    "  Logical, Value : {}",
                                    if ent.value_as_logical(i)? {
                                        "True"
                                    } else {
                                        "False"
                                    }
                                )?;
                            }
                            _ => {}
                        }
                        writeln!(s)?;
                        i += 1;
                    }
                }
            }
            _ => {}
        }
        writeln!(s)?;
        Ok(())
    }
}
