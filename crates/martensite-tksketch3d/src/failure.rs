// SPDX-License-Identifier: GPL-2.0-only
//! Refusals identify the exact edit or line input that failed admission.
use crate::{
    ArcFailure, ArcId, EntityId, FrozenPointSource, LineId, MinimumLength, PointId, SketchLine,
};
use martensite_tkmath::predicates::NonFiniteCoordinate;
use std::{error::Error, fmt};

/// The caller supplied an invalid minimum spatial length.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InvalidMinimumLength {
    /// The nonpositive or nonfinite requested value.
    pub value: f64,
}

impl fmt::Display for InvalidMinimumLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "minimum spatial length {} must be finite and positive",
            self.value
        )
    }
}

impl Error for InvalidMinimumLength {}

/// Stage at which the edit observed cancellation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BatchPhase {
    /// Before creating a private draft.
    BeforeStaging,
    /// During staging, including deletion and orphan scans.
    Staging,
    /// Before checking a curve of the complete draft.
    Validation,
    /// After validating the complete draft.
    Completion,
}

/// A failed or cancelled edit. The source snapshot remains available.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum EditFailure {
    /// An insertion used a key already owned or retired in this batch.
    DuplicateIdentity {
        /// Existing or retired identity.
        existing: EntityId,
        /// Attempted identity.
        attempted: EntityId,
    },
    /// An edited or removed entity has no matching typed identity.
    MissingEditTarget(EntityId),
    /// A move would change a point captured from an external sketch.
    FrozenPointMove {
        /// Destination point identity.
        point: PointId,
        /// Captured source.
        source: FrozenPointSource,
    },
    /// An editable point has no source to detach.
    PointIsNotFrozen(PointId),
    /// A line has an endpoint without an owned point.
    MissingEndpoint {
        /// Line that requires the point.
        line: LineId,
        /// Missing point.
        point: PointId,
    },
    /// A supplied point coordinate is nonfinite.
    NonFinitePoint {
        /// Offending point.
        point: PointId,
        /// Zero-based coordinate index.
        axis: usize,
        /// Offending coordinate.
        value: f64,
    },
    /// One identity was used for both endpoints.
    SameEndpoint {
        /// Offending line.
        line: LineId,
        /// Repeated point identity.
        point: PointId,
    },
    /// Distinct point identities have exactly the same coordinates.
    CoincidentEndpoints(SketchLine),
    /// Exact supplied endpoint distance is below the caller's minimum.
    BelowMinimumLength {
        /// Offending line.
        line: LineId,
        /// Rounded measurement for diagnosis. This can equal the minimum.
        estimate: f64,
        /// Caller-supplied minimum used by the exact comparison.
        minimum: MinimumLength,
    },
    /// A coordinate was refused by the exact comparison.
    MetricInput {
        /// Offending line.
        line: LineId,
        /// Operand location and value.
        source: NonFiniteCoordinate,
    },
    /// Subtracting finite components overflowed binary64.
    UnrepresentableDisplacement {
        /// Offending line.
        line: LineId,
        /// Zero-based coordinate index.
        axis: usize,
        /// Components of the first and second endpoints.
        endpoints: [f64; 2],
    },
    /// The displacement norm overflowed binary64.
    UnrepresentableLength {
        /// Offending line.
        line: LineId,
        /// Finite displacement components.
        displacement: [f64; 3],
    },
    /// A nonzero displacement component vanishes in the unit direction.
    UnrepresentableDirection {
        /// Offending line.
        line: LineId,
        /// Zero-based coordinate index of the lost component.
        axis: usize,
        /// Finite displacement components.
        displacement: [f64; 3],
    },
    /// TKMath refused to construct the unit direction.
    DirectionConstruction {
        /// Offending line.
        line: LineId,
        /// Finite displacement components.
        displacement: [f64; 3],
    },
    /// A circular arc could not be admitted.
    Arc {
        /// Arc being validated.
        arc: ArcId,
        /// Specific placement or carrier refusal.
        source: ArcFailure,
    },
    /// The operation observed cancellation at this stage.
    Cancelled(BatchPhase),
}

impl fmt::Display for EditFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity {
                existing,
                attempted,
            } => write!(
                f,
                "identity {attempted:?} must be unused in the batch, already held by {existing:?}"
            ),
            Self::MissingEditTarget(entity) => write!(f, "edit target {entity:?} must exist"),
            Self::FrozenPointMove { point, source } => {
                write!(
                    f,
                    "point {point:?} captured from {source:?} must be detached before moving"
                )
            }
            Self::PointIsNotFrozen(point) => write!(f, "point {point:?} has no frozen source"),
            Self::MissingEndpoint { line, point } => {
                write!(
                    f,
                    "line {line:?} endpoint {point:?} must refer to an owned point"
                )
            }
            Self::NonFinitePoint { point, axis, value } => {
                write!(
                    f,
                    "point {point:?} coordinate {axis} value {value} must be finite"
                )
            }
            Self::SameEndpoint { line, point } => {
                write!(
                    f,
                    "line {line:?} repeats point {point:?}, expected two identities"
                )
            }
            Self::CoincidentEndpoints(line) => write!(
                f,
                "line {:?} has coincident points {:?}, expected distinct coordinates",
                line.id, line.endpoints
            ),
            Self::BelowMinimumLength {
                line,
                estimate,
                minimum,
            } => write!(
                f,
                "line {line:?} has exact length below {} (rounded estimate {estimate})",
                minimum.value()
            ),
            Self::MetricInput { line, source } => {
                write!(f, "line {line:?} failed coordinate comparison ({source})")
            }
            Self::UnrepresentableDisplacement {
                line,
                axis,
                endpoints,
            } => write!(
                f,
                "line {line:?} axis {axis} endpoints {endpoints:?} must have a finite difference"
            ),
            Self::UnrepresentableLength { line, displacement } => write!(
                f,
                "line {line:?} displacement {displacement:?} must have a finite length estimate"
            ),
            Self::UnrepresentableDirection {
                line,
                axis,
                displacement,
            } => write!(
                f,
                "line {line:?} displacement {displacement:?} must preserve nonzero component {axis} in the unit direction"
            ),
            Self::DirectionConstruction { line, displacement } => write!(
                f,
                "line {line:?} displacement {displacement:?} must construct a unit direction"
            ),
            Self::Arc { arc, source } => write!(f, "arc {arc:?}: {source}"),
            Self::Cancelled(phase) => write!(f, "spatial sketch edit cancelled at {phase:?}"),
        }
    }
}

impl Error for EditFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::MetricInput { source, .. } => Some(source),
            Self::Arc { source, .. } => Some(source),
            _ => None,
        }
    }
}
