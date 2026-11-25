// SPDX-License-Identifier: GPL-2.0-only
use super::*;
/// No partial successful probe batch accompanies this error.
#[derive(Clone, Debug, PartialEq)]
pub struct CylindricalProbeError {
    /// Original aggregate settings.
    pub requested_options: CylindricalProbeOptions,
    /// Number of required probes.
    pub probe_count: usize,
    /// Physical clearance margin from the periodic boundary certificate.
    pub clearance: f64,
    /// Original cause.
    pub cause: CylindricalProbeFailure,
}
/// A refused probe batch and the arithmetic it spent before refusal.
#[derive(Clone, Debug, PartialEq)]
pub struct CylindricalProbeWorkError {
    /// Operations charged across completed and unfinished probes.
    pub operations: u32,
    /// Original typed probe failure.
    pub cause: CylindricalProbeError,
}
impl std::fmt::Display for CylindricalProbeWorkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} after {} arithmetic operations",
            self.cause, self.operations
        )
    }
}
impl std::error::Error for CylindricalProbeWorkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.cause)
    }
}

/// Invalid input or an unresolved strict rounding margin.
#[derive(Clone, Debug, PartialEq)]
pub enum CylindricalProbeFailure {
    /// Policy outside its documented finite bounds.
    Policy,
    /// Number of required probes exceeded the unchanged requested cap.
    Count,
    /// Requested image is not present in the finite image relations.
    Image {
        /// Offending positive image number.
        turn: u32,
        /// Number of available finite images.
        available: usize,
    },
    /// Original or rounded point is nonfinite.
    Point {
        /// Offending input position.
        index: usize,
        /// Offending developed coordinates.
        point: [f64; 2],
    },
    /// Original bounded arithmetic refusal.
    Arithmetic(Box<super::super::super::super::super::BoundedArithmeticFailure>),
    /// Query rounding was not proved strictly smaller than the physical boundary margin.
    Uncertified {
        /// Offending input position.
        index: usize,
        /// Conservative rounding bound.
        rounding: DyadicBound,
    },
}
impl From<super::super::super::super::super::BoundedArithmeticFailure> for CylindricalProbeFailure {
    fn from(cause: super::super::super::super::super::BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(Box::new(cause))
    }
}
impl std::fmt::Display for CylindricalProbeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "periodic probes={}, clearance={}, policy={:?}: {:?}; expected finite image-bound witnesses and rounding strictly below the unchanged physical margin",
            self.probe_count, self.clearance, self.requested_options, self.cause
        )
    }
}
impl std::error::Error for CylindricalProbeError {}
