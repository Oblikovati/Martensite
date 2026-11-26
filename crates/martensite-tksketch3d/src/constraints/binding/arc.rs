// SPDX-License-Identifier: GPL-2.0-only
//! Bind arc rows and exact anchors to the existing authored point slots.
use super::{
    Arc, ArcId, ArcLayout, BlockOwner, EntityId, Equation, EquationBlock, Failure, Layout, Point3,
    Relation, RelationId, Snapshot, distinct, slots,
};
use crate::{
    ArcGeometry, MinimumLength,
    constraints::{ArcBranch, ArcConstraintFailure, ConstraintPolicy, arc_layout},
};
use std::collections::BTreeSet;

impl Layout<'_> {
    pub(super) fn arc_intrinsics(&mut self) -> Result<(), Failure> {
        let mut sources = BTreeSet::new();
        for layout in self.arcs.values() {
            if !sources.insert(layout.midpoint_key()) {
                continue;
            }
            let variables = slots(&layout.points, |point| {
                self.point_columns
                    .get(&point)
                    .copied()
                    .ok_or(Failure::MissingPoint(point))
            })?;
            let branch = if self.crossings.contains(&layout.id) {
                ArcBranch::CrossHalfTurn
            } else {
                // The differential rows remain regular at a bare half turn. Adoption
                // keeps its exact branch until a Radius explicitly permits crossing.
                layout.radius_branch()
            };
            // Exact pins already determine the entire chart. Its actual midpoint and branch
            // still pass the independent physical publication gate.
            if !variables.iter().all(|slot| self.fixed.contains_key(slot)) {
                let source = Arc::new(Equation::ArcIntrinsic {
                    branch,
                    scale: self.scale,
                });
                self.blocks.push(EquationBlock {
                    owner: BlockOwner::IntrinsicArc(layout.id),
                    source,
                    variables: variables.into(),
                });
            }
        }
        Ok(())
    }

    pub(super) fn fix_arc(
        &mut self,
        id: RelationId,
        arc: ArcId,
        anchors: [Point3; 3],
        branch: ArcBranch,
    ) -> Result<(), Failure> {
        let layout = self.strict_arc_operand(id, arc, branch)?;
        // With both endpoints fixed, two sagitta rows and the intrinsic row uniquely fix the
        // witness. Hard pins express the same isolated solution and preserve exact anchor bits.
        for (point, position) in layout.points.into_iter().zip(anchors) {
            self.pin(id, point, position)?;
        }
        self.authored_rows += 8;
        Ok(())
    }

    pub(super) fn radius(
        &mut self,
        id: RelationId,
        arc: ArcId,
        target: MinimumLength,
        branch: ArcBranch,
    ) -> Result<(), Failure> {
        let layout = self.arc_operand(id, arc, branch)?;
        self.equation(
            id,
            &layout.points,
            Equation::Radius {
                scale: self.scale,
                target: target.value(),
                branch,
            },
        )
    }

    pub(super) fn equal_radius(
        &mut self,
        id: RelationId,
        [first, second]: [ArcId; 2],
        branches: [ArcBranch; 2],
    ) -> Result<(), Failure> {
        let a = self.strict_arc_operand(id, first, branches[0])?;
        let b = self.strict_arc_operand(id, second, branches[1])?;
        if a.circle_key() == b.circle_key() {
            // The shared authored triangle already fixes both circles to the same radius.
            return Ok(());
        }
        let points = [a.points, b.points]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        self.equation(
            id,
            &points,
            Equation::EqualRadius {
                scale: self.scale,
                branches,
            },
        )
    }

    fn strict_arc_operand(
        &self,
        relation: RelationId,
        arc: ArcId,
        branch: ArcBranch,
    ) -> Result<ArcLayout, Failure> {
        if branch == ArcBranch::CrossHalfTurn {
            return Err(Failure::Arc(
                arc,
                ArcConstraintFailure::CrossingRequiresRadius,
            ));
        }
        self.arc_operand(relation, arc, branch)
    }

    fn arc_operand(
        &self,
        relation: RelationId,
        arc: ArcId,
        branch: ArcBranch,
    ) -> Result<ArcLayout, Failure> {
        let layout = *self
            .arcs
            .get(&arc)
            .ok_or(Failure::MissingOperand(relation, EntityId::Arc(arc)))?;
        if branch != ArcBranch::CrossHalfTurn && self.crossings.contains(&arc) {
            return Err(Failure::Arc(
                arc,
                ArcConstraintFailure::ConflictingSweep(branch),
            ));
        }
        layout.require_branch(branch)?;
        Ok(layout)
    }
}

pub(super) fn capture_fix(
    geometry: &Snapshot,
    id: RelationId,
    arc: ArcId,
    policy: ConstraintPolicy,
) -> Result<Relation, Failure> {
    let (arc_geometry, layout) = capture_operand(geometry, id, arc, policy)?;
    Ok(Relation::FixArc(
        arc,
        arc_geometry.points(),
        layout.strict_branch()?,
    ))
}

pub(super) fn capture_radius(
    geometry: &Snapshot,
    id: RelationId,
    arc: ArcId,
    target: MinimumLength,
    policy: ConstraintPolicy,
) -> Result<Relation, Failure> {
    let (_, layout) = capture_operand(geometry, id, arc, policy)?;
    if target.value() < geometry.minimum.value() {
        return Err(Failure::RadiusBelowMinimum(id, arc, target.value()));
    }
    Ok(Relation::Radius(arc, target, layout.radius_branch()))
}

pub(super) fn capture_equal(
    geometry: &Snapshot,
    id: RelationId,
    arcs: [ArcId; 2],
    policy: ConstraintPolicy,
) -> Result<Relation, Failure> {
    let [first, second] = arcs;
    distinct(id, EntityId::Arc(first), EntityId::Arc(second))?;
    let (_, a) = capture_operand(geometry, id, first, policy)?;
    let (_, b) = capture_operand(geometry, id, second, policy)?;
    Ok(Relation::EqualRadius(
        arcs,
        [a.strict_branch()?, b.strict_branch()?],
    ))
}

fn capture_operand(
    geometry: &Snapshot,
    relation: RelationId,
    id: ArcId,
    policy: ConstraintPolicy,
) -> Result<(ArcGeometry, ArcLayout), Failure> {
    let arc = arc_layout::operand(geometry, relation, id)?;
    Ok((arc, ArcLayout::capture(arc, policy)?))
}
