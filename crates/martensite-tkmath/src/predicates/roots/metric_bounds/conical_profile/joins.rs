// SPDX-License-Identifier: GPL-2.0-only
//! Source-bound common projection certificates for developed contour joins.
use super::angle::*;
use super::boundary::*;
use super::developed::CombinedFailure;
use super::*;
pub(super) mod chart;
pub use chart::{
    ConicalChartJoinsError, certify_conical_joins_in_chart,
    certify_conical_joins_in_chart_with_refinement,
};

/// One retained boundary and the exact query to which it was bound.
#[derive(Clone, Copy)]
pub struct ConicalJoinSpan<'a> {
    /// The decoded boundary inputs.
    pub query: ConicalBoundaryQuery<'a>,
    /// Certificate previously established for those inputs.
    pub boundary: &'a ConicalBoundaryEvidence,
}
/// Ordered spans, contour lengths and a separate remaining work allowance.
pub struct ConicalJoinsQuery<'a> {
    /// Spans in contour order, without repeating the closing span.
    pub spans: &'a [ConicalJoinSpan<'a>],
    /// Number of spans per contour.
    pub contour_lengths: &'a [usize],
    /// Precision and operations still available after boundary certification.
    pub options: ConeClearanceOptions,
}
/// Typed refusal of a conditional contour-join certificate.
#[derive(Debug)]
pub enum ConicalJoinsFailure {
    /// Span count is outside the supported range.
    SpanCount {
        /// Supplied span count; expected 1 through 128.
        actual: usize,
    },
    /// Contours are empty, contain an empty contour, or do not cover the spans.
    Contours {
        /// Supplied contour count.
        count: usize,
        /// Sum of supplied lengths, or `None` on overflow.
        total: Option<usize>,
        /// Required total number of spans.
        spans: usize,
    },
    /// Bounded arithmetic policy is outside its supported range.
    Policy {
        /// Supplied policy.
        actual: ConeClearanceOptions,
    },
    /// A boundary certificate does not match the supplied query or precision.
    Boundary {
        /// Index of the mismatched span.
        index: usize,
    },
    /// A local proof refused the supplied span or join.
    Proof {
        /// Index of the affected span or starting span of a join.
        index: usize,
        /// Original bounded proof failure.
        cause: ConicalJoinProofFailure,
    },
}
impl std::fmt::Display for ConicalJoinsFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SpanCount { actual } => write!(f, "span count={actual}; expected 1..=128"),
            Self::Contours {
                count,
                total,
                spans,
            } => contours_message(f, *count, *total, *spans),
            Self::Policy { actual } => write!(
                f,
                "policy={actual:?}; expected fractional bits 8..=4096 and at most 1000000 operations"
            ),
            Self::Boundary { index } => write!(
                f,
                "boundary index={index}; expected matching query and precision"
            ),
            Self::Proof { index, cause } => write!(f, "join index={index}: {cause}"),
        }
    }
}
impl std::error::Error for ConicalJoinsFailure {}
/// Original bounded local join failure.
#[derive(Debug)]
pub struct ConicalJoinProofFailure(pub(super) CombinedFailure);
/// Failure class for a local conditional join proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConicalJoinFailureKind {
    /// The source and fit do not share a proven anchor branch.
    Anchor,
    /// The authored original is missing.
    Original,
    /// The two span endpoints or fit settings do not form a join.
    Join,
    /// No candidate direction could be certified.
    Direction,
    /// Bounded arithmetic, cancellation or a component proof did not complete.
    Spatial,
}
impl ConicalJoinProofFailure {
    /// Classify the refusal; `failure.kind()`.
    pub fn kind(&self) -> ConicalJoinFailureKind {
        match self.0 {
            CombinedFailure::Spatial(_) => ConicalJoinFailureKind::Spatial,
            CombinedFailure::UnalignedAnchor => ConicalJoinFailureKind::Anchor,
            CombinedFailure::MissingOriginal => ConicalJoinFailureKind::Original,
            CombinedFailure::JoinMismatch => ConicalJoinFailureKind::Join,
            CombinedFailure::DirectionUnresolved => ConicalJoinFailureKind::Direction,
        }
    }
    /// Recover cancellation, work exhaustion or another arithmetic cause when present.
    /// Example: `failure.arithmetic_cause()`.
    pub fn arithmetic_cause(&self) -> Option<&BoundedArithmeticFailure> {
        match &self.0 {
            CombinedFailure::Spatial(cause) => super::boundary_cause::spatial(cause),
            _ => None,
        }
    }
}
impl std::fmt::Display for ConicalJoinProofFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            CombinedFailure::Spatial(cause) => write!(f, "spatial proof: {cause}"),
            other => write!(f, "{other:?}"),
        }
    }
}
impl std::error::Error for ConicalJoinProofFailure {}
/// Conditional common-direction proof; it does not establish whole-body topology.
pub struct ConicalJoinsEvidence {
    pub(super) chart: Option<super::boundary_chart::ChartBinding>,
    pub(super) input: Vec<BoundarySnapshot>,
    pub(super) contour_lengths: Vec<usize>,
    pub(super) options: ConeClearanceOptions,
    pub(super) directions: Vec<[f64; 2]>,
    pub(super) operations: u32,
}
impl ConicalJoinsEvidence {
    /// Recheck inputs and policy; `evidence.matches(&query)`.
    pub fn matches(&self, query: &ConicalJoinsQuery<'_>) -> bool {
        self.chart.is_none() && self.matches_inputs(query)
    }
    fn matches_inputs(&self, query: &ConicalJoinsQuery<'_>) -> bool {
        self.options == query.options
            && self.contour_lengths == query.contour_lengths
            && self.input.len() == query.spans.len()
            && self.input.iter().zip(query.spans).all(|(snapshot, span)| {
                snapshot.matches(&span.query)
                    && span.boundary.matches(&span.query)
                    && span.boundary.options().fractional_bits == query.options.fractional_bits
            })
    }
    /// Directions in contour order; `evidence.directions()`.
    pub fn directions(&self) -> &[[f64; 2]] {
        &self.directions
    }
    /// Additional arithmetic operations; `evidence.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
}
/// Certify common directions at every contour join; `certify_conical_joins(query, || false)?`.
pub fn certify_conical_joins(
    query: ConicalJoinsQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalJoinsEvidence, ConicalJoinsFailure> {
    admit(&query)?;
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: query.options.fractional_bits,
            max_operations: query.options.max_operations,
        },
        &mut cancelled,
    );
    let requests = requests(&query)?;
    let mut directions = Vec::with_capacity(query.spans.len());
    let mut start = 0;
    for count in query.contour_lengths {
        let end = start + count;
        prove_contour(&requests, start, end, &mut arithmetic, &mut directions)?;
        start = end;
    }
    Ok(joins_evidence(query, directions, arithmetic.operations()))
}

pub(super) fn admit(query: &ConicalJoinsQuery<'_>) -> Result<(), ConicalJoinsFailure> {
    if query.spans.is_empty() || query.spans.len() > 128 {
        return Err(ConicalJoinsFailure::SpanCount {
            actual: query.spans.len(),
        });
    }
    admit_policy(query.options)?;
    admit_contour_lengths(query)
}
fn admit_policy(actual: ConeClearanceOptions) -> Result<(), ConicalJoinsFailure> {
    if !(8..=4096).contains(&actual.fractional_bits) || actual.max_operations > 1_000_000 {
        return Err(ConicalJoinsFailure::Policy { actual });
    }
    Ok(())
}

pub(super) fn requests<'a>(
    query: &'a ConicalJoinsQuery<'a>,
) -> Result<Vec<super::spatial::SpatialRequest<'a>>, ConicalJoinsFailure> {
    query
        .spans
        .iter()
        .enumerate()
        .map(|(index, span)| {
            if !span.boundary.matches(&span.query)
                || span.boundary.options().fractional_bits != query.options.fractional_bits
            {
                return Err(ConicalJoinsFailure::Boundary { index });
            }
            Ok(join_request(span))
        })
        .collect()
}
pub(super) fn prove_contour(
    requests: &[super::spatial::SpatialRequest<'_>],
    start: usize,
    end: usize,
    arithmetic: &mut Arithmetic<'_>,
    directions: &mut Vec<[f64; 2]>,
) -> Result<(), ConicalJoinsFailure> {
    let mut previous = super::developed::prepare_projection(&requests[start], arithmetic)
        .map_err(|cause| failure(start, cause))?;
    for (index, request) in requests.iter().enumerate().take(end).skip(start + 1) {
        let next = super::developed::prepare_projection(request, arithmetic)
            .map_err(|cause| failure(index, cause))?;
        directions.push(
            super::developed::prepared_join_direction(&previous, &next, arithmetic)
                .map_err(|cause| failure(index - 1, cause))?,
        );
        previous = next;
    }
    let first = super::developed::prepare_projection(&requests[start], arithmetic)
        .map_err(|cause| failure(start, cause))?;
    directions.push(
        super::developed::prepared_join_direction(&previous, &first, arithmetic)
            .map_err(|cause| failure(end - 1, cause))?,
    );
    Ok(())
}
pub(super) fn failure(index: usize, cause: CombinedFailure) -> ConicalJoinsFailure {
    ConicalJoinsFailure::Proof {
        index,
        cause: ConicalJoinProofFailure(cause),
    }
}

use super::boundary_snapshot::BoundarySnapshot;

fn admit_contour_lengths(query: &ConicalJoinsQuery<'_>) -> Result<(), ConicalJoinsFailure> {
    if query.contour_lengths.len() > query.spans.len() {
        return Err(contour_count_failure(query, None));
    }
    let total = query
        .contour_lengths
        .iter()
        .try_fold(0_usize, |sum, count| sum.checked_add(*count));
    if query.contour_lengths.is_empty()
        || query.contour_lengths.contains(&0)
        || total != Some(query.spans.len())
    {
        return Err(contour_count_failure(query, total));
    }
    Ok(())
}

fn joins_evidence(
    query: ConicalJoinsQuery<'_>,
    directions: Vec<[f64; 2]>,
    operations: u32,
) -> ConicalJoinsEvidence {
    ConicalJoinsEvidence {
        chart: None,
        input: query
            .spans
            .iter()
            .map(|span| BoundarySnapshot::capture(&span.query))
            .collect(),
        contour_lengths: query.contour_lengths.to_vec(),
        options: query.options,
        directions,
        operations,
    }
}

fn join_request<'a>(span: &'a ConicalJoinSpan<'a>) -> super::spatial::SpatialRequest<'a> {
    let mut request = combined_request(&span.query).spatial;
    if request.original.is_none() {
        request.original = Some(ConicalOriginalSpan {
            poles: span.query.source,
            index: 0,
            level: 0,
        });
    }
    request
}

fn contour_count_failure(
    query: &ConicalJoinsQuery<'_>,
    total: Option<usize>,
) -> ConicalJoinsFailure {
    ConicalJoinsFailure::Contours {
        count: query.contour_lengths.len(),
        total,
        spans: query.spans.len(),
    }
}

fn contours_message(
    out: &mut std::fmt::Formatter<'_>,
    count: usize,
    total: Option<usize>,
    spans: usize,
) -> std::fmt::Result {
    write!(
        out,
        "contours={count}, total={total:?}; expected nonzero lengths totaling {spans}"
    )
}

// Legacy Debug is a replay surface; new chart authority remains private.
impl std::fmt::Debug for ConicalJoinsEvidence {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        out.debug_struct("ConicalJoinsEvidence")
            .field("input", &self.input)
            .field("contour_lengths", &self.contour_lengths)
            .field("options", &self.options)
            .field("directions", &self.directions)
            .field("operations", &self.operations)
            .finish()
    }
}
