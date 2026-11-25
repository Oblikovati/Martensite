// SPDX-License-Identifier: GPL-2.0-only

//! Bounded warm drag for a prepared, immutable equation program.
//!
//! Prepare after a model edit and before pointer events. A typed model adapter must bind the
//! snapshot revision, ensure evaluators are immutable, and bound the cost of each evaluation.
//! A local derivative-width charge cannot bound arbitrary work inside a residual implementation.

mod factor;
mod iterate;
mod layout;
mod local;
mod policy;
mod rebind;

use crate::linalg::{ColumnPivotedQr, DenseMatrix, LinearAlgebraError};
use crate::nonlinear::{SolveError, SolveOptions};
use crate::residual::{EvaluationError, ResidualBlock, ResidualSource, ResidualTolerance};
use factor::Entries;
use layout::Component;
use local::LocalSeeds;
use martensite_cmath::hypot;
use std::{collections::BTreeMap, error::Error, fmt, sync::Arc};

pub use policy::{Incomplete, Limit, Limits, Work};
pub use rebind::SourceReplacement;

/// Frozen step scales expressed in each coordinate's own units.
#[derive(Clone, Copy, Debug)]
pub enum Domain {
    /// A model-space length coordinate.
    Length(f64),
    /// A continuous lifted curve or surface parameter.
    Parameter(f64),
}

impl Domain {
    fn scale(self) -> f64 {
        match self {
            Self::Length(scale) | Self::Parameter(scale) => scale,
        }
    }
}

/// A stable block in a prepared equation program. Sources supplied by the model must be immutable.
pub struct Block {
    /// Evaluator bound to the caller's immutable revision.
    pub source: Arc<dyn ResidualSource + Send + Sync>,
    /// Global coordinate index for each local coordinate, in source order.
    pub variables: Vec<usize>,
    /// Number of equations declared by the source.
    pub equations: usize,
}

#[derive(Clone)]
struct PreparedBlock {
    source: Arc<dyn ResidualSource + Send + Sync>,
    variables: Arc<[usize]>,
    equations: usize,
}

impl From<Block> for PreparedBlock {
    fn from(block: Block) -> Self {
        Self {
            source: block.source,
            variables: block.variables.into(),
            equations: block.equations,
        }
    }
}

impl PreparedBlock {
    fn borrowed(&self) -> ResidualBlock<'_> {
        ResidualBlock {
            source: self.source.as_ref(),
            variables: &self.variables,
            equations: self.equations,
        }
    }
}

/// A completed update whose hard constraints and soft target reached their requested tolerances.
#[derive(Debug)]
pub struct Update {
    /// Coordinates in the prepared input order.
    pub values: Vec<f64>,
    /// Independent hard constraints, including exact fixed coordinates.
    pub rank: usize,
    /// Remaining numerical directions after hard constraints.
    pub dof: usize,
    /// Largest final hard residual in its declared convergence domain.
    pub hard_error: f64,
    /// Rejected retractions or repair trials before completion.
    pub rejected_trials: usize,
    /// Work charged during the update.
    pub work: Work,
}

/// A preview failure leaves ownership of the last accepted snapshot with the caller.
#[derive(Clone, Debug, PartialEq)]
pub enum PreviewError {
    /// Numerical evaluation, linear algebra or request-policy failure.
    Solve(SolveError),
    /// The interactive contract could not produce a complete result.
    Incomplete(Incomplete),
}

impl fmt::Display for PreviewError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Solve(cause) => cause.fmt(output),
            Self::Incomplete(cause) => write!(output, "preview incomplete: {cause:?}"),
        }
    }
}

impl Error for PreviewError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Solve(cause) => Some(cause),
            Self::Incomplete(_) => None,
        }
    }
}

impl From<SolveError> for PreviewError {
    fn from(cause: SolveError) -> Self {
        Self::Solve(cause)
    }
}

impl From<EvaluationError> for PreviewError {
    fn from(cause: EvaluationError) -> Self {
        Self::Solve(cause.into())
    }
}

impl From<LinearAlgebraError> for PreviewError {
    fn from(cause: LinearAlgebraError) -> Self {
        Self::Solve(cause.into())
    }
}

/// A reusable program compiled and structurally admitted before interactive updates.
#[derive(Clone)]
pub struct Prepared {
    blocks: Arc<[PreparedBlock]>,
    domains: Arc<[Domain]>,
    fixed: Arc<[Option<f64>]>,
    components: Arc<[Component]>,
    length_scale: f64,
    options: SolveOptions,
    limits: Limits,
}

impl Prepared {
    /// Compile connected components and validate the structural limits once for a model revision.
    /// Example: `Prepared::new(blocks, domains, &[], length_scale, options, limits)?`.
    pub fn new(
        blocks: Vec<Block>,
        domains: Vec<Domain>,
        fixed: &[(usize, f64)],
        length_scale: f64,
        options: SolveOptions,
        limits: Limits,
    ) -> Result<Self, PreviewError> {
        limits.admit_layout(domains.len(), &blocks)?;
        validate_options(&domains, length_scale, options)?;
        let fixed = bind_constants(domains.len(), fixed)?;
        let components = layout::components(&blocks, &fixed)?;
        Ok(Self {
            blocks: blocks.into_iter().map(PreparedBlock::from).collect(),
            domains: domains.into(),
            fixed: fixed.into(),
            components: components.into(),
            length_scale,
            options,
            limits,
        })
    }

    /// Evaluate one bounded tick from a feasible warm state.
    /// Example: `program.update(&accepted, &[(0, 2.)], || cancelled)?`.
    pub fn update(
        &self,
        values: &[f64],
        targets: &[(usize, f64)],
        cancelled: impl FnMut() -> bool,
    ) -> Result<Update, PreviewError> {
        self.limits.admit_targets(targets.len())?;
        let values = self.read_values(values)?;
        let targets = self.read_targets(targets)?;
        iterate::run(self, values, &targets, cancelled)
    }

    /// Re-evaluate every equation with dense QR for explicit, noninteractive rank diagnostics.
    /// Example: `let (rank, dof) = program.inspect_rank(&accepted)?;`.
    pub fn inspect_rank(&self, values: &[f64]) -> Result<(usize, usize), PreviewError> {
        let values = self.read_values(values)?;
        let mut rank = self.fixed.iter().flatten().count();
        for component in self.components.iter() {
            let evaluated = self.evaluate(component, &values, |_| Ok(()))?;
            let mut rows = vec![vec![0.; component.free.len()]; evaluated.rows.len()];
            for (row, entries) in rows.iter_mut().zip(evaluated.rows) {
                for (index, coefficient) in entries {
                    row[index] = coefficient;
                }
            }
            rank += ColumnPivotedQr::factor(
                &DenseMatrix::new(component.free.len(), rows)?,
                self.options.rank_tolerance,
            )?
            .rank();
        }
        Ok((rank, values.len() - rank))
    }

    fn read_values(&self, values: &[f64]) -> Result<Vec<f64>, SolveError> {
        if values.len() != self.domains.len() || values.iter().any(|v| !v.is_finite()) {
            return Err(SolveError::Options(format!(
                "found {} coordinates: expected {} finite coordinates",
                values.len(),
                self.domains.len()
            )));
        }
        Ok(values
            .iter()
            .zip(self.fixed.iter())
            .map(|(&v, &fixed)| fixed.unwrap_or(v))
            .collect())
    }

    fn read_targets(&self, targets: &[(usize, f64)]) -> Result<BTreeMap<usize, f64>, SolveError> {
        let mut result = BTreeMap::new();
        for &(i, value) in targets {
            if i >= self.domains.len() || !value.is_finite() || result.insert(i, value).is_some() {
                return Err(SolveError::Options(format!(
                    "soft target {i}={value}: expected finite unique value in 0..{}",
                    self.domains.len()
                )));
            }
        }
        Ok(result)
    }

    fn evaluate(
        &self,
        component: &Component,
        values: &[f64],
        mut charge: impl FnMut(usize) -> Result<(), PreviewError>,
    ) -> Result<Rows, PreviewError> {
        let mut result = Rows::default();
        for (index, scope) in &component.rows {
            self.evaluate_block(component, values, (*index, scope), &mut charge, &mut result)?;
        }
        Ok(result)
    }

    fn evaluate_block(
        &self,
        component: &Component,
        values: &[f64],
        (index, scope): (usize, &LocalSeeds),
        charge: &mut impl FnMut(usize) -> Result<(), PreviewError>,
        result: &mut Rows,
    ) -> Result<(), PreviewError> {
        let block = self.blocks[index].borrowed();
        let width = scope.columns.len().saturating_add(1);
        let work = block.variables.len().saturating_add(block.equations);
        charge(width.saturating_mul(work))?;
        let rows = scope.evaluate(values, &block, index)?;
        for (row_index, row) in rows.iter().enumerate() {
            let entries = self.entries(component, &scope.columns, row.gradient());
            result.push(
                entries,
                row.value(),
                block.source.equation_tolerance(row_index),
                self.length_scale,
            )?;
        }
        Ok(())
    }

    fn entries(&self, component: &Component, columns: &[usize], gradient: &[f64]) -> Entries {
        columns
            .iter()
            .zip(gradient)
            .filter(|&(_, &v)| v != 0.)
            .map(|(&global, &v)| {
                (
                    component.free.binary_search(&global).unwrap_or(usize::MAX),
                    self.domains[global].scale() * v,
                )
            })
            .collect()
    }
}

fn bind_constants(count: usize, fixed: &[(usize, f64)]) -> Result<Vec<Option<f64>>, SolveError> {
    let mut constants = vec![None; count];
    for &(index, value) in fixed {
        if index >= count || !value.is_finite() || constants[index].is_some() {
            return Err(SolveError::Options(format!(
                "fixed coordinate {index}={value}: expected finite unique value in 0..{count}"
            )));
        }
        constants[index] = Some(value);
    }
    Ok(constants)
}

fn validate_options(
    domains: &[Domain],
    scale: f64,
    options: SolveOptions,
) -> Result<(), SolveError> {
    let positive = |value: f64| value.is_finite() && value > 0.;
    let admitted = [scale, options.rank_tolerance, options.relative_tolerance]
        .into_iter()
        .all(positive)
        && domains.iter().all(|domain| positive(domain.scale()))
        && options.max_iterations > 0
        && options.max_trial_steps > 0;
    if !admitted {
        return Err(SolveError::Options(format!(
            "preview scales and options {scale} {options:?}: expected finite positive scales and tolerances with nonzero work"
        )));
    }
    Ok(())
}

#[derive(Default)]
struct Rows {
    rows: Vec<Entries>,
    rhs: Vec<f64>,
    maximum: f64,
}

impl Rows {
    fn push(
        &mut self,
        mut row: Entries,
        residual: f64,
        units: ResidualTolerance,
        length: f64,
    ) -> Result<(), SolveError> {
        let norm = row
            .iter()
            .fold(0.0_f64, |sum, &(_, value)| hypot(sum, value));
        let physical = match units {
            ResidualTolerance::ModelLength => length,
            ResidualTolerance::Unitless => 1.,
            ResidualTolerance::ModelRelative => norm.max(1.),
        };
        let normalization = if norm > 0. { norm } else { physical };
        let convergence = (residual / physical).abs();
        let rhs = -residual / normalization;
        if !norm.is_finite()
            || !convergence.is_finite()
            || !rhs.is_finite()
            || row.iter().any(|&(i, _)| i == usize::MAX)
        {
            return Err(SolveError::Arithmetic(format!(
                "preview row residual {residual}, norm {norm}: expected finite arithmetic and admitted columns"
            )));
        }
        for (_, value) in &mut row {
            *value /= normalization;
        }
        self.maximum = self.maximum.max(convergence);
        self.rows.push(row);
        self.rhs.push(rhs);
        Ok(())
    }
}
