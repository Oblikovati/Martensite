// SPDX-License-Identifier: GPL-2.0-only
//! Inputs, completed chart cover and typed planning refusals.
use super::*;

/// Work and representation limits for planning one original cone boundary span.
#[derive(Clone, Copy)]
pub struct ConicalChartPlanQuery<'a> {
    /// Original unsplit polynomial controls in developed coordinates.
    pub authored: &'a [[f64; 2]],
    /// Cone semi-angle in radians.
    pub semi_angle: f64,
    /// Physical angular datum in radians.
    pub chart_origin: f64,
    /// Maximum dyadic split level, up to 63.
    pub max_level: u8,
    /// Maximum emitted charts, in 1..=4096.
    pub max_charts: usize,
    /// Work allowance shared by every chart in the plan.
    pub options: ConeClearanceOptions,
}

/// Executed dyadic chart cover, in increasing source parameter order.
pub struct ConicalChartPlan {
    pub(super) charts: Vec<ConicalChartDomainEvidence>,
    pub(super) operations: u32,
}
impl ConicalChartPlan {
    /// Return the executed charts in source order. Example: `plan.into_charts()`.
    pub fn into_charts(self) -> Vec<ConicalChartDomainEvidence> {
        self.charts
    }

    /// Actual planning work computed under the supplied allowance. Example: `plan.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
}

/// Failure to cover the complete interval under the requested policy.
#[derive(Debug)]
pub struct ConicalChartPlanError {
    /// Typed refusal.
    pub cause: ConicalChartPlanFailure,
    /// Actual work before refusal.
    pub operations: u32,
}

/// Chart planning admission and proof failures.
#[derive(Debug)]
pub enum ConicalChartPlanFailure {
    /// Planning limits are outside their supported range.
    Limits {
        /// Requested maximum dyadic level.
        max_level: u8,
        /// Requested maximum number of output charts.
        max_charts: usize,
    },
    /// This interval needs another split beyond the permitted level.
    Level {
        /// Index of the interval that still needs splitting.
        index: u64,
        /// Its level, equal to the requested maximum.
        level: u8,
    },
    /// The completed and pending intervals require more charts than allowed.
    Capacity {
        /// Minimum number of charts that would be needed after splitting.
        minimum: usize,
        /// Requested maximum number of charts.
        limit: usize,
    },
    /// An original-domain, trigonometric or arithmetic obligation failed.
    Domain(ConicalChartDomainFailure),
}
impl From<ConicalChartDomainFailure> for ConicalChartPlanFailure {
    fn from(cause: ConicalChartDomainFailure) -> Self {
        Self::Domain(cause)
    }
}
impl From<BoundedArithmeticFailure> for ConicalChartPlanFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Domain(ConicalChartDomainFailure::Arithmetic(cause))
    }
}
impl std::fmt::Display for ConicalChartPlanError {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            out,
            "cone chart planning: {}; work={}",
            self.cause, self.operations
        )
    }
}
impl std::error::Error for ConicalChartPlanError {}

impl std::fmt::Display for ConicalChartPlanFailure {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Limits {
                max_level,
                max_charts,
            } => write!(
                out,
                "max level={max_level}, max charts={max_charts}; expected level <= 63 and charts in 1..=4096"
            ),
            Self::Level { index, level } => write!(
                out,
                "interval={index}/{level}; expected width < pi at or below the maximum level"
            ),
            Self::Capacity { minimum, limit } => write!(
                out,
                "required charts >= {minimum}; expected at most {limit}"
            ),
            Self::Domain(cause) => cause.fmt(out),
        }
    }
}
