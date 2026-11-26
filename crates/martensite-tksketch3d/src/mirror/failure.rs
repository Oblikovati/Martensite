// SPDX-License-Identifier: GPL-2.0-only
use crate::{EditFailure, EntityId, LineId, PointId};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::predicates::ReflectionComparisonError;
use std::{error::Error, fmt};

/// A declined or cancelled mirror. The source snapshot remains available.
#[derive(Debug)]
#[non_exhaustive]
pub enum MirrorFailure {
    /// The request contains no points or lines.
    EmptySelection,
    /// The caller's error limit must be finite and nonnegative.
    InvalidErrorLimit(f64),
    /// The supplied plane could not be admitted as a stored right-handed frame.
    InvalidPlane(Failure),
    /// The requested source is missing or its key denotes a different kind.
    MissingSource(EntityId),
    /// A source appears more than once in the request.
    RepeatedSource(EntityId),
    /// The shared numeric key is already owned or assigned to another copy.
    DuplicateTarget {
        /// Entity that owns or has already reserved the key.
        existing: EntityId,
        /// Entity proposed at that key.
        attempted: EntityId,
    },
    /// A selected line endpoint has no explicit point-copy mapping.
    UnmappedEndpoint {
        /// The source line requiring the endpoint.
        line: LineId,
        /// The source point absent from the mapping.
        point: PointId,
    },
    /// A reflected position cannot be rounded to finite coordinates or checked.
    MetricInput {
        /// The source point being reflected.
        point: PointId,
        /// The exact predicate's refusal.
        source: ReflectionComparisonError,
    },
    /// The reflected position exceeded the caller's model-space error limit.
    PositionError {
        /// The source point being reflected.
        point: PointId,
        /// The caller's inclusive error limit.
        limit: f64,
    },
    /// The draft failed the normal spatial geometry admission.
    Geometry(EditFailure),
    /// The request observed cancellation before returning a new snapshot.
    Cancelled,
}

impl fmt::Display for MirrorFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySelection => write!(output, "mirror requires at least one point or line"),
            Self::InvalidErrorLimit(limit) => write!(
                output,
                "mirror error limit {limit} must be finite and nonnegative"
            ),
            Self::InvalidPlane(error) => write!(output, "mirror requires a valid plane ({error})"),
            Self::MissingSource(source) => write!(
                output,
                "mirror source {source:?} must exist with its requested kind"
            ),
            Self::RepeatedSource(source) => {
                write!(output, "mirror source {source:?} must occur once")
            }
            Self::DuplicateTarget {
                existing,
                attempted,
            } => {
                write!(
                    output,
                    "mirror target {attempted:?} must be fresh, key already assigned to {existing:?}"
                )
            }
            Self::UnmappedEndpoint { line, point } => {
                write!(
                    output,
                    "mirror of line {line:?} requires a copy mapping for endpoint {point:?}"
                )
            }
            Self::MetricInput { point, source } => {
                write!(
                    output,
                    "mirror of point {point:?} requires a finite, comparable position ({source})"
                )
            }
            Self::PositionError { point, limit } => write!(
                output,
                "mirror of point {point:?} exceeds position limit {limit}"
            ),
            Self::Geometry(error) => error.fmt(output),
            Self::Cancelled => write!(output, "mirror cancelled"),
        }
    }
}

impl Error for MirrorFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidPlane(source) => Some(source),
            Self::MetricInput { source, .. } => Some(source),
            Self::Geometry(source) => Some(source),
            _ => None,
        }
    }
}

impl From<EditFailure> for MirrorFailure {
    fn from(error: EditFailure) -> Self {
        match error {
            EditFailure::Cancelled(_) => Self::Cancelled,
            other => Self::Geometry(other),
        }
    }
}
