// SPDX-License-Identifier: GPL-2.0-only
//! Replace numerical sources while sharing the original prepared component layout.
use super::factor::Cholesky;
use super::{
    Incomplete, Prepared, PreviewError, ResidualSource, Rows, SolveError, Work, policy::Meter,
};
use std::sync::Arc;

/// A new immutable evaluator for an existing block's local inputs and equation slots.
pub struct SourceReplacement {
    /// Index in the original prepared block sequence.
    pub block: usize,
    /// New evaluator with the same local input, row count and convergence domains.
    pub source: Arc<dyn ResidualSource + Send + Sync>,
}

impl Prepared {
    /// Replace sorted, distinct source indices and validate their rows at the accepted coordinates.
    ///
    /// The block scopes, fixed coordinates, scale, and limits are shared with the original program.
    /// The original program and values remain available on refusal or cancellation. The caller
    /// passes the current tick's recorded charge to verify sparse rank in affected components.
    /// Pass `None` for noninteractive preparation, where base rank diagnostics stay explicit.
    /// Keep the returned total for any active tick.
    /// Example: `let (prepared, work, error) = prepared.with_rebound_sources(&changes, &values, Some(work), || false)?;`
    pub fn with_rebound_sources(
        &self,
        replacements: &[SourceReplacement],
        accepted: &[f64],
        spent: Option<Work>,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<(Self, Work, f64), PreviewError> {
        let mut meter = Meter::resume(self.limits, spent.unwrap_or_default(), &mut cancelled)?;
        meter.charge(accepted.len())?;
        let values = self.read_values(accepted)?;
        let prepared = self.rebind(replacements, &mut meter)?;
        let maximum =
            prepared.check_rebound_rows(replacements, &values, spent.is_some(), &mut meter)?;
        meter.poll()?;
        Ok((prepared, meter.work(), maximum))
    }

    fn rebind(
        &self,
        replacements: &[SourceReplacement],
        meter: &mut Meter<'_>,
    ) -> Result<Self, PreviewError> {
        self.check_replacement_order(replacements, meter)?;
        if replacements.is_empty() {
            return Ok(self.clone());
        }
        meter.charge(self.blocks.len())?;
        let mut blocks = self.blocks.to_vec();
        for replacement in replacements {
            let block = &mut blocks[replacement.block];
            meter.charge(block.equations.saturating_add(1))?;
            if !(0..block.equations).all(|row| {
                block.source.equation_tolerance(row) == replacement.source.equation_tolerance(row)
                    && block.source.equation_unit(row) == replacement.source.equation_unit(row)
            }) {
                return Err(SolveError::Options(format!(
                    "rebound block {}: expected original equation units and convergence domains",
                    replacement.block
                ))
                .into());
            }
            block.source = Arc::clone(&replacement.source);
        }
        Ok(Self {
            blocks: blocks.into(),
            ..self.clone()
        })
    }

    fn check_replacement_order(
        &self,
        replacements: &[SourceReplacement],
        meter: &mut Meter<'_>,
    ) -> Result<(), PreviewError> {
        let mut prior = None;
        for replacement in replacements {
            meter.charge(1)?;
            let block = replacement.block;
            if block >= self.blocks.len() || prior.is_some_and(|index| block <= index) {
                return Err(SolveError::Options(format!(
                    "rebound block {block} after {prior:?}: expected increasing indices below {}",
                    self.blocks.len()
                ))
                .into());
            }
            prior = Some(block);
        }
        Ok(())
    }

    fn check_rebound_rows(
        &self,
        replacements: &[SourceReplacement],
        values: &[f64],
        bounded_rank: bool,
        meter: &mut Meter<'_>,
    ) -> Result<f64, PreviewError> {
        let mut maximum: f64 = 0.;
        if replacements.is_empty() {
            return Ok(maximum);
        }
        for component in self.components.iter() {
            let selected = |index: &usize| {
                replacements
                    .binary_search_by_key(index, |change| change.block)
                    .is_ok()
            };
            meter.charge(component.rows.len().saturating_add(1))?;
            if component.rows.iter().any(|(index, _)| selected(index)) {
                let rows =
                    self.rebound_component(component, values, selected, bounded_rank, meter)?;
                maximum = maximum.max(rows.maximum);
            }
        }
        Ok(maximum)
    }

    fn rebound_component(
        &self,
        component: &super::Component,
        values: &[f64],
        selected: impl Fn(&usize) -> bool,
        bounded_rank: bool,
        meter: &mut Meter<'_>,
    ) -> Result<Rows, PreviewError> {
        let mut rows = Rows::default();
        for (index, scope) in &component.rows {
            meter.poll()?;
            if bounded_rank || selected(index) {
                self.evaluate_block(
                    component,
                    values,
                    (*index, scope),
                    &mut |work| meter.charge(work),
                    &mut rows,
                )?;
            }
        }
        if rows.maximum > self.options.relative_tolerance {
            return Err(PreviewError::Incomplete(Incomplete::InfeasibleWarmState));
        }
        if bounded_rank {
            Cholesky::new(
                &rows.rows,
                component.free.len(),
                self.options.rank_tolerance,
                meter,
            )?;
        }
        Ok(rows)
    }
}
