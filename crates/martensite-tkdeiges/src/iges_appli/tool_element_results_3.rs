//! More of IgesAppliToolElementResults.
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
use crate::iges_appli::{IgesAppliElementResults, IgesAppliToolElementResults};
use crate::iges_data::{
    IgesDataDumpResult, IgesDataIgesDumper, iges_dump_entities, iges_dump_vals, iges_real_text,
};
use martensite_tkernel::standard::Handle;
use std::io::Write;

impl IgesAppliToolElementResults {
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesAppliElementResults>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut sublevel: i32 = if level > 4 { 1 } else { 0 };
        writeln!(s, "IgesAppliElementResults")?;
        write!(s, "General Note   : ")?;
        dumper.dump(&ent.note(), s, sublevel, -1)?;
        writeln!(s)?;
        writeln!(s, "Subcase Number : {}", ent.sub_case_number())?;
        writeln!(s, "Time           : {}", iges_real_text(ent.time()))?;
        writeln!(s, "Number of Result Values : {}", ent.nb_result_values())?;
        writeln!(s, "Result Report Flag  : {}", ent.result_report_flag())?;
        write!(s, "Element Identifiers : ")?;
        iges_dump_vals(s, level, 1, ent.nb_elements()?, |first| {
            ent.element_identifier(first)
        })?;
        write!(s, "\nElements : ")?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_elements()?, |first| {
            ent.element(first)
        })?;
        write!(s, "\nElement Topology Types : ")?;
        iges_dump_vals(s, level, 1, ent.nb_elements()?, |first| {
            ent.element_topology_type(first)
        })?;
        write!(s, "\nNumbers of Layers      : ")?;
        iges_dump_vals(s, level, 1, ent.nb_elements()?, |first| {
            ent.nb_layers(first)
        })?;
        write!(s, "\nData Layer Flags       : ")?;
        iges_dump_vals(s, level, 1, ent.nb_elements()?, |first| {
            ent.data_layer_flag(first)
        })?;
        write!(s, "\nNumbers of Result Data Locations : ")?;
        iges_dump_vals(s, level, 1, ent.nb_elements()?, |first| {
            ent.nb_result_data_locs(first)
        })?;
        write!(s, "\nResult Data Locations : ")?;
        write!(s, " TO BE DONE  ")?;
        if level <= 4 {
            writeln!(s, " [ ask level > 4 for more, > 5 for complete ]")?;
        } else {
            let mut i: i32 = Default::default();
            for i in 1..=ent.nb_elements()? {
                let mut nloc: i32 = ent.nb_result_data_locs(i)?;
                write!(s, " [{}:NbLoc={}]:", i, nloc)?;
                for j in 1..=nloc {
                    write!(s, " {}", ent.result_data_loc(i, j)?)?;
                }
                writeln!(s)?;
            }
            write!(s, "Result Data : ")?;
            for i in 1..=ent.nb_elements()? {
                let mut nres: i32 = ent.nb_results(i)?;
                write!(s, " [{}:NbRes={}]:", i, nres)?;
                if level <= 5 {
                    writeln!(s, " [ ask level > 5 for complete Data ]")?;
                } else {
                    for j in 1..=nres {
                        write!(s, " {}", iges_real_text(ent.result_data(i, j)?))?;
                    }
                }
            }
        }
        Ok(())
    }
}
