// SPDX-License-Identifier: GPL-2.0-only
use super::{
    ConstraintFailure as Failure, ConstraintPolicy, ConstraintRank, PolicyQuantity, Relation,
    RelationId, RelationSpec, metric, poll,
    residual::{self, Equation},
};
use crate::{Edit, EditFailure, EntityId, LineGeometry, LineId, PointId, Snapshot};
use martensite_tkmath::gp::Point3;
use martensite_tksolve::{
    nonlinear::{SolveError, SolveOptions, SolveTermination, solve_constraints_with_constants},
    preview::{Block, Domain, Incomplete, Prepared, PreviewError, Update},
    residual::{EvaluationError, ResidualBlock},
};
use std::{collections::BTreeMap, sync::Arc};

struct EquationBlock {
    id: RelationId,
    source: Arc<Equation>,
    variables: Vec<usize>,
}

pub(super) struct Program {
    points: Vec<PointId>,
    point_columns: BTreeMap<PointId, usize>,
    fixed: Vec<(usize, f64)>,
    blocks: Vec<EquationBlock>,
    authored_rows: usize,
    frozen_coordinates: usize,
    prepared: Prepared,
}

impl Program {
    pub(super) fn new(
        geometry: &Snapshot,
        relations: &BTreeMap<RelationId, Relation>,
        policy: ConstraintPolicy,
    ) -> Result<Self, Failure> {
        admit(geometry, relations, policy)?;
        let points = geometry
            .owned_ids()
            .filter_map(|key| match key {
                EntityId::Point(point) => Some(point),
                EntityId::Line(_) | EntityId::Arc(_) => None,
            })
            .collect::<Vec<_>>();
        let point_columns = points
            .iter()
            .enumerate()
            .map(|(i, point)| (*point, 3 * i))
            .collect::<BTreeMap<_, _>>();
        let mut builder = Layout {
            geometry,
            point_columns: &point_columns,
            fixed: BTreeMap::new(),
            blocks: Vec::new(),
            authored_rows: 0,
            scale: 1. / policy.length_scale.value(),
        };
        for (&id, &start) in &point_columns {
            if geometry.frozen_source(id).is_some() {
                let point = geometry.point(id).ok_or(Failure::MissingPoint(id))?;
                builder
                    .fixed
                    .extend((start..start + 3).zip(*point.position.coordinates()));
            }
        }
        let frozen_coordinates = builder.fixed.len();
        for (&id, &relation) in relations {
            builder.bind(id, relation)?;
        }
        let (blocks, authored_rows) = (builder.blocks, builder.authored_rows);
        let fixed = builder.fixed.into_iter().collect::<Vec<_>>();
        let domains = vec![Domain::Length(policy.length_scale.value()); 3 * points.len()];
        let mut limits = policy.preview;
        limits.work = limits
            .work
            .min(residual::numeric_budget(policy.residual_work));
        let bound_blocks = blocks
            .iter()
            .map(|block| Block {
                source: block.source.clone(),
                variables: block.variables.clone(),
                equations: block.source.rows(),
            })
            .collect();
        let prepared = Prepared::new(
            bound_blocks,
            domains,
            &fixed,
            policy.length_scale.value(),
            policy.solve,
            limits,
        )
        .map_err(|error| preview_failure(&blocks, error))?;
        Ok(Self {
            points,
            point_columns,
            fixed,
            blocks,
            authored_rows,
            frozen_coordinates,
            prepared,
        })
    }

    pub(super) fn establish(
        &self,
        geometry: &Snapshot,
        options: SolveOptions,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Vec<f64>, Failure> {
        poll(cancelled)?;
        let values = self.values(geometry)?;
        let blocks = self
            .blocks
            .iter()
            .map(|block| ResidualBlock {
                source: block.source.as_ref(),
                variables: &block.variables,
                equations: block.source.rows(),
            })
            .collect::<Vec<_>>();
        let result =
            solve_constraints_with_constants(&values, &blocks, &self.fixed, options, cancelled)
                .map_err(|cause| Failure::Solve(failure_relation(&self.blocks, &cause), cause))?;
        match result.termination {
            SolveTermination::Converged => Ok(result.values),
            SolveTermination::Cancelled => Err(Failure::Cancelled),
            other => Err(Failure::Unsettled(
                other,
                result
                    .conflicts
                    .into_iter()
                    .filter_map(|i| self.blocks.get(i).map(|block| block.id))
                    .collect(),
            )),
        }
    }

    pub(super) fn drag(
        &self,
        geometry: &Snapshot,
        point: PointId,
        target: Point3,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Update, Failure> {
        let base = *self
            .point_columns
            .get(&point)
            .ok_or(Failure::MissingPoint(point))?;
        let target = *target.coordinates();
        if target.iter().any(|value| !value.is_finite()) {
            return Err(Failure::InvalidTarget(point, target));
        }
        if let Some(source) = geometry.frozen_source(point) {
            let original = geometry
                .point(point)
                .ok_or(Failure::MissingPoint(point))?
                .position;
            if target.map(f64::to_bits) != original.coordinates().map(f64::to_bits) {
                return Err(Failure::FrozenDrag(point, source));
            }
        }
        let targets = [
            (base, target[0]),
            (base + 1, target[1]),
            (base + 2, target[2]),
        ];
        self.prepared
            .update(&self.values(geometry)?, &targets, cancelled)
            .map_err(|error| preview_failure(&self.blocks, error))
    }

    pub(super) fn inspect(&self, geometry: &Snapshot) -> Result<ConstraintRank, Failure> {
        let (rank, dof) = self
            .prepared
            .inspect_rank(&self.values(geometry)?)
            .map_err(|error| preview_failure(&self.blocks, error))?;
        Ok(self.rank_report(rank, dof))
    }

    pub(super) fn rank_report(&self, rank: usize, dof: usize) -> ConstraintRank {
        ConstraintRank {
            rank,
            degrees_of_freedom: dof,
            equations: self.authored_rows,
            frozen_coordinates: self.frozen_coordinates,
        }
    }

    fn values(&self, geometry: &Snapshot) -> Result<Vec<f64>, Failure> {
        let mut values = Vec::with_capacity(3 * self.points.len());
        for &id in &self.points {
            values.extend(
                geometry
                    .point(id)
                    .ok_or(Failure::MissingPoint(id))?
                    .position
                    .coordinates(),
            );
        }
        Ok(values)
    }

    pub(super) fn adopt(
        &self,
        geometry: &Snapshot,
        relations: &BTreeMap<RelationId, Relation>,
        values: &[f64],
        policy: ConstraintPolicy,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Snapshot, Failure> {
        let expected = 3 * self.points.len();
        if values.len() != expected {
            return Err(Failure::ReturnedCoordinates {
                expected,
                actual: values.len(),
            });
        }
        let mut edits = Vec::new();
        for (&point, &xyz) in self.points.iter().zip(values.as_chunks::<3>().0) {
            poll(cancelled)?;
            let position = geometry
                .point(point)
                .ok_or(Failure::MissingPoint(point))?
                .position;
            if position.coordinates().map(f64::to_bits) != xyz.map(f64::to_bits) {
                edits.push(Edit::MovePoint {
                    point,
                    position: Point3::new(xyz[0], xyz[1], xyz[2]),
                });
            }
        }
        let next = geometry
            .apply(&edits, &mut *cancelled)
            .map_err(|error| match error {
                EditFailure::Cancelled(_) => Failure::Cancelled,
                error => Failure::Geometry(error),
            })?;
        for (&id, &relation) in relations {
            poll(cancelled)?;
            if !metric::feasible(&next, relation, policy) {
                return Err(Failure::Infeasible(id, relation));
            }
        }
        poll(cancelled)?;
        Ok(next)
    }
}

struct Layout<'a> {
    geometry: &'a Snapshot,
    point_columns: &'a BTreeMap<PointId, usize>,
    fixed: BTreeMap<usize, f64>,
    blocks: Vec<EquationBlock>,
    authored_rows: usize,
    scale: f64,
}

impl Layout<'_> {
    fn bind(&mut self, id: RelationId, relation: Relation) -> Result<(), Failure> {
        match relation {
            Relation::FixPoint(point, anchor) => self.fix(id, point, anchor),
            Relation::FixLine(line, anchors) => {
                let endpoints = line_of(self.geometry, id, line)?.line.endpoints;
                self.fix(id, endpoints[0], anchors[0])?;
                self.fix(id, endpoints[1], anchors[1])
            }
            Relation::Coincident(points) => {
                self.equation(id, &points, Equation::Coincident { scale: self.scale })
            }
            Relation::Length(line, target) => {
                let points = line_of(self.geometry, id, line)?.line.endpoints;
                self.equation(
                    id,
                    &points,
                    Equation::Length {
                        scale: self.scale,
                        target: target.value(),
                    },
                )
            }
            Relation::Parallel(lines, direction) => {
                let [a, b] = [
                    line_of(self.geometry, id, lines[0])?,
                    line_of(self.geometry, id, lines[1])?,
                ];
                let [a0, a1] = a.line.endpoints;
                let [b0, b1] = b.line.endpoints;
                self.equation(
                    id,
                    &[a0, a1, b0, b1],
                    Equation::Parallel {
                        direction,
                        reference: reference_axis(a),
                    },
                )
            }
        }
    }

    fn fix(&mut self, id: RelationId, point: PointId, position: Point3) -> Result<(), Failure> {
        let start = self.start(id, point)?;
        for (axis, &value) in position.coordinates().iter().enumerate() {
            if let Some(previous) = self.fixed.insert(start + axis, value)
                && previous.to_bits() != value.to_bits()
            {
                return Err(Failure::ConflictingFix(id, point));
            }
        }
        self.authored_rows += 3;
        Ok(())
    }

    fn equation(
        &mut self,
        id: RelationId,
        points: &[PointId],
        source: Equation,
    ) -> Result<(), Failure> {
        let mut variables = Vec::with_capacity(3 * points.len());
        for &point in points {
            let start = self.start(id, point)?;
            variables.extend(start..start + 3);
        }
        self.authored_rows += source.rows();
        self.blocks.push(EquationBlock {
            id,
            source: Arc::new(source),
            variables,
        });
        Ok(())
    }

    fn start(&self, id: RelationId, point: PointId) -> Result<usize, Failure> {
        self.point_columns
            .get(&point)
            .copied()
            .ok_or(Failure::MissingOperand(id, EntityId::Point(point)))
    }
}

fn reference_axis(line: LineGeometry) -> usize {
    let direction = line.carrier.direction();
    let [x, y, z] = direction.coordinates().map(f64::abs);
    if x <= y && x <= z {
        0
    } else if y <= z {
        1
    } else {
        2
    }
}

fn admit(
    geometry: &Snapshot,
    relations: &BTreeMap<RelationId, Relation>,
    policy: ConstraintPolicy,
) -> Result<(), Failure> {
    let sine = policy.parallel_sine_tolerance;
    if !(sine > 0. && sine < 1.) {
        return Err(Failure::InvalidParallelSine(sine));
    }
    if !(1. / policy.length_scale.value()).is_finite() {
        return Err(Failure::InvalidLengthScale(policy.length_scale.value()));
    }
    bound(
        PolicyQuantity::ValidationItems,
        geometry.entities.len().saturating_add(relations.len()),
        policy.validation_items,
    )?;
    if let Some(&arc) = geometry.arcs.keys().next() {
        return Err(Failure::UnsupportedEntity(EntityId::Arc(arc)));
    }
    let points = geometry
        .owned_ids()
        .filter(|entity| matches!(entity, EntityId::Point(_)))
        .count();
    bound(
        PolicyQuantity::Coordinates,
        points.saturating_mul(3),
        policy.preview.coordinates,
    )
}

fn bound(quantity: PolicyQuantity, requested: usize, maximum: usize) -> Result<(), Failure> {
    if requested > maximum {
        return Err(Failure::Limit {
            quantity,
            requested,
            maximum,
        });
    }
    Ok(())
}

pub(super) fn capture(
    geometry: &Snapshot,
    id: RelationId,
    spec: RelationSpec,
) -> Result<Relation, Failure> {
    let point = |key| {
        geometry
            .point(key)
            .map(|point| point.position)
            .ok_or(Failure::MissingOperand(id, EntityId::Point(key)))
    };
    let line = |key| line_of(geometry, id, key);
    match spec {
        RelationSpec::FixPoint(key) => Ok(Relation::FixPoint(key, point(key)?)),
        RelationSpec::FixLine(key) => Ok(Relation::FixLine(key, line(key)?.points)),
        RelationSpec::Coincident([a, b]) => {
            distinct(id, EntityId::Point(a), EntityId::Point(b))?;
            point(a)?;
            point(b)?;
            Ok(Relation::Coincident([a, b]))
        }
        RelationSpec::Parallel([a, b], direction) => {
            distinct(id, EntityId::Line(a), EntityId::Line(b))?;
            line(a)?;
            line(b)?;
            Ok(Relation::Parallel([a, b], direction))
        }
        RelationSpec::Length(key, target) => {
            line(key)?;
            if target.value() < geometry.minimum.value() {
                return Err(Failure::LengthBelowMinimum(id, target.value()));
            }
            Ok(Relation::Length(key, target))
        }
    }
}

fn line_of(geometry: &Snapshot, id: RelationId, line: LineId) -> Result<LineGeometry, Failure> {
    geometry
        .line(line)
        .ok_or(Failure::MissingOperand(id, EntityId::Line(line)))
}

fn distinct(id: RelationId, a: EntityId, b: EntityId) -> Result<(), Failure> {
    if a == b {
        return Err(Failure::RepeatedOperand(id, a));
    }
    Ok(())
}

fn failure_relation(blocks: &[EquationBlock], error: &SolveError) -> Option<RelationId> {
    let SolveError::Evaluation(EvaluationError::Source { block, .. }) = error else {
        return None;
    };
    blocks.get(*block).map(|block| block.id)
}

fn preview_failure(blocks: &[EquationBlock], error: PreviewError) -> Failure {
    match error {
        PreviewError::Incomplete(Incomplete::Cancelled) => Failure::Cancelled,
        PreviewError::Solve(ref cause) => Failure::Preview(failure_relation(blocks, cause), error),
        error => Failure::Preview(None, error),
    }
}
