// SPDX-License-Identifier: GPL-2.0-only
use super::*;

/// Error-bounded signed normal volume of the exact cap reference.
#[derive(Debug)]
pub struct ConicalCapVolumeEvidence {
    pub(super) enclosure: [DyadicBound; 2],
    pub(super) operations: u32,
}
impl ConicalCapVolumeEvidence {
    /// Outward volume endpoints. Example: `let interval = proof.enclosure();`.
    pub fn enclosure(&self) -> &[DyadicBound; 2] {
        &self.enclosure
    }

    /// Bounded arithmetic work, charged once. Example: `let work = proof.operations();`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
}

/// A cap-volume obligation refused before completing the integral.
#[derive(Debug)]
pub struct ConicalCapVolumeError {
    /// Typed failing obligation.
    pub cause: ConicalCapVolumeFailure,
    /// Affected contour span, when arithmetic refused in the polynomial integral.
    pub span: Option<usize>,
    /// Actual arithmetic consumed.
    pub operations: u32,
}

/// Domain or arithmetic refusal for a cap-volume enclosure.
#[derive(Debug)]
pub enum ConicalCapVolumeFailure {
    /// An invalid precision or arithmetic grant.
    Policy(ConicalSectorFailure),
    /// Bounded arithmetic did not complete.
    Arithmetic(BoundedArithmeticFailure),
    /// The cone semi-angle enclosure failed.
    Trigonometric(CylindricalProfileFailure),
    /// The retained cap has no contours.
    Empty,
    /// The meridian radius was not proved positive over the profile and depths.
    Radius {
        /// Outward lower endpoint of the radius bound.
        lower: DyadicBound,
    },
}
impl From<BoundedArithmeticFailure> for ConicalCapVolumeFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(cause)
    }
}
impl std::fmt::Display for ConicalCapVolumeError {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            out,
            "cap normal volume at span {:?}; work={}: {}",
            self.span, self.operations, self.cause
        )
    }
}
impl std::error::Error for ConicalCapVolumeError {}
impl std::fmt::Display for ConicalCapVolumeFailure {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Policy(cause) => cause.fmt(out),
            Self::Arithmetic(cause) => write!(out, "{cause:?}"),
            Self::Trigonometric(cause) => cause.fmt(out),
            Self::Empty => write!(out, "cap spans=0; expected at least one closed contour"),
            Self::Radius { lower } => write!(
                out,
                "cap normal radius lower bound={lower}; expected strictly positive"
            ),
        }
    }
}

pub(super) fn error(
    cause: ConicalCapVolumeFailure,
    span: Option<usize>,
    arithmetic: &Arithmetic<'_>,
) -> ConicalCapVolumeError {
    ConicalCapVolumeError {
        cause,
        span,
        operations: arithmetic.operations(),
    }
}
