// SPDX-License-Identifier: GPL-2.0-only
//! Bind serialized hard relations to the already accepted geometry.
use super::{
    ConstrainedSketch, ConstraintFailure, ConstraintPolicy, PolicyQuantity, Relation, RelationId,
    RelationSpec, adopt::verify_relations, binding::capture, poll, program::Program,
};
use crate::Snapshot;
use std::{
    collections::{BTreeMap, btree_map::Entry},
    sync::Arc,
};

impl ConstrainedSketch {
    /// Rebuild a prepared sketch from accepted geometry and captured relation values.
    ///
    /// Checks authored relation identity, operands and physical feasibility before publishing.
    /// The supplied points and Fix anchors retain their original coordinate bits. Example:
    /// `ConstrainedSketch::restore(geometry, &relations, policy, || false)?`.
    pub fn restore(
        geometry: Snapshot,
        relations: &[(RelationId, Relation)],
        policy: ConstraintPolicy,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<Self, ConstraintFailure> {
        poll(&mut cancelled)?;
        let values = staged_relations(&geometry, relations, policy, &mut cancelled)?;
        let program = Program::new(&geometry, &values, policy, &mut cancelled)?;
        verify_relations(&geometry, &values, policy, &mut cancelled)?;
        program.verify_fit_replay(
            &geometry,
            &values,
            policy.solve.relative_tolerance,
            &mut cancelled,
        )?;
        Ok(Self {
            geometry: Arc::new(geometry),
            relations: Arc::new(values),
            program: Arc::new(program),
            policy,
        })
    }
}

fn staged_relations(
    geometry: &Snapshot,
    relations: &[(RelationId, Relation)],
    policy: ConstraintPolicy,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<BTreeMap<RelationId, Relation>, ConstraintFailure> {
    let requested = geometry.entities.len().saturating_add(relations.len());
    if requested > policy.validation_items {
        return Err(ConstraintFailure::Limit {
            quantity: PolicyQuantity::ValidationItems,
            requested,
            maximum: policy.validation_items,
        });
    }
    let mut values = BTreeMap::new();
    for &(id, relation) in relations {
        poll(cancelled)?;
        let Entry::Vacant(entry) = values.entry(id) else {
            return Err(ConstraintFailure::DuplicateId(id));
        };
        capture(geometry, id, spec(relation), policy)?;
        // Capture validates operands and dimensional limits. The persisted value owns the Fix anchors.
        entry.insert(relation);
    }
    Ok(values)
}

fn spec(relation: Relation) -> RelationSpec {
    match relation {
        Relation::FixPoint(point, _) => RelationSpec::FixPoint(point),
        Relation::FixLine(line, _) => RelationSpec::FixLine(line),
        Relation::FixArc(arc, _, _) => RelationSpec::FixArc(arc),
        Relation::Coincident(points) => RelationSpec::Coincident(points),
        Relation::PointOnLine(point, line) => RelationSpec::PointOnLine(point, line),
        Relation::Colinear(lines, direction) => RelationSpec::Colinear(lines, direction),
        Relation::Parallel(lines, direction) => RelationSpec::Parallel(lines, direction),
        Relation::Length(line, length) => RelationSpec::Length(line, length),
        Relation::Radius(arc, target, _) => RelationSpec::Radius(arc, target),
        Relation::EqualRadius(arcs, _) => RelationSpec::EqualRadius(arcs),
        Relation::ParallelToAxis(line, axis, direction) => {
            RelationSpec::ParallelToAxis(line, axis, direction)
        }
        Relation::ParallelToPlane(line, normal) => RelationSpec::ParallelToPlane(line, normal),
        Relation::Perpendicular(lines) => RelationSpec::Perpendicular(lines),
        Relation::EqualLength(lines) => RelationSpec::EqualLength(lines),
        Relation::Angle(lines, radians) => RelationSpec::Angle(lines, radians),
        Relation::Tangent(join) => RelationSpec::Tangent(join),
        Relation::G2(join, cap) => RelationSpec::G2(join, cap),
    }
}
