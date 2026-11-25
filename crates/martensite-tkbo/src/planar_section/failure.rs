// SPDX-License-Identifier: GPL-2.0-only
use super::{
    PlanarSectionError, SectionBoundaryId, SectionEndpointOrigin, SectionParents, SectionTrimUse,
};
use crate::bop_algo::ProgressCompletion;
use martensite_tkbrep::brep::face_input_capture::FaceCaptureIncomplete;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use std::{error::Error, fmt};

/// Typed incomplete, unsupported, or degenerate Section observation.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PlanarSectionCause {
    /// The supplied raw hierarchy did not contain the stated shape kind.
    ShapeKind {
        /// Actual kind of a read topology occurrence.
        actual: ShapeEnum,
        /// Required topology kind.
        expected: ShapeEnum,
    },
    /// A face, boundary or vertex had a non-boundary orientation.
    Orientation(Orientation),
    /// The number of outer wires, edges or vertices was outside the allowed range.
    TopologyCount {
        /// Requested type of direct child.
        child: ShapeEnum,
        /// Actual child count.
        actual: usize,
        /// Admitted inclusive interval.
        allowed: [usize; 2],
    },
    /// The located edges do not form a uniquely directed closed cycle.
    OpenOrBranchedBoundary,
    /// A selected boundary ID has no unique matching source edge.
    BoundaryBinding(SectionBoundaryId),
    /// Number of supplied boundary identities differs from the observed trim.
    BoundaryBindingCount {
        /// Caller-provided boundary count.
        supplied: usize,
        /// Actual number of directed wire edges.
        expected: usize,
    },
    /// Actual native representation is outside the straight planar domain.
    Representation {
        /// Actual stored native geometry type, when present.
        actual: Option<String>,
        /// Admitted carrier.
        expected: &'static str,
    },
    /// The actual effective transform did not use unit scale.
    PlacementScale(f64),
    /// An observed raw source modelling tolerance was negative.
    SourceTolerance {
        /// Invocation-local observed owner.
        owner: usize,
        /// Invalid modelling tolerance stored on this owner.
        actual: f64,
    },
    /// The source could not provide its finite coherent input view.
    Capture(Box<FaceCaptureIncomplete>),
    /// Private copy did not preserve or separate the relevant source representation.
    PrivateCopy,
    /// The distinct roles name coincident infinite supports.
    CoincidentSupports,
    /// The exact support intersection contains an original trim edge.
    BoundaryOverlap(SectionBoundaryId),
    /// The intersection is only a trim vertex and provides no regular segment.
    PointOnly(SectionEndpointOrigin),
    /// Native operation completion was unqualified or failed.
    NativeBuild(Box<ProgressCompletion>),
    /// Completion lacked coherent native reader state.
    NativeState {
        /// Native make-shape done flag.
        done: bool,
        /// Native retained report had errors.
        reported_errors: bool,
    },
    /// Native Section yielded an unexpected number of geometric children.
    NativeResultCount {
        /// Count of native direct children.
        actual: usize,
        /// Required count from the exact trim.
        expected: usize,
    },
    /// Native Section did not bind the selected output to the indicated private operand.
    NativeAncestor {
        /// Actual native operand role.
        operand: u8,
    },
    /// A native constructor, reader or ancestry query failed.
    Native(Box<Failure>),
    /// A named face boundary could not supply the admitted source evidence.
    BoundarySource {
        /// Located original boundary use.
        boundary: SectionTrimUse,
        /// Actual structural or numeric refusal.
        cause: Box<PlanarSectionCause>,
    },
    /// Exact source or full-interval certificate could not be obtained.
    Certificate(PlanarSectionError),
    /// An operation-local cancellation was observed.
    Cancelled,
}

/// Section failure with the original locatable parents.
#[derive(Clone, Debug)]
pub struct PlanarSectionFailure {
    /// Source and cutting-plane occurrences, in native operand order.
    pub parents: SectionParents,
    /// Typed cause.
    pub cause: PlanarSectionCause,
}

impl fmt::Display for PlanarSectionFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "planar Section {:?}: {}", self.parents, self.cause)
    }
}

impl Error for PlanarSectionFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.cause)
    }
}

impl fmt::Display for PlanarSectionCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ShapeKind { actual, expected } => {
                write!(f, "shape {actual:?} requires {expected:?}")
            }
            Self::Orientation(actual) => {
                write!(f, "orientation {actual:?} requires Forward or Reversed")
            }
            Self::TopologyCount {
                child,
                actual,
                allowed,
            } => write!(f, "{actual} {child:?} children, expected {allowed:?}"),
            Self::OpenOrBranchedBoundary => {
                write!(f, "face requires one closed directed trim cycle")
            }
            Self::BoundaryBinding(id) => {
                write!(f, "boundary {id:?} requires one distinct located edge")
            }
            Self::BoundaryBindingCount { supplied, expected } => {
                write!(f, "{supplied} boundary bindings, expected {expected}")
            }
            Self::Representation { actual, expected } => {
                write!(f, "stored carrier {actual:?} requires {expected}")
            }
            Self::PlacementScale(actual) => {
                write!(f, "location scale {actual} requires unit scale")
            }
            Self::SourceTolerance { owner, actual } => {
                write!(
                    f,
                    "source owner {owner} tolerance {actual} requires a nonnegative length"
                )
            }
            Self::Capture(cause) => write!(f, "source capture incomplete: {cause:?}"),
            Self::PrivateCopy => write!(
                f,
                "native operands require separate, unchanged stored geometry"
            ),
            Self::CoincidentSupports => write!(f, "work-plane and face supports coincide"),
            Self::BoundaryOverlap(id) => write!(f, "work-plane contains trim edge {id:?}"),
            Self::PointOnly(origin) => write!(f, "intersection contains only {origin:?}"),
            Self::NativeBuild(cause) => {
                write!(f, "native Section requires completed build: {cause:?}")
            }
            Self::NativeState {
                done,
                reported_errors,
            } => write!(
                f,
                "native completed status needs done=true and reported_errors=false, got {done} and {reported_errors}"
            ),
            Self::NativeResultCount { actual, expected } => write!(
                f,
                "native Section returned {actual} direct children, expected {expected}"
            ),
            Self::NativeAncestor { operand } => write!(
                f,
                "native Section ancestry requires selected operand {operand}"
            ),
            Self::Native(cause) => cause.fmt(f),
            Self::BoundarySource { boundary, cause } => {
                write!(f, "source boundary {boundary:?} refused: {cause}")
            }
            Self::Certificate(cause) => cause.fmt(f),
            Self::Cancelled => write!(f, "planar Section cancelled"),
        }
    }
}

impl Error for PlanarSectionCause {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Native(cause) => Some(cause.as_ref()),
            Self::BoundarySource { cause, .. } => Some(cause.as_ref()),
            Self::Certificate(cause) => Some(cause),
            Self::Capture(cause) => match cause.as_ref() {
                FaceCaptureIncomplete::Geometry { cause, .. } => Some(cause.as_ref()),
                _ => None,
            },
            Self::NativeBuild(cause) => match cause.as_ref() {
                ProgressCompletion::Failed { cause, .. }
                | ProgressCompletion::Incomplete { cause, .. } => {
                    cause.as_ref().map(|cause| cause as &(dyn Error + 'static))
                }
                _ => None,
            },
            _ => None,
        }
    }
}

impl From<Failure> for PlanarSectionCause {
    fn from(cause: Failure) -> Self {
        Self::Native(Box::new(cause))
    }
}

impl From<PlanarSectionError> for PlanarSectionCause {
    fn from(cause: PlanarSectionError) -> Self {
        match cause {
            PlanarSectionError::Cancelled => Self::Cancelled,
            cause => Self::Certificate(cause),
        }
    }
}

impl From<FaceCaptureIncomplete> for PlanarSectionCause {
    fn from(cause: FaceCaptureIncomplete) -> Self {
        match cause {
            FaceCaptureIncomplete::Cancelled => Self::Cancelled,
            cause => Self::Capture(Box::new(cause)),
        }
    }
}
