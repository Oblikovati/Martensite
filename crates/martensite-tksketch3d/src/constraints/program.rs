// SPDX-License-Identifier: GPL-2.0-only
use super::{
    ConstraintFailure as Failure, ConstraintPolicy, ConstraintRank, Relation, RelationId,
    adopt::apply_checked,
    binding::{Binding, BlockOwner, EquationBlock},
    continuity::JoinReference,
    poll, residual,
};
use crate::{Edit, EntityId, PointId, Snapshot};
use martensite_tkmath::gp::Point3;
use martensite_tksolve::{
    nonlinear::{SolveOptions, SolveTermination, solve_constraints_with_constants},
    preview::{Block, Domain, Incomplete, PreferenceReport, Prepared, PreviewError, Update},
    residual::ResidualBlock,
};
use std::{borrow::Cow, collections::BTreeMap, sync::Arc};

mod admission;
mod chart;
mod failure;
use admission::admit;
use failure::{preview_failure, solve_failure};

#[derive(Clone)]
pub(super) struct Program {
    points: Arc<[PointId]>,
    point_columns: Arc<BTreeMap<PointId, usize>>,
    fixed: Arc<[(usize, f64)]>,
    blocks: Arc<[EquationBlock]>,
    chart_sources: Arc<[(usize, RelationId, JoinReference)]>,
    authored_rows: usize,
    frozen_coordinates: usize,
    prepared: Prepared,
    attachment_block: Option<usize>,
}

impl Program {
    pub(super) fn new(
        geometry: &Snapshot,
        relations: &BTreeMap<RelationId, Relation>,
        policy: ConstraintPolicy,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Self, Failure> {
        Self::compile(geometry, relations, policy, None, cancelled)
    }

    pub(super) fn compile(
        geometry: &Snapshot,
        relations: &BTreeMap<RelationId, Relation>,
        policy: ConstraintPolicy,
        attachment: Option<(RelationId, PointId, super::on_face::AttachmentEquation)>,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Self, Failure> {
        admit(geometry, relations, policy, attachment.is_some())?;
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
            mut blocks,
            mut authored_rows,
            frozen_coordinates,
        } = super::binding::bind(geometry, relations, &point_columns, policy, cancelled)?;
        let mut domains = vec![Domain::Length(policy.length_scale.value()); 3 * points.len()];
        let attachment_block = attachment.map(|_| blocks.len());
        if let Some((id, point, equation)) = attachment {
            let start = *point_columns
                .get(&point)
                .ok_or(Failure::MissingPoint(point))?;
            let uv = domains.len();
            blocks.push(EquationBlock {
                owner: BlockOwner::Relation(id),
                source: Arc::new(residual::Equation::SurfaceAttachment(equation)),
                variables: [start, start + 1, start + 2, uv, uv + 1].into(),
            });
            authored_rows += 3;
            domains.extend(equation.steps().map(Domain::Parameter));
        }
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
            attachment_block,
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
            .map_err(|cause| solve_failure(&self.blocks, cause))?;
        match result.termination {
            SolveTermination::Converged => Ok(result.values),
            SolveTermination::Cancelled => Err(Failure::Cancelled),
            other => Err(Failure::Unsettled(
                other,
                result
                    .conflicts
                    .into_iter()
                    .filter_map(|i| self.blocks.get(i).and_then(EquationBlock::relation_id))
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
        self.prefer_with_uv(
            (geometry, None),
            (point, target),
            tangent_tolerance,
            cancelled,
        )
    }

    pub(super) fn prefer_with_uv(
        &self,
        (geometry, uv): (&Snapshot, Option<[f64; 2]>),
        (point, target): (PointId, Point3),
        tangent_tolerance: f64,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(Update, PreferenceReport), Failure> {
        let targets = self.cursor_targets(geometry, point, target)?;
        self.prepared
            .update_preferred(
                &self.values_with_uv(geometry, uv)?,
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
        if geometry.point_is_read_only(point) {
            let original = geometry
                .point(point)
                .ok_or(Failure::MissingPoint(point))?
                .position;
            if target.map(f64::to_bits) != original.coordinates().map(f64::to_bits) {
                return Err(read_only_drag(geometry, point));
            }
        }
        Ok([
            (base, target[0]),
            (base + 1, target[1]),
            (base + 2, target[2]),
        ])
    }

    pub(super) fn verify_attachment_state(
        &self,
        geometry: &Snapshot,
        uv: [f64; 2],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(), Failure> {
        let values = self.values_with_uv(geometry, Some(uv))?;
        // No cursor targets means a complete warm-state replay without motion.
        let (verified, _) = self
            .prepared
            .update_preferred(&values, &[], 1., cancelled)
            .map_err(|error| preview_failure(&self.blocks, error))?;
        if values
            .iter()
            .zip(&verified.values)
            .any(|(before, after)| before.to_bits() != after.to_bits())
        {
            return Err(Failure::Preview(
                None,
                PreviewError::Incomplete(Incomplete::InfeasibleWarmState),
            ));
        }
        Ok(())
    }

    pub(super) fn inspect(&self, geometry: &Snapshot) -> Result<ConstraintRank, Failure> {
        self.inspect_with_uv(geometry, None)
    }

    pub(super) fn inspect_with_uv(
        &self,
        geometry: &Snapshot,
        uv: Option<[f64; 2]>,
    ) -> Result<ConstraintRank, Failure> {
        let (rank, dof) = self
            .prepared
            .inspect_rank(&self.values_with_uv(geometry, uv)?)
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
        self.values_with_uv(geometry, None)
    }

    pub(super) fn values_with_uv(
        &self,
        geometry: &Snapshot,
        uv: Option<[f64; 2]>,
    ) -> Result<Vec<f64>, Failure> {
        let expected = self.value_count();
        let mut values = Vec::with_capacity(expected);
        for &id in self.points.iter() {
            values.extend(
                geometry
                    .point(id)
                    .ok_or(Failure::MissingPoint(id))?
                    .position
                    .coordinates(),
            );
        }
        values.extend(uv.into_iter().flatten());
        if values.len() != expected {
            return Err(Failure::ReturnedCoordinates {
                expected,
                actual: values.len(),
            });
        }
        Ok(values)
    }

    fn value_count(&self) -> usize {
        3 * self.points.len() + 2 * usize::from(self.attachment_block.is_some())
    }

    pub(super) fn set_point_and_uv(
        &self,
        point: PointId,
        xyz: [f64; 3],
        uv: [f64; 2],
        values: &mut [f64],
    ) -> Result<(), Failure> {
        let start = *self
            .point_columns
            .get(&point)
            .ok_or(Failure::MissingPoint(point))?;
        self.parameter_values(values)?;
        values[start..start + 3].copy_from_slice(&xyz);
        values[3 * self.points.len()..].copy_from_slice(&uv);
        Ok(())
    }

    pub(super) fn parameter_values(&self, values: &[f64]) -> Result<[f64; 2], Failure> {
        let Some([u, v]) = values.get(3 * self.points.len()..) else {
            return Err(Failure::ReturnedCoordinates {
                expected: self.value_count(),
                actual: values.len(),
            });
        };
        Ok([*u, *v])
    }

    pub(super) fn adopt(
        &self,
        geometry: &Snapshot,
        relations: &BTreeMap<RelationId, Relation>,
        values: &[f64],
        policy: ConstraintPolicy,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Snapshot, Failure> {
        let expected = self.value_count();
        if values.len() != expected {
            return Err(Failure::ReturnedCoordinates {
                expected,
                actual: values.len(),
            });
        }
        let mut edits = Vec::new();
        for (&point, &xyz) in self
            .points
            .iter()
            .zip(values[..3 * self.points.len()].as_chunks::<3>().0)
        {
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

fn read_only_drag(geometry: &Snapshot, point: PointId) -> Failure {
    if let Some(source) = geometry.frozen_source(point) {
        return Failure::FrozenDrag(point, source);
    }
    geometry
        .included_point_source(point)
        .map_or(Failure::MissingPoint(point), |source| {
            Failure::IncludedDrag(point, source)
        })
}
