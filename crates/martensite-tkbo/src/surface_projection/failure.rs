// SPDX-License-Identifier: GPL-2.0-only
use super::SurfaceProjectionParents;
use crate::bop_algo::ProgressCompletion;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::{
    gp::UnitPlacementError,
    predicates::roots::metric_bounds::cylindrical_projection::CylinderProjectionError,
};
use std::{error::Error, fmt};

/// Why the native operands are outside the admitted original domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SurfaceProjectionScope {
    /// Actual topology did not have the expected kind.
    Shape {
        /// Actual shape kind.
        actual: ShapeEnum,
        /// Expected shape kind.
        expected: ShapeEnum,
    },
    /// An occurrence lacked a directed boundary orientation.
    Orientation(Orientation),
    /// Actual child count or directed incidence was outside the rectangular domain.
    Topology,
    /// The original boundary identities did not uniquely resolve.
    BoundaryIdentity,
    /// Missing, ambiguous, or unsupported stored curve or support.
    Representation,
    /// A native location chain exhausted the finite bound.
    LocationDepth,
    /// Private native operands changed a relevant original value or shared a mutable owner.
    PrivateCopy,
    /// Native Section did not verify the indicated registered parent.
    Ancestry(u8),
}

/// Typed refusal or incomplete native projection.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum SurfaceProjectionCause {
    /// Unsupported actual storage or ancestry.
    Scope(SurfaceProjectionScope),
    /// Native placement did not have one consistent unit affine map.
    Placement(UnitPlacementError),
    /// Certified mathematical decision or missing numerical evidence.
    Numeric(CylinderProjectionError),
    /// Failed, incomplete, or inconsistent native build.
    Section(Box<ProgressCompletion>),
    /// Native completion did not have the required result state.
    SectionState {
        /// Make-shape completion flag.
        done: bool,
        /// Whether the actual report contains errors.
        reported_errors: bool,
    },
    /// Native read or construction returned a typed failure.
    Native(Box<Failure>),
    /// Operation-local cancellation.
    Cancelled,
}

/// A failure bound to both original projection parents.
#[derive(Clone, Debug)]
pub struct SurfaceProjectionFailure {
    /// Original operand occurrences.
    pub parents: SurfaceProjectionParents,
    /// Actual reason the operation did not complete.
    pub cause: SurfaceProjectionCause,
}

impl fmt::Display for SurfaceProjectionFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "cylindrical projection {:?} refused ({})",
            self.parents, self.cause
        )
    }
}

impl Error for SurfaceProjectionFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.cause)
    }
}

impl fmt::Display for SurfaceProjectionCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scope(cause) => write!(f, "projection domain refused {cause:?}"),
            Self::Placement(cause) => cause.fmt(f),
            Self::Numeric(cause) => cause.fmt(f),
            Self::Section(status) => {
                write!(f, "native Section did not qualify as complete ({status:?})")
            }
            Self::SectionState {
                done,
                reported_errors,
            } => write!(
                f,
                "native Section needs done=true and reported_errors=false, got {done} and {reported_errors}"
            ),
            Self::Native(cause) => cause.fmt(f),
            Self::Cancelled => f.write_str("cylindrical projection cancelled"),
        }
    }
}

impl Error for SurfaceProjectionCause {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Placement(cause) => Some(cause),
            Self::Numeric(cause) => Some(cause),
            Self::Native(cause) => Some(cause.as_ref()),
            Self::Section(status) => match status.as_ref() {
                ProgressCompletion::Failed { cause, .. }
                | ProgressCompletion::Incomplete { cause, .. } => {
                    cause.as_ref().map(|value| value as &(dyn Error + 'static))
                }
                _ => None,
            },
            _ => None,
        }
    }
}

impl From<SurfaceProjectionScope> for SurfaceProjectionCause {
    fn from(cause: SurfaceProjectionScope) -> Self {
        Self::Scope(cause)
    }
}

impl From<UnitPlacementError> for SurfaceProjectionCause {
    fn from(cause: UnitPlacementError) -> Self {
        Self::Placement(cause)
    }
}

impl From<CylinderProjectionError> for SurfaceProjectionCause {
    fn from(cause: CylinderProjectionError) -> Self {
        match cause {
            CylinderProjectionError::Cancelled => Self::Cancelled,
            cause => Self::Numeric(cause),
        }
    }
}

impl From<Failure> for SurfaceProjectionCause {
    fn from(cause: Failure) -> Self {
        Self::Native(Box::new(cause))
    }
}
