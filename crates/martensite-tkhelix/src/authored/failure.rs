// SPDX-License-Identifier: GPL-2.0-only
use martensite_tkernel::standard::Failure;
use martensite_tkmath::predicates::roots::metric_bounds::HelixTraceFailure;
use std::{error::Error, fmt};

/// An authored or derived helix quantity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelixQuantity {
    /// Opening radius in model units.
    OpeningRadius,
    /// Closing radius in model units.
    ClosingRadius,
    /// Axial rise per turn in model units.
    Pitch,
    /// Total axial rise in model units.
    Height,
    /// Number of complete or fractional turns.
    Turns,
    /// Effective angular-domain end in radians.
    ParameterEnd,
    /// Radial change per radian in model units.
    RadialRate,
    /// Axial rise per radian in model units.
    AxialRate,
    /// Native spline approximation request in model units.
    ApproximationTolerance,
    /// Allowed stored native edge/vertex tolerance in model units.
    TopologyTolerance,
    /// Maximum admitted spatial position error in model units.
    PositionTolerance,
}

/// Phase at which the requested native result stopped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelixStage {
    /// Request admission and resolved authored law.
    Admission,
    /// Native polynomial fitting or analytic evaluator.
    Fit,
    /// Native edge and wire construction.
    Assembly,
    /// Final actual topology/geometry and source correspondence.
    Verification,
    /// Copy-independent native byte capture before publication.
    Publication,
    /// Fresh retained geometry decode for an independent consumer.
    Export,
}

/// Retained native item required by the finite-helix representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum HelixNativeInvariant {
    /// Exactly one forward edge occurrence under the wire root.
    SingleForwardEdge,
    /// An open nondegenerate wire with two distinct placed terminals.
    OpenDistinctTerminals,
    /// Valid native wire/edge topology and edge consistency.
    ValidWire,
    /// An unlocated nonrational open polynomial B-spline.
    PolynomialCarrier,
    /// Bounded literal native polynomial degree and pole counts.
    PolynomialExtent,
    /// Forward retained edges in recipe order.
    OrderedWire,
    /// Neighboring edges lack the same native station or an edge reuses its endpoint.
    SharedStation,
    /// The actual clamped endpoint and its topological vertex have different coordinates.
    LiteralEndpoint,
    /// A native archive with a representable byte length.
    ArchiveLength,
    /// A complete native wire archive.
    CompleteArchive,
}

impl fmt::Display for HelixNativeInvariant {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        let expected = match self {
            Self::SingleForwardEdge => "one forward edge under the wire root",
            Self::OpenDistinctTerminals => "an open nondegenerate wire with two distinct terminals",
            Self::ValidWire => "valid native wire and edge topology",
            Self::PolynomialCarrier => "an unlocated open polynomial B-spline",
            Self::PolynomialExtent => "bounded literal polynomial degree and pole counts",
            Self::OrderedWire => "one open ordered wire with the requested forward edges",
            Self::SharedStation => "one shared native vertex at each directed join",
            Self::LiteralEndpoint => "equal literal carrier and topological endpoints",
            Self::ArchiveLength => "a representable native wire archive length",
            Self::CompleteArchive => "a complete native wire archive",
        };
        output.write_str(expected)
    }
}

/// A typed refusal or incomplete bounded operation.
#[derive(Debug)]
#[non_exhaustive]
pub enum HelixFailure {
    /// An authored scalar is outside its allowed domain.
    InvalidScalar {
        /// The quantity that failed admission.
        quantity: HelixQuantity,
        /// The supplied or computed value.
        value: f64,
    },
    /// Resolved geometry cannot retain its intended finite nonzero parameters.
    DerivedScalar {
        /// The quantity that failed admission.
        quantity: HelixQuantity,
        /// The supplied or computed value.
        value: f64,
    },
    /// The placement does not define a finite right-handed frame.
    InvalidFrame(Failure),
    /// Evaluation requires a finite parameter in the authored domain.
    Parameter {
        /// The supplied native parameter in radians.
        parameter: f64,
        /// The finite upper domain endpoint.
        end: f64,
    },
    /// The world point or derivative is not finitely representable.
    Evaluation {
        /// The supplied native parameter in radians.
        parameter: f64,
        /// Derivative order, with zero denoting position.
        order: u8,
    },
    /// The intended turns exceed the caller's scoped native work envelope.
    TurnLimit {
        /// The resolved number of turns.
        requested: f64,
        /// The corresponding caller limit.
        limit: f64,
    },
    /// Native bounds must admit 1–512 effective segments and at least one evaluation.
    InvalidWorkLimits {
        /// Effective native segment limit.
        max_segments: usize,
        /// Maximum permitted native evaluator calls.
        max_evaluations: u32,
    },
    /// The fit consumed all permitted evaluator invocations.
    EvaluationLimit {
        /// Completed evaluator calls.
        completed: u32,
        /// The corresponding caller limit.
        limit: u32,
    },
    /// The retained native fitting engine could not complete.
    NativeFitIncomplete(i32),
    /// The native fitter reported an invalid or out-of-tolerance estimate.
    NativeEstimate {
        /// Native fit error estimate.
        estimate: f64,
        /// The corresponding caller limit.
        limit: f64,
    },
    /// Native assembly or verification did not yield the expected retained edge/wire representation.
    Topology(HelixNativeInvariant),
    /// An actual stored topology tolerance exceeds the caller's policy.
    TopologyTolerance {
        /// Measured or certified actual upper bound.
        achieved: f64,
        /// The corresponding caller limit.
        limit: f64,
    },
    /// A retained terminal vertex exceeds the certified positional allowance.
    TerminalPosition {
        /// Endpoint index in the directed native range.
        endpoint: usize,
        /// Measured or certified actual upper bound.
        achieved: f64,
        /// The corresponding caller limit.
        limit: f64,
    },
    /// Native readback cannot prove the actual whole-interval source trace.
    Trace(Box<HelixTraceFailure>),
    /// A native exception was raised during the indicated phase.
    Native {
        /// Phase in which the native call failed.
        stage: HelixStage,
        /// Original native failure.
        cause: Box<Failure>,
    },
    /// Caller cancellation was observed before immutable publication or detached export.
    Cancelled(HelixStage),
}

impl fmt::Display for HelixFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidScalar {
                quantity: HelixQuantity::PositionTolerance,
                value,
            } => write!(
                output,
                "helix position cap {value} must be finite and nonnegative"
            ),
            Self::InvalidScalar { quantity, value } => write!(
                output,
                "helix {quantity:?} value {value} must be positive and finite"
            ),
            Self::DerivedScalar { quantity, value } => write!(
                output,
                "resolved helix {quantity:?} value {value} is outside its representable domain"
            ),
            Self::Topology(expected) => write!(output, "native helix requires {expected}"),
            Self::Trace(cause) => cause.fmt(output),
            Self::Native { stage, cause } => {
                write!(output, "native helix {stage:?} failed: {cause}")
            }
            other => write!(output, "checked helix could not complete: {other:?}"),
        }
    }
}

impl Error for HelixFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidFrame(cause) => Some(cause),
            Self::Native { cause, .. } => Some(cause.as_ref()),
            Self::Trace(cause) => Some(cause.as_ref()),
            _ => None,
        }
    }
}

pub(super) fn poll(
    cancelled: &mut impl FnMut() -> bool,
    stage: HelixStage,
) -> Result<(), HelixFailure> {
    if cancelled() {
        return Err(HelixFailure::Cancelled(stage));
    }
    Ok(())
}

pub(super) fn native<T>(value: Result<T, Failure>, stage: HelixStage) -> Result<T, HelixFailure> {
    value.map_err(|cause| HelixFailure::Native {
        stage,
        cause: Box::new(cause),
    })
}
