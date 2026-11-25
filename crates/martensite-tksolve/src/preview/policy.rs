// SPDX-License-Identifier: GPL-2.0-only

use super::{Block, PreviewError};

/// Structural and per-tick limits qualified by the caller for its residual program.
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    /// Maximum dimension used to compile a prepared program.
    pub coordinates: usize,
    /// Maximum authored block count.
    pub blocks: usize,
    /// Maximum number of declared rows across all blocks.
    pub equations: usize,
    /// Maximum local arity of any source, including repeated coordinates.
    pub block_inputs: usize,
    /// Maximum number of authored soft preferences per update.
    pub targets: usize,
    /// Maximum postings products across every Gram construction in one tick.
    pub gram_terms: usize,
    /// Maximum sparse factor updates across all factorizations in one tick.
    pub factor_updates: usize,
    /// Maximum combined numerical work charged to a tick.
    pub work: usize,
}

/// Quantity that prevented an interactive update.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Limit {
    /// Prepared coordinate count.
    Coordinates,
    /// Prepared source count.
    Blocks,
    /// Declared equation count.
    Equations,
    /// Declared arity of a single source.
    BlockInputs,
    /// Soft preference count.
    Targets,
    /// Charged products from row postings.
    GramTerms,
    /// Charged sparse factor updates.
    FactorUpdates,
    /// Combined charged work.
    Work,
}

/// An incomplete interactive computation leaves the prior snapshot with its caller.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Incomplete {
    /// A structural or per-tick quantity exceeded its selected allowance.
    Limit {
        /// Quantity that required additional work.
        quantity: Limit,
        /// Caller-selected maximum.
        limit: usize,
        /// Minimum amount needed to proceed with this step.
        requested: usize,
    },
    /// Sparse factorization could not separate full row rank at the selected tolerance.
    RankRecovery {
        /// Number of rows in the affected component.
        rows: usize,
        /// Number of free columns in that component.
        columns: usize,
    },
    /// The caller cancelled the update.
    Cancelled,
    /// The input snapshot did not satisfy the hard constraints.
    InfeasibleWarmState,
    /// A declined trial or hard constraint prevented further progress toward the preference.
    BlockedTarget,
    /// The nonlinear loop exhausted its iteration allowance.
    Iterations,
    /// The independent final evaluation exceeded the hard tolerance.
    FinalHardResidual {
        /// Observed maximum dimensionless hard error.
        observed: f64,
        /// Requested dimensionless hard tolerance.
        tolerance: f64,
    },
}

/// Work charged for a tick, including rejected evaluation and factor attempts.
#[derive(Clone, Copy, Debug, Default)]
pub struct Work {
    /// Sum of charged Gram products.
    pub gram_terms: usize,
    /// Sum of charged sparse factor updates.
    pub factor_updates: usize,
    /// Combined solver charge for the tick.
    pub work: usize,
}

impl Limits {
    pub(super) fn admit_layout(
        self,
        coordinates: usize,
        blocks: &[Block],
    ) -> Result<(), PreviewError> {
        bounded(Limit::Coordinates, self.coordinates, coordinates)?;
        bounded(Limit::Blocks, self.blocks, blocks.len())?;
        let mut rows = 0_usize;
        for block in blocks {
            bounded(Limit::BlockInputs, self.block_inputs, block.variables.len())?;
            rows = rows.saturating_add(block.equations);
            bounded(Limit::Equations, self.equations, rows)?;
        }
        Ok(())
    }

    pub(super) fn admit_targets(self, count: usize) -> Result<(), PreviewError> {
        bounded(Limit::Targets, self.targets, count)
    }
}

pub(super) struct Meter<'a> {
    cancelled: &'a mut dyn FnMut() -> bool,
    limits: Limits,
    spent: Work,
}

impl<'a> Meter<'a> {
    pub(super) fn new(limits: Limits, cancelled: &'a mut dyn FnMut() -> bool) -> Self {
        Self {
            cancelled,
            limits,
            spent: Work::default(),
        }
    }

    pub(super) fn poll(&mut self) -> Result<(), PreviewError> {
        if (self.cancelled)() {
            return Err(PreviewError::Incomplete(Incomplete::Cancelled));
        }
        Ok(())
    }

    pub(super) fn charge(&mut self, work: usize) -> Result<(), PreviewError> {
        self.poll()?;
        let requested = self.spent.work.saturating_add(work);
        bounded(Limit::Work, self.limits.work, requested)?;
        self.spent.work = requested;
        Ok(())
    }

    pub(super) fn gram(&mut self, count: usize) -> Result<(), PreviewError> {
        let requested = self.spent.gram_terms.saturating_add(count);
        bounded(Limit::GramTerms, self.limits.gram_terms, requested)?;
        self.spent.gram_terms = requested;
        self.charge(count)
    }

    pub(super) fn fill(&mut self, count: usize) -> Result<(), PreviewError> {
        let requested = self.spent.factor_updates.saturating_add(count);
        bounded(Limit::FactorUpdates, self.limits.factor_updates, requested)?;
        self.spent.factor_updates = requested;
        self.charge(count)
    }

    pub(super) fn work(&self) -> Work {
        self.spent
    }
}

fn bounded(quantity: Limit, limit: usize, requested: usize) -> Result<(), PreviewError> {
    if requested > limit {
        return Err(PreviewError::Incomplete(Incomplete::Limit {
            quantity,
            limit,
            requested,
        }));
    }
    Ok(())
}
