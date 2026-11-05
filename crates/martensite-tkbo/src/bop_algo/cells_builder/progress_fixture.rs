//! Retained identities and caller-owned fillers cover the concrete Cells entry policies.
use super::*;
use crate::bop_algo::{
    pave_filler::owned_progress_fixture::section_progress_attribute_mask,
    perform_with_filler_progress,
    progress_test_support::entry_shapes::{
        configure_entry_builder, entry_sources, prepared_entry_filler,
    },
};
use martensite_tkernel::message::ProgressRange;
pub(super) struct CellsCase {
    pub cells: BOPAlgoCellsBuilder,
    pub sources: Vec<Shape>,
    pub supplied_filler: Option<BOPAlgoPaveFiller>,
}
impl CellsCase {
    /// Retain the actual supplied/owned entry; `case.perform(range)?`.
    pub fn perform(
        &mut self,
        range: &ProgressRange<'_>,
    ) -> Result<crate::bop_algo::ProgressCompletion, Failure> {
        if let Some(filler) = &mut self.supplied_filler {
            return perform_with_filler_progress(&mut self.cells, filler, range);
        }
        self.cells.perform_with_progress(range)
    }
}
/// Construct native-profile inputs without coordinate-based identity; `cells_case(16, false)` retains a shared edge.
pub(super) fn cells_case(kind: usize, parallel: bool) -> CellsCase {
    let sources = entry_sources(kind);
    let mut cells = BOPAlgoCellsBuilder::new();
    configure_entry_builder(&mut cells.builder, &sources, kind, parallel);
    let supplied_filler = if kind >= 20 {
        Some(prepared_entry_filler(&cells.builder, kind, parallel))
    } else {
        None
    };
    CellsCase {
        cells,
        sources,
        supplied_filler,
    }
}
/// Observe caller-owned data without inventing builder ownership; `snapshot(&case, observed, queries)`.
pub(super) fn snapshot(case: &CellsCase, observed: bool, queries: usize) -> String {
    let available = case
        .supplied_filler
        .as_ref()
        .or_else(|| case.cells.pave_filler());
    let structure = available.and_then(BOPAlgoPaveFiller::data_structure);
    let base = crate::bop_algo::builder::entry_progress::snapshot::builder_snapshot_with_structure(
        &case.cells.builder,
        &case.sources,
        structure,
        observed,
        queries,
    );
    let mask = case
        .supplied_filler
        .as_ref()
        .map_or(-1, section_progress_attribute_mask);
    format!("{base};{}", cell_index_snapshot(case, mask))
}

fn cell_index_snapshot(case: &CellsCase, mask: i32) -> String {
    let cells = &case.cells;
    let mut all = martensite_tkbrep::toptools::IndexedMapOfShape::with_hasher(ShapeMapHasher, 1);
    if !cells.all_parts.is_null() {
        martensite_tkbrep::topexp::map_shapes(&cells.all_parts, &mut all, true, true);
    }
    let rows: Vec<_> = (1..=cells.index.extent())
        .map(|position| cell_origin_row(case, position, &all))
        .collect();
    format!(
        "[{},{},{},{},{},{},{},{},[{}]]",
        case.supplied_filler.is_some(),
        mask,
        cells.all_parts.is_null(),
        crate::bop_algo::builder::entry_progress::snapshot::result_counts(&cells.all_parts),
        cells.index.extent(),
        cells.materials.extent(),
        cells.shape_material.extent(),
        cells.map_modified.extent(),
        rows.join(",")
    )
}
fn cell_origin_row(
    case: &CellsCase,
    position: usize,
    all: &martensite_tkbrep::toptools::IndexedMapOfShape,
) -> String {
    let part = case.cells.index.find_key(position).unwrap();
    let parents = case.cells.index.find_from_index(position).unwrap();
    let mut origins = Vec::new();
    for (index, source) in case.sources.iter().enumerate() {
        for parent in parents.iter() {
            if parent.is_same(source) {
                origins.push(index.to_string());
            }
        }
    }
    format!(
        "[{},{},{},[{}]]",
        part.shape_type().unwrap() as i32,
        all.contains(part),
        parents.extent(),
        origins.join(",")
    )
}
