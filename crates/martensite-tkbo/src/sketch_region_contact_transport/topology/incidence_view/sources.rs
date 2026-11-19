// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) type SourceBindings = BTreeMap<i32, Vec<(usize, usize)>>;
pub(super) fn capture(
    segments: &[CandidateSegment],
    sources: &[SourcePartitionObservation],
    current: &PartitionTransactionInspection,
    index: Option<&SourceIndex>,
    observe: &mut impl FnMut(DiscoveryStep),
) -> SourceBindings {
    let mut bindings = BTreeMap::new();
    for (occurrence, segment) in segments.iter().enumerate() {
        append_sources(
            &mut bindings,
            occurrence,
            segment,
            sources,
            current,
            index,
            observe,
        );
    }
    bindings
}
fn matches_source(
    segment: &CandidateSegment,
    source: &SourcePartitionObservation,
    current: &PartitionTransactionInspection,
    index: Option<&SourceIndex>,
    occurrence: usize,
    position: usize,
) -> bool {
    match index {
        Some(index) => index
            .matching_source_rows(occurrence)
            .binary_search(&position)
            .is_ok(),
        None => super::super::source_matches_parents(&segment.parents, source, current),
    }
}

fn append_sources(
    bindings: &mut SourceBindings,
    occurrence: usize,
    segment: &CandidateSegment,
    sources: &[SourcePartitionObservation],
    current: &PartitionTransactionInspection,
    index: Option<&SourceIndex>,
    observe: &mut impl FnMut(DiscoveryStep),
) {
    for (position, source) in sources.iter().enumerate() {
        observe(DiscoveryStep::SourceMembership);
        if matches_source(segment, source, current, index, occurrence, position) {
            bindings
                .entry(source.source_index)
                .or_insert_with(Vec::new)
                .push((occurrence, position));
        }
    }
}
