// SPDX-License-Identifier: GPL-2.0-only
//! Restore a fit tangent only when the retained predictor is also a feasible warm state.
use super::{
    BTreeMap, EquationBlock, Failure, Program, Relation, RelationId, Snapshot, poll,
    residual::Equation,
};
use martensite_tksolve::{ad::DualNumber, residual::ResidualSource};

impl Program {
    pub(in crate::constraints) fn verify_fit_replay(
        &self,
        geometry: &Snapshot,
        relations: &BTreeMap<RelationId, Relation>,
        tolerance: f64,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(), Failure> {
        if geometry.fit_splines.is_empty() {
            return Ok(());
        }
        let mut fits = self.blocks.iter().filter_map(fit_block).peekable();
        if fits.peek().is_none() {
            return Ok(());
        }
        let values = self.values(geometry)?;
        for (id, block) in fits {
            poll(cancelled)?;
            if !ready(block, &values, tolerance) {
                let relation = *relations.get(&id).ok_or(Failure::MissingRelation(id))?;
                return Err(Failure::Infeasible(id, relation));
            }
        }
        Ok(())
    }
}

fn fit_block(block: &EquationBlock) -> Option<(RelationId, &EquationBlock)> {
    if !matches!(*block.source, Equation::FitTangent { .. }) {
        return None;
    }
    Some((block.relation_id()?, block))
}

fn ready(block: &EquationBlock, accepted: &[f64], tolerance: f64) -> bool {
    let inputs = block
        .variables
        .iter()
        .map(|&slot| DualNumber::constant(accepted[slot]));
    let Ok(inputs) = inputs.collect::<Result<Vec<_>, _>>() else {
        return false;
    };
    block
        .source
        .evaluate(&inputs)
        .is_ok_and(|rows| rows.into_iter().all(|row| row.value().abs() <= tolerance))
}
