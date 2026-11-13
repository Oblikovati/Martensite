// SPDX-License-Identifier: GPL-2.0-only
use super::{super::BoundedArithmeticFailure, CylindricalProfileOptions};
use crate::predicates::roots::DyadicBound;
pub(super) type Checked<T> = Result<T, CylindricalProfileFailure>;

/// No profile certificate accompanies an unsuccessful computation.
#[derive(Clone, Debug, PartialEq)]
pub struct CylindricalProfileError {
    /// Original arithmetic policy.
    pub requested_options: CylindricalProfileOptions,
    /// Original developed-position cap.
    pub requested_spatial_limit: f64,
    /// Original typed failure or incomplete evidence.
    pub cause: CylindricalProfileFailure,
}

/// Unsupported input or incomplete evidence, never a topology verdict.
#[derive(Clone, Debug, PartialEq)]
pub enum CylindricalProfileFailure {
    /// Precision or work settings outside the admitted range.
    Policy(CylindricalProfileOptions),
    /// Invalid exact dyadic interval.
    Interval {
        /// Offending source or dyadic interval index.
        index: u64,
        /// Requested dyadic subdivision level.
        level: u8,
    },
    /// Unsupported count or degree; authored poles require 2..=4, stored poles 2..=10.
    Count {
        /// Input role identifying the failed constraint.
        role: &'static str,
        /// Observed count or size.
        actual: usize,
    },
    /// Unsupported nonfinite or nonpositive value.
    Value {
        /// Input role identifying the failed constraint.
        role: &'static str,
        /// Offending source or dyadic interval index.
        index: usize,
        /// Offending scalar value.
        value: f64,
    },
    /// Missing retained weights are distinct from unit weights.
    Weights {
        /// Retained pole count.
        poles: usize,
        /// Retained weight count.
        weights: usize,
    },
    /// Original bounded arithmetic cause.
    Arithmetic(Box<BoundedArithmeticFailure>),
    /// Argument reduction did not produce a bounded scalar Taylor chart.
    TrigonometricRange {
        /// Certified lower endpoint.
        lower: DyadicBound,
        /// Certified upper endpoint.
        upper: DyadicBound,
    },
    /// The residual did not establish a unique nearby angular direction.
    DirectionUnresolved {
        /// Position residual that failed to establish direction.
        residual: f64,
        /// Requested positive reference radius.
        radius: f64,
    },
    /// The requested fit could not meet its ideal-error share within the geometric level cap.
    FitLevel {
        /// Maximum admitted geometric subdivision level.
        maximum: u8,
        /// Computed conservative upper bound.
        bound: f64,
        /// Original requested spatial error limit.
        requested: f64,
    },
    /// Invalid requested planning resource limits.
    FitPolicy {
        /// Requested additional subdivision cap.
        maximum_level: u8,
        /// Requested arithmetic operation cap.
        maximum_operations: u32,
    },
    /// A conservative position enclosure exceeded the unchanged requested cap.
    Uncertified {
        /// Computed conservative upper bound.
        bound: f64,
        /// Original requested spatial error limit.
        requested: f64,
    },
}
impl From<BoundedArithmeticFailure> for CylindricalProfileFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(Box::new(cause))
    }
}
impl std::fmt::Display for CylindricalProfileFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{self:?}; expected finite admitted sources and complete bounds within the requested policy"
        )
    }
}
impl std::fmt::Display for CylindricalProfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cylindrical profile certificate: {}", self.cause)
    }
}
impl std::error::Error for CylindricalProfileError {}
