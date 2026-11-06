//! IgesDefsToolAttributeDef.
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
use crate::iges_data::iges_read_integer_item;
use crate::iges_data::{
    IgesDataIgesEntity, IgesDataIgesReaderData, IgesDataParamReader, iges_array1_set, iges_live,
    iges_new_array1,
};
use crate::iges_defs::{IgesDefsAttributeDef, IgesDefsHArray1OfHArray1OfTextDisplayTemplate};
use crate::iges_graph::IgesGraphTextDisplayTemplate;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;

iges_tool!(
    IgesDefsToolAttributeDef,
    "The own-parameter work on `IgesDefsAttributeDef`."
);

impl IgesDefsToolAttributeDef {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDefsAttributeDef>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut a_name: Handle<HAsciiString> = Default::default();
        let mut a_list_type: i32 = Default::default();
        crate::iges_shared_blocks::iges_block_0031!(
            attr_types,
            attr_value_counts,
            attr_value_data_types,
            attr_value_pointers,
            attr_values
        );
        let mut nbval: i32 = Default::default();
        let mut fn_: i32 = ent.form_number();
        if pr.defined_else_skip()? {
            pr.read_text(&pr.current(), "Attribute Table Name", &mut a_name)?;
        }
        pr.read_integer_text(&pr.current(), "Attribute List Type", &mut a_list_type)?;
        let mut st: bool =
            pr.read_integer_text(&pr.current(), "Number of Attributes", &mut nbval)?;
        if st && (nbval > 0) {
            attr_types = iges_new_array1::<i32>(1, nbval)?;
            attr_value_data_types = iges_new_array1::<i32>(1, nbval)?;
            attr_value_counts = iges_new_array1::<i32>(1, nbval)?;
            if fn_ > 0 {
                attr_values = iges_new_array1::<Handle<StandardTransient>>(1, nbval)?;
            }
            if fn_ > 1 {
                attr_value_pointers = Handle::new(
                    IgesDefsHArray1OfHArray1OfTextDisplayTemplate::with_bounds(1, nbval)?,
                );
            }
        } else {
            pr.add_fail("Number of Attributes: Not Positive", "");
        }
        if !attr_types.is_null() {
            for i in 1..=nbval {
                let mut attr_value_data_type: i32 = Default::default();
                let mut avc: i32 = Default::default();
                let mut attr_value_pointer: Handle<HArray1<Handle<IgesGraphTextDisplayTemplate>>> =
                    Default::default();
                iges_read_integer_item(
                    pr,
                    "Attribute Type",
                    &attr_types,
                    i,
                    "IgesDefsToolAttributeDef::read_own_params",
                )?;
                st = pr.read_integer_text(
                    &pr.current(),
                    "Attribute Data Type",
                    &mut attr_value_data_type,
                )?;
                if st {
                    iges_array1_set(
                        &attr_value_data_types,
                        i,
                        attr_value_data_type,
                        "IgesDefsToolAttributeDef::read_own_params",
                    )?;
                }
                if pr.defined_else_skip()? {
                    st = pr.read_integer_text(&pr.current(), "Attribute Value Count", &mut avc)?;
                } else {
                    avc = 1;
                }
                if st {
                    iges_array1_set(
                        &attr_value_counts,
                        i,
                        avc,
                        "IgesDefsToolAttributeDef::read_own_params",
                    )?;
                    if fn_ > 1 {
                        attr_value_pointer =
                            iges_new_array1::<Handle<IgesGraphTextDisplayTemplate>>(1, avc)?;
                    }
                }
                if !attr_values.is_null() {
                    if fn_ > 0 {
                        let mut attr_int: Handle<HArray1<i32>> = Default::default();
                        let mut attr_real: Handle<HArray1<f64>> = Default::default();
                        let mut attr_str: Handle<HArray1<Handle<HAsciiString>>> =
                            Default::default();
                        let mut attr_ent: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
                            Default::default();
                        match attr_value_data_type {
                            1 => {
                                attr_int = iges_new_array1::<i32>(1, avc)?;
                                iges_array1_set(
                                    &attr_values,
                                    i,
                                    attr_int.cast(),
                                    "IgesDefsToolAttributeDef::read_own_params",
                                )?;
                            }
                            2 => {
                                attr_real = iges_new_array1::<f64>(1, avc)?;
                                iges_array1_set(
                                    &attr_values,
                                    i,
                                    attr_real.cast(),
                                    "IgesDefsToolAttributeDef::read_own_params",
                                )?;
                            }
                            3 => {
                                attr_str = iges_new_array1::<Handle<HAsciiString>>(1, avc)?;
                                iges_array1_set(
                                    &attr_values,
                                    i,
                                    attr_str.cast(),
                                    "IgesDefsToolAttributeDef::read_own_params",
                                )?;
                            }
                            4 => {
                                attr_ent = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, avc)?;
                                iges_array1_set(
                                    &attr_values,
                                    i,
                                    attr_ent.cast(),
                                    "IgesDefsToolAttributeDef::read_own_params",
                                )?;
                            }
                            6 => {
                                attr_int = iges_new_array1::<i32>(1, avc)?;
                                iges_array1_set(
                                    &attr_values,
                                    i,
                                    attr_int.cast(),
                                    "IgesDefsToolAttributeDef::read_own_params",
                                )?;
                            }
                            _ => {}
                        }
                        for j in 1..=avc {
                            match attr_value_data_type {
                                0 => {
                                    pr.set_current_number(pr.current_number() + 1);
                                }
                                1 => {
                                    let mut temp: i32 = Default::default();
                                    if pr.read_integer_text(
                                        &pr.current(),
                                        "Attribute Value",
                                        &mut temp,
                                    )? {
                                        iges_array1_set(
                                            &attr_int,
                                            j,
                                            temp,
                                            "IgesDefsToolAttributeDef::read_own_params",
                                        )?;
                                    }
                                }
                                2 => {
                                    let mut temp: f64 = Default::default();
                                    if pr.read_real_text(
                                        &pr.current(),
                                        "Attribute Value",
                                        &mut temp,
                                    )? {
                                        iges_array1_set(
                                            &attr_real,
                                            j,
                                            temp,
                                            "IgesDefsToolAttributeDef::read_own_params",
                                        )?;
                                    }
                                }
                                3 => {
                                    let mut temp: Handle<HAsciiString> = Default::default();
                                    if pr.read_text(&pr.current(), "Attribute Value", &mut temp)? {
                                        iges_array1_set(
                                            &attr_str,
                                            j,
                                            temp.clone(),
                                            "IgesDefsToolAttributeDef::read_own_params",
                                        )?;
                                    }
                                }
                                4 => {
                                    let mut temp: Handle<IgesDataIgesEntity> = Default::default();
                                    if pr.read_entity(
                                        ir,
                                        &pr.current(),
                                        "Attribute Value",
                                        &mut temp,
                                        false,
                                    )? {
                                        iges_array1_set(
                                            &attr_ent,
                                            j,
                                            temp.clone(),
                                            "IgesDefsToolAttributeDef::read_own_params",
                                        )?;
                                    }
                                }
                                5 => {
                                    pr.set_current_number(pr.current_number() + 1);
                                }
                                6 => {
                                    let mut temp: bool = Default::default();
                                    if pr.read_boolean(
                                        &pr.current(),
                                        "Attribute Value",
                                        &mut temp,
                                        true,
                                    )? {
                                        iges_array1_set(
                                            &attr_int,
                                            j,
                                            if temp { 1 } else { 0 },
                                            "IgesDefsToolAttributeDef::read_own_params",
                                        )?;
                                    }
                                }
                                _ => {}
                            }
                            if fn_ == 2 {
                                let mut temp_text: Handle<IgesGraphTextDisplayTemplate> =
                                    Default::default();
                                if pr.read_entity_typed(
                                    ir,
                                    &pr.current(),
                                    "Attribute Val. Pointer",
                                    <IgesGraphTextDisplayTemplate>::static_type(),
                                    &mut temp_text,
                                    false,
                                )? {
                                    iges_array1_set(
                                        &attr_value_pointer,
                                        j,
                                        temp_text.clone(),
                                        "IgesDefsToolAttributeDef::read_own_params",
                                    )?;
                                }
                            }
                        }
                        if fn_ == 2 {
                            iges_live(
                                &attr_value_pointers,
                                "IgesDefsToolAttributeDef::read_own_params",
                            )?
                            .set_value(i, &attr_value_pointer)?;
                        }
                    }
                }
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            &a_name,
            a_list_type,
            &attr_types,
            &attr_value_data_types,
            &attr_value_counts,
            &attr_values,
            &attr_value_pointers,
        )?;
        Ok(())
    }
}
