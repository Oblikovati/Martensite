// SPDX-License-Identifier: GPL-2.0-only
//! One final geometry and physical relation gate for solver and direct edits.
use super::{ConstraintFailure, ConstraintPolicy, Relation, RelationId, metric, poll};
use crate::{Edit, EditFailure, Snapshot};
use std::collections::BTreeMap;

pub(super) fn apply_checked(
    geometry: &Snapshot,
    relations: &BTreeMap<RelationId, Relation>,
    edits: &[Edit],
    policy: ConstraintPolicy,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Snapshot, ConstraintFailure> {
    let next = geometry
        .apply(edits, &mut *cancelled)
        .map_err(|error| match error {
            EditFailure::Cancelled(_) => ConstraintFailure::Cancelled,
            error => ConstraintFailure::Geometry(error),
        })?;
    verify_relations(&next, relations, policy, cancelled)?;
    Ok(next)
}

pub(super) fn verify_relations(
    geometry: &Snapshot,
    relations: &BTreeMap<RelationId, Relation>,
    policy: ConstraintPolicy,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), ConstraintFailure> {
    for (&id, &relation) in relations {
        poll(cancelled)?;
        if !metric::feasible(geometry, id, relation, policy) {
            return Err(ConstraintFailure::Infeasible(id, relation));
        }
    }
    poll(cancelled)?;
    Ok(())
}
