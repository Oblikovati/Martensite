// SPDX-License-Identifier: GPL-2.0-only

use super::{Incomplete, PreviewError, policy::Meter};
use crate::linalg::{ColumnPivotedQr, DenseMatrix};
use std::collections::BTreeMap;

pub(super) type Entries = Vec<(usize, f64)>;

pub(super) struct Cholesky {
    diagonal: Vec<f64>,
    lower: Vec<Entries>,
}

impl Cholesky {
    pub(super) fn new(
        rows: &[Entries],
        columns: usize,
        tolerance: f64,
        meter: &mut Meter<'_>,
    ) -> Result<Self, PreviewError> {
        let recovery = PreviewError::Incomplete(Incomplete::RankRecovery {
            rows: rows.len(),
            columns,
        });
        if rows.len() > columns {
            return Err(recovery);
        }
        let mut triangle = gram_triangle(rows, columns, meter)?;
        let mut factor = Self {
            diagonal: vec![0.; rows.len()],
            lower: vec![Vec::new(); rows.len()],
        };
        for column in 0..rows.len() {
            if !factor.eliminate(&mut triangle, column, meter)? {
                return Err(recovery);
            }
        }
        meter.charge(rows.len().saturating_mul(2))?;
        factor
            .separated_rank(tolerance)
            .then_some(factor)
            .ok_or(recovery)
    }

    fn eliminate(
        &mut self,
        triangle: &mut [BTreeMap<usize, f64>],
        column: usize,
        meter: &mut Meter<'_>,
    ) -> Result<bool, PreviewError> {
        meter.charge(triangle.len() - column)?;
        let Some(pivot) = triangle[column].remove(&column) else {
            return Ok(false);
        };
        if !pivot.is_finite() || pivot <= 64. * f64::EPSILON {
            return Ok(false);
        }
        self.diagonal[column] = pivot.sqrt();
        let entries = triangle
            .iter_mut()
            .enumerate()
            .skip(column + 1)
            .filter_map(|(i, row)| {
                row.remove(&column)
                    .map(|value| (i, value / self.diagonal[column]))
            })
            .filter(|&(_, value)| value != 0.)
            .collect::<Vec<_>>();
        for (index, &(row, factor)) in entries.iter().enumerate() {
            meter.fill(index + 1)?;
            self.lower[row].push((column, factor));
            for &(col, other) in &entries[..=index] {
                *triangle[row].entry(col).or_default() -= factor * other;
            }
        }
        Ok(true)
    }

    fn separated_rank(&self, tolerance: f64) -> bool {
        let forward = self.forward_bound();
        let reverse = self.reverse_bound();
        let size = self.diagonal.len().max(1) as f64;
        let cutoff = (32. * tolerance * size.sqrt()).max(32. * f64::EPSILON * size);
        let bound = forward * reverse * cutoff;
        bound < 0.5
    }

    fn forward_bound(&self) -> f64 {
        let mut sums = vec![1.; self.diagonal.len()];
        for i in 0..sums.len() {
            sums[i] = (1.
                + self.lower[i]
                    .iter()
                    .map(|&(k, v)| v.abs() * sums[k])
                    .sum::<f64>())
                / self.diagonal[i];
        }
        sums.into_iter().fold(1., f64::max)
    }

    fn reverse_bound(&self) -> f64 {
        let mut sums = vec![1.; self.diagonal.len()];
        for i in (0..sums.len()).rev() {
            sums[i] /= self.diagonal[i];
            for &(k, v) in &self.lower[i] {
                sums[k] += v.abs() * sums[i];
            }
        }
        sums.into_iter().fold(1., f64::max)
    }

    pub(super) fn rank(&self) -> usize {
        self.diagonal.len()
    }

    pub(super) fn correction(&self, rows: &[Entries], rhs: &[f64], columns: usize) -> Vec<f64> {
        transpose_mul(rows, &self.solve(rhs), columns)
    }

    fn solve(&self, rhs: &[f64]) -> Vec<f64> {
        let mut x = rhs.to_vec();
        for i in 0..x.len() {
            for &(k, v) in &self.lower[i] {
                x[i] -= v * x[k];
            }
            x[i] /= self.diagonal[i];
        }
        for i in (0..x.len()).rev() {
            x[i] /= self.diagonal[i];
            for &(k, v) in &self.lower[i] {
                x[k] -= v * x[i];
            }
        }
        x
    }

    pub(super) fn preference(
        &self,
        rows: &[Entries],
        targets: &[(usize, f64)],
        columns: usize,
        tolerance: f64,
    ) -> Result<Vec<f64>, PreviewError> {
        if rows.len() == columns {
            return Ok(vec![0.; columns]);
        }
        let rhs = targets.iter().map(|&(_, value)| value).collect::<Vec<_>>();
        let basis = targets
            .iter()
            .map(|&(axis, _)| self.project_axis(rows, axis, columns))
            .collect::<Vec<_>>();
        let observed = targets
            .iter()
            .map(|&(axis, _)| basis.iter().map(|column| column[axis]).collect())
            .collect();
        let matrix = DenseMatrix::new(targets.len(), observed)?;
        let factor = ColumnPivotedQr::factor(&matrix, tolerance)?;
        let coeffs = if factor.rank() == targets.len() {
            factor.least_squares(&rhs)?
        } else {
            // The hard equations can leave fewer free directions than observed coordinates.
            // This rank reduction is confined to the admitted soft-target matrix.
            ColumnPivotedQr::factor(&matrix.transpose(), tolerance)?
                .best_fit_transpose(&rhs, tolerance)?
        };
        let mut step = vec![0.; columns];
        for (column, coefficient) in basis.into_iter().zip(coeffs) {
            for (value, direction) in step.iter_mut().zip(column) {
                *value += coefficient * direction;
            }
        }
        Ok(step)
    }

    fn project_axis(&self, rows: &[Entries], axis: usize, columns: usize) -> Vec<f64> {
        let rhs = rows
            .iter()
            .map(|row| {
                row.iter()
                    .find(|&&(i, _)| i == axis)
                    .map_or(0., |&(_, v)| -v)
            })
            .collect::<Vec<_>>();
        let mut column = transpose_mul(rows, &self.solve(&rhs), columns);
        column[axis] += 1.;
        column
    }
}

fn gram_triangle(
    rows: &[Entries],
    columns: usize,
    meter: &mut Meter<'_>,
) -> Result<Vec<BTreeMap<usize, f64>>, PreviewError> {
    meter.charge(columns.saturating_add(rows.len()))?;
    let mut postings = vec![Vec::new(); columns];
    for (row, entries) in rows.iter().enumerate() {
        meter.charge(entries.len())?;
        for &(column, value) in entries {
            postings[column].push((row, value));
        }
    }
    let mut triangle = vec![BTreeMap::new(); rows.len()];
    for uses in postings {
        for (i, &(row, value)) in uses.iter().enumerate() {
            meter.gram(i + 1)?;
            for &(col, other) in &uses[..=i] {
                *triangle[row].entry(col).or_default() += value * other;
            }
        }
    }
    Ok(triangle)
}

fn transpose_mul(rows: &[Entries], coefficients: &[f64], columns: usize) -> Vec<f64> {
    let mut result = vec![0.; columns];
    for (row, &coefficient) in rows.iter().zip(coefficients) {
        for &(column, value) in row {
            result[column] += coefficient * value;
        }
    }
    result
}
