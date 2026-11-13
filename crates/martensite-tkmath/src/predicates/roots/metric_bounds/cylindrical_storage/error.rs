// SPDX-License-Identifier: GPL-2.0-only
use super::{CylindricalStorageLimits, CylindricalStorageOptions, CylindricalStorageResidual};
pub(super) type Checked<T> = Result<T, CylindricalStorageFailure>;

/// No storage certificate accompanies an unsuccessful operation.
#[derive(Clone, Debug, PartialEq)]
pub struct CylindricalStorageError {
    /// Unchanged work policy.
    pub requested_options: CylindricalStorageOptions,
    /// Unchanged spatial caps.
    pub requested_limits: CylindricalStorageLimits,
    /// Input, arithmetic or incomplete-bound cause.
    pub cause: CylindricalStorageFailure,
}

/// A conservative enclosure above a limit is incomplete, not proof of invalid geometry.
#[derive(Clone, Debug, PartialEq)]
pub enum CylindricalStorageFailure {
    /// Precision, stored degree or operation limit outside the supported shape.
    Policy(CylindricalStorageOptions),
    /// Nonfinite value, nonpositive radius/weight, or negative limit.
    Value {
        /// Ordered curve, when this value belongs to one curve.
        depth: Option<usize>,
        /// Coordinate, radius, weight or limit role.
        field: &'static str,
        /// Diagnostic input position within that role.
        index: usize,
        /// Offending supplied value.
        value: f64,
    },
    /// A curve must have at least two poles and a matching weight count when present.
    PoleCount {
        /// Ordered depth, zero or one.
        depth: usize,
        /// Supplied control-point count.
        poles: usize,
        /// Supplied weight count, or omitted weights.
        weights: Option<usize>,
    },
    /// Actual stored degree exceeded the original policy.
    Degree {
        /// Ordered depth, zero or one.
        depth: usize,
        /// Supplied stored degree.
        actual: usize,
        /// Original requested maximum degree.
        limit: u8,
    },
    /// No more arithmetic is performed after the requested operation count.
    WorkLimit {
        /// Interval operations completed before refusal.
        completed: u32,
        /// Unchanged requested operation maximum.
        limit: u32,
    },
    /// Cancellation was observed before another interval operation.
    Cancelled,
    /// A transient integer would exceed the fixed implementation storage limit.
    ArithmeticBits {
        /// Number of bits needed by the intermediate integer.
        actual: u64,
        /// Fixed admitted integer-size bound.
        limit: u64,
    },
    /// Internal polynomial operands did not have the required common degree.
    CoefficientCount {
        /// First coefficient count.
        left: usize,
        /// Second coefficient count.
        right: usize,
    },
    /// Rounding could not retain a strictly positive denominator.
    DenominatorUnresolved {
        /// Actual outward dyadic lower endpoint, which must be strictly positive.
        lower: crate::predicates::roots::DyadicBound,
    },
    /// The first curve's certified radial bound did not exclude the cylinder axis.
    ReferenceDirectionUnresolved {
        /// Conservative radial distance bound.
        radial_bound: f64,
        /// Requested first radius, which the bound must be strictly below.
        radius: f64,
    },
    /// A finite binary64 bound could not represent the enclosure endpoint.
    UnrepresentableBound {
        /// Actual exact dyadic endpoint that did not fit finite binary64.
        bound: crate::predicates::roots::DyadicBound,
    },
    /// The unchanged requested cap was not established by the conservative enclosure.
    Uncertified {
        /// Residual for which no sufficient bound was obtained.
        residual: CylindricalStorageResidual,
        /// Conservative upper bound, never a measured maximum.
        bound: f64,
        /// Unchanged requested cap.
        requested: f64,
    },
}
impl std::fmt::Display for CylindricalStorageFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}; expected {}", self.expected())
    }
}
impl CylindricalStorageFailure {
    fn expected(&self) -> &'static str {
        match self {
            Self::Policy(_) => "fractional bits 8..=4096, degree 1..=25, operations <= 1000000",
            Self::Value { .. } => "finite coordinates; radii/weights > 0; limits >= 0",
            Self::PoleCount { .. } => "at least two poles and matching weights",
            Self::Degree { .. } => "stored degree within the requested maximum",
            Self::WorkLimit { .. } => "a completed bound within the unchanged work budget",
            Self::Cancelled => "an uncancelled operation",
            Self::ArithmeticBits { .. } => "transient integers no larger than 16384 bits",
            Self::CoefficientCount { .. } => "matching Bernstein coefficient counts",
            Self::DenominatorUnresolved { .. } => "positive dyadic denominator lower bound",
            Self::ReferenceDirectionUnresolved { .. } => "first radial bound < radius",
            Self::UnrepresentableBound { .. } => "a finite outward binary64 bound",
            Self::Uncertified { .. } => "a certified upper bound within the requested cap",
        }
    }
}
impl std::fmt::Display for CylindricalStorageError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "cylindrical storage certificate: {}", self.cause)
    }
}
impl std::error::Error for CylindricalStorageError {}

impl
    From<
        crate::predicates::roots::certificate_arithmetic::bounded_failure::BoundedArithmeticFailure,
    > for CylindricalStorageFailure
{
    fn from(
        cause: crate::predicates::roots::certificate_arithmetic::bounded_failure::BoundedArithmeticFailure,
    ) -> Self {
        use crate::predicates::roots::certificate_arithmetic::bounded_failure::BoundedArithmeticFailure as Arithmetic;
        match cause {
            Arithmetic::NonFiniteScalar { value } => Self::Value {
                depth: None,
                field: "dyadic scalar",
                index: 0,
                value,
            },
            Arithmetic::WorkLimit { completed, limit } => Self::WorkLimit { completed, limit },
            Arithmetic::Cancelled => Self::Cancelled,
            Arithmetic::ArithmeticBits { actual, limit } => Self::ArithmeticBits { actual, limit },
            Arithmetic::CoefficientCount { left, right } => Self::CoefficientCount { left, right },
            Arithmetic::DenominatorUnresolved { lower } => Self::DenominatorUnresolved { lower },
            Arithmetic::UnrepresentableBound { bound } => Self::UnrepresentableBound { bound },
        }
    }
}
