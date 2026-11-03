//! Policy and typed outcomes never turn missing proof into validity.
use martensite_tkmath::predicates::roots::{
    DyadicBound,
    closed_extrusion::{
        ClosedExtrusionCause, ClosedExtrusionPolicy, ClosedExtrusionProjectionContact,
    },
    metric_bounds::{CurveIdentityDecision, ResidualError},
};

/// Finite proof settings; `ClosedFacePolicy::bounded()` selects the coincident contract.
#[derive(Clone, Copy, Debug)]
pub struct ClosedFacePolicy {
    /// Explicit boundary check version: one is coincident, two is piecewise, three admits relative carrier frames.
    pub version: u32,
    /// Whole-support span and exact proof limits.
    pub support: ClosedExtrusionPolicy,
    /// Whole-face exact identity and metric query allowance.
    pub max_queries: u64,
    /// Whole-face completed metric subdivisions allowance.
    pub max_bisections: u64,
    /// Maximum combined datum chain length.
    pub max_location_steps: usize,
    /// Maximum absolute datum power for bounded replay.
    pub max_location_power: u32,
}
impl ClosedFacePolicy {
    /// Select finite defaults; `ClosedFacePolicy::bounded()` never means unlimited.
    pub fn bounded() -> Self {
        Self {
            version: 1,
            support: ClosedExtrusionPolicy::bounded(),
            max_queries: 4096,
            max_bisections: 4096,
            max_location_steps: 64,
            max_location_power: 1024,
        }
    }
}
/// Exact obligations selected for the represented face; inspect `policy.check_mode()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClosedFaceCheckMode {
    /// Actual spatial boundary is exactly coincident with the ideal trimmed support.
    CoincidentBoundary,
    /// Ideal surface and actual boundary are separately injective with exact transverse joins.
    /// Their same-edge boundary representations need only satisfy stored metric limits.
    PiecewiseRepresentations,
}
impl ClosedFacePolicy {
    /// Select finite piecewise boundary proofs explicitly; `ClosedFacePolicy::piecewise_boundary()`.
    pub fn piecewise_boundary() -> Self {
        Self {
            version: 2,
            ..Self::bounded()
        }
    }
    /// Derive carrier-frame candidates, then certify their exact world relation (#213).
    /// Example: `ClosedFacePolicy::placed_piecewise_boundary()` retains finite limits.
    pub fn placed_piecewise_boundary() -> Self {
        Self {
            version: 3,
            ..Self::piecewise_boundary()
        }
    }
    /// Read the selected represented-geometry contract; `policy.check_mode()`.
    pub fn check_mode(&self) -> Option<ClosedFaceCheckMode> {
        match self.version {
            1 => Some(ClosedFaceCheckMode::CoincidentBoundary),
            2 | 3 => Some(ClosedFaceCheckMode::PiecewiseRepresentations),
            _ => None,
        }
    }
}
/// An exact coordinate-functional candidate failed a necessary carrier condition.
#[derive(Clone, Debug)]
pub enum ClosedFacePlaneRefusal {
    /// The extrusion has zero height in this coordinate functional.
    ZeroDirection,
    /// A source coefficient did not lie in the ideal basal plane.
    IdealVariation {
        /// Original coefficient position in the retained basis.
        coefficient: usize,
        /// Exact relation result for this coefficient.
        actual: CurveIdentityDecision,
    },
    /// An actual carrier coefficient did not lie at the declared reference height.
    CarrierLevel {
        /// Original coefficient position in the retained basis.
        coefficient: usize,
        /// Exact relation result for this coefficient.
        actual: CurveIdentityDecision,
    },
}
/// A source field outside this finite operation's represented domain.
#[derive(Clone, Debug, PartialEq)]
pub struct ClosedFaceUnsupported {
    /// Invocation-local source owner from the retained input.
    pub owner: usize,
    /// Rejected represented field.
    pub field: &'static str,
    /// Actual rejected representation or value.
    pub actual: String,
    /// Required finite representation shape.
    pub expected: &'static str,
}
/// An independently established contradiction in the declared face representation.
#[derive(Clone, Debug, PartialEq)]
pub struct ClosedFaceViolation {
    /// Invocation-local source entity.
    pub owner: usize,
    /// Failed declared invariant.
    pub invariant: &'static str,
    /// Actual contradictory values, not a sampled estimate.
    pub actual: String,
}
/// Numerical candidate failures never establish an exact geometric contradiction.
#[derive(Clone, Debug, PartialEq)]
pub enum ClosedFaceDirectionCandidateFailure {
    /// The existing matrix inverse refused its finite numerical domain.
    InverseUnavailable {
        /// Original inverse failure, retained for diagnosis.
        cause: String,
    },
    /// Candidate arithmetic produced a nonfinite component.
    NonFinite {
        /// Rejected component position.
        coordinate: usize,
        /// Exact rejected floating-point representation.
        bits: u64,
    },
    /// A zero candidate cannot represent the admitted extrusion direction.
    ZeroVector,
}
/// An unfinished required proof, distinct from a proven geometric defect.
#[derive(Debug)]
pub enum ClosedFaceIncomplete {
    /// A numerical inverse candidate could not be represented; no exact relation was accepted.
    CarrierDirectionCandidate {
        /// Actual source owner and required relation.
        query: ClosedFaceQueryContext,
        /// Original inverse failure or rejected candidate shape.
        cause: ClosedFaceDirectionCandidateFailure,
    },
    /// Cancellation was observed on owned input.
    Cancelled,
    /// A finite caller allowance was exhausted.
    Limit {
        /// Exhausted work category.
        kind: &'static str,
        /// Requested unchanged limit.
        requested: u64,
        /// Actually requested or spent work.
        actual: u64,
        /// Specific query when the limit interrupted a known relation.
        query: Option<ClosedFaceQueryContext>,
    },
    /// The support proof could not exclude every required product.
    Support(ClosedExtrusionCause),
    /// Projected noninjectivity alone is not a finite-interval 3D intersection witness.
    Projection(ClosedExtrusionProjectionContact),
    /// The exact metric operation did not complete for this source relation (#378).
    Metric {
        /// Actual source owner and requested relation.
        query: ClosedFaceQueryContext,
        /// Original numerical failure, retained without translation.
        cause: ResidualError,
    },
    /// A required exact actual-boundary relation did not hold.
    BoundaryRelation {
        /// Invocation-local boundary owner in the retained snapshot.
        owner: usize,
        /// Required relation that could not be established.
        relation: &'static str,
        /// Exact comparison result, never a metric approximation.
        actual: CurveIdentityDecision,
    },
    /// No tested coordinate functional certified a common transverse plane.
    BoundaryPlane([ClosedFacePlaneRefusal; 3]),
    /// The actual carrier's independent support proof remained incomplete.
    BoundarySupport(ClosedExtrusionCause),
    /// An actual carrier projection could not be excluded.
    BoundaryProjection(ClosedExtrusionProjectionContact),
    /// Metric conformance alone does not transfer support injectivity to an altered boundary.
    BoundaryIdentity(ClosedFaceErrorBound),
    /// A coefficient upper was insufficient; this does not prove curve exceedance.
    CoefficientBound {
        /// Source edge occurrence.
        edge: usize,
        /// Unchanged spatial limit.
        limit: f64,
    },
}
/// Exact face checking keeps unsupported and unproved inputs separate; inspect `report.completion()`.
#[derive(Debug)]
pub enum ClosedFaceCompletion {
    /// All declared face obligations are certified.
    Certified,
    /// The supplied representation is outside this finite domain.
    Unsupported(ClosedFaceUnsupported),
    /// A declared topology or numeric consistency invariant is contradicted.
    Invalid(ClosedFaceViolation),
    /// At least one required proof is unfinished.
    Incomplete(ClosedFaceIncomplete),
}
/// Ordered obligation evidence; `report.coverage().topology` is never inferred from completion flags.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClosedFaceCoverage {
    /// Complete rectangle, seam and endpoint quotient.
    pub topology: bool,
    /// Stored curves and vertices conform to the represented support.
    pub geometry: bool,
    /// Whole represented support is injective modulo its declared seam.
    pub self_exclusion: bool,
    /// Every accepted error bound respects every applicable stored raw limit.
    pub tolerances: bool,
}
/// Actual finite work, including unsuccessful queries; `report.work().queries`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClosedFaceWork {
    /// Exact identity and metric queries actually entered.
    pub queries: u64,
    /// Completed metric root subdivisions, never refunded.
    pub bisections: u64,
    /// Datum replay steps actually entered.
    pub location_steps: usize,
    /// Spans declared by all admitted support proof inputs.
    pub support_spans: usize,
    /// Required full-support span products.
    pub support_obligations: usize,
    /// Support products actually certified.
    pub support_finished: usize,
    /// Support proof checkpoints actually entered.
    pub support_steps: u64,
}
/// One certified spatial upper, not a measured tight supremum.
#[derive(Clone, Debug)]
pub struct ClosedFaceErrorBound {
    /// Source edge occurrence in canonical boundary traversal.
    pub edge: usize,
    /// Optional endpoint occurrence within that edge.
    pub vertex: Option<usize>,
    /// Bound role: cap coefficient, seam, vertex/curve or vertex/support.
    pub role: &'static str,
    /// Unchanged minimum applicable raw spatial limit.
    pub spatial_limit: f64,
    /// Certified squared upper for the complete requested interval.
    pub squared_upper: DyadicBound,
}
/// Read-only checking creates no replacement geometry; `report.history()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClosedFaceHistory {
    /// No input geometry or topology was changed.
    NoGeometryChanges,
}

/// A finite relation within the owned face operation; `ClosedFaceRelation::SeamConformance`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClosedFaceRelation {
    /// Trim endpoint and parameter-space corner identity.
    TrimEndpoint,
    /// Located vertex identity in the rectangle quotient.
    VertexIncidence,
    /// Exact identity of an already bounded spatial relation.
    BoundaryIdentity,
    /// Complete spatial seam conformance to the ideal support.
    SeamConformance,
    /// One cap coefficient's spatial conformance to the ideal support.
    CapCoefficientConformance,
    /// Vertex conformance to its spatial curve endpoint.
    VertexCurveConformance,
    /// Vertex conformance to its support corner.
    VertexSupportConformance,
    /// Equality of actual and ideal world extrusion directions.
    CarrierDirection,
    /// Actual cap coefficients on the independent carrier.
    CarrierCap,
    /// Actual seam on the independent carrier.
    CarrierSeam,
    /// Actual seam on the canonical support.
    CanonicalSeam,
    /// Vertex on its independent carrier corner.
    CarrierVertex,
    /// A nonzero transverse coordinate of the extrusion direction.
    TransverseDirection,
    /// Constant canonical basal height.
    CanonicalHeight,
    /// Actual carrier basal height at the declared reference parameter.
    CarrierHeight,
}
impl ClosedFaceRelation {
    /// Read the relation's diagnostic label; `ClosedFaceRelation::SeamConformance.label()`.
    pub const fn label(self) -> &'static str {
        match self {
            Self::TrimEndpoint => "trim endpoint corner",
            Self::VertexIncidence => "located endpoint quotient",
            Self::BoundaryIdentity => "coincident boundary identity",
            Self::SeamConformance => "seam",
            Self::CapCoefficientConformance => "cap coefficient",
            Self::VertexCurveConformance => "vertex/curve",
            Self::VertexSupportConformance => "vertex/support",
            Self::CarrierDirection => "same actual world extrusion direction",
            Self::CarrierCap => "actual cap coefficient on carrier",
            Self::CarrierSeam => "actual seam on boundary carrier",
            Self::CanonicalSeam => "actual seam on canonical support",
            Self::CarrierVertex => "actual vertex on carrier corner",
            Self::TransverseDirection => "transverse coordinate",
            Self::CanonicalHeight => "ideal basal height",
            Self::CarrierHeight => "actual carrier height",
        }
    }
}
/// A failed query's actual source identity, never a boundary-list index (#378).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClosedFaceQueryContext {
    /// Invocation-local owner label in `report.input()`.
    pub owner: usize,
    /// Exact relation whose query was entered or refused.
    pub relation: ClosedFaceRelation,
}
impl ClosedFaceQueryContext {
    /// Bind a query to a source owner; `ClosedFaceQueryContext::new(owner, relation)`.
    pub const fn new(owner: usize, relation: ClosedFaceRelation) -> Self {
        Self { owner, relation }
    }
    pub(super) fn metric(self, cause: ResidualError) -> ClosedFaceCompletion {
        ClosedFaceCompletion::Incomplete(ClosedFaceIncomplete::Metric { query: self, cause })
    }
    pub(super) fn attach_limit(self, completion: ClosedFaceCompletion) -> ClosedFaceCompletion {
        match completion {
            ClosedFaceCompletion::Incomplete(ClosedFaceIncomplete::Limit {
                kind,
                requested,
                actual,
                ..
            }) => ClosedFaceCompletion::Incomplete(ClosedFaceIncomplete::Limit {
                kind,
                requested,
                actual,
                query: Some(self),
            }),
            other => other,
        }
    }
}
