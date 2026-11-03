//! Policy and typed outcomes never turn missing proof into validity.
use martensite_tkmath::predicates::roots::{
    DyadicBound,
    closed_extrusion::{
        ClosedExtrusionCause, ClosedExtrusionPolicy, ClosedExtrusionProjectionContact,
    },
    metric_bounds::ResidualError,
};

/// Finite version-one settings; `ClosedFacePolicy::bounded()` selects explicit caps.
#[derive(Clone, Copy, Debug)]
pub struct ClosedFacePolicy {
    /// Accepted operation version, currently one.
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
/// An unfinished required proof, distinct from a proven geometric defect.
#[derive(Debug)]
pub enum ClosedFaceIncomplete {
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
    },
    /// The support proof could not exclude every required product.
    Support(ClosedExtrusionCause),
    /// Projected noninjectivity alone is not a finite-interval 3D intersection witness.
    Projection(ClosedExtrusionProjectionContact),
    /// The exact metric operation did not complete.
    Metric(ResidualError),
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
