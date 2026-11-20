// SPDX-License-Identifier: GPL-2.0-only
//! #567: execution-local report addresses preserve the first matching retained outcome.
use super::{BoundaryCensus, PairedQueryMode};
use std::collections::BTreeMap;

type QueryKey = ([usize; 2], bool);
pub(super) trait ReportLookup {
    fn find(&mut self, sources: [usize; 2], mode: PairedQueryMode) -> Option<usize>;
    fn remember(&mut self, sources: [usize; 2], mode: PairedQueryMode, report: usize);
}
pub(super) enum ReportReuseIndex {
    ColdUnique,
    IndexedMap(BTreeMap<QueryKey, usize>),
}
impl ReportReuseIndex {
    pub(super) fn capture(census: &BoundaryCensus) -> Self {
        let Some(paired) = &census.paired else {
            return Self::from_keys([]);
        };
        let unique = paired.reports.is_empty()
            && paired.occurrences.len() == census.inputs.segments.len()
            && paired
                .occurrences
                .iter()
                .enumerate()
                .all(|(slot, row)| row.physical_source == slot);
        // Complete pair enumeration cannot repeat a key when physical sources are their positions.
        if unique {
            return Self::ColdUnique;
        }
        let reports = paired
            .reports
            .iter()
            .map(|record| (record.sources, record.mode));
        Self::from_keys(reports)
    }
    pub(super) fn from_keys(keys: impl IntoIterator<Item = ([usize; 2], PairedQueryMode)>) -> Self {
        let mut retained = Self::IndexedMap(BTreeMap::new());
        for (report, (sources, mode)) in keys.into_iter().enumerate() {
            retained.remember(sources, mode, report);
        }
        retained
    }
}
impl ReportLookup for ReportReuseIndex {
    fn find(&mut self, sources: [usize; 2], mode: PairedQueryMode) -> Option<usize> {
        match self {
            Self::ColdUnique => None,
            Self::IndexedMap(first_reports) => {
                first_reports.get(&query_key(sources, mode)).copied()
            }
        }
    }
    fn remember(&mut self, sources: [usize; 2], mode: PairedQueryMode, report: usize) {
        if let Self::IndexedMap(first_reports) = self {
            first_reports
                .entry(query_key(sources, mode))
                .or_insert(report);
        }
    }
}
fn query_key(sources: [usize; 2], mode: PairedQueryMode) -> QueryKey {
    // Mode is part of equality; ordering only addresses the private lookup tree.
    let self_domain = match mode {
        PairedQueryMode::Distinct => false,
        PairedQueryMode::SelfOffDiagonal => true,
    };
    (sources, self_domain)
}

#[cfg(test)]
mod tests;
