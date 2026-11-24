// SPDX-License-Identifier: GPL-2.0-only
//! Preserve cancellation as an original typed cause across certificate leaves.
use super::*;

impl ConicalBoundaryError {
    /// Identify an arithmetic cancellation; `error.is_cancelled()`.
    pub fn is_cancelled(&self) -> bool {
        match self.cause() {
            ConicalBoundaryFailure::Proof(cause) => cancelled(cause.arithmetic_cause()),
            _ => false,
        }
    }
}
impl ConicalStorageError {
    /// Identify an arithmetic cancellation; `error.is_cancelled()`.
    pub fn is_cancelled(&self) -> bool {
        cancelled(boundary_cause::storage(self.cause()))
    }
}
impl ConicalJoinsFailure {
    /// Identify an arithmetic cancellation; `error.is_cancelled()`.
    pub fn is_cancelled(&self) -> bool {
        match self {
            Self::Proof { cause, .. } => cancelled(cause.arithmetic_cause()),
            _ => false,
        }
    }
}
impl ConeRequestError {
    /// Identify cancellation of the original clearance proof; `error.is_cancelled()`.
    pub fn is_cancelled(&self) -> bool {
        depth_cancelled(self.cause())
    }
}
impl ConicalAuthoredFluxError {
    /// Identify cancellation of volume or depth certification; `error.is_cancelled()`.
    pub fn is_cancelled(&self) -> bool {
        cancelled(self.arithmetic_cause())
            || self
                .depth_cause()
                .is_some_and(|(_, _, cause)| depth_cancelled(cause))
    }
}
impl ConicalSectorFailure {
    /// Identify an arithmetic cancellation; `error.is_cancelled()`.
    pub fn is_cancelled(&self) -> bool {
        match self {
            Self::Arithmetic(cause) => cancelled(Some(cause)),
            Self::Trigonometric(CylindricalProfileFailure::Arithmetic(cause)) => {
                cancelled(Some(cause))
            }
            _ => false,
        }
    }
}
fn depth_cancelled(cause: &ConicalDepthFailure) -> bool {
    match cause {
        ConicalDepthFailure::Arithmetic(cause)
        | ConicalDepthFailure::Source(ConicalSourceFailure::Arithmetic(cause)) => {
            cancelled(Some(cause))
        }
        ConicalDepthFailure::Trigonometric(CylindricalProfileFailure::Arithmetic(cause)) => {
            cancelled(Some(cause))
        }
        _ => false,
    }
}
fn cancelled(cause: Option<&BoundedArithmeticFailure>) -> bool {
    matches!(cause, Some(BoundedArithmeticFailure::Cancelled))
}
