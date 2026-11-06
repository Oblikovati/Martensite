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
use crate::iges_data::{
    IgesDataDumpResult, IgesDataIgesDumper, iges_dump_string, iges_dump_vals, iges_real_text,
};
use crate::iges_defs::{IgesDefsAttributeDef, IgesDefsToolAttributeDef};
use martensite_tkernel::standard::Handle;
use std::io::Write;

impl IgesDefsToolAttributeDef {
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDefsAttributeDef>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut sublevel: i32 = if level > 4 { 1 } else { 0 };
        write!(s, "IgesDefsAttributeDef\nAttribute Table Name: ")?;
        iges_dump_string(s, &ent.table_name())?;
        writeln!(
            s,
            "\nAttribute List Type  : {}\nNumber of Attributes : {}\nAttribute Types :\nAttribute Value Data Types :\nAttribute Value Counts :",
            ent.list_type(),
            ent.nb_attributes()?
        )?;
        if ent.has_values() {
            writeln!(s, "Attribute Values :")?;
        }
        if ent.has_text_display() {
            writeln!(s, "Attribute Value Entities :")?;
        }
        iges_dump_vals(s, -level, 1, ent.nb_attributes()?, |first| {
            ent.attribute_type(first)
        })?;
        writeln!(s)?;
        if level > 4 {
            let mut upper: i32 = ent.nb_attributes()?;
            for i in 1..=upper {
                let mut avc: i32 = ent.attribute_value_count(i)?;
                let mut typ: i32 = ent.attribute_value_data_type(i)?;
                write!(
                    s,
                    "[{}]:  Attribute Type : {}  Value Data Type : {}",
                    i,
                    ent.attribute_type(i)?,
                    typ
                )?;
                match typ {
                    0 => {
                        write!(s, "  (Void)")?;
                    }
                    1 => {
                        write!(s, " : Integer ")?;
                    }
                    2 => {
                        write!(s, " : Real    ")?;
                    }
                    3 => {
                        write!(s, " : String  ")?;
                    }
                    4 => {
                        write!(s, " : Entity  ")?;
                    }
                    5 => {
                        write!(s, " (Not Used)")?;
                    }
                    6 => {
                        write!(s, " : Logical ")?;
                    }
                    _ => {}
                }
                writeln!(s, "   Count : {}", avc)?;
                if ent.has_values() {
                    if level <= 5 {
                        writeln!(s, " [ content (Values) : ask level > 5 ]")?;
                        continue;
                    }
                    for j in 1..=avc {
                        write!(s, "[{}]: ", j)?;
                        match ent.attribute_value_data_type(i)? {
                            0 => {
                                write!(s, "(Void) ")?;
                            }
                            1 => {
                                write!(s, "{}", ent.attribute_as_integer(i, j)?)?;
                            }
                            2 => {
                                write!(s, "{}", iges_real_text(ent.attribute_as_real(i, j)?))?;
                            }
                            3 => {
                                iges_dump_string(s, &ent.attribute_as_string(i, j)?)?;
                            }
                            4 => {
                                dumper.dump(&ent.attribute_as_entity(i, j)?, s, level - 5, -1)?;
                            }
                            5 => {
                                write!(s, "(Not Used)")?;
                            }
                            6 => {
                                write!(
                                    s,
                                    "{}",
                                    if ent.attribute_as_logical(i, j)? {
                                        "True"
                                    } else {
                                        "False"
                                    }
                                )?;
                            }
                            _ => {}
                        }
                        if ent.has_text_display() {
                            write!(s, "  Attribute Value Pointer : ")?;
                            dumper.dump(&ent.attribute_text_display(i, j)?, s, sublevel, -1)?;
                        }
                        writeln!(s)?;
                    }
                }
            }
        }
        writeln!(s)?;
        Ok(())
    }
}
