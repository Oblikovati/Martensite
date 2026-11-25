// SPDX-License-Identifier: GPL-2.0-only

use crate::{
    ad::DualNumber,
    residual::{EvaluationError, ResidualBlock},
};

pub(crate) struct LocalSeeds {
    pub(crate) columns: Vec<usize>,
    positions: Vec<Option<usize>>,
}

impl LocalSeeds {
    pub(crate) fn new(mapping: &[usize], mut active: impl FnMut(usize) -> bool) -> Self {
        let mut columns = mapping
            .iter()
            .copied()
            .filter(|&i| active(i))
            .collect::<Vec<_>>();
        columns.sort_unstable();
        columns.dedup();
        let positions = mapping
            .iter()
            .map(|global| columns.binary_search(global).ok())
            .collect();
        Self { columns, positions }
    }

    pub(crate) fn evaluate(
        &self,
        values: &[f64],
        block: &ResidualBlock<'_>,
        index: usize,
    ) -> Result<Vec<DualNumber>, EvaluationError> {
        let width = self.columns.len();
        let seeds = block
            .variables
            .iter()
            .zip(&self.positions)
            .map(|(&global, &local)| match local {
                Some(local) => DualNumber::variable(values[global], local, width),
                None => DualNumber::constant(values[global]),
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(EvaluationError::Input)?;
        let rows = block
            .source
            .evaluate(&seeds)
            .map_err(|cause| EvaluationError::Source {
                block: index,
                cause,
            })?;
        Self::validate_rows(block, index, width, &rows)?;
        Ok(rows)
    }

    fn validate_rows(
        block: &ResidualBlock<'_>,
        index: usize,
        width: usize,
        rows: &[DualNumber],
    ) -> Result<(), EvaluationError> {
        if rows.len() != block.equations {
            return Err(EvaluationError::Rows {
                block: index,
                expected: block.equations,
                actual: rows.len(),
            });
        }
        for (row, value) in rows.iter().enumerate() {
            let actual = value.gradient().len();
            if actual != 0 && actual != width {
                return Err(EvaluationError::Columns {
                    block: index,
                    row,
                    expected: width,
                    actual,
                });
            }
        }
        Ok(())
    }
}
