// SPDX-License-Identifier: GPL-2.0-only
use crate::ad::DerivativeError;
use std::{error::Error, fmt};

/// A source can refuse a geometric domain or propagate checked derivative arithmetic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResidualError {
    /// The trial lies outside a residual source’s supported domain.
    Domain(String),
    /// Could not evaluate the value or derivatives in finite arithmetic.
    Derivative(DerivativeError),
}
impl From<DerivativeError> for ResidualError {
    fn from(error: DerivativeError) -> Self {
        Self::Derivative(error)
    }
}
impl fmt::Display for ResidualError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Domain(reason) => reason.fmt(output),
            Self::Derivative(error) => error.fmt(output),
        }
    }
}
impl Error for ResidualError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Domain(_) => None,
            Self::Derivative(error) => Some(error),
        }
    }
}

/// Failed block/row coordinates and expected shape; no partial evaluation is returned.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EvaluationError {
    /// Could not seed an input coordinate for automatic differentiation.
    Input(DerivativeError),
    /// A local coordinate references an invalid global column.
    Variable {
        /// Source block index.
        block: usize,
        /// Coordinate position within the source block.
        local: usize,
        /// Invalid index in the complete variable array.
        global: usize,
        /// Number of available global coordinates.
        count: usize,
    },
    /// The calculation for a source block failed.
    Source {
        /// Source block index.
        block: usize,
        /// Reason the source declined or failed.
        cause: ResidualError,
    },
    /// A source returned the wrong equation count.
    Rows {
        /// Source block index.
        block: usize,
        /// Declared equation count.
        expected: usize,
        /// Returned equation count.
        actual: usize,
    },
    /// A source returned the wrong derivative width.
    Columns {
        /// Source block index.
        block: usize,
        /// Source-relative equation index.
        row: usize,
        /// Declared derivative width.
        expected: usize,
        /// Returned derivative width.
        actual: usize,
    },
}
impl fmt::Display for EvaluationError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(error) => write!(output, "solver input: {error}"),
            Self::Variable {
                block,
                local,
                global,
                count,
            } => write!(
                output,
                "residual block {block} local variable {local} maps to {global}: expected global index below {count}"
            ),
            Self::Source { block, cause } => write!(output, "residual block {block}: {cause}"),
            Self::Rows {
                block,
                expected,
                actual,
            } => write!(
                output,
                "residual block {block} returned {actual} equations: expected {expected}"
            ),
            Self::Columns {
                block,
                row,
                expected,
                actual,
            } => write!(
                output,
                "residual block {block} row {row} returned {actual} partials: expected {expected} global columns or a constant"
            ),
        }
    }
}
impl Error for EvaluationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Input(error) => Some(error),
            Self::Source { cause, .. } => Some(cause),
            _ => None,
        }
    }
}
