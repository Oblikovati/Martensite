// SPDX-License-Identifier: GPL-2.0-only
use super::super::BoundedArithmeticFailure;
use super::*;
pub(super) type Checked<T> = Result<T, CylindricalMeasureFailure>;

/// Original policy and typed incomplete result; no partial measure accompanies it.
#[derive(Clone, Debug, PartialEq)]
pub struct CylindricalMeasureError {
    /// Original arithmetic policy.
    pub requested_options: CylindricalMeasureOptions,
    /// Explicit requested volume-error treatment.
    pub requested_limit: CylindricalMeasureLimit,
    /// Original depth offsets.
    pub requested_depths: [f64; 2],
    /// Number of required pieces.
    pub source_count: usize,
    /// Underlying refusal.
    pub cause: CylindricalMeasureFailure,
}
/// Invalid source or policy, or incomplete arithmetic evidence.
#[derive(Clone, Debug, PartialEq)]
pub enum CylindricalMeasureFailure {
    /// Unsupported original policy.
    Policy,
    /// Expected between one and the declared finite source limit.
    SourceCount,
    /// Source or chart did not match the retained evidence or first source's chart.
    Source {
        /// Offending source position.
        index: usize,
    },
    /// Offsets must be finite, ordered, and leave both radii positive.
    Depths,
    /// Positivity of a radius was unresolved at this precision.
    Radius {
        /// Offset index whose ideal radius was not proven positive.
        depth: usize,
        /// Outward lower bound on the exact radius plus offset.
        lower: crate::predicates::roots::DyadicBound,
    },
    /// Original bounded arithmetic failure.
    Arithmetic(Box<BoundedArithmeticFailure>),
    /// The unchanged geometric volume-error cap was not established.
    Uncertified {
        /// Conservative volume discrepancy.
        bound: f64,
        /// Original finite volume allowance.
        requested: f64,
    },
}
impl From<BoundedArithmeticFailure> for CylindricalMeasureFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(Box::new(cause))
    }
}
impl std::fmt::Display for CylindricalMeasureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "cylindrical normal measure sources={}, depths={:?}, policy={:?}, limit={:?}: {:?}; expected source-bound common-chart profiles, positive radii, ordered depths and completed arithmetic within the requested limits",
            self.source_count,
            self.requested_depths,
            self.requested_options,
            self.requested_limit,
            self.cause
        )
    }
}
impl std::error::Error for CylindricalMeasureError {}
