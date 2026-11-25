// SPDX-License-Identifier: GPL-2.0-only
//! The explicit directed-face policy, measured work, and typed refusal.
use super::*;

/// Original limits and the current operation loan for a directed material proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirectedCapPolicy {
    /// Shared pair bisections available to this invocation.
    pub pair_bisections: u16,
    /// Maximum components across the complete selected face, including holes.
    pub max_components: u16,
    /// Original caller-requested aggregate integration limit, validated before the loan.
    pub requested_integration_cells: u64,
    /// Remaining operation integration allowance at entry.
    pub remaining_integration_cells: u64,
}

/// Measured work, kept separately for each numerical provider.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirectedCapWork {
    /// Original request and entry loan, retained even when fewer cells may be spent.
    pub requested: DirectedCapPolicy,
    /// Minimum of requested integration limit and available loan.
    pub effective_integration_cells: u64,
    /// Actual pair subdivision work, including a failing algebraic report.
    pub pair: PlanarProofWorkReceipt,
    /// Actual integration work, including a failing integral report.
    pub integration_cells: u64,
    /// Exact wire bisections, including failed witness queries.
    pub winding_bisections: u64,
    /// Number of logical pair and self domains certified.
    pub pair_domains: usize,
    /// Distinct pairs certified by the common-carrier solver.
    pub carrier_pairs: usize,
    /// Number of selected outer and hole cycles.
    pub loops: usize,
}

impl DirectedCapWork {
    pub(super) fn start(policy: DirectedCapPolicy) -> Self {
        Self {
            requested: policy,
            effective_integration_cells: policy
                .requested_integration_cells
                .min(policy.remaining_integration_cells),
            pair: PlanarProofWorkReceipt {
                requested_pair_bisections: policy.pair_bisections,
                effective_pair_bisections: policy.pair_bisections,
                consumed_pair_bisections: 0,
            },
            integration_cells: 0,
            winding_bisections: 0,
            pair_domains: 0,
            carrier_pairs: 0,
            loops: 0,
        }
    }
    pub(in crate::body_check::certified_conical::material) fn loan(self) -> [u64; 3] {
        [
            self.requested.remaining_integration_cells,
            self.effective_integration_cells,
            self.integration_cells,
        ]
    }
}

/// A bounded material failure and any numerical work observed before it.
#[derive(Debug)]
pub struct DirectedCapFailure {
    /// Typed source, policy, incidence or numerical refusal.
    pub cause: Box<DirectedCapCause>,
    /// Reported once numerical certification started. Admission has no numerical receipt.
    pub observed: Option<DirectedCapWork>,
    /// Whether the call observed cancellation through its supplied callback.
    pub cancelled: bool,
}
impl std::fmt::Display for DirectedCapFailure {
    fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            format,
            "directed selected UV material: {:?}; observed {:?}",
            self.cause, self.observed
        )
    }
}
impl std::error::Error for DirectedCapFailure {}

/// Failure detail at the directed cap proof boundary.
#[derive(Debug)]
pub enum DirectedCapCause {
    /// The caller cancelled before the next work unit started.
    Cancelled,
    /// An original policy field is outside its allowed interval.
    Policy {
        /// Name of the original policy field.
        field: &'static str,
        /// Requested limit.
        actual: u64,
        /// Smallest admissible limit.
        minimum: u64,
        /// Largest admissible limit.
        maximum: u64,
    },
    /// The complete selected face needs more components than allowed.
    Components {
        /// Observed number of cycles.
        actual: usize,
        /// Maximum across outer and hole cycles.
        limit: u16,
    },
    /// Cheap profile or ring limits rejected the source before native admission.
    Shape {
        /// Name of the source constraint.
        field: &'static str,
        /// Observed source count.
        actual: usize,
        /// Count required by the constraint.
        expected: usize,
    },
    /// Standard planar weld, spline or frame admission refused the source.
    Profile(Box<crate::profile_extrusion::DeclineCause>),
    /// Exact source-keyed wire coverage was incomplete.
    Topology(Failure),
    /// A residual pair could not be certified, or had an unexpected contact.
    Pair {
        /// Authored IDs of the two uses. Self checks repeat the ID.
        curves: [ProfileCurveId; 2],
        /// Provider refusal or unsupported instance of contact.
        detail: DirectedCapPairCause,
    },
    /// Whole-cycle sign integration failed.
    Integral {
        /// Zero-based index with the outer cycle first.
        ring: usize,
        /// Whole-cycle numerical refusal.
        cause: Box<IntegralProgramError>,
    },
    /// Actual directed Green sign did not match the selected role.
    Orientation {
        /// Zero-based index with the outer cycle first.
        ring: usize,
        /// Sign required for the cycle role.
        expected: i64,
        /// Observed Green sign.
        actual: i64,
    },
    /// Exact wire query failed with its original provider evidence.
    Winding {
        /// Zero-based index with the outer cycle first.
        ring: usize,
        /// Zero-based index of the containing or excluded cycle.
        against: usize,
        /// Refusal and work reported by the winding provider.
        cause: Box<CapturedBezierWireFailure>,
    },
    /// An actual witness did not prove the expected nesting relation.
    Nesting {
        /// Zero-based index with the outer cycle first.
        ring: usize,
        /// Zero-based index of the containing or excluded cycle.
        against: usize,
        /// Whether the witness should be inside the other cycle.
        expected: bool,
    },
    /// A completed winding query reported boundary or a wrong signed winding.
    Location {
        /// Zero-based index with the outer cycle first.
        ring: usize,
        /// Zero-based index of the containing or excluded cycle.
        against: usize,
        /// Actual classification from the winding provider.
        location: BezierWireLocation,
    },
    /// The lower level returned work exceeding its loan.
    Accounting,
}

/// Retained pair provider failure or failure to match the authored endpoint inventory.
pub enum DirectedCapPairCause {
    /// The owned source could not be captured.
    Source(BezierPairCaptureFailure),
    /// The exact geometric provider refused the actual source and policy.
    Geometric(Box<BezierPairGeometricReport>),
    /// The exact algebraic provider refused with the original source and work.
    Algebraic(Box<BezierPairReport>),
    /// A contact was not a certified endpoint on both occurrences.
    Interior {
        /// Intersection witness in the selected planar chart.
        point: [f64; 2],
    },
    /// Complete provider contacts disagree with the actual wire incidence.
    Contacts {
        /// Ordered endpoint pairs required by the selected cycle.
        expected: Vec<[u8; 2]>,
        /// Certified endpoint pairs returned by the solver.
        actual: Vec<[u8; 2]>,
    },
}

impl std::fmt::Debug for DirectedCapPairCause {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Source(error) => out.debug_tuple("Source").field(error).finish(),
            Self::Geometric(report) => out
                .debug_tuple("Geometric")
                .field(&report.failure())
                .finish(),
            Self::Algebraic(report) => out
                .debug_tuple("Algebraic")
                .field(&report.failure())
                .finish(),
            Self::Interior { point } => out.debug_struct("Interior").field("point", point).finish(),
            Self::Contacts { expected, actual } => out
                .debug_struct("Contacts")
                .field("expected", expected)
                .field("actual", actual)
                .finish(),
        }
    }
}
