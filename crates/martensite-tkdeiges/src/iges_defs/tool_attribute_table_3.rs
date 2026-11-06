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
    IgesDataDumpResult, IgesDataIgesDumper, iges_dump_string, iges_live, iges_real_text,
};
use crate::iges_defs::{IgesDefsAttributeDef, IgesDefsAttributeTable, IgesDefsToolAttributeTable};
use martensite_tkernel::standard::Handle;
use std::io::Write;

impl IgesDefsToolAttributeTable {
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDefsAttributeTable>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesDefsAttributeTable")?;
        let mut ab: Handle<IgesDefsAttributeDef> = ent.definition();
        let mut na: i32 = ent.nb_attributes()?;
        let mut nr: i32 = ent.nb_rows()?;
        if ent.form_number() == 1 {
            writeln!(
                s,
                "Number of Rows (i.e. complete sets of Attributes) : {}",
                nr
            )?;
        } else {
            writeln!(s, "One set of Attributes")?;
        }
        writeln!(s, "Number of defined Attributes : {}", na)?;
        if level <= 4 {
            writeln!(
                s,
                " [ structure : see Structure in Directory Entry; content : level > 4 ]"
            )?;
        } else {
            for k in 1..=nr {
                for i in 1..=na {
                    let mut avc: i32 = iges_live(&ab, "IgesDefsToolAttributeTable::own_dump")?
                        .attribute_value_count(i)?;
                    write!(s, "[At.no.{} Row:{}]", i, k)?;
                    match iges_live(&ab, "IgesDefsToolAttributeTable::own_dump")?
                        .attribute_value_data_type(i)?
                    {
                        0 => {
                            write!(s, "  (Void) ")?;
                        }
                        1 => {
                            write!(s, "  Integer")?;
                        }
                        2 => {
                            write!(s, "  Real   ")?;
                        }
                        3 => {
                            write!(s, "  String ")?;
                        }
                        4 => {
                            write!(s, "  Entity ")?;
                        }
                        5 => {
                            write!(s, " (Not used)")?;
                        }
                        6 => {
                            write!(s, "  Logical")?;
                        }
                        _ => {}
                    }
                    write!(s, " :")?;
                    for j in 1..=avc {
                        write!(s, "  ")?;
                        match iges_live(&ab, "IgesDefsToolAttributeTable::own_dump")?
                            .attribute_value_data_type(i)?
                        {
                            1 => {
                                write!(s, "{}", ent.attribute_as_integer(i, k, j)?)?;
                            }
                            2 => {
                                write!(s, "{}", iges_real_text(ent.attribute_as_real(i, k, j)?))?;
                            }
                            3 => {
                                iges_dump_string(s, &ent.attribute_as_string(i, k, j)?)?;
                            }
                            4 => {
                                dumper.dump(
                                    &ent.attribute_as_entity(i, k, j)?,
                                    s,
                                    level - 5,
                                    -1,
                                )?;
                            }
                            6 => {
                                write!(
                                    s,
                                    "{}",
                                    if ent.attribute_as_logical(i, k, j)? {
                                        "True"
                                    } else {
                                        "False"
                                    }
                                )?;
                            }
                            _ => {}
                        }
                    }
                    writeln!(s)?;
                }
            }
        }
        writeln!(s)?;
        Ok(())
    }
}
