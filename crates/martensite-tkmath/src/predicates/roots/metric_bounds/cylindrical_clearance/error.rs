// SPDX-License-Identifier: GPL-2.0-only
use super::super::BoundedArithmeticFailure;
use super::*;
use crate::predicates::roots::DyadicBound;
pub(super) type Checked<T> = Result<T, CylindricalClearanceFailure>;

/// Requested relation did not obtain a complete clearance certificate.
#[derive(Clone, Debug, PartialEq)]
pub struct CylindricalClearanceError {
    /// Original spatial margin.
    pub requested_clearance: f64,
    /// Original arithmetic and storage policy.
    pub requested_options: CylindricalClearanceOptions,
    /// Input count before capture.
    pub source_count: usize,
    /// Original typed cause or incomplete evidence.
    pub cause: CylindricalClearanceFailure,
}

/// Invalid input, an exact witness below the margin, or exhausted certification.
#[derive(Clone, Debug, PartialEq)]
pub enum CylindricalClearanceFailure {
    /// Resource settings outside the admitted finite range.
    Policy(CylindricalClearanceOptions),
    /// Unsupported source or exclusion count.
    InputCount {
        /// Input collection role.
        field: &'static str,
        /// Supplied number of entries.
        actual: usize,
        /// Largest admitted number of entries.
        maximum: usize,
    },
    /// Invalid finite/positive scalar requirement.
    Value {
        /// Input scalar role.
        field: &'static str,
        /// Coordinate within that role.
        index: usize,
        /// Offending input.
        value: f64,
    },
    /// The source slab could not establish separation from all true periodic images.
    /// This is incomplete evidence, not a witness of overlapping material.
    PeriodicSlabUnresolved {
        /// Conservative first-period slab gap, which can be negative.
        separation_lower: DyadicBound,
    },
    /// An original polynomial must have two through four control points.
    SourceShape {
        /// Local source position.
        index: usize,
        /// Actual pole count.
        poles: usize,
    },
    /// An original polynomial contains a nonfinite coordinate.
    SourceValue {
        /// Local source position.
        source: usize,
        /// Original control point index.
        pole: usize,
        /// Coordinate index.
        axis: usize,
        /// Offending supplied coordinate.
        value: f64,
    },
    /// Exclusions require increasing unique pairs inside the source slice.
    ExcludedPair {
        /// Supplied pair.
        pair: [usize; 2],
        /// Admitted input source count.
        source_count: usize,
    },
    /// Original interval-arithmetic cause.
    Arithmetic(Box<BoundedArithmeticFailure>),
    /// No further hierarchy or curve-pair domain could be visited.
    DomainLimit {
        /// Completed domain visits.
        completed: u32,
        /// Original requested maximum.
        limit: u32,
    },
    /// The original curve-node allowance would be exceeded.
    NodeLimit {
        /// Existing cached curve nodes.
        used: u32,
        /// Additional nodes needed.
        additional: u32,
        /// Original requested maximum.
        limit: u32,
    },
    /// A pair remained unresolved at the original local depth cap.
    RefinementLimit {
        /// Original local source positions.
        sources: [usize; 2],
        /// Local dyadic cell indices.
        indices: [u64; 2],
        /// Local dyadic cell levels.
        levels: [u8; 2],
        /// Original refinement cap.
        maximum: u8,
    },
    /// A pair of actual polynomial points lies strictly below the requested clearance.
    WitnessWithinMargin {
        /// Original local source positions.
        sources: [usize; 2],
        /// Exact parameters in the original authored polynomials.
        parameters: Box<[DyadicBound; 2]>,
        /// Certified upper bound on this witness's squared Euclidean distance.
        squared_distance_upper: DyadicBound,
    },
}
impl From<BoundedArithmeticFailure> for CylindricalClearanceFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(Box::new(cause))
    }
}
impl std::fmt::Display for CylindricalClearanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "cylindrical clearance sources={}, margin={}: {:?}; expected finite coordinates/origin/translation, positive radius/clearance, 2..=4 poles per source, unique increasing exclusions, admitted resources and a complete separation proof",
            self.source_count, self.requested_clearance, self.cause
        )
    }
}
impl std::error::Error for CylindricalClearanceError {}
