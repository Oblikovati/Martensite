//! More of IgesAppliToolFlow.
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
use crate::iges_appli::{IgesAppliFlow, IgesAppliToolFlow};
use crate::iges_data::{
    IgesDataDumpResult, IgesDataIgesDumper, iges_dump_entities, iges_dump_strings,
};
use martensite_tkernel::standard::Handle;
use std::io::Write;

impl IgesAppliToolFlow {
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesAppliFlow>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesAppliFlow")?;
        writeln!(s, "Number of Context Flags : {}", ent.nb_context_flags())?;
        let mut tf: i32 = ent.type_of_flow();
        write!(s, "Type of Flow : {}", tf)?;
        if tf == 1 {
            writeln!(s, " (logical)")?;
        } else {
            if tf == 2 {
                writeln!(s, " (physical)")?;
            } else {
                writeln!(s, " (not specified)")?;
            }
        }
        tf = ent.function_flag();
        write!(s, "Function Flag : {}", tf)?;
        if tf == 1 {
            writeln!(s, " (electrical signal)")?;
        } else {
            if tf == 2 {
                writeln!(s, " (fluid flow path)")?;
            } else {
                writeln!(s, " (not specified)")?;
            }
        }
        write!(s, "Flow Associativities : ")?;
        iges_dump_entities(
            s,
            dumper,
            level,
            1,
            ent.nb_flow_associativities()?,
            |first| ent.flow_associativity(first),
        )?;
        write!(s, "\nConnect Points : ")?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_connect_points()?, |first| {
            ent.connect_point(first)
        })?;
        write!(s, "\nJoins : ")?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_joins()?, |first| {
            ent.join(first)
        })?;
        write!(s, "\nFlow Names : ")?;
        iges_dump_strings(s, level, 1, ent.nb_flow_names()?, |first| {
            ent.flow_name(first)
        })?;
        write!(s, "\nText Display Templates : ")?;
        iges_dump_entities(
            s,
            dumper,
            level,
            1,
            ent.nb_text_display_templates()?,
            |first| ent.text_display_template(first),
        )?;
        write!(s, "\nContinuation Flow Associativities : ")?;
        iges_dump_entities(
            s,
            dumper,
            level,
            1,
            ent.nb_cont_flow_associativities()?,
            |first| ent.cont_flow_associativity(first),
        )?;
        writeln!(s)?;
        Ok(())
    }
}
