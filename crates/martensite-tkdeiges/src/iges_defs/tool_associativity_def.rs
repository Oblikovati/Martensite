//! IgesDefsToolAssociativityDef.
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
use crate::iges_basic::IgesBasicHArray1OfHArray1OfInteger;
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_array1_set,
    iges_dump_vals, iges_live, iges_new_array1,
};
use crate::iges_defs::IgesDefsAssociativityDef;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDefsToolAssociativityDef,
    "The own-parameter work on `IgesDefsAssociativityDef`."
);

impl IgesDefsToolAssociativityDef {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDefsAssociativityDef>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0043!(items, num_items, orders, requirements);
        let mut nbval: i32 = Default::default();
        let mut st: bool =
            pr.read_integer_text(&pr.current(), "No. of Class definitions", &mut nbval)?;
        if st && (nbval > 0) {
            requirements = iges_new_array1::<i32>(1, nbval)?;
            orders = iges_new_array1::<i32>(1, nbval)?;
            num_items = iges_new_array1::<i32>(1, nbval)?;
            items = Handle::new(IgesBasicHArray1OfHArray1OfInteger::with_bounds(1, nbval)?);
        } else {
            pr.add_fail("No. of Class definitions: Not Positive", "");
        }
        if !requirements.is_null() {
            for i in 1..=nbval {
                let mut requirement: i32 = Default::default();
                let mut order: i32 = Default::default();
                let mut num_item: i32 = Default::default();
                let mut item: Handle<HArray1<i32>> = Default::default();
                if pr.read_integer_text(
                    &pr.current(),
                    "Back Pointer Requirement",
                    &mut requirement,
                )? {
                    iges_array1_set(
                        &requirements,
                        i,
                        requirement,
                        "IgesDefsToolAssociativityDef::read_own_params",
                    )?;
                }
                if pr.read_integer_text(&pr.current(), "Ordered/Unordered Class", &mut order)? {
                    iges_array1_set(
                        &orders,
                        i,
                        order,
                        "IgesDefsToolAssociativityDef::read_own_params",
                    )?;
                }
                if pr.read_integer_text(&pr.current(), "No. of items per entry", &mut num_item)? {
                    let mut temp: i32 = Default::default();
                    iges_array1_set(
                        &num_items,
                        i,
                        num_item,
                        "IgesDefsToolAssociativityDef::read_own_params",
                    )?;
                    item = iges_new_array1::<i32>(1, num_item)?;
                    for j in 1..=num_item {
                        if pr.read_integer_text(&pr.current(), "Item", &mut temp)? {
                            iges_array1_set(
                                &item,
                                j,
                                temp,
                                "IgesDefsToolAssociativityDef::read_own_params",
                            )?;
                        }
                    }
                    iges_live(&items, "IgesDefsToolAssociativityDef::read_own_params")?
                        .set_value(i, &item)?;
                }
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(&requirements, &orders, &num_items, &items)?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDefsAssociativityDef>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut upper: i32 = ent.nb_class_defs()?;
        iw.send(upper)?;
        for i in 1..=upper {
            iw.send(ent.back_pointer_req(i)?)?;
            iw.send(ent.class_order(i)?)?;
            iw.send(ent.nb_items_per_class(i)?)?;
            let mut items: i32 = ent.nb_items_per_class(i)?;
            for j in 1..=items {
                iw.send(ent.item(i, j)?)?;
            }
        }
        Ok(())
    }
    /// Lists the Entities shared by a AssociativityDef `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesDefsAssociativityDef>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDefsAssociativityDef>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_forms(302, 5001, 9999);
        crate::iges_shared_blocks::iges_block_0014!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesDefsAssociativityDef>,
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
        another: &Handle<IgesDefsAssociativityDef>,
        ent: &Handle<IgesDefsAssociativityDef>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0043!(items, num_items, orders, requirements);
        let mut nbval: i32 = another.nb_class_defs()?;
        requirements = iges_new_array1::<i32>(1, nbval)?;
        orders = iges_new_array1::<i32>(1, nbval)?;
        num_items = iges_new_array1::<i32>(1, nbval)?;
        items = Handle::new(IgesBasicHArray1OfHArray1OfInteger::with_bounds(1, nbval)?);
        for i in 1..=nbval {
            let mut requirement: i32 = another.back_pointer_req(i)?;
            iges_array1_set(
                &requirements,
                i,
                requirement,
                "IgesDefsToolAssociativityDef::own_copy",
            )?;
            let mut order: i32 = another.class_order(i)?;
            iges_array1_set(&orders, i, order, "IgesDefsToolAssociativityDef::own_copy")?;
            let mut num_item: i32 = another.nb_items_per_class(i)?;
            iges_array1_set(
                &num_items,
                i,
                num_item,
                "IgesDefsToolAssociativityDef::own_copy",
            )?;
            let mut item: Handle<HArray1<i32>> = Default::default();
            item = iges_new_array1::<i32>(1, num_item)?;
            for j in 1..=num_item {
                let mut temp: i32 = another.item(i, j)?;
                iges_array1_set(&item, j, temp, "IgesDefsToolAssociativityDef::own_copy")?;
            }
            iges_live(&items, "IgesDefsToolAssociativityDef::own_copy")?.set_value(i, &item)?;
        }
        ent.init(&requirements, &orders, &num_items, &items)?;
        ent.set_form_number(another.form_number());
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDefsAssociativityDef>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        writeln!(
            s,
            "IgesDefsAssociativityDef\nNumber of Class Definitions : {}\nBack Pointer Requirement  :\nOrdered / Unordered Class :\nNumber Of Items per Entry :\nItems :",
            ent.nb_class_defs()?
        )?;
        iges_dump_vals(s, -level, 1, ent.nb_class_defs()?, |first| {
            ent.back_pointer_req(first)
        })?;
        writeln!(s)?;
        if level > 4 {
            let mut upper: i32 = ent.nb_class_defs()?;
            for i in 1..=upper {
                writeln!(s, "[{}]:", i)?;
                write!(
                    s,
                    "Back Pointer Requirement : {}  ",
                    ent.back_pointer_req(i)?
                )?;
                if ent.is_back_pointer_req(i)? {
                    write!(s, "(Yes)  ")?;
                } else {
                    write!(s, "(No)   ")?;
                }
                write!(s, " Ordered/Unordered Class : {} ", ent.class_order(i)?)?;
                if ent.is_ordered(i)? {
                    writeln!(s, "(Yes)")?;
                } else {
                    writeln!(s, "(No)")?;
                }
                write!(
                    s,
                    "Number Of Items per Entry : {}",
                    ent.nb_items_per_class(i)?
                )?;
                if level < 6 {
                    writeln!(s, " [ask level > 5 for more]")?;
                    continue;
                }
                write!(s, "\n [")?;
                {
                    let mut j: i32 = 1;
                    while j <= ent.nb_items_per_class(i)? {
                        write!(s, "  {}", ent.item(i, j)?)?;
                        j += 1;
                    }
                }
                writeln!(s, "]")?;
            }
        }
        writeln!(s)?;
        Ok(())
    }
}
