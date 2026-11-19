// SPDX-License-Identifier: GPL-2.0-only
use super::*;
/// Original failure before a complete row transition could be issued.
#[derive(Debug)]
pub enum ConicalBoundaryZRowFailure {
    /// Full actual source/policy bits differ from the issued authority.
    Binding {
        /// Offending field.
        field: &'static str,
        /// Actual operand bits.
        actual: Vec<u64>,
        /// Expected operand bits.
        expected: Vec<u64>,
    },
    /// An actual row value differs from the issued representable construction.
    Row {
        /// Physical row.
        depth: usize,
        /// Actual control index.
        index: usize,
        /// Actual supplied Z.
        actual: f64,
        /// Issued expected Z.
        expected: f64,
    },
    /// Original boundary admission or composed-limit refusal.
    Boundary(ConicalBoundaryError),
    /// Original issued family movement refusal.
    Gamma(ConicalActualPatchError),
    /// Existing bounded arithmetic cause.
    Arithmetic(BoundedArithmeticFailure),
    /// Actual parent-plus-new work exceeds its original producing grant.
    Ancestry {
        /// Actual cumulative work.
        actual: u64,
        /// Original grant.
        limit: u32,
    },
}
/// Typed refusal with actual completed incremental work and the remaining grant.
#[derive(Debug)]
pub struct ConicalBoundaryZRowError {
    /// Physical row when a particular row failed.
    pub depth: Option<usize>,
    /// Actual completed incremental work.
    pub operations: u32,
    /// Requested remaining policy.
    pub options: ConeClearanceOptions,
    /// Original typed failure.
    pub cause: Box<ConicalBoundaryZRowFailure>,
}
impl ConicalBoundaryZRowError {
    /// Whether the original failure observed cancellation; `error.is_cancelled()`.
    pub fn is_cancelled(&self) -> bool {
        match self.cause.as_ref() {
            ConicalBoundaryZRowFailure::Arithmetic(BoundedArithmeticFailure::Cancelled) => true,
            ConicalBoundaryZRowFailure::Gamma(cause) => cause.is_cancelled(),
            ConicalBoundaryZRowFailure::Boundary(cause) => {
                matches!(cause.cause(), ConicalBoundaryFailure::Proof(p) if matches!(p.arithmetic_cause(), Some(BoundedArithmeticFailure::Cancelled)))
            }
            _ => false,
        }
    }
}
impl From<BoundedArithmeticFailure> for ConicalBoundaryZRowFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(cause)
    }
}
pub(super) fn failed(
    cause: ConicalBoundaryZRowFailure,
    options: ConeClearanceOptions,
    operations: u32,
) -> ConicalBoundaryZRowError {
    let depth = match &cause {
        ConicalBoundaryZRowFailure::Row { depth, .. } => Some(*depth),
        _ => None,
    };
    ConicalBoundaryZRowError {
        depth,
        operations,
        options,
        cause: Box::new(cause),
    }
}
pub(super) fn mismatch(
    field: &'static str,
    actual: Vec<u64>,
    expected: Vec<u64>,
) -> ConicalBoundaryZRowFailure {
    ConicalBoundaryZRowFailure::Binding {
        field,
        actual,
        expected,
    }
}
impl std::fmt::Display for ConicalBoundaryZRowFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binding { .. } | Self::Row { .. } => {
                write!(f, "{self:?}; expected exact issued operands")
            }
            Self::Boundary(cause) => write!(f, "{cause}"),
            Self::Gamma(cause) => write!(f, "{cause}"),
            Self::Arithmetic(cause) => write!(f, "{cause:?}"),
            Self::Ancestry { actual, limit } => write!(
                f,
                "cumulative work={actual}; expected <= original grant {limit}"
            ),
        }
    }
}
impl std::error::Error for ConicalBoundaryZRowFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Boundary(v) => Some(v),
            Self::Gamma(v) => Some(v),
            _ => None,
        }
    }
}
impl std::fmt::Display for ConicalBoundaryZRowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "directed row depth={:?} operations={}: {}",
            self.depth, self.operations, self.cause
        )
    }
}
impl std::error::Error for ConicalBoundaryZRowError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.cause.as_ref())
    }
}
