// SPDX-License-Identifier: GPL-2.0-only
use super::{
    ConstraintFailure as Failure, ConstraintPolicy, ConstraintRank, PolicyQuantity, Relation,
    RelationId,
    adopt::apply_checked,
    binding::{Binding, EquationBlock},
    continuity::JoinReference,
    poll, residual,
};
use crate::{Edit, EntityId, PointId, Snapshot};
use martensite_tkmath::gp::Point3;
use martensite_tksolve::{
    nonlinear::{SolveError, SolveOptions, SolveTermination, solve_constraints_with_constants},
    preview::{Block, Domain, Incomplete, PreferenceReport, Prepared, PreviewError, Update},
    residual::{EvaluationError, ResidualBlock},
};
use std::{borrow::Cow, collections::BTreeMap, sync::Arc};

mod chart;

#[derive(Clone)]
pub(super) struct Program {
    points: Arc<[PointId]>,
    point_columns: Arc<BTreeMap<PointId, usize>>,
    fixed: Arc<[(usize, f64)]>,
    blocks: Arc<[EquationBlock]>,
    chart_sources: Arc<[(usize, JoinReference)]>,
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
                EntityId::Line(_)
                | EntityId::Arc(_)
                | EntityId::ControlSpline(_)
                | EntityId::FitSpline(_) => None,
            })
            .collect::<Vec<_>>();
        let point_columns = points
            .iter()
            .enumerate()
            .map(|(i, point)| (*point, 3 * i))
            .collect::<BTreeMap<_, _>>();
        let Binding {
            fixed,
            blocks,
            authored_rows,
            frozen_coordinates,
        } = super::binding::bind(geometry, relations, &point_columns, policy)?;
        let domains = vec![Domain::Length(policy.length_scale.value()); 3 * points.len()];
        let mut limits = policy.preview;
        limits.work = limits
            .work
            .min(residual::numeric_budget(policy.residual_work));
        let bound_blocks = blocks
            .iter()
            .map(|block| Block {
                source: block.source.clone(),
                variables: block.variables.to_vec(),
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
        let chart_sources = chart::indexed(&blocks, relations);
        Ok(Self {
            points: points.into(),
            point_columns: Arc::new(point_columns),
            fixed: fixed.into(),
            blocks: blocks.into(),
            chart_sources,
            authored_rows,
            frozen_coordinates,
            prepared,
        })
    }

    pub(super) fn establish(
        &self,
        geometry: &Snapshot,
        references: impl Iterator<Item = (RelationId, JoinReference)>,
        options: SolveOptions,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Vec<f64>, Failure> {
        poll(cancelled)?;
        let mut values = self.values(geometry)?;
        let fixed = self.authoring_fixed(geometry, references, &values, cancelled)?;
        super::initialization::initialize(
            &mut values,
            &self.blocks,
            &fixed,
            options.rank_tolerance,
            geometry,
            &self.point_columns,
            cancelled,
        )?;
        let blocks = self
            .blocks
            .iter()
            .map(|block| ResidualBlock {
                source: block.source.as_ref(),
                variables: &block.variables,
                equations: block.source.rows(),
            })
            .collect::<Vec<_>>();
        let result = solve_constraints_with_constants(&values, &blocks, &fixed, options, cancelled)
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

    fn authoring_fixed(
        &self,
        geometry: &Snapshot,
        mut references: impl Iterator<Item = (RelationId, JoinReference)>,
        values: &[f64],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Cow<'_, [(usize, f64)]>, Failure> {
        let Some(first) = references.next() else {
            return Ok(Cow::Borrowed(&self.fixed));
        };
        let mut fixed = self.fixed.iter().copied().collect::<BTreeMap<_, _>>();
        for (relation, reference) in std::iter::once(first).chain(references) {
            poll(cancelled)?;
            for point in reference.authoring_points(geometry, relation)? {
                let start = *self
                    .point_columns
                    .get(&point)
                    .ok_or(Failure::MissingPoint(point))?;
                pin_columns(&mut fixed, start, values);
            }
        }
        Ok(Cow::Owned(fixed.into_iter().collect()))
    }

    pub(super) fn drag(
        &self,
        geometry: &Snapshot,
        point: PointId,
        target: Point3,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Update, Failure> {
        let targets = self.cursor_targets(geometry, point, target)?;
        self.prepared
            .update(&self.values(geometry)?, &targets, cancelled)
            .map_err(|error| preview_failure(&self.blocks, error))
    }

    pub(super) fn drag_preferred(
        &self,
        geometry: &Snapshot,
        point: PointId,
        target: Point3,
        tangent_tolerance: f64,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(Update, PreferenceReport), Failure> {
        let targets = self.cursor_targets(geometry, point, target)?;
        self.prepared
            .update_preferred(
                &self.values(geometry)?,
                &targets,
                tangent_tolerance,
                cancelled,
            )
            .map_err(|error| preview_failure(&self.blocks, error))
    }

    pub(super) fn recheck_preferred(
        &self,
        geometry: &Snapshot,
        cursor: (PointId, Point3),
        output: Update,
        tangent_tolerance: f64,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(Update, PreferenceReport), Failure> {
        let (point, target) = cursor;
        let targets = self.cursor_targets(geometry, point, target)?;
        self.prepared
            .verify_preferred(output, &targets, tangent_tolerance, cancelled)
            .map_err(|error| preview_failure(&self.blocks, error))
    }

    fn cursor_targets(
        &self,
        geometry: &Snapshot,
        point: PointId,
        target: Point3,
    ) -> Result<[(usize, f64); 3], Failure> {
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
        Ok([
            (base, target[0]),
            (base + 1, target[1]),
            (base + 2, target[2]),
        ])
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
        for &id in self.points.iter() {
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
        apply_checked(geometry, relations, &edits, policy, cancelled)
    }
}

fn pin_columns(fixed: &mut BTreeMap<usize, f64>, start: usize, values: &[f64]) {
    for (offset, &value) in values[start..start + 3].iter().enumerate() {
        fixed.entry(start + offset).or_insert(value);
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
    if let Some(&spline) = geometry.fit_splines.keys().next() {
        return Err(Failure::UnsupportedEntity(EntityId::FitSpline(spline)));
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
