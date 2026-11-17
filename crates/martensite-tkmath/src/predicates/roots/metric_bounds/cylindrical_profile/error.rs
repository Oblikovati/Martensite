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
        write!(f, "{self:?}; expected {}", self.expected())
    }
}

const PROFILE_POLICY: &str = "fractional bits in 8..=4096 and operations in 0..=1000000";
const PROFILE_INTERVAL: &str = "level <= 63 and index < 2^level";
const PROFILE_WEIGHTS: &str = "one retained weight per pole";
const PROFILE_TRIGONOMETRIC_RANGE: &str = "reduced interval within [-7, 7]";
const PROFILE_DIRECTION: &str = "position residual < positive stored radius";
const PROFILE_FIT_LEVEL: &str =
    "ideal developed bound <= requested/2 within the maximum subdivision level";
const PROFILE_FIT_POLICY: &str =
    "source interval level + maximum_level <= 63 and maximum_operations in 0..=1000000";
const PROFILE_POSITION_CAP: &str = "developed-position bound <= requested spatial limit";

impl CylindricalProfileFailure {
    fn expected(&self) -> &'static str {
        match self {
            Self::Policy(_) => PROFILE_POLICY,
            Self::Interval { .. } => PROFILE_INTERVAL,
            Self::Count { role, .. } => expected_count(role),
            Self::Value { role, .. } => expected_value(role),
            Self::Weights { .. } => PROFILE_WEIGHTS,
            Self::Arithmetic(cause) => expected_arithmetic(cause),
            Self::TrigonometricRange { .. } => PROFILE_TRIGONOMETRIC_RANGE,
            Self::DirectionUnresolved { .. } => PROFILE_DIRECTION,
            Self::FitLevel { .. } => PROFILE_FIT_LEVEL,
            Self::FitPolicy { .. } => PROFILE_FIT_POLICY,
            Self::Uncertified { .. } => PROFILE_POSITION_CAP,
        }
    }
}

fn expected_count(role: &str) -> &'static str {
    // #524: public callers can construct unfamiliar roles without recording a maximum.
    match role {
        "authored poles" => "authored pole count in 2..=4",
        "stored poles" => "stored pole count in 2..=10",
        _ => "a supported count for the named input role; role-specific maximum unavailable",
    }
}

fn expected_value(role: &str) -> &'static str {
    match role {
        "support radius" | "stored radius" | "weight" => "finite value > 0",
        "spatial limit" => "finite value >= 0",
        "angular origin" | "authored coordinate" | "stored coordinate" => "a finite binary64 value",
        _ => "a finite value satisfying the named input constraint",
    }
}

fn expected_arithmetic(cause: &BoundedArithmeticFailure) -> &'static str {
    use BoundedArithmeticFailure as Arithmetic;
    match cause {
        Arithmetic::NonFiniteScalar { .. } => "a finite binary64 scalar",
        Arithmetic::WorkLimit { .. } => "completion within the unchanged work budget",
        Arithmetic::Cancelled => "an uncancelled operation; no partial certificate is published",
        Arithmetic::ArithmeticBits { .. } => "intermediate integer bits <= the recorded limit",
        Arithmetic::CoefficientCount { .. } => "equal Bernstein coefficient counts",
        Arithmetic::DenominatorUnresolved { .. } => "dyadic denominator lower bound > 0",
        Arithmetic::UnrepresentableBound { .. } => "a finite outward binary64 bound",
    }
}
impl std::fmt::Display for CylindricalProfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cylindrical profile certificate: {}", self.cause)
    }
}
impl std::error::Error for CylindricalProfileError {}

#[cfg(test)]
#[path = "diagnostic_tests.rs"]
mod diagnostic_tests;
