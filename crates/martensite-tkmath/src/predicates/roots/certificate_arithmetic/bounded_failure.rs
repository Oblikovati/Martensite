// SPDX-License-Identifier: GPL-2.0-only
use crate::predicates::roots::DyadicBound;
pub(super) type Checked<T> = Result<T, BoundedArithmeticFailure>;

/// An incomplete bounded arithmetic operation; no partial bound is published.
#[derive(Clone, Debug, PartialEq)]
pub enum BoundedArithmeticFailure {
    /// A scalar was not a finite binary64 value.
    NonFiniteScalar { value: f64 },
    /// The original operation allowance was exhausted.
    WorkLimit { completed: u32, limit: u32 },
    /// Cancellation was observed before another arithmetic operation.
    Cancelled,
    /// An intermediate exceeded the fixed integer bound.
    ArithmeticBits { actual: u64, limit: u64 },
    /// Operands did not have compatible Bernstein coefficient counts.
    CoefficientCount { left: usize, right: usize },
    /// A positive denominator could not be established at the requested precision.
    DenominatorUnresolved { lower: DyadicBound },
    /// An outward endpoint did not fit finite binary64.
    UnrepresentableBound { bound: DyadicBound },
}
