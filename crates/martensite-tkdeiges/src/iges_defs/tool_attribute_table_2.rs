//! More of IgesDefsToolAttributeTable.
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
use crate::iges_data::{
    IgesDataIgesEntity, iges_array1_item, iges_array1_set, iges_array2_set, iges_live,
    iges_new_array1, iges_new_array2,
};
use crate::iges_defs::{IgesDefsAttributeDef, IgesDefsAttributeTable, IgesDefsToolAttributeTable};
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{InterfaceCheck, InterfaceCopyTool, InterfaceShareTool};

impl IgesDefsToolAttributeTable {
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDefsAttributeTable>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.definition().is_null() {
            if ent.has_structure() {
                ach.add_fail_text(
                    "Structure in Directory Entry is not an Attribute Definition Table",
                    "",
                );
            } else {
                ach.add_fail_text("No Attribute Definition defined", "");
            }
        }
        if (ent.form_number() == 0) && (ent.nb_rows()? != 1) {
            ach.add_fail_text("Form 0 with several Rows", "");
        }
        if ent.nb_attributes()?
            != (iges_live(&ent.definition(), "IgesDefsToolAttributeTable::own_check")?
                .nb_attributes()?)
        {
            ach.add_fail_text("Mismatch between Definition (Structure) and Content", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDefsAttributeTable>,
        ent: &Handle<IgesDefsAttributeTable>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut j: i32 = 1;
        let mut ab: Handle<IgesDefsAttributeDef> = another.definition();
        let mut na: i32 = another.nb_attributes()?;
        let mut nr: i32 = another.nb_rows()?;
        let mut list2: Handle<HArray2<Handle<StandardTransient>>> =
            iges_new_array2::<Handle<StandardTransient>>(1, na, 1, nr)?;
        for k in 1..=nr {
            for i in 1..=na {
                let mut avc: i32 = iges_live(&ab, "IgesDefsToolAttributeTable::own_copy")?
                    .attribute_value_count(i)?;
                let mut atype: i32 = iges_live(&ab, "IgesDefsToolAttributeTable::own_copy")?
                    .attribute_value_data_type(i)?;
                match atype {
                    0 => {}
                    1 => {
                        let mut other_int: Handle<HArray1<i32>> =
                            another.attribute_list(i, k)?.cast();
                        let mut attr_int: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, avc)?;
                        iges_array2_set(
                            &list2,
                            i,
                            k,
                            attr_int.cast(),
                            "IgesDefsToolAttributeTable::own_copy",
                        )?;
                        for j in 1..=avc {
                            iges_array1_set(
                                &attr_int,
                                j,
                                iges_array1_item(
                                    &other_int,
                                    j,
                                    "IgesDefsToolAttributeTable::own_copy",
                                )?,
                                "IgesDefsToolAttributeTable::own_copy",
                            )?;
                        }
                    }
                    2 => {
                        let mut other_real: Handle<HArray1<f64>> =
                            another.attribute_list(i, k)?.cast();
                        let mut attr_real: Handle<HArray1<f64>> = iges_new_array1::<f64>(1, avc)?;
                        iges_array2_set(
                            &list2,
                            i,
                            k,
                            attr_real.cast(),
                            "IgesDefsToolAttributeTable::own_copy",
                        )?;
                        for j in 1..=avc {
                            iges_array1_set(
                                &attr_real,
                                j,
                                iges_array1_item(
                                    &other_real,
                                    j,
                                    "IgesDefsToolAttributeTable::own_copy",
                                )?,
                                "IgesDefsToolAttributeTable::own_copy",
                            )?;
                        }
                    }
                    3 => {
                        let mut other_str: Handle<HArray1<Handle<HAsciiString>>> =
                            another.attribute_list(i, k)?.cast();
                        let mut attr_str: Handle<HArray1<Handle<HAsciiString>>> =
                            iges_new_array1::<Handle<HAsciiString>>(1, avc)?;
                        iges_array2_set(
                            &list2,
                            i,
                            k,
                            attr_str.cast(),
                            "IgesDefsToolAttributeTable::own_copy",
                        )?;
                        for j in 1..=avc {
                            iges_array1_set(
                                &attr_str,
                                j,
                                Handle::new(HAsciiString::copied(iges_live(
                                    &iges_array1_item(
                                        &other_str,
                                        j,
                                        "IgesDefsToolAttributeTable::own_copy",
                                    )?,
                                    "IgesDefsToolAttributeTable::own_copy",
                                )?)),
                                "IgesDefsToolAttributeTable::own_copy",
                            )?;
                        }
                    }
                    4 => {
                        let mut other_ent: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
                            another.attribute_list(i, k)?.cast();
                        let mut attr_ent: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
                            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, avc)?;
                        iges_array2_set(
                            &list2,
                            i,
                            k,
                            attr_ent.cast(),
                            "IgesDefsToolAttributeTable::own_copy",
                        )?;
                        for j in 1..=avc {
                            iges_array1_set(
                                &attr_ent,
                                j,
                                tc.transferred(
                                    &iges_array1_item(
                                        &other_ent,
                                        j,
                                        "IgesDefsToolAttributeTable::own_copy",
                                    )?
                                    .cast(),
                                )?
                                .cast::<IgesDataIgesEntity>(),
                                "IgesDefsToolAttributeTable::own_copy",
                            )?;
                        }
                    }
                    5 => {}
                    6 => {
                        let mut other_int: Handle<HArray1<i32>> =
                            another.attribute_list(i, k)?.cast();
                        let mut attr_int: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, avc)?;
                        iges_array2_set(
                            &list2,
                            i,
                            k,
                            attr_int.cast(),
                            "IgesDefsToolAttributeTable::own_copy",
                        )?;
                        for j in 1..=avc {
                            iges_array1_set(
                                &attr_int,
                                j,
                                iges_array1_item(
                                    &other_int,
                                    j,
                                    "IgesDefsToolAttributeTable::own_copy",
                                )?,
                                "IgesDefsToolAttributeTable::own_copy",
                            )?;
                        }
                    }
                    _ => {}
                }
            }
        }
        ent.init(&list2)?;
        Ok(())
    }
}
