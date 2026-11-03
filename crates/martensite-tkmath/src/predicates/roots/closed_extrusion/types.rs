//! Typed admission, work and completion keep missing proof separate from geometric validity.
use crate::predicates::roots::sketch_incidence::SketchIncidenceCause;

#[derive(Clone, Debug, PartialEq)]
/// Invalid numeric storage; no geometric validity conclusion follows.
pub enum ClosedExtrusionInputError {
    /// Inconsistent support array lengths or multiplicities.
    Storage {
        /// Observed pole count.
        poles: usize,
        /// Observed weight count.
        weights: usize,
        /// Observed distinct knot count.
        knots: usize,
        /// Observed multiplicity count.
        multiplicities: usize,
    },
    /// A numeric value is nonfinite, or a required weight is not positive.
    Numeric {
        /// Offending numeric input role.
        field: &'static str,
        /// Offending source element index.
        index: usize,
        /// Offending supplied value.
        value: f64,
    },
    /// A knot/extrusion interval is not finite, increasing and representable.
    Interval {
        /// Offending numeric input role.
        field: &'static str,
        /// Offending source element index.
        index: usize,
        /// Offending supplied interval.
        bounds: [f64; 2],
    },
    /// The first and last represented poles differ exactly.
    OpenEndpoints {
        /// First represented endpoint.
        first: [f64; 3],
        /// Last represented endpoint, which differs exactly.
        last: [f64; 3],
    },
    /// Zero extrusion direction cannot parameterize a surface.
    ZeroDirection([f64; 3]),
}
impl std::fmt::Display for ClosedExtrusionInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "closed quadratic extrusion input={self:?}; expected finite clamped degree-two storage, exact endpoint closure, positive weights and nonzero direction"
        )
    }
}
impl std::error::Error for ClosedExtrusionInputError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Explicit bounded version-one proof policy; no search allowance is silently increased.
pub struct ClosedExtrusionPolicy {
    /// Accepted behavior version, currently one.
    pub version: u32,
    /// Maximum nonempty knot spans in the request.
    pub max_spans: usize,
    /// Maximum scheduled self and distinct-pair obligations for the whole request.
    pub max_obligations: usize,
    /// Global span-preparation and exact separation-attempt allowance.
    pub max_proof_steps: u64,
    /// Maximum retained exact numerator or denominator bit length.
    pub max_coefficient_bits: u64,
}
impl ClosedExtrusionPolicy {
    /// Select finite defaults; `ClosedExtrusionPolicy::bounded()` never means no limit.
    pub fn bounded() -> Self {
        Self {
            version: 1,
            max_spans: 256,
            max_obligations: 32_896,
            max_proof_steps: 1_000_000,
            max_coefficient_bits: 16_384,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
/// A required proof stayed unproved, without inventing a surface intersection.
pub enum ClosedExtrusionCause {
    /// Unsupported behavior version.
    PolicyVersion(u32),
    /// Positive nonconstant rational weights are outside this first operation.
    NonConstantWeights,
    /// A caller-requested finite limit was exceeded.
    Limit {
        /// The limit or representation invariant that refused completion.
        kind: &'static str,
        /// The unchanged requested limit.
        requested: u64,
        /// The observed value or source size.
        actual: u64,
    },
    /// Cancellation was observed at a concrete checkpoint.
    Cancelled,
    /// Exact source admission failed.
    SpanAdmission(SketchIncidenceCause),
    /// Exact span construction violated a required representation invariant.
    SpanInvariant {
        /// Required representation invariant.
        kind: &'static str,
        /// Observed source count or ordinal.
        actual: usize,
    },
    /// These exact hulls did not establish separation.
    UnseparatedPair {
        /// First source span.
        first: usize,
        /// Second source span.
        second: usize,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// A projected span is proved noninjective; finite-interval surface validity is separate.
pub enum ClosedExtrusionProjectionContact {
    /// The entire projected span is one point.
    CollapsedSpan {
        /// Source span whose complete projection is constant.
        span: usize,
    },
    /// A projected quadratic reverses along its line inside the span.
    BacktrackingSpan {
        /// Source span with an interior derivative sign reversal.
        span: usize,
    },
}

#[derive(Clone, Debug, PartialEq)]
/// A support certificate is distinct from incomplete and failed exclusion.
pub enum ClosedExtrusionCompletion {
    /// Every required obligation is proved on the declared endpoint seam quotient.
    Certified,
    /// A proved projected contact prevents this sufficient exclusion.
    NotExcluded(ClosedExtrusionProjectionContact),
    /// At least one required proof remains unaccounted.
    Incomplete(ClosedExtrusionCause),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
/// The actual stage of the latest proof checkpoint.
pub enum ClosedExtrusionStage {
    /// Policy and support scope admission.
    #[default]
    Admission,
    /// Exact projected span preparation and domain coverage.
    Spans,
    /// Canonical off-diagonal span exclusion.
    SelfSpan,
    /// Distinct-span product exclusion.
    SpanPair,
    /// Final complete-coverage publication check.
    Publication,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
/// Observed whole-request work; declarations are not evidence of completion.
pub struct ClosedExtrusionWork {
    /// Stage reached before completion or refusal.
    pub stage: ClosedExtrusionStage,
    /// Nonempty spans declared by the admitted source knots.
    pub spans: usize,
    /// Required self and distinct-span products, before any early termination.
    pub scheduled_obligations: usize,
    /// Obligations actually entered.
    pub started_obligations: usize,
    /// Obligations whose exclusion proof finished.
    pub finished_obligations: usize,
    /// Started exact preparation/separation checkpoints, including failure.
    pub started_proof_steps: u64,
    /// This exact hull route performs no root subdivisions.
    pub root_subdivisions: u64,
}
