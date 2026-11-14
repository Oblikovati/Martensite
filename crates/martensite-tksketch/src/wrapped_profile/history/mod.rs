//! Authored parents bind to semantic keys in the retained encoding, never native order.
use super::*;
use crate::surface_sketch::{HistoryRole, HistorySource};
use std::collections::{BTreeMap, BTreeSet};
mod error;
mod records;
mod source;
#[cfg(test)]
mod tests;
pub(super) use error::HistoryFailure;
type Checked<T> = Result<T, HistoryFailure>;
type Parents = BTreeSet<HistorySource>;
type Records = BTreeMap<BindingKey, HistoryRecord>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum HistoryRelation {
    Derived,
    Split,
    Created,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct HistoryRecord {
    pub result: BindingKey,
    pub owner: SurfaceRegionId,
    pub relation: HistoryRelation,
    pub parents: Vec<HistorySource>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct HistoryEvidence {
    pub records: Vec<HistoryRecord>,
}
pub(super) fn certify(
    originals: &[Request],
    selection: &selection::QualifiedSelection,
    keys: &[BindingKey],
) -> Checked<HistoryEvidence> {
    bind_sources(originals, selection)?;
    let mut records = Records::new();
    for prepared in &selection.prepared.prepared {
        records::region(prepared, &mut records)?;
    }
    bind_targets(&records, keys)?;
    Ok(HistoryEvidence {
        records: records.into_values().collect(),
    })
}
fn bind_sources(originals: &[Request], selection: &selection::QualifiedSelection) -> Checked<()> {
    let planar = &selection.prepared.planar.members;
    if planar.len() != originals.len()
        || !originals
            .iter()
            .all(|r| planar.get(&r.region).is_some_and(|e| e.matches_request(r)))
    {
        return Err(HistoryFailure::SourceBinding);
    }
    planar_clearance::bind_originals(
        &selection.prepared.boundary_clearance,
        originals,
        &selection.prepared.prepared,
    )
    .map_err(|cause| HistoryFailure::Clearance(Box::new(cause)))?;
    source::bind(originals, &selection.prepared.prepared)
}
fn bind_targets(records: &Records, keys: &[BindingKey]) -> Checked<()> {
    let mut actual = BTreeSet::new();
    for key in keys {
        if !actual.insert(*key) {
            return Err(HistoryFailure::DuplicateTarget(*key));
        }
    }
    let expected: BTreeSet<_> = records.keys().copied().collect();
    if actual != expected {
        return Err(HistoryFailure::Targets {
            missing: expected.difference(&actual).copied().collect(),
            extra: actual.difference(&expected).copied().collect(),
        });
    }
    Ok(())
}
fn insert(records: &mut Records, record: HistoryRecord) -> Checked<()> {
    let key = record.result;
    if records.insert(key, record).is_some() {
        return Err(HistoryFailure::DuplicateTarget(key));
    }
    Ok(())
}
