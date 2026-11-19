// SPDX-License-Identifier: GPL-2.0-only
use super::super::RationalIntegralFailure;
use super::IntegralProgramOptions;

/// Incomplete program, retaining exact source position and cumulative actual work.
#[derive(Clone, Debug, PartialEq)]
pub struct IntegralProgramError {
    /// Original unclamped policy.
    pub requested_options: Box<IntegralProgramOptions>,
    /// Whole loop index, where available (outer is0).
    pub loop_index: Option<usize>,
    /// Component index within that loop, where available.
    pub component_index: Option<usize>,
    /// Flattened occurrence index, where available; not an entity identity.
    pub occurrence_index: Option<usize>,
    /// Fully integrated occurrences before failure, never a partial scalar proof.
    pub completed_occurrences: usize,
    /// Actual examined cells, including cells of the failing occurrence.
    pub integration_subintervals: u64,
    /// Typed original numerical cause or actual program admission/work failure.
    pub cause: IntegralProgramFailure,
}

/// New program failures leave all legacy numerical failure variants unchanged.
#[derive(Clone, Debug, PartialEq)]
pub enum IntegralProgramFailure {
    /// A program admission field lies outside its explicit closed range.
    Policy {
        /// Rejected policy field.
        field: &'static str,
        /// Supplied field value.
        actual: u64,
        /// Smallest admitted value.
        minimum: u64,
        /// Largest admitted value, computed from the actual integration policy when needed.
        maximum: u64,
    },
    /// Empty program, loop or component, with source position in the enclosing error.
    Empty,
    /// Actual component count exceeds the explicit requested maximum.
    ComponentCount {
        /// Observed prefix component count at the first exceeded bound.
        actual: usize,
        /// Requested maximum.
        limit: u16,
    },
    /// Aggregate cell cap exhausted; nested legacy work-limit retains its effective per-curve allowance.
    AggregateWork {
        /// Requested aggregate cap.
        limit: u64,
        /// Original numerical refusal from the running curve.
        cause: Box<RationalIntegralFailure>,
    },
    /// Original numerical cause, without a manufactured legacy classification.
    Numerical(Box<RationalIntegralFailure>),
}
impl std::fmt::Display for IntegralProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "integral program loop {:?} component {:?} occurrence {:?}: {}; requested {:?}",
            self.loop_index,
            self.component_index,
            self.occurrence_index,
            self.cause,
            self.requested_options
        )
    }
}
impl std::error::Error for IntegralProgramError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.cause {
            IntegralProgramFailure::Numerical(cause)
            | IntegralProgramFailure::AggregateWork { cause, .. } => Some(cause.as_ref()),
            _ => None,
        }
    }
}

impl std::fmt::Display for IntegralProgramFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Policy {
                field,
                actual,
                minimum,
                maximum,
            } => write!(f, "{field} {actual}; expected {minimum}..={maximum}"),
            Self::Empty => write!(f, "empty program or loop; expected at least one component"),
            Self::ComponentCount { actual, limit } => {
                write!(f, "component count {actual}; expected 1..={limit}")
            }
            Self::AggregateWork { limit, cause } => write!(
                f,
                "aggregate cell cap {limit} exhausted: {cause}; expected completion within that cap"
            ),
            Self::Numerical(cause) => cause.fmt(f),
        }
    }
}
