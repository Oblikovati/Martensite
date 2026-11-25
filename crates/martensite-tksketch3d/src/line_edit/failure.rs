// SPDX-License-Identifier: GPL-2.0-only
use crate::{
    EditFailure, EntityId, FrozenPointSource, LineId, PointId, constraints::ConstraintFailure,
};
use martensite_tkmath::predicates::roots::metric_bounds::DirectedLineIncidenceFailure;
use std::{error::Error, fmt};

/// A source incidence that the line edit cannot remove.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndpointProtection {
    /// The discarded end is still used by an unchanged curve.
    Shared(EntityId),
    /// The discarded end is a frozen external capture.
    Frozen(FrozenPointSource),
    /// The endpoint is owned by a read-only native edge link.
    Included(crate::include_geometry::IncludedPointSource),
}

/// Why a directed line edit could not publish a replacement.
#[derive(Debug)]
#[non_exhaustive]
pub enum LineEditFailure {
    /// The source or boundary identity has no owned straight line.
    MissingLine(LineId),
    /// The source line was listed as its own boundary.
    SourceIsBoundary(LineId),
    /// An Extend boundary identity appeared more than once.
    RepeatedBoundary(LineId),
    /// Extend would have to evaluate more boundary segments than authorized.
    BoundaryLimit {
        /// Number of requested boundaries.
        requested: usize,
        /// Maximum count admitted by the caller.
        maximum: usize,
    },
    /// The position error limit must be finite and nonnegative.
    InvalidLimit(f64),
    /// Trim requires a finite authored pick in `[0,1]`.
    InvalidPick(f64),
    /// The picked Trim parameter is exactly at the cut.
    PickAtCut {
        /// Source that the pick selected.
        source: LineId,
        /// Boundary that defines the contact.
        boundary: LineId,
    },
    /// The source's Trim segment or Extend continuation overlaps the boundary.
    Overlap {
        /// Source of the edit.
        source: LineId,
        /// Boundary that supplies a nonisolated contact.
        boundary: LineId,
    },
    /// Two distinct authored boundaries share the nearest exact forward hit.
    AmbiguousExtension {
        /// Extended source.
        source: LineId,
        /// Two nearest boundaries in ascending ID order.
        boundaries: [LineId; 2],
    },
    /// Retiring the affected endpoint would change a protected incidence.
    ProtectedEndpoint {
        /// Endpoint on the edited side.
        point: PointId,
        /// Existing incidence that protects it.
        protection: EndpointProtection,
    },
    /// The rounded position exceeds the requested error limit.
    PositionError {
        /// Certified Euclidean upper bound in model units.
        upper_bound: f64,
        /// Error limit requested in model units.
        limit: f64,
    },
    /// The exact edit changes the extent, but its rounded contact leaves it unchanged.
    UnrepresentableExtent {
        /// Source line whose extent would be unchanged.
        source: LineId,
        /// Original endpoint on the edited side.
        endpoint: PointId,
        /// Exact contact's outward normalized parameter enclosure.
        parameter: [f64; 2],
    },
    /// An exact source or its contact evidence was refused.
    Intersection(DirectedLineIncidenceFailure),
    /// Normal sketch geometry or identity admission refused the batch.
    Geometry(EditFailure),
    /// The existing relation program cannot retain its exact operands and anchors.
    Constraint(ConstraintFailure),
    /// The restored relation program did not retain its original numerical rank.
    RankChanged {
        /// Checked rank of the source program.
        before: usize,
        /// Checked rank after the proposed line replacement.
        after: usize,
    },
    /// The caller cancelled the draft.
    Cancelled,
}

impl fmt::Display for LineEditFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingLine(id) => {
                write!(output, "line edit requires owned straight line {id:?}")
            }
            Self::SourceIsBoundary(id) => write!(
                output,
                "line edit source {id:?} needs another line as boundary"
            ),
            Self::RepeatedBoundary(id) => {
                write!(output, "line edit boundary {id:?} must occur once")
            }
            Self::BoundaryLimit { requested, maximum } => write!(
                output,
                "line edit needs {requested} boundary checks within a limit of {maximum}"
            ),
            Self::InvalidLimit(value) => write!(
                output,
                "line edit position limit {value} must be finite and nonnegative"
            ),
            Self::InvalidPick(value) => write!(
                output,
                "trim pick {value} must be a finite parameter within [0, 1]"
            ),
            Self::PickAtCut { source, boundary } => write!(
                output,
                "trim source {source:?} needs a pick off its cut at {boundary:?}"
            ),
            Self::Overlap { source, boundary } => write!(
                output,
                "line edit source {source:?} overlaps boundary {boundary:?} and needs an isolated contact"
            ),
            Self::AmbiguousExtension { source, boundaries } => write!(
                output,
                "extension of {source:?} needs a unique nearest boundary, found {boundaries:?}"
            ),
            Self::ProtectedEndpoint { point, protection } => write!(
                output,
                "line edit needs a free endpoint, found {point:?} with {protection:?}"
            ),
            Self::PositionError { upper_bound, limit } => write!(
                output,
                "line edit position error {upper_bound} exceeds limit {limit}"
            ),
            Self::UnrepresentableExtent {
                source,
                endpoint,
                parameter,
            } => write!(
                output,
                "line edit of {source:?} at {parameter:?} must move {endpoint:?} in binary64 coordinates"
            ),
            Self::Intersection(cause) => cause.fmt(output),
            Self::Geometry(cause) => cause.fmt(output),
            Self::Constraint(cause) => cause.fmt(output),
            Self::RankChanged { before, after } => write!(
                output,
                "line edit relation rank {after} must equal the source rank {before}"
            ),
            Self::Cancelled => write!(output, "line edit cancelled"),
        }
    }
}

impl Error for LineEditFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Intersection(cause) => Some(cause),
            Self::Geometry(cause) => Some(cause),
            Self::Constraint(cause) => Some(cause),
            _ => None,
        }
    }
}

impl From<DirectedLineIncidenceFailure> for LineEditFailure {
    fn from(cause: DirectedLineIncidenceFailure) -> Self {
        Self::Intersection(cause)
    }
}

impl From<EditFailure> for LineEditFailure {
    fn from(cause: EditFailure) -> Self {
        match cause {
            EditFailure::Cancelled(_) => Self::Cancelled,
            other => Self::Geometry(other),
        }
    }
}

impl From<ConstraintFailure> for LineEditFailure {
    fn from(cause: ConstraintFailure) -> Self {
        match cause {
            ConstraintFailure::Cancelled => Self::Cancelled,
            other => Self::Constraint(other),
        }
    }
}
