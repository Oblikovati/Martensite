//! Joined raw DS snapshots are diagnostics; no identity is inferred from output order or coordinates.
use super::*;
use crate::bop_ds::{DataStructure, PaveBlockHandle};
use martensite_tkernel::standard::RuntimeTyped;
/// Snapshot after execution; `intersection_snapshot(&filler, observed, queries)`.
pub(super) fn intersection_snapshot(
    filler: &BOPAlgoPaveFiller,
    observed: bool,
    queries: usize,
) -> String {
    let options = filler.options();
    let reports = format!(
        "{},{},{},{observed},{queries}",
        options.has_errors(),
        options.has_warnings(),
        options.has_error(crate::bop_algo::AlertUserBreak::static_type())
    );
    let Some(structure) = filler.data_structure() else {
        return format!("[false,-1,-1,{reports},[]];[];[]");
    };
    let core = format!(
        "[true,{},{},{reports},{}]",
        structure.nb_shapes(),
        structure.nb_source_shapes(),
        interference_counts(structure)
    );
    format!(
        "{core};{};{}",
        pool_rows(structure),
        section_rows(structure)
    )
}
fn interference_counts(structure: &DataStructure) -> String {
    format!(
        "[{},{},{},{},{},{}]",
        structure.interf_vv_records().len(),
        structure.interf_ve_records().len(),
        structure.interf_ee_records().len(),
        structure.interf_vf_records().len(),
        structure.interf_ef_records().len(),
        structure.interf_ff_records().len()
    )
}
fn pool_rows(structure: &DataStructure) -> String {
    let rows: Vec<_> = structure
        .pave_blocks_pool()
        .iter()
        .flatten()
        .map(|block| block_row(block, true))
        .collect();
    format!("[{}]", rows.join(","))
}
fn block_row(block: &PaveBlockHandle, with_original: bool) -> String {
    let block = block.lock().unwrap();
    let (first, last) = block.indices();
    let (low, high) = block.range();
    let original = if with_original {
        format!("{},", block.original_edge())
    } else {
        String::new()
    };
    format!(
        "[{original}{first},{last},{},{},{}]",
        low.to_bits(),
        high.to_bits(),
        block.edge()
    )
}
fn section_rows(structure: &DataStructure) -> String {
    let mut records = Vec::new();
    for record in structure.interf_ff_records().iter() {
        let (first, second) = record.indices();
        let rows: Vec<_> = record
            .curves()
            .iter()
            .flat_map(|curve| curve.pave_blocks())
            .map(|block| block_row(block, false))
            .collect();
        records.push(format!("[{first},{second},[{}]]", rows.join(",")));
    }
    format!("[{}]", records.join(","))
}
