// SPDX-License-Identifier: GPL-2.0-only
use crate::spatial_prism::SpatialPrismFailure;
use martensite_tkbo::body_check::{ResolutionRejected, ValidationReport};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::predicates::NonFiniteCoordinate;
use martensite_tksketch3d::wire::{WireClosure, WireFailure};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIPipeError;
use std::{error::Error, fmt};

/// Source side of the operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SweepInput {
    /// Ordered open spine.
    Spine,
    /// Closed initial profile.
    Profile,
}

/// Stage of a Sweep refusal or cancellation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SweepStage {
    /// Exact authored scope and root clearance.
    Admission,
    /// Creation of private operand wires.
    Materialize,
    /// Native construction or approximation.
    Construction,
    /// Ordered native body validation.
    Validation,
    /// Output geometry, correspondence and history.
    Binding,
    /// Complete native trim correspondence and integration of solid face flux.
    Integration,
    /// Immutable publication.
    Publication,
    /// Detached native export.
    Export,
}

/// Typed refusal or incomplete work from the bounded round-corner sweep.
#[derive(Debug)]
#[non_exhaustive]
pub enum SweepFailure {
    /// The caller's limits must be finite and positive.
    Resolution(ResolutionRejected),
    /// The source recipe has an unsupported opening or closure.
    Closure {
        /// Requested spine closure.
        path: WireClosure,
        /// Requested profile closure.
        section: WireClosure,
    },
    /// This operation requires three mutually orthogonal axis-aligned steps.
    PathScope,
    /// This operation requires one axial rectangle centered at the spine start.
    ProfileScope,
    /// Exact source plane admission failed.
    ProfilePlane(SpatialPrismFailure),
    /// The ordinary rounded path has insufficient clearance.
    Clearance {
        /// Authored leg ordinal.
        leg: usize,
        /// Certified lower leg length.
        lower: f64,
        /// Required strictly smaller tube diameter.
        required: f64,
    },
    /// An exact predicate refused its input.
    Predicate(NonFiniteCoordinate),
    /// Operand wire materialization failed.
    Wire {
        /// Original operand.
        owner: SweepInput,
        /// Typed cause from the native wire export.
        source: WireFailure,
    },
    /// An operand wire exceeds the accepted trace limit.
    InputTrace {
        /// Original operand.
        owner: SweepInput,
        /// Computed upper bound.
        bound: f64,
        /// Original trace limit.
        limit: f64,
    },
    /// The ordinary native builder did not complete.
    Construction(BRepBuilderAPIPipeError),
    /// Native approximation estimate is unavailable or exceeds the explicit limit.
    SurfaceEstimate {
        /// Reported native estimate.
        estimate: f64,
        /// Requested estimate limit.
        limit: f64,
    },
    /// Ordered native validation did not pass.
    Validation(Box<ValidationReport>),
    /// Result role or correspondence is missing or ambiguous.
    Binding(&'static str),
    /// Retained geometric evidence is outside the bounded supported family.
    Geometry(&'static str),
    /// The actual retained support lies outside the requested source trace bound.
    Trace {
        /// Decoded source correspondence under test.
        description: &'static str,
        /// Certified conservative upper error.
        bound: f64,
        /// Original requested tolerance.
        limit: f64,
    },
    /// The actual shell ends could not be closed into a solid.
    SolidClosure,
    /// A retained face trim cannot be certified against its declared carrier or source.
    FaceTrim {
        /// Face ordinal in stable retained ancestry.
        face: usize,
        /// Missing geometric evidence.
        expected: &'static str,
    },
    /// Physical volume evidence did not establish a positive bounded volume.
    Volume(&'static str),
    /// The computed cubic-model-unit error exceeds the requested bound.
    VolumeErrorBound {
        /// Achieved outward volume error.
        bound: f64,
        /// Caller volume error limit.
        limit: f64,
    },
    /// Trigonometric endpoint enclosure did not complete.
    Trigonometry(Box<martensite_tkmath::predicates::roots::metric_bounds::FrameRotationError>),
    /// An original native error.
    Native {
        /// Stage that failed.
        stage: SweepStage,
        /// Native cause.
        source: Box<Failure>,
    },
    /// Cooperative cancellation was observed.
    Cancelled(SweepStage),
}

impl fmt::Display for SweepFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Resolution(r) => write!(
                f,
                "sweep resolution {}={} must be finite and positive",
                r.field, r.value
            ),
            Self::Closure { path, section } => write!(
                f,
                "sweep requires an open spine and closed section, got {path:?} and {section:?}"
            ),
            Self::PathScope => write!(
                f,
                "sweep requires three directed straight steps on distinct coordinate axes"
            ),
            Self::ProfileScope => write!(
                f,
                "sweep requires one coordinate-aligned rectangle centered in the opening cross section"
            ),
            Self::ProfilePlane(cause) => {
                write!(f, "sweep profile requires an exact plane ({cause})")
            }
            Self::Clearance {
                leg,
                lower,
                required,
            } => write!(
                f,
                "sweep leg {leg} length lower bound {lower} must exceed {required}"
            ),
            Self::Predicate(cause) => write!(f, "exact sweep predicate failed ({cause})"),
            Self::Wire { owner, source } => write!(f, "sweep {owner:?} wire failed ({source})"),
            Self::InputTrace {
                owner,
                bound,
                limit,
            } => write!(f, "sweep {owner:?} trace bound {bound} exceeds {limit}"),
            Self::Construction(status) => {
                write!(f, "native pipe shell did not complete ({status:?})")
            }
            Self::SurfaceEstimate { estimate, limit } => write!(
                f,
                "native sweep surface estimate {estimate} must be finite and at most {limit}"
            ),
            Self::Validation(report) => write!(
                f,
                "sweep requires all ordered body checks ({:?})",
                report.first_non_pass()
            ),
            Self::Binding(expected) => write!(f, "sweep history requires {expected}"),
            Self::Geometry(expected) => write!(f, "sweep geometry requires {expected}"),
            Self::Trace {
                description,
                bound,
                limit,
            } => write!(
                f,
                "sweep trace for {description} has bound {bound}, exceeding {limit}"
            ),
            Self::SolidClosure => write!(f, "rounded sweep requires two native closing cap faces"),
            Self::FaceTrim { face, expected } => {
                write!(f, "sweep retained face {face} requires {expected}")
            }
            Self::Volume(expected) => {
                write!(f, "bounded physical sweep volume requires {expected}")
            }
            Self::VolumeErrorBound { bound, limit } => {
                write!(f, "sweep volume error {bound} must not exceed {limit}")
            }
            Self::Trigonometry(source) => {
                write!(f, "sweep volume trigonometry is incomplete ({source})")
            }
            Self::Native { stage, source } => write!(f, "sweep {stage:?} failed ({source})"),
            Self::Cancelled(stage) => write!(f, "sweep cancelled during {stage:?}"),
        }
    }
}

impl Error for SweepFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ProfilePlane(source) => Some(source),
            Self::Predicate(source) => Some(source),
            Self::Wire { source, .. } => Some(source),
            Self::Native { source, .. } => Some(source.as_ref()),
            Self::Trigonometry(source) => Some(source.as_ref()),
            _ => None,
        }
    }
}
