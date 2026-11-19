// SPDX-License-Identifier: GPL-2.0-only
//! #555: index immutable parent reports without making any native freshness claim.
use crate::sketch_region::{OriginalMemberIdentity, SourcePartitionObservation};
#[cfg(test)]
mod tests;

/// Original source-vector positions; equal source indices remain distinct rows.
pub(in crate::sketch_region_contact_transport) struct SourceIndex {
    matching_rows: Vec<Vec<usize>>,
}
impl SourceIndex {
    pub(super) fn new<'a>(
        input_parents: impl IntoIterator<Item = &'a [OriginalMemberIdentity]>,
        sources: &'a [SourcePartitionObservation],
        domains: impl IntoIterator<Item = (i32, Option<&'a OriginalMemberIdentity>, bool)>,
    ) -> Self {
        let mut prepared = ParentIndex::new();
        prepared.admit_domains(domains);
        prepared.capture_sources(sources);
        let matching_rows = input_parents
            .into_iter()
            .map(|parents| prepared.matching(parents))
            .collect();
        Self { matching_rows }
    }
    pub(super) fn matching_source_rows(&self, occurrence: usize) -> &[usize] {
        self.matching_rows
            .get(occurrence)
            .map_or(&[], Vec::as_slice)
    }
}
struct ParentIndex<'a> {
    identities: Vec<&'a OriginalMemberIdentity>,
    admitted: Vec<(i32, usize)>,
    source_parents: Vec<Vec<usize>>,
    candidates: Vec<Vec<usize>>,
}
impl<'a> ParentIndex<'a> {
    fn new() -> Self {
        Self {
            identities: Vec::new(),
            admitted: Vec::new(),
            source_parents: Vec::new(),
            candidates: Vec::new(),
        }
    }
    fn intern(&mut self, parent: &'a OriginalMemberIdentity) -> usize {
        if let Some(index) = self.identities.iter().position(|old| *old == parent) {
            return index;
        }
        let index = self.identities.len();
        self.identities.push(parent);
        self.candidates.push(Vec::new());
        index
    }
    fn admit_domains(
        &mut self,
        domains: impl IntoIterator<Item = (i32, Option<&'a OriginalMemberIdentity>, bool)>,
    ) {
        for (source_index, parent, accepted) in domains {
            if !accepted {
                continue;
            }
            let Some(parent) = parent else {
                continue;
            };
            let key = (source_index, self.intern(parent));
            self.admitted.push(key);
        }
        self.admitted.sort_unstable();
        self.admitted.dedup();
    }
    fn capture_sources(&mut self, sources: &'a [SourcePartitionObservation]) {
        for (row, source) in sources.iter().enumerate() {
            self.capture_source(row, source);
        }
    }
    fn capture_source(&mut self, row: usize, source: &'a SourcePartitionObservation) {
        let parents = source
            .parents
            .iter()
            .map(|parent| self.intern(parent))
            .collect::<Vec<_>>();
        let admitted = parents.iter().all(|parent| {
            self.admitted
                .binary_search(&(source.source_index, *parent))
                .is_ok()
        });
        if admitted {
            for parent in &parents {
                self.insert_candidate(*parent, row);
            }
        }
        self.source_parents.push(parents);
    }
    fn insert_candidate(&mut self, parent: usize, row: usize) {
        let candidates = &mut self.candidates[parent];
        if candidates.last() != Some(&row) {
            candidates.push(row);
        }
    }
    fn matching(&mut self, parents: &'a [OriginalMemberIdentity]) -> Vec<usize> {
        if parents.is_empty() {
            return Vec::new();
        }
        let parents = parents
            .iter()
            .map(|parent| self.intern(parent))
            .collect::<Vec<_>>();
        let Some(candidates) = self.candidates.get(parents[0]) else {
            return Vec::new();
        };
        candidates
            .iter()
            .copied()
            .filter(|row| self.matches_row(&parents, *row))
            .collect()
    }
    fn matches_row(&self, input: &[usize], row: usize) -> bool {
        let source = &self.source_parents[row];
        // Duplicate multiplicity affects length only; the legacy predicate is one-way subset.
        input.len() == source.len() && input.iter().all(|parent| source.contains(parent))
    }
}
