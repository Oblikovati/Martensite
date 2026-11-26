// SPDX-License-Identifier: GPL-2.0-only
use martensite_tkernel::standard::{Failure, SharedReadFailure};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::{gp::UnitPlacementError, predicates::NonFiniteCoordinate};
use std::{error::Error, fmt};

/// Which live cell declined coherent nonblocking access.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineEdgeReadCell {
    /// Native edge child list.
    EdgeTopology,
    /// Native edge representation list.
    EdgeRepresentations,
    /// Stored spatial location, range and evaluator reference.
    SpatialRepresentation,
    /// Geometric line carrier.
    Carrier,
    /// Actual endpoint topology or position.
    Endpoint(usize),
}

/// Incomplete or unsupported capture of a native line edge.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum LineEdgeCaptureError {
    /// The actual topology was outside a two-endpoint native edge.
    Topology,
    /// The edge or vertex use was not directed in the native occurrence.
    Orientation(Orientation),
    /// Each original endpoint must match one distinct nominated vertex.
    VertexBinding,
    /// The actual list exceeded bounded capture work.
    RepresentationLimit {
        /// Observed count before allocation.
        actual: usize,
        /// Maximum consumed records.
        maximum: usize,
    },
    /// Native representations contain zero or multiple 3D curves, or unknown storage.
    Representation,
    /// The stored spatial evaluator is outside the admitted direct analytic line domain.
    Carrier,
    /// Actual trim parameters must be finite, bounded and increasing.
    Range([f64; 2]),
    /// A native occurrence exceeded the admissible datum chain.
    LocationDepth(usize),
    /// Native placement needs one consistent unit affine map.
    Placement(UnitPlacementError),
    /// Actual stored modelling tolerance was invalid.
    Tolerance(f64),
    /// An original finite coordinate was required.
    Coordinate(NonFiniteCoordinate),
    /// Required exact world-space points cannot be represented in the sketch.
    Unrepresentable,
    /// The two original trim endpoints coincide after placement.
    Collapsed,
    /// The original native vertex and original active line disagree.
    Endpoint {
        /// Directed endpoint index.
        index: usize,
        /// Exact carrier endpoint.
        expected: [f64; 3],
        /// Exact placed vertex position.
        observed: [f64; 3],
    },
    /// A live lease could not be acquired without waiting.
    Read(LineEdgeReadCell, SharedReadFailure),
    /// The underlying native operation failed.
    Native(Box<Failure>),
    /// Operation-local cancellation.
    Cancelled,
}

impl fmt::Display for LineEdgeCaptureError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Placement(cause) => cause.fmt(output),
            Self::Coordinate(cause) => cause.fmt(output),
            Self::Native(cause) => cause.fmt(output),
            other => write!(
                output,
                "expected a bounded native line with two exact named endpoints, got {other:?}"
            ),
        }
    }
}

impl Error for LineEdgeCaptureError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Placement(cause) => Some(cause),
            Self::Coordinate(cause) => Some(cause),
            Self::Native(cause) => Some(cause.as_ref()),
            _ => None,
        }
    }
}
impl From<UnitPlacementError> for LineEdgeCaptureError {
    fn from(cause: UnitPlacementError) -> Self {
        Self::Placement(cause)
    }
}
impl From<NonFiniteCoordinate> for LineEdgeCaptureError {
    fn from(cause: NonFiniteCoordinate) -> Self {
        Self::Coordinate(cause)
    }
}
impl From<Failure> for LineEdgeCaptureError {
    fn from(cause: Failure) -> Self {
        Self::Native(Box::new(cause))
    }
}
