//! Retained identities and caller-owned fillers cover the concrete Section entry policies.
use super::*;
use crate::bop_algo::{
    pave_filler::owned_progress_fixture::section_progress_attribute_mask,
    perform_with_filler_progress,
    progress_test_support::entry_shapes::{
        configure_entry_builder, entry_sources, prepared_entry_filler,
    },
};
use martensite_tkernel::message::ProgressRange;
pub(super) struct SectionCase {
    pub section: BOPAlgoSection,
    pub sources: Vec<Shape>,
    pub supplied_filler: Option<BOPAlgoPaveFiller>,
}
impl SectionCase {
    /// Retain the actual supplied/owned entry; `case.perform(range)?`.
    pub fn perform(
        &mut self,
        range: &ProgressRange<'_>,
    ) -> Result<crate::bop_algo::ProgressCompletion, Failure> {
        if let Some(filler) = &mut self.supplied_filler {
            return perform_with_filler_progress(&mut self.section, filler, range);
        }
        self.section.perform_with_progress(range)
    }
}
/// Construct native-profile inputs without coordinate-based identity; `section_case(16, false)` retains a shared edge.
pub(super) fn section_case(kind: usize, parallel: bool) -> SectionCase {
    let sources = entry_sources(kind);
    let mut section = BOPAlgoSection::new();
    configure_entry_builder(&mut section.builder, &sources, kind, parallel);
    let supplied_filler = if kind >= 20 {
        Some(prepared_entry_filler(&section.builder, kind, parallel))
    } else {
        None
    };
    SectionCase {
        section,
        sources,
        supplied_filler,
    }
}
/// Observe caller-owned data without inventing builder ownership; `snapshot(&case, observed, queries)`.
pub(super) fn snapshot(case: &SectionCase, observed: bool, queries: usize) -> String {
    let available = case
        .supplied_filler
        .as_ref()
        .or_else(|| case.section.pave_filler());
    let structure = available.and_then(BOPAlgoPaveFiller::data_structure);
    let base = crate::bop_algo::builder::entry_progress::snapshot::builder_snapshot_with_structure(
        &case.section.builder,
        &case.sources,
        structure,
        observed,
        queries,
    );
    let mask = case
        .supplied_filler
        .as_ref()
        .map_or(-1, section_progress_attribute_mask);
    format!("{base};[{},{}]", case.supplied_filler.is_some(), mask)
}
