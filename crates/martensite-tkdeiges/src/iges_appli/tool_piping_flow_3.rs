//! More of IgesAppliToolPipingFlow.
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
use crate::iges_appli::{IgesAppliPipingFlow, IgesAppliToolPipingFlow};
use crate::iges_data::{
    IgesDataDumpResult, IgesDataIgesDumper, iges_dump_entities, iges_dump_strings,
};
use martensite_tkernel::standard::Handle;
use std::io::Write;

impl IgesAppliToolPipingFlow {
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesAppliPipingFlow>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesAppliPipingFlow")?;
        writeln!(s, "Number of Context Flags : {}", ent.nb_context_flags())?;
        writeln!(s, "Type of Flow : {}", ent.type_of_flow())?;
        write!(s, "Flow Associativities : ")?;
        iges_dump_entities(
            s,
            dumper,
            level,
            1,
            ent.nb_flow_associativities()?,
            |first| ent.flow_associativity(first),
        )?;
        write!(s, "Connect Points : ")?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_connect_points()?, |first| {
            ent.connect_point(first)
        })?;
        write!(s, "Joins : ")?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_joins()?, |first| {
            ent.join(first)
        })?;
        write!(s, "Flow Names : ")?;
        iges_dump_strings(s, level, 1, ent.nb_flow_names()?, |first| {
            ent.flow_name(first)
        })?;
        write!(s, "Text Display Templates : ")?;
        iges_dump_entities(
            s,
            dumper,
            level,
            1,
            ent.nb_text_display_templates()?,
            |first| ent.text_display_template(first),
        )?;
        write!(s, "Continuation Flow Associativities : ")?;
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
