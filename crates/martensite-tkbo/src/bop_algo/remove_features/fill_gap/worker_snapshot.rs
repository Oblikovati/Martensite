//! Post-worker state uses original source handles, with canonical geometry only as diagnostics.
use super::*;
use crate::bop_algo::progress_test_support::parent_census::ParentCensusJournal;
use crate::bop_algo::remove_features::fill_gap::geometry_snapshot::{
    source_geometry, source_history_flags,
};
use std::sync::atomic::Ordering;
pub(super) fn worker_snapshot(
    gap: &FillGap,
    fixture: &GapIncidenceFixture,
    journal: &ParentCensusJournal,
) -> String {
    let state = [
        journal.queries.load(Ordering::SeqCst),
        usize::from(journal.observed.load(Ordering::SeqCst)),
        gap.feature_faces.len(),
        usize::from(gap.has_adjacent_faces),
        gap.solids.extent(),
        gap.faces.extent(),
        usize::from(gap.history.get().is_some()),
        usize::from(gap.solids.contains(&fixture.solid)),
    ];
    let cached: Vec<_> = fixture
        .faces
        .iter()
        .map(|face| usize::from(gap.feature_faces.contains(face)))
        .collect();
    let history = gap
        .history
        .get()
        .expect("worker history=null; expected allocated history");
    let flags: Vec<_> = fixture
        .faces
        .iter()
        .map(|source| source_history_flags(gap, history, source))
        .collect();
    let geometry: Vec<_> = fixture
        .faces
        .iter()
        .map(|source| source_geometry(gap, source))
        .collect();
    format!("{state:?};{cached:?};{flags:?};{geometry:?}")
}
