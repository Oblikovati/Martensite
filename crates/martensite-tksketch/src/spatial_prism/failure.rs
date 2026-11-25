// SPDX-License-Identifier: GPL-2.0-only
use martensite_tkbo::profile_extrusion::{HistoryRole, OperationDecline, PlanarRegionWorkFailure};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::predicates::NonFiniteCoordinate;
use martensite_tksketch3d::{LineId, PointId};
use std::{cmp::Ordering, error::Error, fmt};

/// A cancellable or native boundary of a spatial prism operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialPrismStage {
    /// Prove the plane and lower the authored loop.
    Plane,
    /// Build the native planar region under the planar proof work policy.
    Region,
    /// Bind actual planar region topology and geometry to the source.
    Binding,
    /// Run the ordinary prism and its required ordered validation.
    Sweep,
    /// Compose semantic history before publishing.
    History,
    /// Copy native solid and semantic handles to a detached export.
    Export,
}

/// Refusal or incomplete work from an ordinary Sketch3D extrusion.
#[derive(Debug)]
#[non_exhaustive]
pub enum SpatialPrismFailure {
    /// A simple closed wire is required.
    OpenWire,
    /// An explicit finite positive source-to-region trace limit is required.
    InvalidTraceLimit(f64),
    /// No noncollinear authored anchors could be established.
    DegeneratePlane,
    /// The exact determinant of an authored point against the anchor plane is nonzero.
    Skew {
        /// Ordered noncollinear plane anchors.
        anchors: [PointId; 3],
        /// Off-plane authored vertex.
        point: PointId,
        /// Sign relative to the ordered anchors.
        sign: Ordering,
    },
    /// The requested vector lies in the exact authored plane.
    TangentDirection([f64; 3]),
    /// An exact predicate refused an input.
    Predicate {
        /// Authored point used in this check, where applicable.
        point: Option<PointId>,
        /// Exact predicate input failure.
        source: NonFiniteCoordinate,
    },
    /// The source-supported chart did not yield a finite, positively oriented plane.
    UnboundChart([PointId; 3]),
    /// The actual region failed the requested source correspondence.
    RegionBinding {
        /// Affected source curve, where applicable.
        line: Option<LineId>,
        /// Required correspondence.
        expected: &'static str,
    },
    /// The certified bound on a native edge trace did not fit the source limit.
    SourceTrace {
        /// Affected source curve.
        line: LineId,
        /// Computed certified upper bound in model units.
        bound: f64,
        /// Unchanged caller limit.
        limit: f64,
    },
    /// The retained native affine sweep could not be verified.
    UnboundSweep {
        /// Affected native face index, when available.
        face: Option<usize>,
        /// Required native evidence or correspondence.
        expected: &'static str,
    },
    /// A retained cap vertex exceeds the exact authored translation trace limit.
    SweepVertexTrace {
        /// Authored point producing the vertex.
        point: PointId,
        /// Start or end image of that point.
        role: HistoryRole,
        /// Computed world trace enclosure.
        bound: f64,
        /// Unchanged source trace acceptance limit.
        limit: f64,
    },
    /// A retained face trim exceeds its affine curve or vertex trace limit.
    SweepFaceTrace {
        /// Native face index in the ordinary extrusion.
        face: usize,
        /// Computed world trace enclosure.
        bound: f64,
        /// Unchanged acceptance limit.
        limit: f64,
    },
    /// Existing planar-region failure with its work and provenance intact.
    Region(Box<PlanarRegionWorkFailure>),
    /// Existing ordinary extrusion failure with its stage and cause intact.
    Sweep(Box<OperationDecline>),
    /// Input or detached-output lineage could not be completely composed.
    History(&'static str),
    /// A native operation declined before returning a complete result.
    Native {
        /// Active stage.
        stage: SpatialPrismStage,
        /// Original native failure.
        source: Box<Failure>,
    },
    /// Cooperative cancellation was observed.
    Cancelled(SpatialPrismStage),
}

impl fmt::Display for SpatialPrismFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use SpatialPrismFailure as Cause;
        match self {
            Cause::OpenWire => write!(f, "spatial prism requires a closed wire"),
            Cause::InvalidTraceLimit(value) => {
                write!(f, "source trace limit {value} must be finite and positive")
            }
            Cause::DegeneratePlane => write!(
                f,
                "spatial prism requires three noncollinear authored anchors"
            ),
            Cause::Skew {
                anchors,
                point,
                sign,
            } => write!(
                f,
                "point {point:?} has signed relation {sign:?} to the plane of {anchors:?}; expected coplanarity"
            ),
            Cause::TangentDirection(direction) => write!(
                f,
                "sweep direction {direction:?} must be transverse to the authored plane"
            ),
            Cause::Predicate { point, source } => write!(
                f,
                "exact spatial prism predicate refused {point:?} ({source})"
            ),
            Cause::UnboundChart(anchors) => write!(
                f,
                "anchors {anchors:?} did not yield a finite positively oriented native chart"
            ),
            Cause::RegionBinding { line, expected } => write!(
                f,
                "native planar region for {line:?} must preserve {expected}"
            ),
            Cause::SourceTrace { line, bound, limit } => write!(
                f,
                "native line {line:?} has source trace bound {bound}; expected at most {limit}"
            ),
            Cause::UnboundSweep { face, expected } => {
                write!(f, "retained sweep face {face:?} needs {expected}")
            }
            Cause::SweepVertexTrace {
                point,
                role,
                bound,
                limit,
            } => write!(
                f,
                "retained cap image {point:?} {role:?} has trace bound {bound}, above {limit}"
            ),
            Cause::SweepFaceTrace { face, bound, limit } => write!(
                f,
                "retained sweep face {face} has trim trace bound {bound}, above {limit}"
            ),
            Cause::Region(source) => write!(f, "spatial prism region failed ({source})"),
            Cause::Sweep(source) => write!(f, "spatial prism sweep failed ({source})"),
            Cause::History(expected) => write!(f, "spatial prism history must preserve {expected}"),
            Cause::Native { stage, source } => {
                write!(f, "native spatial prism {stage:?} failed ({source})")
            }
            Cause::Cancelled(stage) => write!(f, "spatial prism was cancelled at {stage:?}"),
        }
    }
}

impl Error for SpatialPrismFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Predicate { source, .. } => Some(source),
            Self::Region(source) => Some(source.as_ref()),
            Self::Sweep(source) => Some(source.as_ref()),
            Self::Native { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

pub(super) fn native<T>(
    result: Result<T, Failure>,
    stage: SpatialPrismStage,
) -> Result<T, SpatialPrismFailure> {
    result.map_err(|source| SpatialPrismFailure::Native {
        stage,
        source: Box::new(source),
    })
}
