// SPDX-License-Identifier: GPL-2.0-only
//! #567: execution-local report addresses preserve the first matching retained outcome.
use super::{BoundaryCensus, PairedQueryMode};
use std::collections::BTreeMap;

type QueryKey = ([usize; 2], bool);
pub(super) trait ReportLookup {
    fn find(&mut self, sources: [usize; 2], mode: PairedQueryMode) -> Option<usize>;
    fn remember(&mut self, sources: [usize; 2], mode: PairedQueryMode, report: usize);
}
pub(super) struct ReportReuseIndex {
    first_reports: BTreeMap<QueryKey, usize>,
}
impl ReportReuseIndex {
    pub(super) fn capture(census: &BoundaryCensus) -> Self {
        let reports = census.paired.as_ref().into_iter().flat_map(|paired| {
            paired
                .reports
                .iter()
                .map(|record| (record.sources, record.mode))
        });
        Self::from_keys(reports)
    }
    pub(super) fn from_keys(keys: impl IntoIterator<Item = ([usize; 2], PairedQueryMode)>) -> Self {
        let mut retained = Self {
            first_reports: BTreeMap::new(),
        };
        for (report, (sources, mode)) in keys.into_iter().enumerate() {
            retained.remember(sources, mode, report);
        }
        retained
    }
}
impl ReportLookup for ReportReuseIndex {
    fn find(&mut self, sources: [usize; 2], mode: PairedQueryMode) -> Option<usize> {
        let key = query_key(sources, mode);
        self.first_reports.get(&key).copied()
    }
    fn remember(&mut self, sources: [usize; 2], mode: PairedQueryMode, report: usize) {
        let key = query_key(sources, mode);
        self.first_reports.entry(key).or_insert(report);
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
