//! Cells wraps the actual General Fuse build and retains its successful indexing tail.
use super::*;
use crate::bop_algo::{
    ProgressCompletion,
    builder::operation_progress::{BuildMode, run_general_fuse},
};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
/// Retain Cells history and error ordering; `run_cells_progress(cells, filler, range)?`.
pub(super) fn run_cells_progress(
    cells: &mut BOPAlgoCellsBuilder,
    filler: &mut BOPAlgoPaveFiller,
    range: &ProgressRange<'_>,
) -> Result<ProgressCompletion, Failure> {
    let fill_history = cells.builder.has_history();
    cells.builder.set_to_fill_history(false);
    let scope = ProgressScope::with_literal(range, "Performing MakeCells operation", 1., false);
    let completion = run_general_fuse(
        &mut cells.builder,
        filler,
        &scope.next(1.),
        BuildMode::Ranged,
    )?;
    if cells.builder.options().has_errors() {
        return Ok(completion);
    }
    // A silent pulse is retained in completion even when the native report check allows indexing.
    cells.finish_split_index(filler)?;
    cells.builder.set_to_fill_history(fill_history);
    Ok(completion)
}
#[cfg(test)]
#[path = "progress_census.rs"]
mod census;
#[cfg(test)]
#[path = "progress_fixture.rs"]
mod fixture;
#[cfg(test)]
#[path = "progress_contract.rs"]
mod tests;
