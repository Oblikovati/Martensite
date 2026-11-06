//! Canonical geometry is a diagnostic multiset; source relations use actual retained handles.
use super::*;
use crate::bop_algo::remove_features::fill_gap::geometry_snapshot::{
    source_geometry, source_history_flags,
};

use crate::bop_algo::progress_test_support::parent_census::ParentCensusJournal;
use std::sync::atomic::Ordering;
pub(super) fn trimming_snapshot(
    gap: &FillGap,
    history: &History,
    sources: &[Shape],
    error: &str,
    journal: &ParentCensusJournal,
) -> String {
    let state = [
        journal.queries.load(Ordering::SeqCst),
        usize::from(journal.observed.load(Ordering::SeqCst)),
        gap.faces.extent(),
        usize::from(!error.is_empty()),
    ];
    let flags: Vec<_> = sources
        .iter()
        .map(|source| source_history_flags(gap, history, source))
        .collect();
    let geometry: Vec<_> = sources
        .iter()
        .map(|source| source_geometry(gap, source))
        .collect();
    format!("{state:?};{flags:?};{geometry:?};{error}")
}
