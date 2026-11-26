// SPDX-License-Identifier: GPL-2.0-only
//! Admit the radius family with one variable chord and no other coupled curve.
use super::{
    ArcId, ArcLayout, BTreeMap, BTreeSet, Equation, EquationBlock, Failure, Layout, PointId, poll,
};
use crate::{
    Entity, EntityId,
    constraints::{ArcBranch, ArcConstraintFailure, RelationId},
};

#[derive(Default)]
struct CrossingScope {
    points: BTreeMap<PointId, ArcLayout>,
    slots: BTreeMap<usize, ArcId>,
}

impl Layout<'_> {
    pub(super) fn check_crossings(
        &self,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(), Failure> {
        if self.crossings.is_empty() {
            return Ok(());
        }
        let scope = self.crossing_scope(cancelled)?;
        for &entity in self.geometry.entities.values() {
            poll(cancelled)?;
            scope.check_geometry(entity)?;
        }
        self.check_crossing_rows(&scope, cancelled)
    }

    fn crossing_scope(
        &self,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<CrossingScope, Failure> {
        let mut scope = CrossingScope::default();
        for layout in self
            .arcs
            .values()
            .copied()
            .filter(|arc| self.crossings.contains(&arc.id))
        {
            poll(cancelled)?;
            let columns = self.crossing_columns(layout)?;
            self.check_crossing_pins(layout, columns)?;
            for (point, column) in layout.points.into_iter().zip(columns) {
                if let Some(other) = scope.points.insert(point, layout) {
                    return Err(Failure::Arc(
                        other.id,
                        ArcConstraintFailure::AliasedCrossing(layout.id),
                    ));
                }
                scope.slots.insert(column, layout.id);
            }
        }
        Ok(scope)
    }

    fn crossing_columns(&self, layout: ArcLayout) -> Result<[usize; 3], Failure> {
        let [start, end, witness] = layout.points.map(|point| {
            self.point_columns
                .get(&point)
                .copied()
                .ok_or(Failure::MissingPoint(point))
        });
        Ok([start?, end?, witness?])
    }

    fn check_crossing_pins(
        &self,
        layout: ArcLayout,
        [start, end, witness]: [usize; 3],
    ) -> Result<(), Failure> {
        let pinned = |start| (start..start + 3).any(|column| self.fixed.contains_key(&column));
        if pinned(start) && pinned(end) {
            let [first, second, _] = layout.points;
            return Err(Failure::Arc(
                layout.id,
                ArcConstraintFailure::LockedCrossing([first, second]),
            ));
        }
        if pinned(witness) {
            return Err(Failure::Arc(
                layout.id,
                ArcConstraintFailure::CrossingPinnedWitness(layout.points[2]),
            ));
        }
        Ok(())
    }

    fn check_crossing_rows(
        &self,
        scope: &CrossingScope,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(), Failure> {
        let mut radii = BTreeSet::new();
        let mut axes = BTreeSet::new();
        for block in &self.blocks {
            poll(cancelled)?;
            let Some((arc, relation)) = scope.relation(block) else {
                continue;
            };
            let chart = self
                .arcs
                .get(&arc)
                .ok_or(Failure::MissingOperand(relation, EntityId::Arc(arc)))?;
            let columns = self.crossing_columns(*chart)?;
            if !admitted_row(block, columns, arc, &mut radii, &mut axes) {
                return Err(Failure::Arc(
                    arc,
                    ArcConstraintFailure::CrossingCoupledRelation(relation),
                ));
            }
        }
        Ok(())
    }
}

impl CrossingScope {
    fn check_geometry(&self, entity: Entity) -> Result<(), Failure> {
        match entity {
            Entity::Point(_) => Ok(()),
            Entity::Line(line) => self.check_line(line.endpoints, EntityId::Line(line.id)),
            Entity::Arc(curve) => {
                if let Some(crossing) = self.touching(&curve.points())
                    && crossing.id != curve.id
                {
                    return Err(Failure::Arc(
                        crossing.id,
                        ArcConstraintFailure::AliasedCrossing(curve.id),
                    ));
                }
                Ok(())
            }
            Entity::ControlSpline(curve) => {
                self.require_unshared(&curve.poles, EntityId::ControlSpline(curve.id))
            }
            Entity::FitSpline(curve) => {
                self.require_unshared(&curve.stations, EntityId::FitSpline(curve.id))
            }
            Entity::QuadraticFitSpline(curve) => {
                self.require_unshared(&curve.stations, EntityId::FitSpline(curve.id))
            }
        }
    }

    fn check_line(&self, endpoints: [PointId; 2], entity: EntityId) -> Result<(), Failure> {
        let Some(crossing) = self.touching(&endpoints) else {
            return Ok(());
        };
        let [start, end, _] = crossing.points;
        if endpoints == [start, end] || endpoints == [end, start] {
            return Ok(());
        }
        Err(Failure::Arc(
            crossing.id,
            ArcConstraintFailure::CrossingSharedEntity(entity),
        ))
    }

    fn require_unshared(&self, points: &[PointId], entity: EntityId) -> Result<(), Failure> {
        if let Some(arc) = self.touching(points) {
            return Err(Failure::Arc(
                arc.id,
                ArcConstraintFailure::CrossingSharedEntity(entity),
            ));
        }
        Ok(())
    }

    fn touching(&self, points: &[PointId]) -> Option<ArcLayout> {
        points
            .iter()
            .find_map(|point| self.points.get(point).copied())
    }

    fn relation(&self, block: &EquationBlock) -> Option<(ArcId, RelationId)> {
        let arc = *point_columns(block).find_map(|column| self.slots.get(&column))?;
        Some((arc, block.relation_id()?))
    }
}

fn admitted_row(
    block: &EquationBlock,
    [start, end, witness]: [usize; 3],
    arc: ArcId,
    radii: &mut BTreeSet<ArcId>,
    axes: &mut BTreeSet<ArcId>,
) -> bool {
    // With one free chord endpoint, only a direction-locking row preserves
    // the connected radius family a² + (b − r)² = r² on its regular arc.
    match *block.source {
        Equation::Radius {
            branch: ArcBranch::CrossHalfTurn,
            ..
        } => point_columns(block).eq([start, end, witness]) && radii.insert(arc),
        Equation::Axis(..) => {
            let chord =
                point_columns(block).eq([start, end]) || point_columns(block).eq([end, start]);
            chord && axes.insert(arc)
        }
        _ => false,
    }
}

fn point_columns(block: &EquationBlock) -> impl Iterator<Item = usize> + '_ {
    block
        .variables
        .as_chunks::<3>()
        .0
        .iter()
        .map(|columns| columns[0])
}
