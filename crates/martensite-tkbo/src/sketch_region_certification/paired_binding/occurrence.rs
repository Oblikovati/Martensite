// SPDX-License-Identifier: GPL-2.0-only
//! #572: only a sealed owned epoch can retain immutable occurrence decisions.
use super::*;

#[derive(Clone, Copy)]
pub(super) enum Predicate {
    #[cfg(test)]
    Relation,
    #[cfg(test)]
    PhysicalPrefix,
}
pub(super) fn capture(census: &BoundaryCensus, observer: &mut impl FnMut(Predicate)) -> usize {
    let count = census.inputs.segments.len();
    (0..count)
        .find(|&index| !observed(census, index, observer))
        .unwrap_or(count)
}
pub(super) fn current_or_retained(
    census: &BoundaryCensus,
    index: usize,
    retained: Option<usize>,
    observer: &mut impl FnMut(Predicate),
) -> bool {
    if index >= census.inputs.segments.len() {
        return false;
    }
    match retained {
        Some(marker) if index < marker => true,
        Some(marker) if index == marker => false,
        _ => observed(census, index, observer),
    }
}
pub(super) fn observed(
    census: &BoundaryCensus,
    index: usize,
    observer: &mut impl FnMut(Predicate),
) -> bool {
    let Some(record) = census
        .paired
        .as_ref()
        .and_then(|paired| paired.occurrences.get(index))
    else {
        return false;
    };
    let Some(current) = census.inputs.segments.get(index) else {
        return false;
    };
    same_occurrence_metadata(&record.source, current)
        && kinded_parents_complete(record)
        && current_relations(census, current, &record.relations, observer)
        && current_physical_group(census, index, current, observer) == Some(record.physical_source)
}
fn same_occurrence_metadata(
    old: &crate::sketch_region_candidates::CandidateSegment,
    current: &crate::sketch_region_candidates::CandidateSegment,
) -> bool {
    census::same_capture(old, current)
        && old.edge.is_equal(&current.edge)
        && old.parents == current.parents
        && old.occurrence == current.occurrence
        && old.reversed == current.reversed
}
fn current_physical_group(
    census: &BoundaryCensus,
    index: usize,
    current: &crate::sketch_region_candidates::CandidateSegment,
    _observer: &mut impl FnMut(Predicate),
) -> Option<usize> {
    census.inputs.segments[..=index].iter().position(|source| {
        #[cfg(test)]
        _observer(Predicate::PhysicalPrefix);
        census::same_capture(source, current)
    })
}

fn current_relations(
    census: &BoundaryCensus,
    source: &crate::sketch_region_candidates::CandidateSegment,
    recorded: &[crate::sketch_region::ArrangementRelation],
    _observer: &mut impl FnMut(Predicate),
) -> bool {
    let actual = census
        .inputs
        .relations
        .iter()
        .filter(|row| {
            #[cfg(test)]
            _observer(Predicate::Relation);
            row.output.is_same(&source.edge)
        })
        .collect::<Vec<_>>();
    actual.len() == recorded.len()
        && actual.iter().zip(recorded).all(|(a, b)| {
            a.output.is_equal(&b.output)
                && a.preserved == b.preserved
                && a.modified == b.modified
                && a.generated == b.generated
        })
}
fn kinded_parents_complete(record: &PairedOccurrenceSnapshot) -> bool {
    let kinded = recorded_kinded_parents(record);
    !record.source.parents.is_empty()
        && !kinded.is_empty()
        && record
            .source
            .parents
            .iter()
            .all(|parent| kinded.contains(&parent))
        && kinded
            .iter()
            .all(|parent| record.source.parents.contains(parent))
}

fn recorded_kinded_parents(
    record: &PairedOccurrenceSnapshot,
) -> Vec<&crate::sketch_region::OriginalMemberIdentity> {
    record
        .relations
        .iter()
        .flat_map(|row| {
            row.preserved
                .iter()
                .chain(&row.modified)
                .chain(&row.generated)
        })
        .collect()
}

#[cfg(test)]
mod tests;
