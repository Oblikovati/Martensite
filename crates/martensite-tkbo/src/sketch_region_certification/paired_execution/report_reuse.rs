// SPDX-License-Identifier: GPL-2.0-only
//! #567: execution-local report addresses preserve the first matching retained outcome.
use super::{BoundaryCensus, PairedQueryMode};
use std::collections::BTreeMap;

type QueryKey = ([usize; 2], bool);
pub(in crate::sketch_region_certification) trait ReportLookup {
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
        let selected = SelectedStore::capture(paired);
        let unique = selected.is_empty()
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
        selected.index()
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

enum SelectedStore<'a> {
    Solver(&'a [super::super::paired_types::PairedStoredReport]),
    Conditioned(&'a [super::super::bernstein_execution::BernsteinStoredReport]),
}
impl<'a> SelectedStore<'a> {
    fn capture(paired: &'a super::super::paired_types::PairedCensusEvidence) -> Self {
        match paired.exclusion_version {
            None => Self::Solver(&paired.reports),
            Some(_) => Self::Conditioned(&paired.exclusions),
        }
    }
    fn is_empty(&self) -> bool {
        match self {
            Self::Solver(records) => records.is_empty(),
            Self::Conditioned(records) => records.is_empty(),
        }
    }
    fn index(self) -> ReportReuseIndex {
        match self {
            Self::Solver(records) => ReportReuseIndex::from_keys(
                records.iter().map(|record| (record.sources, record.mode)),
            ),
            Self::Conditioned(records) => ReportReuseIndex::from_keys(
                records.iter().map(|record| (record.sources, record.mode)),
            ),
        }
    }
}
#[cfg(test)]
mod tests;
