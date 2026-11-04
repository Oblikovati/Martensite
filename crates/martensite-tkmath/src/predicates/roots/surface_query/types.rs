//! Owned supplied coefficients and finite proof accounting are separate from face validity.
/// One row-major rational Bézier tensor on its normalized unit square.
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceQueryPatch {
    /// U/V polynomial degrees, each one or two.
    pub degree: [usize; 2],
    /// Row-major Cartesian control points, with V varying fastest.
    pub poles: Vec<[f64; 3]>,
    /// Positive effective evaluator weights; nonrational callers supply ones explicitly.
    pub weights: Vec<f64>,
}
/// Owned canonical query domain, before a proof starts.
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceQueryInput {
    /// One nonperiodic patch or one ordered three-patch periodic cycle.
    pub patches: Vec<SurfaceQueryPatch>,
    /// Only U-periodic aliases are admitted; no V endpoint collapse is inferred.
    pub periodic_u: bool,
}
/// Explicit bounded version-one exact query proof; `SurfaceQueryPolicy::bounded()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceQueryPolicy {
    /// Supported behavior version, currently one.
    pub version: u32,
    /// Maximum exact coefficient operations across all attempted sufficient proofs.
    pub max_steps: u64,
    /// Maximum retained numerator or denominator bit length.
    pub max_coefficient_bits: u64,
}
impl SurfaceQueryPolicy {
    /// Select finite limits without widening them during search; `SurfaceQueryPolicy::bounded()`.
    pub const fn bounded() -> Self {
        Self {
            version: 1,
            max_steps: 1_000_000,
            max_coefficient_bits: 16_384,
        }
    }
}
/// A sufficient proof is unavailable; this never asserts a geometric intersection.
#[derive(Clone, Debug, PartialEq)]
pub enum SurfaceQueryCause {
    /// The requested version is unsupported.
    Version(u32),
    /// The observed source shape differs from the bounded family.
    Layout {
        /// Source patch ordinal, not a geometry identity.
        patch: usize,
        /// Offending storage or domain role.
        field: &'static str,
        /// Actual source count or value.
        actual: usize,
    },
    /// An input coefficient is nonfinite or a weight is not positive.
    Numeric {
        /// Source patch ordinal.
        patch: usize,
        /// Offending coefficient role.
        field: &'static str,
        /// Source coefficient position.
        index: usize,
        /// Actual source value.
        value: f64,
    },
    /// A finite bound was exhausted without increasing its requested value.
    Limit {
        /// Exhausted dimension.
        field: &'static str,
        /// Requested finite limit.
        requested: u64,
        /// Observed work or bit count.
        actual: u64,
    },
    /// Cancellation was observed before an exact coefficient operation.
    Cancelled,
    /// A required exact relation was not established.
    Unproved {
        /// Source patch ordinal.
        patch: usize,
        /// Required sufficient relation, not a sampled failure.
        relation: &'static str,
    },
}
/// Executed exact arithmetic, including unsuccessful sufficient projection attempts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SurfaceQueryWork {
    /// Exact coefficient operations consumed.
    pub steps: u64,
    /// Largest retained exact numerator or denominator bit length.
    pub coefficient_bits: u64,
    /// Successful strict-positive polynomial hull obligations.
    pub positive_hulls: u64,
}
/// Which sufficient embedding theorem was established on the canonical input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SurfaceQueryTheorem {
    /// Positive symmetric projected Jacobian on a convex parameter square.
    StrictProjection {
        /// Ordered spatial projection coordinates.
        coordinates: [usize; 2],
        /// Applied coordinate signs, each minus or plus one.
        signs: [i32; 2],
    },
    /// Strict axial separation and one positive angular winding modulo the declared U cycle.
    AxialAngularCycle,
}
/// Immutable source-bound proof report; `report.certificate()` is absent after incomplete work.
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceQueryReport {
    pub(super) input: SurfaceQueryInput,
    pub(super) policy: SurfaceQueryPolicy,
    pub(super) work: SurfaceQueryWork,
    pub(super) outcome: Result<SurfaceQueryTheorem, SurfaceQueryCause>,
}
impl SurfaceQueryReport {
    /// Read the complete owned coefficient input; `report.input()`.
    pub fn input(&self) -> &SurfaceQueryInput {
        &self.input
    }
    /// Read the unchanged requested limits; `report.policy()`.
    pub fn policy(&self) -> SurfaceQueryPolicy {
        self.policy
    }
    /// Read actual consumed exact work; `report.work()`.
    pub fn work(&self) -> SurfaceQueryWork {
        self.work
    }
    /// Read the sufficient theorem only on complete proof; `report.certificate()`.
    pub fn certificate(&self) -> Option<&SurfaceQueryTheorem> {
        self.outcome.as_ref().ok()
    }
    /// Read a typed missing obligation; `report.incomplete()`.
    pub fn incomplete(&self) -> Option<&SurfaceQueryCause> {
        self.outcome.as_ref().err()
    }
}
