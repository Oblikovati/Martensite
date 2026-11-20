// SPDX-License-Identifier: GPL-2.0-only
//! PositiveX common-chart admission with quantitative angular margins.
use super::*;
pub(super) mod bounds;
#[cfg(test)]
pub(super) mod diagnostics_tests;
#[cfg(test)]
mod family_tests;
#[cfg(test)]
mod recentered_tests;
#[cfg(test)]
mod tests;
/// Select the common-chart geometry and finite source capacity.
/// Example: `let version = ConicalChartDomainVersion::FamilyV3;`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ConicalChartDomainVersion {
    /// Original admission of at most 128 complete authored source spans.
    #[default]
    OriginalV2,
    /// Explicit selected-family admission of at most 4096 original source spans.
    FamilyV3,
    /// Fit around a separate finite center while retaining the original physical datum.
    RecenteredV4,
    /// Independently centered region in a source-bound cone chart atlas.
    AtlasMemberV5,
}
impl ConicalChartDomainVersion {
    /// Fixed original-source cap; `assert_eq!(ConicalChartDomainVersion::FamilyV3.span_limit(), 4096)`.
    pub const fn span_limit(self) -> usize {
        match self {
            Self::OriginalV2 => 128,
            Self::FamilyV3 | Self::RecenteredV4 | Self::AtlasMemberV5 => 4096,
        }
    }
    pub(super) fn allows_offset(self) -> bool {
        matches!(self, Self::RecenteredV4 | Self::AtlasMemberV5)
    }
    pub(super) const fn tag(self) -> u64 {
        match self {
            Self::OriginalV2 => 2,
            Self::FamilyV3 => 3,
            Self::RecenteredV4 => 4,
            Self::AtlasMemberV5 => 5,
        }
    }
}
/// Complete authored boundary and explicit common angular lift.
#[derive(Clone, Copy)]
pub struct ConicalChartDomainQuery<'a> {
    /// Original unsplit polynomial controls, in stable source order.
    pub spans: &'a [&'a [[f64; 2]]],
    /// Cone semi-angle in radians.
    pub semi_angle: f64,
    /// Physical angular datum.
    pub chart_origin: f64,
    /// Common rational-fit center. Datum-centered versions require the datum's exact bits.
    pub fit_center: f64,
    /// Continuous developed lift; this behavior requires zero.
    pub developed_turn: i32,
    /// Physical lift; this behavior requires zero.
    pub physical_turn: i32,
    /// Remaining bounded arithmetic resources.
    pub options: ConeClearanceOptions,
}
/// Immutable source-bound PositiveX wedge, sine and physical-range certificate.
pub struct ConicalChartDomainEvidence {
    version: ConicalChartDomainVersion,
    span_limit: usize,
    pub(super) sector: sector::SectorEvidence,
    chart: [u64; 2],
    turns: [i32; 2],
    pub(super) physical: Interval,
    pub(super) fit_offset: Option<Interval>,
    pub(super) sine: Interval,
    pub(super) cosine: Interval,
    pub(super) minimum_x: Interval,
}
/// A chart admission or bounded proof could not finish.
#[derive(Debug)]
pub struct ConicalChartDomainError {
    /// Specific failed obligation.
    pub cause: ConicalChartDomainFailure,
    /// Actual arithmetic work before completion or refusal.
    pub operations: u32,
}
/// Typed common-chart refusal, independent of legacy sector behavior.
#[derive(Debug)]
pub enum ConicalChartDomainFailure {
    /// The selected family exceeds its fixed original-source capacity.
    SourceCount {
        /// Complete original source count before any proof work.
        actual: usize,
        /// Fixed cap of the explicitly selected behavior.
        limit: usize,
    },
    /// Invalid original geometry, precision or work policy.
    Sector(ConicalSectorFailure),
    /// The supplied chart or turns do not select this PositiveX common-chart behavior.
    Chart {
        /// Requested physical datum and fit center.
        centers: [f64; 2],
        /// Requested developed and physical turns.
        turns: [i32; 2],
    },
    /// A strict numerical obligation remained unresolved with its exact original operands.
    Unresolved {
        /// Failure-only exact interval and the requirement tested against it.
        bound: Box<ConicalChartDomainBound>,
    },
    /// Actual common wedge did not select the required positive-X domain.
    Wedge {
        /// Original bit mask returned by the common-wedge predicate.
        actual_mask: u8,
        /// Required strict wedge.
        expected: ConicalSectorWedge,
    },
    /// An original point did not admit a bounded finite principal angle.
    Point {
        /// Original binary64 coordinates, retained without rounding.
        actual: [f64; 2],
        /// Required point and angle shape.
        expected: &'static str,
    },
    /// Arithmetic was cancelled or exhausted its resources.
    Arithmetic(BoundedArithmeticFailure),
    /// Trigonometric enclosure did not finish.
    Trigonometric(CylindricalProfileFailure),
}
/// Exact measured interval and strict condition used by a refused domain predicate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConicalChartDomainBound {
    /// Original failed obligation.
    pub field: &'static str,
    /// Exact original interval; width conditions apply to its endpoint difference.
    pub actual: [DyadicBound; 2],
    /// Exact strict requirement applied to this interval.
    pub expected: ConicalChartDomainRequirement,
}
/// Strict domain condition with the exact threshold used by the predicate.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConicalChartDomainRequirement {
    /// The lower endpoint must exceed zero.
    PositiveLower,
    /// The interval width must be strictly below this exact bound.
    WidthStrictlyBelow(DyadicBound),
    /// Both endpoints must be strictly inside these exact bounds.
    StrictlyInside([DyadicBound; 2]),
}
/// Certify one quantitative common chart; `certify_conical_chart_domain(query, || false)?`.
pub fn certify_conical_chart_domain(
    query: ConicalChartDomainQuery<'_>,
    cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalChartDomainEvidence, ConicalChartDomainError> {
    certify_conical_chart_domain_with_version(
        query,
        ConicalChartDomainVersion::OriginalV2,
        cancelled,
    )
}
/// Certify an explicitly bounded original-source family using the same chart pipeline.
/// Example: `certify_conical_chart_domain_with_version(query, ConicalChartDomainVersion::FamilyV3, || false)?`.
pub fn certify_conical_chart_domain_with_version(
    query: ConicalChartDomainQuery<'_>,
    version: ConicalChartDomainVersion,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalChartDomainEvidence, ConicalChartDomainError> {
    bounds::admit(&query, version).map_err(|cause| domain_error(cause, 0))?;
    let policy = sector_api::policy(query.options);
    let mut arithmetic = Arithmetic::new(policy, &mut cancelled);
    bounds::certify(&query, version, &mut arithmetic)
        .map_err(|cause| domain_error(cause, arithmetic.operations()))
}
impl ConicalChartDomainEvidence {
    pub(super) fn contains_original(&self, original: &[[f64; 2]]) -> bool {
        self.sector
            .original_index
            .contains_exact(&self.sector.source, original)
    }
    /// Executed chart behavior; `proof.version()`.
    pub fn version(&self) -> ConicalChartDomainVersion {
        self.version
    }
    /// Fixed admitted source cap; `proof.span_limit()`.
    pub fn span_limit(&self) -> usize {
        self.span_limit
    }
    /// Complete captured original-source count; `proof.source_span_count()`.
    pub fn source_span_count(&self) -> usize {
        self.sector.source.len()
    }
    /// Captured physical datum; `proof.chart_origin()`.
    pub fn chart_origin(&self) -> f64 {
        f64::from_bits(self.chart[0])
    }
    /// Captured common fit center; `proof.fit_center()`.
    pub fn fit_center(&self) -> f64 {
        f64::from_bits(self.chart[1])
    }
    /// Captured developed lift; `proof.developed_turn()`.
    pub fn developed_turn(&self) -> i32 {
        self.turns[0]
    }
    /// Captured physical lift; `proof.physical_turn()`.
    pub fn physical_turn(&self) -> i32 {
        self.turns[1]
    }
    /// Exact requested fixed-grid policy; `proof.options()`.
    pub fn options(&self) -> ConeClearanceOptions {
        ConeClearanceOptions {
            fractional_bits: self.sector.policy.fractional_bits,
            max_operations: self.sector.policy.max_operations,
        }
    }
    /// Certified physical angle relative to the datum; `proof.relative_physical_bounds()`.
    pub fn relative_physical_bounds(&self) -> [DyadicBound; 2] {
        exact_enclosure(&self.physical, self.sector.policy.fractional_bits)
    }
    /// Certified positive sine interval; `proof.sine_bounds()`.
    pub fn sine_bounds(&self) -> [DyadicBound; 2] {
        exact_enclosure(&self.sine, self.sector.policy.fractional_bits)
    }

    /// Rebind the original capacity and all input fields; `proof.matches(&query)`.
    pub fn matches(&self, query: &ConicalChartDomainQuery<'_>) -> bool {
        self.matches_with_version(query, ConicalChartDomainVersion::OriginalV2)
    }
    /// Rebind the explicit capacity and all original inputs; `proof.matches_with_version(&query, proof.version())`.
    pub fn matches_with_version(
        &self,
        query: &ConicalChartDomainQuery<'_>,
        version: ConicalChartDomainVersion,
    ) -> bool {
        self.version == version
            && self.span_limit == version.span_limit()
            && (1..=self.span_limit).contains(&query.spans.len())
            && self.sector.matches(
                query.spans,
                query.semi_angle,
                sector_api::policy(query.options),
            )
            && self.chart == [query.chart_origin.to_bits(), query.fit_center.to_bits()]
            && self.turns == [query.developed_turn, query.physical_turn]
    }
    /// Actual work, charged once by the composing operation; `proof.operations()`.
    pub fn operations(&self) -> u32 {
        self.sector.operations
    }
    /// Retained strict wedge; `proof.wedge()`.
    pub fn wedge(&self) -> ConicalSectorWedge {
        ConicalSectorWedge::PositiveX
    }
    /// Require the complete homotopy tube to remain in the wedge; `proof.admits_bound(&bound)`.
    pub fn admits_bound(&self, bound: &DyadicBound) -> bool {
        self.sector.admits_bound(bound)
    }
}
fn domain_error(cause: ConicalChartDomainFailure, operations: u32) -> ConicalChartDomainError {
    ConicalChartDomainError { cause, operations }
}
impl From<BoundedArithmeticFailure> for ConicalChartDomainFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(cause)
    }
}
impl ConicalChartDomainError {
    /// Whether the actual proof observed cancellation; `error.is_cancelled()`.
    pub fn is_cancelled(&self) -> bool {
        match &self.cause {
            ConicalChartDomainFailure::Arithmetic(BoundedArithmeticFailure::Cancelled) => true,
            ConicalChartDomainFailure::Sector(ConicalSectorFailure::Arithmetic(
                BoundedArithmeticFailure::Cancelled,
            )) => true,
            ConicalChartDomainFailure::Trigonometric(CylindricalProfileFailure::Arithmetic(
                cause,
            )) => matches!(cause.as_ref(), BoundedArithmeticFailure::Cancelled),
            _ => false,
        }
    }
}
impl std::fmt::Display for ConicalChartDomainError {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            out,
            "common chart: {}; work={}",
            self.cause, self.operations
        )
    }
}
impl std::error::Error for ConicalChartDomainError {}

impl std::fmt::Display for ConicalChartDomainFailure {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SourceCount { actual, limit } => {
                write!(out, "family source spans={actual}; expected 1..={limit}")
            }
            Self::Unresolved { bound } => write!(out, "{bound}"),
            Self::Wedge {
                actual_mask,
                expected,
            } => write!(out, "wedge mask={actual_mask}; expected {expected:?}"),
            Self::Point { actual, expected } => {
                write!(out, "point={actual:?}; expected {expected}")
            }
            Self::Chart { centers, turns } => write!(
                out,
                "centers={centers:?}, turns={turns:?}; expected finite centers, equal center bits for datum-centered versions, and zero turns"
            ),
            _ => write!(out, "{self:?}"),
        }
    }
}
impl std::fmt::Display for ConicalChartDomainBound {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            out,
            "{}: actual=[{}, {}]; expected {}",
            self.field, self.actual[0], self.actual[1], self.expected
        )
    }
}
impl std::fmt::Display for ConicalChartDomainRequirement {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PositiveLower => out.write_str("strictly positive lower bound"),
            Self::WidthStrictlyBelow(limit) => write!(out, "interval width strictly below {limit}"),
            Self::StrictlyInside([lower, upper]) => {
                write!(out, "strictly inside ({lower}, {upper})")
            }
        }
    }
}
