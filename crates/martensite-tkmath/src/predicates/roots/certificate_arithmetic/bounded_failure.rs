// SPDX-License-Identifier: GPL-2.0-only
use crate::predicates::roots::DyadicBound;
pub(super) type Checked<T> = Result<T, BoundedArithmeticFailure>;

/// An incomplete bounded arithmetic operation; no partial bound is published.
#[derive(Clone, Debug, PartialEq)]
pub enum BoundedArithmeticFailure {
    /// A scalar was not a finite binary64 value.
    NonFiniteScalar {
        /// Offending scalar value.
        value: f64,
    },
    /// The original operation allowance was exhausted.
    WorkLimit {
        /// Number of completed arithmetic operations.
        completed: u32,
        /// Declared resource limit.
        limit: u32,
    },
    /// Cancellation was observed before another arithmetic operation.
    Cancelled,
    /// An intermediate exceeded the fixed integer bound.
    ArithmeticBits {
        /// Observed count or size.
        actual: u64,
        /// Declared resource limit.
        limit: u64,
    },
    /// Operands did not have compatible Bernstein coefficient counts.
    CoefficientCount {
        /// Left operand coefficient count.
        left: usize,
        /// Right operand coefficient count.
        right: usize,
    },
    /// A positive denominator could not be established at the requested precision.
    DenominatorUnresolved {
        /// Certified lower endpoint.
        lower: DyadicBound,
    },
    /// An outward endpoint did not fit finite binary64.
    UnrepresentableBound {
        /// Computed conservative upper bound.
        bound: DyadicBound,
    },
}
