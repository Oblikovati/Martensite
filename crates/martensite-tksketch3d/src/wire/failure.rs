// SPDX-License-Identifier: GPL-2.0-only
use super::WireClosure;
use crate::{EntityId, LineId, PointId};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::predicates::{NonFiniteCoordinate, SegmentContact3d};
use std::{error::Error, fmt};

/// A boundary where a wire operation observes cancellation or native failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WirePhase {
    /// Read the directed occurrences and their point identities from the snapshot.
    Resolve,
    /// Prove incidence of the closed authored segments.
    Incidence,
    /// Build a fresh native topology.
    Materialize,
    /// Read and prepare the verified detached export and its bindings.
    VerifyNative,
}

/// A wire request was refused, incomplete or cancelled. No partial wire was published.
#[derive(Debug)]
#[non_exhaustive]
pub enum WireFailure {
    /// At least one directed line use is required.
    Empty,
    /// A line occurrence is absent from the source snapshot.
    MissingLine(LineId),
    /// A construction line was selected as a model wire occurrence.
    ConstructionLine(LineId),
    /// Using one authored line more than once makes its occurrence identity ambiguous.
    RepeatedLine(LineId),
    /// Consecutive line uses have different terminal point identities.
    DisjointUses {
        /// Consecutive lines in request order.
        lines: [LineId; 2],
        /// Last point of the first line and first point of the second line.
        points: [PointId; 2],
    },
    /// The first and last point identities disagree with the requested closure.
    WrongClosure {
        /// Requested topology.
        expected: WireClosure,
        /// First and last point in request order.
        points: [PointId; 2],
    },
    /// An exact segment relationship disagrees with the ordered wire's topology.
    SegmentIncidence {
        /// Related lines in request order.
        lines: [LineId; 2],
        /// Whether these uses share one required adjacent endpoint.
        adjacent: bool,
        /// Observed exact contact.
        contact: SegmentContact3d,
    },
    /// The next pair could not be evaluated under the caller's work budget.
    PairLimit {
        /// Next pair in request order.
        lines: [LineId; 2],
        /// Maximum pair checks permitted and already consumed.
        limit: usize,
    },
    /// A coordinate could not be evaluated by the exact incidence predicate.
    IncidenceInput {
        /// Related lines in request order.
        lines: [LineId; 2],
        /// Rejected coordinate and value.
        source: NonFiniteCoordinate,
    },
    /// The native kernel refused an operation on a detached draft.
    Native {
        /// Active stage.
        phase: WirePhase,
        /// Authored feature under consideration, when applicable.
        entity: Option<EntityId>,
        /// Original native failure.
        source: Box<Failure>,
    },
    /// The detached topology, geometry or stored tolerance differed from the recipe.
    NativeMismatch {
        /// Authored feature under consideration, when applicable.
        entity: Option<EntityId>,
        /// Required invariant that was not observed.
        expected: &'static str,
    },
    /// The verified line carrier error did not fit the immutable native tolerance.
    NativeErrorBound {
        /// Affected line.
        line: LineId,
        /// Upper bound across the whole carrier trim in model coordinates.
        bound: f64,
        /// Native tolerance limit applied to the edge and vertices.
        limit: f64,
    },
    /// A caller cancellation was observed.
    Cancelled(WirePhase),
}

impl fmt::Display for WireFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "wire requires at least one directed line use, got zero"),
            Self::MissingLine(line) => write!(f, "wire line {line:?} must exist in the snapshot"),
            Self::ConstructionLine(line) => write!(
                f,
                "wire line {line:?} has the construction role, expected model geometry"
            ),
            Self::RepeatedLine(line) => write!(f, "wire line {line:?} must occur once"),
            Self::DisjointUses { lines, points } => {
                write!(
                    f,
                    "wire lines {lines:?} must share an endpoint identity, got {points:?}"
                )
            }
            Self::WrongClosure { expected, points } => {
                write!(
                    f,
                    "wire endpoints {points:?} must have {expected:?} connectivity"
                )
            }
            Self::SegmentIncidence {
                lines,
                adjacent,
                contact,
            } => {
                let expected = if *adjacent {
                    SegmentContact3d::Point
                } else {
                    SegmentContact3d::Disjoint
                };
                write!(
                    f,
                    "wire lines {lines:?} have {contact:?} contact, expected {expected:?}"
                )
            }
            Self::PairLimit { lines, limit } => {
                write!(
                    f,
                    "wire line pair {lines:?} needs another check beyond the limit of {limit}"
                )
            }
            Self::IncidenceInput { lines, source } => {
                write!(
                    f,
                    "wire line pair {lines:?} failed exact incidence input validation ({source})"
                )
            }
            Self::Native {
                phase,
                entity,
                source,
            } => {
                write!(f, "wire {phase:?} failed for {entity:?} ({source})")
            }
            Self::NativeMismatch { entity, expected } => {
                write!(f, "detached wire for {entity:?} must preserve {expected}")
            }
            Self::NativeErrorBound { line, bound, limit } => {
                write!(
                    f,
                    "native wire line {line:?} error bound {bound} must fit tolerance {limit}"
                )
            }
            Self::Cancelled(phase) => write!(f, "wire operation cancelled at {phase:?}"),
        }
    }
}

impl Error for WireFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Native { source, .. } => Some(source.as_ref()),
            Self::IncidenceInput { source, .. } => Some(source),
            _ => None,
        }
    }
}
