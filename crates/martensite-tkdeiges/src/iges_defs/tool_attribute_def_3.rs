//! More of IgesDefsToolAttributeDef.
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
use crate::iges_data::{IgesDataIgesEntity, iges_array1_set, iges_live, iges_new_array1};
use crate::iges_defs::{
    IgesDefsAttributeDef, IgesDefsHArray1OfHArray1OfTextDisplayTemplate, IgesDefsToolAttributeDef,
};
use crate::iges_graph::IgesGraphTextDisplayTemplate;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::InterfaceCopyTool;

impl IgesDefsToolAttributeDef {
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDefsAttributeDef>,
        ent: &Handle<IgesDefsAttributeDef>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut a_name: Handle<HAsciiString> = Default::default();
        if !(another.table_name().is_null()) {
            a_name = Handle::new(HAsciiString::copied(iges_live(
                &another.table_name(),
                "IgesDefsToolAttributeDef::own_copy",
            )?));
        }
        let mut a_list_type: i32 = another.list_type();
        crate::iges_shared_blocks::iges_block_0031!(
            attr_types,
            attr_value_counts,
            attr_value_data_types,
            attr_value_pointers,
            attr_values
        );
        let mut nbval: i32 = another.nb_attributes()?;
        attr_types = iges_new_array1::<i32>(1, nbval)?;
        attr_value_data_types = iges_new_array1::<i32>(1, nbval)?;
        attr_value_counts = iges_new_array1::<i32>(1, nbval)?;
        if another.has_values() {
            attr_values = iges_new_array1::<Handle<StandardTransient>>(1, nbval)?;
        }
        if another.has_text_display() {
            attr_value_pointers = Handle::new(
                IgesDefsHArray1OfHArray1OfTextDisplayTemplate::with_bounds(1, nbval)?,
            );
        }
        for i in 1..=nbval {
            let mut attr_type: i32 = another.attribute_type(i)?;
            iges_array1_set(
                &attr_types,
                i,
                attr_type,
                "IgesDefsToolAttributeDef::own_copy",
            )?;
            let mut attr_value_data_type: i32 = another.attribute_value_data_type(i)?;
            iges_array1_set(
                &attr_value_data_types,
                i,
                attr_value_data_type,
                "IgesDefsToolAttributeDef::own_copy",
            )?;
            let mut avc: i32 = another.attribute_value_count(i)?;
            iges_array1_set(
                &attr_value_counts,
                i,
                avc,
                "IgesDefsToolAttributeDef::own_copy",
            )?;
            let mut attr_value_pointer: Handle<HArray1<Handle<IgesGraphTextDisplayTemplate>>> =
                Default::default();
            if another.has_text_display() {
                attr_value_pointer =
                    iges_new_array1::<Handle<IgesGraphTextDisplayTemplate>>(1, avc)?;
            }
            if another.has_values() {
                let mut attr_int: Handle<HArray1<i32>> = Default::default();
                let mut attr_real: Handle<HArray1<f64>> = Default::default();
                let mut attr_str: Handle<HArray1<Handle<HAsciiString>>> = Default::default();
                let mut attr_ent: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Default::default();
                match attr_value_data_type {
                    1 => {
                        attr_int = iges_new_array1::<i32>(1, avc)?;
                        iges_array1_set(
                            &attr_values,
                            i,
                            attr_int.cast(),
                            "IgesDefsToolAttributeDef::own_copy",
                        )?;
                    }
                    2 => {
                        attr_real = iges_new_array1::<f64>(1, avc)?;
                        iges_array1_set(
                            &attr_values,
                            i,
                            attr_real.cast(),
                            "IgesDefsToolAttributeDef::own_copy",
                        )?;
                    }
                    3 => {
                        attr_str = iges_new_array1::<Handle<HAsciiString>>(1, avc)?;
                        iges_array1_set(
                            &attr_values,
                            i,
                            attr_str.cast(),
                            "IgesDefsToolAttributeDef::own_copy",
                        )?;
                    }
                    4 => {
                        attr_ent = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, avc)?;
                        iges_array1_set(
                            &attr_values,
                            i,
                            attr_ent.cast(),
                            "IgesDefsToolAttributeDef::own_copy",
                        )?;
                    }
                    6 => {
                        attr_int = iges_new_array1::<i32>(1, avc)?;
                        iges_array1_set(
                            &attr_values,
                            i,
                            attr_int.cast(),
                            "IgesDefsToolAttributeDef::own_copy",
                        )?;
                    }
                    _ => {}
                }
                for j in 1..=avc {
                    match attr_value_data_type {
                        0 => {}
                        1 => {
                            iges_array1_set(
                                &attr_int,
                                j,
                                another.attribute_as_integer(i, j)?,
                                "IgesDefsToolAttributeDef::own_copy",
                            )?;
                        }
                        2 => {
                            iges_array1_set(
                                &attr_real,
                                j,
                                another.attribute_as_real(i, j)?,
                                "IgesDefsToolAttributeDef::own_copy",
                            )?;
                        }
                        3 => {
                            iges_array1_set(
                                &attr_str,
                                j,
                                Handle::new(HAsciiString::copied(iges_live(
                                    &another.attribute_as_string(i, j)?,
                                    "IgesDefsToolAttributeDef::own_copy",
                                )?)),
                                "IgesDefsToolAttributeDef::own_copy",
                            )?;
                        }
                        4 => {
                            let mut ent: Handle<IgesDataIgesEntity> = tc
                                .transferred(&another.attribute_as_entity(i, j)?.cast())?
                                .cast();
                            iges_array1_set(
                                &attr_ent,
                                j,
                                ent.clone(),
                                "IgesDefsToolAttributeDef::own_copy",
                            )?;
                        }
                        5 => {}
                        6 => {
                            iges_array1_set(
                                &attr_int,
                                j,
                                if another.attribute_as_logical(i, j)? {
                                    1
                                } else {
                                    0
                                },
                                "IgesDefsToolAttributeDef::own_copy",
                            )?;
                        }
                        _ => {}
                    }
                    if another.has_text_display() {
                        let mut temptext: Handle<IgesGraphTextDisplayTemplate> = tc
                            .transferred(&another.attribute_text_display(i, j)?.cast())?
                            .cast();
                        iges_array1_set(
                            &attr_value_pointer,
                            j,
                            temptext.clone(),
                            "IgesDefsToolAttributeDef::own_copy",
                        )?;
                    }
                }
                if another.has_text_display() {
                    iges_live(&attr_value_pointers, "IgesDefsToolAttributeDef::own_copy")?
                        .set_value(i, &attr_value_pointer)?;
                }
            }
        }
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
