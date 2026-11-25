// SPDX-License-Identifier: GPL-2.0-only
//! Refresh direction charts between accepted states, without compiling another layout.
use super::{
    Arc, BTreeMap, ConstraintPolicy, EquationBlock, Failure, Incomplete, JoinReference,
    PreviewError, Program, Relation, RelationId, Snapshot, poll, preview_failure, residual,
};
use martensite_tksolve::preview::{Limit, SourceReplacement, Work};

pub(in crate::constraints) struct Reframed {
    pub program: Arc<Program>,
    pub work: Work,
    pub hard_error: f64,
}

impl Program {
    pub(in crate::constraints) fn reframe(
        self: &Arc<Self>,
        geometry: &Snapshot,
        policy: ConstraintPolicy,
        spent: Option<Work>,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Reframed, Failure> {
        self.reframe_with_uv(geometry, None, policy, spent, cancelled)
    }

    pub(in crate::constraints) fn reframe_with_uv(
        self: &Arc<Self>,
        geometry: &Snapshot,
        uv: Option<[f64; 2]>,
        policy: ConstraintPolicy,
        spent: Option<Work>,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Reframed, Failure> {
        let mut work = spent.unwrap_or_default();
        if let Some(spent) = spent {
            work = charge(policy, spent, self.chart_sources.len())?;
        }
        let updates = self.collect_frames(geometry, cancelled)?;
        if updates.is_empty() {
            return Ok(Reframed {
                program: self.clone(),
                work,
                hard_error: 0.,
            });
        }
        self.replace_frames(
            (geometry, uv),
            updates,
            policy,
            spent.map(|_| work),
            cancelled,
        )
    }

    fn collect_frames(
        &self,
        geometry: &Snapshot,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Vec<(usize, Arc<residual::Equation>)>, Failure> {
        let mut updates = Vec::new();
        for &(index, reference) in self.chart_sources.iter() {
            poll(cancelled)?;
            let block = &self.blocks[index];
            let direction = reference.chart_direction(geometry, block.id)?;
            if let Some(next) = block.source.refreshed_chart(direction) {
                updates.push((index, Arc::new(next)));
            }
        }
        Ok(updates)
    }

    fn replace_frames(
        &self,
        (geometry, uv): (&Snapshot, Option<[f64; 2]>),
        updates: Vec<(usize, Arc<residual::Equation>)>,
        policy: ConstraintPolicy,
        spent: Option<Work>,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Reframed, Failure> {
        let adapter_work = self
            .blocks
            .len()
            .saturating_add(self.points.len())
            .saturating_add(updates.len());
        let work = spent
            .map(|work| charge(policy, work, adapter_work))
            .transpose()?;
        let mut blocks = self.blocks.to_vec();
        let mut replacements = Vec::with_capacity(updates.len());
        for (block, source) in updates {
            poll(cancelled)?;
            blocks[block].source = source.clone();
            replacements.push(SourceReplacement { block, source });
        }
        let (prepared, work, hard_error) = self
            .prepared
            .with_rebound_sources(
                &replacements,
                &self.values_with_uv(geometry, uv)?,
                work,
                cancelled,
            )
            .map_err(|error| preview_failure(&blocks, error))?;
        Ok(Reframed {
            program: Arc::new(Self {
                prepared,
                blocks: blocks.into(),
                ..self.clone()
            }),
            work,
            hard_error,
        })
    }
}

pub(super) fn indexed(
    blocks: &[EquationBlock],
    relations: &BTreeMap<RelationId, Relation>,
) -> Arc<[(usize, JoinReference)]> {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(index, block)| match *relations.get(&block.id)? {
            Relation::PointOnLine(_, line) | Relation::Colinear([line, _], _) => {
                Some((index, JoinReference::Line(line)))
            }
            Relation::Tangent(join) | Relation::G2(join, _) => Some((index, join.reference())),
            _ => None,
        })
        .collect()
}

fn charge(policy: ConstraintPolicy, mut spent: Work, work: usize) -> Result<Work, Failure> {
    let limit = policy
        .preview
        .work
        .min(residual::numeric_budget(policy.residual_work));
    let requested = spent.work.saturating_add(work);
    if requested > limit {
        return Err(Failure::Preview(
            None,
            PreviewError::Incomplete(Incomplete::Limit {
                quantity: Limit::Work,
                limit,
                requested,
            }),
        ));
    }
    spent.work = requested;
    Ok(spent)
}
