//! Joined reports and stored record identities are observed without adding polls.
use super::*;
use crate::bop_algo::AlertUserBreak;
use crate::bop_ds::Interference;
use martensite_tkernel::standard::RuntimeTyped;
/// Observe actual final state and pair records; `snapshot(checker, stopped, queries)`.
pub(super) fn snapshot(checker: &mut BOPAlgoCheckerSI, observed: bool, queries: usize) -> String {
    let options = checker.options();
    let structure = checker.data_structure();
    let state = format!(
        "[{},{},{},{},{},{},{},{},{},{},{queries},{observed}]",
        options.has_errors(),
        options.has_warnings(),
        options.has_error(AlertUserBreak::static_type()),
        options.has_error(AlertMultipleArguments::static_type()),
        options.has_error(AlertIntersectionFailed::static_type()),
        structure.is_some(),
        structure.map_or(-1, DataStructure::nb_shapes),
        structure.map_or(-1, DataStructure::nb_source_shapes),
        checker.non_destructive(),
        checker.level_of_check()
    );
    let Some(structure) = checker.data_structure_mut() else {
        return format!("{state};[];[]");
    };
    let records = record_arrays(structure);
    let pairs = pair_rows(structure);
    format!("{state};{records};{pairs}")
}
fn record_arrays(structure: &mut DataStructure) -> String {
    let arrays = [
        record_rows(structure.interf_vv_records().iter()),
        record_rows(structure.interf_ve_records().iter()),
        record_rows(structure.interf_ee_records().iter()),
        record_rows(structure.interf_vf_records().iter()),
        record_rows(structure.interf_ef_records().iter()),
        record_rows(structure.interf_ff_records().iter()),
        record_rows(structure.interf_vz().iter()),
        record_rows(structure.interf_ez().iter()),
        record_rows(structure.interf_fz().iter()),
        record_rows(structure.interf_zz().iter()),
    ];
    format!("[{}]", arrays.join(","))
}
fn record_rows<'a, T: std::ops::Deref<Target = Interference> + 'a>(
    records: impl Iterator<Item = &'a T>,
) -> String {
    let rows: Vec<_> = records
        .map(|record| {
            let (a, b) = record.indices();
            format!("[{a},{b}]")
        })
        .collect();
    format!("[{}]", rows.join(","))
}
fn pair_rows(structure: &DataStructure) -> String {
    let rows: Vec<_> = structure
        .interferences()
        .iter()
        .map(|pair| {
            let (a, b) = pair.indices();
            format!(
                "[{a},{b},{},{}]",
                structure.shape_info(a).unwrap().shape_type() as i32,
                structure.shape_info(b).unwrap().shape_type() as i32
            )
        })
        .collect();
    format!("[{}]", rows.join(","))
}
