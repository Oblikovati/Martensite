//! IgesDefsToolAttributeTable.
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
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesEntity, IgesDataIgesReaderData,
    IgesDataIgesWriter, IgesDataParamReader, iges_array1_set, iges_array2_set, iges_live,
    iges_new_array1, iges_new_array2,
};
use crate::iges_defs::{IgesDefsAttributeDef, IgesDefsAttributeTable};
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::InterfaceEntityIterator;

iges_tool!(
    IgesDefsToolAttributeTable,
    "The own-parameter work on `IgesDefsAttributeTable`."
);

impl IgesDefsToolAttributeTable {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDefsAttributeTable>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut nr: i32 = 1;
        let mut j: i32 = Default::default();
        let mut st: bool = true;
        let mut ab: Handle<IgesDefsAttributeDef> = ent.definition();
        let mut list2: Handle<HArray2<Handle<StandardTransient>>> = Default::default();
        if ab.is_null() {
            pr.add_fail("No Attribute Definition as Structure", "");
            return Ok(());
        }
        let mut na: i32 =
            iges_live(&ab, "IgesDefsToolAttributeTable::read_own_params")?.nb_attributes()?;
        if ent.form_number() == 1 {
            st = pr.read_integer_text(&pr.current(), "No. of rows", &mut nr)?;
        }
        if st {
            list2 = iges_new_array2::<Handle<StandardTransient>>(1, na, 1, nr)?;
        }
        for k in 1..=nr {
            for i in 1..=na {
                let mut avc: i32 = iges_live(&ab, "IgesDefsToolAttributeTable::read_own_params")?
                    .attribute_value_count(i)?;
                let mut atype: i32 = iges_live(&ab, "IgesDefsToolAttributeTable::read_own_params")?
                    .attribute_value_data_type(i)?;
                match atype {
                    0 => {
                        for j in 1..=avc {
                            pr.set_current_number(pr.current_number() + 1);
                        }
                    }
                    1 => {
                        let mut attr_int: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, avc)?;
                        iges_array2_set(
                            &list2,
                            i,
                            k,
                            attr_int.cast(),
                            "IgesDefsToolAttributeTable::read_own_params",
                        )?;
                        let mut item: i32 = Default::default();
                        for j in 1..=avc {
                            if pr.read_integer_text(&pr.current(), "Value", &mut item)? {
                                iges_array1_set(
                                    &attr_int,
                                    j,
                                    item,
                                    "IgesDefsToolAttributeTable::read_own_params",
                                )?;
                            }
                        }
                    }
                    2 => {
                        let mut attr_real: Handle<HArray1<f64>> = iges_new_array1::<f64>(1, avc)?;
                        iges_array2_set(
                            &list2,
                            i,
                            k,
                            attr_real.cast(),
                            "IgesDefsToolAttributeTable::read_own_params",
                        )?;
                        let mut item: f64 = Default::default();
                        for j in 1..=avc {
                            if pr.read_real_text(&pr.current(), "Value", &mut item)? {
                                iges_array1_set(
                                    &attr_real,
                                    j,
                                    item,
                                    "IgesDefsToolAttributeTable::read_own_params",
                                )?;
                            }
                        }
                    }
                    3 => {
                        let mut attr_str: Handle<HArray1<Handle<HAsciiString>>> =
                            iges_new_array1::<Handle<HAsciiString>>(1, avc)?;
                        iges_array2_set(
                            &list2,
                            i,
                            k,
                            attr_str.cast(),
                            "IgesDefsToolAttributeTable::read_own_params",
                        )?;
                        let mut item: Handle<HAsciiString> = Default::default();
                        for j in 1..=avc {
                            if pr.read_text(&pr.current(), "Value", &mut item)? {
                                iges_array1_set(
                                    &attr_str,
                                    j,
                                    item.clone(),
                                    "IgesDefsToolAttributeTable::read_own_params",
                                )?;
                            }
                        }
                    }
                    4 => {
                        let mut attr_ent: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
                            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, avc)?;
                        iges_array2_set(
                            &list2,
                            i,
                            k,
                            attr_ent.cast(),
                            "IgesDefsToolAttributeTable::read_own_params",
                        )?;
                        let mut item: Handle<IgesDataIgesEntity> = Default::default();
                        for j in 1..=avc {
                            if pr.read_entity(ir, &pr.current(), "Value", &mut item, false)? {
                                iges_array1_set(
                                    &attr_ent,
                                    j,
                                    item.clone(),
                                    "IgesDefsToolAttributeTable::read_own_params",
                                )?;
                            }
                        }
                    }
                    5 => {
                        for j in 1..=avc {
                            pr.set_current_number(pr.current_number() + 1);
                        }
                    }
                    6 => {
                        let mut attr_int: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, avc)?;
                        iges_array2_set(
                            &list2,
                            i,
                            k,
                            attr_int.cast(),
                            "IgesDefsToolAttributeTable::read_own_params",
                        )?;
                        let mut item: i32 = Default::default();
                        for j in 1..=avc {
                            if pr.read_integer_text(&pr.current(), "Value", &mut item)? {
                                iges_array1_set(
                                    &attr_int,
                                    j,
                                    item,
                                    "IgesDefsToolAttributeTable::read_own_params",
                                )?;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(&list2)?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDefsAttributeTable>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut ab: Handle<IgesDefsAttributeDef> = ent.definition();
        let mut nr: i32 = ent.nb_rows()?;
        let mut na: i32 = ent.nb_attributes()?;
        if ent.form_number() == 1 {
            iw.send(nr)?;
        }
        for k in 1..=nr {
            for i in 1..=na {
                let mut count: i32 =
                    iges_live(&ab, "IgesDefsToolAttributeTable::write_own_params")?
                        .attribute_value_count(i)?;
                for j in 1..=count {
                    match iges_live(&ab, "IgesDefsToolAttributeTable::write_own_params")?
                        .attribute_value_data_type(i)?
                    {
                        0 => {
                            iw.send_void()?;
                        }
                        1 => {
                            iw.send(ent.attribute_as_integer(i, k, j)?)?;
                        }
                        2 => {
                            iw.send(ent.attribute_as_real(i, k, j)?)?;
                        }
                        3 => {
                            iw.send(ent.attribute_as_string(i, k, j)?)?;
                        }
                        4 => {
                            iw.send_entity(&ent.attribute_as_entity(i, k, j)?, false)?;
                        }
                        5 => {
                            iw.send_void()?;
                        }
                        6 => {
                            iw.send_boolean(ent.attribute_as_logical(i, k, j)?)?;
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
    /// Lists the Entities shared by a AttributeTable `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDefsAttributeTable>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let mut ab: Handle<IgesDefsAttributeDef> = ent.definition();
        let mut na: i32 = ent.nb_attributes()?;
        let mut nr: i32 = ent.nb_rows()?;
        for k in 1..=nr {
            for i in 1..=na {
                if (iges_live(&ab, "IgesDefsToolAttributeTable::own_shared")?
                    .attribute_value_data_type(i)?)
                    != 4
                {
                    continue;
                }
                let mut avc: i32 = iges_live(&ab, "IgesDefsToolAttributeTable::own_shared")?
                    .attribute_value_count(i)?;
                for j in 1..=avc {
                    iter.get_one_item(&ent.attribute_as_entity(i, k, j)?.cast());
                }
            }
        }
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDefsAttributeTable>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_forms(422, 0, 1);
        dc.structure(IgesDataDefType::DefReference);
        dc.graphics_ignored(-1);
        dc.blank_status_ignored();
        dc.hierarchy_status_ignored();
        dc
    }
}
