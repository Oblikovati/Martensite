// SPDX-License-Identifier: GPL-2.0-only
//! Provisional source-bound cone boundary error-budget interface.
use super::angle::*;
use super::combined::CombinedFailure;
use super::storage::ConicalStorageQuery;
use super::*;

pub use super::boundary_kind::ConicalBoundaryFailureKind;
pub use super::joins::ConicalJoinFailureKind;
pub use super::joins::ConicalJoinProofFailure;
pub use super::joins::ConicalJoinSpan;
pub use super::joins::ConicalJoinsEvidence;
pub use super::joins::ConicalJoinsFailure;
pub use super::joins::ConicalJoinsQuery;
pub use super::joins::certify_conical_joins;

use super::boundary_snapshot::BoundarySnapshot;

/// Source, fit, support and two supplied stored-edge coefficient sets.
#[derive(Clone, Copy)]
pub struct ConicalBoundaryQuery<'a> {
    /// Rounded child poles on local `[0,1]`; when `original` is `None`, these are the authored poles.
    pub source: &'a [[f64; 2]],
    /// Original authored poles and exact interval, when `source` is a rounded child.
    pub original: Option<ConicalOriginalSpan<'a>>,
    /// Bernstein half-angle tangent fit coefficients.
    pub angular_fit: &'a [f64],
    /// Homogeneous slant numerator coefficients.
    pub slant_numerator: &'a [f64],
    /// Positive homogeneous slant weights.
    pub slant_weights: &'a [f64],
    /// Cone semi-angle in radians.
    pub semi_angle: f64,
    /// Physical-angle chart origin.
    pub chart_origin: f64,
    /// Center of the rational angular fit.
    pub fit_center: f64,
    /// Whole developed-plane turns, applied before cone angle scaling.
    pub developed_turn: i32,
    /// Whole physical turns, applied after cone angle scaling.
    pub physical_turn: i32,
    /// Increasing signed normal-depth endpoints.
    pub depths: [f64; 2],
    /// Whether positive depth follows the outward normal.
    pub outward: bool,
    /// Original requested local-axis spatial error limit.
    pub spatial_limit: f64,
    /// Stored Bézier coefficients at the first and last depths.
    pub edges: [ConicalStorageQuery<'a>; 2],
}
/// Stage that declined a boundary error proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConicalBoundaryStage {
    /// Arithmetic policy or supported shape admission.
    Admission,
    /// The supplied endpoint data did not match the source and fit.
    Binding,
    /// Source-span capture was incomplete or invalid.
    Source,
    /// Original-to-child error proof was incomplete or invalid.
    Original,
    /// Angular error proof was incomplete or invalid.
    Angular,
    /// Slant error proof was incomplete or invalid.
    Slant,
    /// First stored-edge comparison failed.
    FirstStorage,
    /// Last stored-edge comparison failed.
    LastStorage,
    /// Shared bounded arithmetic did not finish.
    Arithmetic,
    /// The original requested error limit was not certified.
    Budget,
}
/// Completed bounded ideal-and-endpoint comparison with immutable source binding.
pub struct ConicalBoundaryEvidence {
    pub(super) chart: Option<super::boundary_chart::ChartBinding>,
    pub(super) retained: Option<std::sync::Arc<super::spatial::RetainedSpatial>>,
    pub(super) total: DyadicBound,
    pub(super) ideal: DyadicBound,
    pub(super) original: DyadicBound,
    pub(super) angular_derivative: DyadicBound,
    pub(super) original_angular_derivative: DyadicBound,
    pub(super) total_angular_derivative: DyadicBound,
    pub(super) authored_level: u8,
    pub(super) anchor: super::original::AnchorEvidence,
    pub(super) storage: [DyadicBound; 2],
    pub(super) operations: u32,
    pub(super) options: ConeClearanceOptions,
    pub(super) input: BoundarySnapshot,
}
/// Typed refusal retaining the stage, cause, policy and bounded input binding.
#[derive(Debug)]
pub struct ConicalBoundaryError {
    pub(super) stage: ConicalBoundaryStage,
    pub(super) cause: ConicalBoundaryFailure,
    pub(super) operations: u32,
    pub(super) options: ConeClearanceOptions,
    pub(super) input: Option<Box<BoundarySnapshot>>,
}
/// Typed reason why the boundary proof could not complete.
#[derive(Debug)]
pub enum ConicalBoundaryFailure {
    /// The arithmetic policy is outside the supported bounds.
    Policy {
        /// Requested precision in fractional bits.
        fractional_bits: u16,
        /// Requested operation limit.
        max_operations: u32,
    },
    /// An input collection is outside the supported shape.
    Shape {
        /// Input collection name.
        role: &'static str,
        /// Supplied number of elements.
        actual: usize,
        /// Accepted shape.
        expected: &'static str,
    },
    /// A component proof refused or could not complete.
    Proof(ConicalBoundaryProofFailure),
}
/// Component failure retaining its stage and underlying diagnostic.
#[derive(Debug)]
pub struct ConicalBoundaryProofFailure {
    pub(super) stage: ConicalBoundaryStage,
    pub(super) inner: Box<CombinedFailure>,
}
impl ConicalBoundaryProofFailure {
    /// Stage which failed; `failure.stage()`.
    pub fn stage(&self) -> ConicalBoundaryStage {
        self.stage
    }
    /// Stable component failure class; `failure.kind()`.
    pub fn kind(&self) -> ConicalBoundaryFailureKind {
        super::boundary_kind::classify(&self.inner)
    }
    /// Original bounded arithmetic cause, when applicable; `failure.arithmetic_cause()`.
    pub fn arithmetic_cause(&self) -> Option<&BoundedArithmeticFailure> {
        super::boundary_cause::combined(&self.inner)
    }
    /// Original source failure, when applicable; `failure.source_cause()`.
    pub fn source_cause(&self) -> Option<&ConicalSourceFailure> {
        match self.inner.as_ref() {
            CombinedFailure::Spatial(super::spatial::SpatialFailure::Source(cause)) => Some(cause),
            _ => None,
        }
    }
    /// Original-to-child comparison failure; `failure.original_cause()`.
    pub fn original_cause(&self) -> Option<&ConicalOriginalFailure> {
        match self.inner.as_ref() {
            CombinedFailure::Spatial(super::spatial::SpatialFailure::Original(cause)) => {
                Some(cause)
            }
            _ => None,
        }
    }
    /// Edge index and original storage failure; `failure.storage_cause()`.
    pub fn storage_cause(&self) -> Option<(usize, &super::storage::ConicalStorageFailure)> {
        match self.inner.as_ref() {
            CombinedFailure::Storage { depth, cause } => Some((*depth, cause)),
            _ => None,
        }
    }
    /// Certified upper bound and original limit on budget refusal; `failure.budget()`.
    pub fn budget(&self) -> Option<(&DyadicBound, f64)> {
        match self.inner.as_ref() {
            CombinedFailure::Uncertified { upper, limit }
            | CombinedFailure::Spatial(super::spatial::SpatialFailure::Uncertified {
                upper,
                limit,
            }) => Some((upper, *limit)),
            _ => None,
        }
    }
}
impl std::fmt::Display for ConicalBoundaryProofFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
impl std::error::Error for ConicalBoundaryProofFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.inner.as_ref())
    }
}
/// Bound the ideal mapping and supplied retained depth edges under one spatial limit.
/// The caller must establish native `[0,1]` parameter, location, ruled-row and topology correspondence.
/// With an original interval, source and child must use the angle lifts induced by
/// the apex-avoiding joining segments, the same support, depth and normal sense.
/// The original interval is reparameterized to the child's `[0,1]` domain.
/// Example: `let proof = certify_conical_boundary(query, options, || false)?;`.
pub fn certify_conical_boundary(
    query: ConicalBoundaryQuery<'_>,
    options: ConeClearanceOptions,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalBoundaryEvidence, ConicalBoundaryError> {
    let input = admit_query(&query, options)?;
    execute(query, options, input, &mut cancelled)
}
pub(super) fn admit_query(
    query: &ConicalBoundaryQuery<'_>,
    options: ConeClearanceOptions,
) -> Result<BoundarySnapshot, ConicalBoundaryError> {
    let shape = admit_shape(query);
    if !(8..=4096).contains(&options.fractional_bits) || options.max_operations > 1_000_000 {
        return Err(policy_error(query, options, shape.is_ok()));
    }
    shape.map_err(|cause| ConicalBoundaryError {
        stage: ConicalBoundaryStage::Admission,
        cause,
        operations: 0,
        options,
        input: None,
    })?;
    Ok(BoundarySnapshot::capture(query))
}
pub(super) fn policy_error(
    query: &ConicalBoundaryQuery<'_>,
    options: ConeClearanceOptions,
    supported: bool,
) -> ConicalBoundaryError {
    ConicalBoundaryError {
        stage: ConicalBoundaryStage::Admission,
        cause: ConicalBoundaryFailure::Policy {
            fractional_bits: options.fractional_bits,
            max_operations: options.max_operations,
        },
        operations: 0,
        options,
        input: supported.then(|| Box::new(BoundarySnapshot::capture(query))),
    }
}
pub(super) fn admit_shape(query: &ConicalBoundaryQuery<'_>) -> Result<(), ConicalBoundaryFailure> {
    count("source", query.source.len(), 2..=4, "2..=4")?;
    if let Some(original) = query.original {
        count("original source", original.poles.len(), 2..=4, "2..=4")?;
    }
    count("angular fit", query.angular_fit.len(), 1..=4, "1..=4")?;
    count(
        "slant numerator",
        query.slant_numerator.len(),
        1..=10,
        "1..=10",
    )?;
    count("slant weights", query.slant_weights.len(), 1..=10, "1..=10")?;
    for edge in &query.edges {
        admit_edge_counts(edge)?;
    }
    Ok(())
}
pub(super) fn admit_edge_counts(
    edge: &ConicalStorageQuery<'_>,
) -> Result<(), ConicalBoundaryFailure> {
    count("edge tangent", edge.tangent.len(), 1..=4, "1..=4")?;
    count("edge numerator", edge.numerator.len(), 1..=10, "1..=10")?;
    count("edge fit weights", edge.weights.len(), 1..=10, "1..=10")?;
    count("edge poles", edge.stored_poles.len(), 2..=16, "2..=16")?;
    count(
        "edge stored weights",
        edge.stored_weights.len(),
        2..=16,
        "2..=16",
    )
}
pub(super) fn count(
    role: &'static str,
    actual: usize,
    range: std::ops::RangeInclusive<usize>,
    expected: &'static str,
) -> Result<(), ConicalBoundaryFailure> {
    if !range.contains(&actual) {
        return Err(ConicalBoundaryFailure::Shape {
            role,
            actual,
            expected,
        });
    }
    Ok(())
}
pub(super) fn execute(
    query: ConicalBoundaryQuery<'_>,
    options: ConeClearanceOptions,
    input: BoundarySnapshot,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Result<ConicalBoundaryEvidence, ConicalBoundaryError> {
    let request = combined_request(&query);
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: options.fractional_bits,
            max_operations: options.max_operations,
        },
        cancelled,
    );
    match super::combined::certify_combined(&request, &mut arithmetic) {
        Ok(result) => Ok(evidence(
            result,
            options,
            input,
            query.original.map_or(0, |value| value.level),
        )),
        Err(cause) => Err(proof_error(cause, arithmetic.operations(), options, input)),
    }
}
pub(super) fn proof_error(
    cause: CombinedFailure,
    operations: u32,
    options: ConeClearanceOptions,
    input: BoundarySnapshot,
) -> ConicalBoundaryError {
    let stage = proof_stage(&cause);
    ConicalBoundaryError {
        stage,
        cause: ConicalBoundaryFailure::Proof(ConicalBoundaryProofFailure {
            stage,
            inner: Box::new(cause),
        }),
        operations,
        options,
        input: Some(Box::new(input)),
    }
}

pub(super) fn combined_request<'a>(
    query: &'a ConicalBoundaryQuery<'a>,
) -> super::combined::CombinedRequest<'a> {
    let anchor = boundary_anchor(query);
    let spatial = super::spatial::SpatialRequest {
        source: query.source,
        original: query.original,
        angular_fit: query.angular_fit,
        slant_numerator: query.slant_numerator,
        weights: query.slant_weights,
        anchor,
        depths: query.depths,
        outward: query.outward,
        limit: query.spatial_limit,
    };
    super::combined::CombinedRequest {
        spatial,
        edges: query.edges,
    }
}
pub(super) fn boundary_anchor(query: &ConicalBoundaryQuery<'_>) -> AnchorRequest {
    AnchorRequest {
        point: query.source[0],
        semi_angle: query.semi_angle,
        origin: query.chart_origin,
        developed_turn: query.developed_turn,
        physical_turn: query.physical_turn,
        fit_center: query.fit_center,
        fit_half_angle_tangent: query.angular_fit[0],
    }
}
pub(super) fn proof_stage(cause: &CombinedFailure) -> ConicalBoundaryStage {
    match cause {
        CombinedFailure::Binding { .. } => ConicalBoundaryStage::Binding,
        CombinedFailure::Storage { depth: 0, .. } => ConicalBoundaryStage::FirstStorage,
        CombinedFailure::Storage { .. } => ConicalBoundaryStage::LastStorage,
        CombinedFailure::Arithmetic(_) => ConicalBoundaryStage::Arithmetic,
        CombinedFailure::Uncertified { .. } => ConicalBoundaryStage::Budget,
        CombinedFailure::Spatial(cause) => match cause {
            super::spatial::SpatialFailure::Domain { .. } => ConicalBoundaryStage::Admission,
            super::spatial::SpatialFailure::Source(_) => ConicalBoundaryStage::Source,
            super::spatial::SpatialFailure::Original(_) => ConicalBoundaryStage::Original,
            super::spatial::SpatialFailure::Angular(_) => ConicalBoundaryStage::Angular,
            super::spatial::SpatialFailure::Slant(_) => ConicalBoundaryStage::Slant,
            super::spatial::SpatialFailure::Arithmetic(_) => ConicalBoundaryStage::Arithmetic,
            super::spatial::SpatialFailure::Uncertified { .. } => ConicalBoundaryStage::Budget,
        },
    }
}
pub(super) fn evidence(
    result: super::combined::CombinedEvidence,
    options: ConeClearanceOptions,
    input: BoundarySnapshot,
    authored_level: u8,
) -> ConicalBoundaryEvidence {
    let exponent = -i32::from(options.fractional_bits);
    let original_angular_derivative =
        DyadicBound::new(result.original_angular_derivative.upper, exponent);
    ConicalBoundaryEvidence {
        chart: None,
        retained: result.retained,
        total: DyadicBound::new(result.total.upper, exponent),
        ideal: DyadicBound::new(result.ideal.upper, exponent),
        original: DyadicBound::new(result.original.upper, exponent),
        angular_derivative: DyadicBound::new(result.angular_derivative.upper, exponent),
        original_angular_derivative,
        total_angular_derivative: DyadicBound::new(result.total_angular_derivative.upper, exponent),
        authored_level,
        anchor: result.anchor,
        storage: storage_bounds(result.storage, exponent),
        operations: result.operations,
        options,
        input,
    }
}
fn storage_bounds(storage: [Interval; 2], exponent: i32) -> [DyadicBound; 2] {
    storage.map(|bound| {
        // Release each unused lower endpoint before converting the next interval.
        DyadicBound::new(bound.upper, exponent)
    })
}
impl ConicalBoundaryError {
    /// Typed cause with retained component diagnostics; `error.cause()`.
    pub fn cause(&self) -> &ConicalBoundaryFailure {
        &self.cause
    }
    /// Stage that refused the proof; `error.stage()`.
    pub fn stage(&self) -> ConicalBoundaryStage {
        self.stage
    }
    /// Charged work before refusal; `error.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Requested arithmetic policy; `error.options()`.
    pub fn options(&self) -> ConeClearanceOptions {
        self.options
    }
    /// Compare retained input bits, or `None` for unsupported input shapes.
    /// Example: `let same = error.matches(&query);`.
    pub fn matches(&self, query: &ConicalBoundaryQuery<'_>) -> Option<bool> {
        self.input.as_ref().map(|input| input.matches(query))
    }
}
impl std::fmt::Display for ConicalBoundaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}: {} after {} operations",
            self.stage, self.cause, self.operations
        )
    }
}
impl std::fmt::Display for ConicalBoundaryFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Policy {
                fractional_bits,
                max_operations,
            } => write!(
                f,
                "precision={fractional_bits}, work={max_operations}; expected precision 8..=4096 and work <=1000000"
            ),
            Self::Shape {
                role,
                actual,
                expected,
            } => write!(f, "{role} count={actual}; expected {expected}"),
            Self::Proof(cause) => write!(f, "{cause}"),
        }
    }
}
impl std::error::Error for ConicalBoundaryFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Proof(cause) => Some(cause),
            _ => None,
        }
    }
}
impl std::error::Error for ConicalBoundaryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.cause)
    }
}
