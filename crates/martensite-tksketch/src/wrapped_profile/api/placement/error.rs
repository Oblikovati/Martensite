//! Refusals retain native, arithmetic and bounded frozen-correspondence evidence.
use super::*;
#[derive(Debug)]
pub(in crate::surface_sketch::wrapped_profile) enum PlacementFailure {
    TraceTolerance {
        region: SurfaceRegionId,
        span: CurveSpan,
        depth: usize,
        actual: f64,
        retained_limit: f64,
    },
    SpatialInput {
        radius: f64,
        spatial_error: f64,
    },
    InputFrame {
        support: crate::surface_sketch::CylinderSupport,
        cause: Failure,
    },
    Frame(FrameTransportError),
    Rotation(martensite_tkmath::predicates::roots::metric_bounds::FrameRotationError),
    Encoding(crate::surface_sketch::failure::SurfaceFailureCause),
    Native {
        step: &'static str,
        cause: Failure,
    },
    Budget(publication_budget::BudgetFailure),
    LocationChain {
        identity: bool,
        power: Option<i32>,
        tail_identity: bool,
    },
    Coefficients {
        actual: [[f64; 4]; 3],
        expected: [[f64; 4]; 3],
        scale: f64,
    },
    SemanticDatum(BindingKey),
    FrozenGraph {
        actual_length: usize,
        expected_length: usize,
        offset: usize,
        actual: Option<u8>,
        expected: Option<u8>,
    },
    Member(SurfaceRegionId),
    Interval {
        actual: [f64; 2],
    },
}
impl std::fmt::Display for PlacementFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TraceTolerance { .. } => self.trace_message(f),
            Self::Rotation(cause) => write!(f, "physical placement: {cause}"),
            Self::Frame(cause) => write!(f, "physical placement: {cause}"),
            Self::Encoding(cause) => write!(f, "physical encoding: {cause:?}"),
            Self::Native { step, cause } => write!(f, "physical {step}: {cause}"),
            Self::Budget(cause) => write!(f, "physical placement: {cause}"),
            Self::Coefficients {
                actual,
                expected,
                scale,
            } => write!(
                f,
                "physical Coefficients actual: {actual:?}, expected: {expected:?}, scale: {scale:?}; expected equal coefficient bits and unit scale"
            ),
            _ => self.input_message(f),
        }
    }
}
impl PlacementFailure {
    fn trace_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TraceTolerance {
                region,
                span,
                depth,
                actual,
                retained_limit,
            } => write!(
                f,
                "physical trace region={region:?}, span={span:?}, depth={depth}, bound={actual}; expected bound <= retained edge/face limit {retained_limit}"
            ),
            _ => write!(f, "{self:?}"),
        }
    }
    fn input_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SpatialInput {
                radius,
                spatial_error,
            } => write!(
                f,
                "physical radius={radius}, spatial_error={spatial_error}; expected finite radius>0, finite spatial_error>0 and finite 4*spatial_error"
            ),
            Self::InputFrame { support, cause } => write!(
                f,
                "physical support: {support:?}: {cause}; expected finite origin/vectors and a nonzero nonparallel rigid frame"
            ),
            _ => self.location_message(f),
        }
    }
    fn location_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LocationChain {
                identity,
                power,
                tail_identity,
            } => write!(
                f,
                "physical LocationChain identity: {identity}, power: {power:?}, tail_identity: {tail_identity}; expected one elementary datum at power one"
            ),
            Self::SemanticDatum(key) => write!(
                f,
                "physical SemanticDatum binding={key:?} has a different datum; expected the shared decoded root datum"
            ),
            Self::Member(region) => write!(f, "missing {region:?}; expected one owned solid"),
            _ => self.graph_message(f),
        }
    }
    fn graph_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FrozenGraph {
                actual_length,
                expected_length,
                offset,
                actual,
                expected,
            } => write!(
                f,
                "physical FrozenGraph actual_length: {actual_length}, expected_length: {expected_length}, offset: {offset}, actual: {actual:?}, expected: {expected:?}; expected equal lengths and byte-identical stripped canonical graph"
            ),
            Self::Interval { actual } => write!(
                f,
                "physical interval={actual:?}; expected finite ordered endpoints"
            ),
            _ => write!(f, "{self:?}"),
        }
    }
}
impl std::error::Error for PlacementFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InputFrame { cause, .. } | Self::Native { cause, .. } => Some(cause),
            Self::Frame(cause) => Some(cause),
            Self::Rotation(cause) => Some(cause),
            _ => None,
        }
    }
}
pub(super) fn fail(cause: PlacementFailure) -> WrappedNormalFailure {
    let class = classification(&cause);
    super::super::error::with_entities(
        WrappedStage::Placement,
        class,
        entities(&cause),
        super::super::error::Cause::Placement(Box::new(cause)),
    )
}
fn classification(cause: &PlacementFailure) -> WrappedFailureClass {
    match cause {
        PlacementFailure::Rotation(cause) => rotation_class(&cause.cause),
        PlacementFailure::InputFrame { .. } | PlacementFailure::SpatialInput { .. } => {
            WrappedFailureClass::InvalidInput
        }
        PlacementFailure::Frame(
            FrameTransportError::NonFinite { .. }
            | FrameTransportError::ZeroAxis { .. }
            | FrameTransportError::ParallelRadial { .. }
            | FrameTransportError::SpatialInput { .. },
        ) => WrappedFailureClass::InvalidInput,
        PlacementFailure::Native { .. } | PlacementFailure::Encoding(_) => {
            WrappedFailureClass::Native
        }
        _ => WrappedFailureClass::Incomplete,
    }
}
fn entities(cause: &PlacementFailure) -> Vec<crate::surface_sketch::HistorySource> {
    match cause {
        PlacementFailure::TraceTolerance { region, span, .. } => vec![
            crate::surface_sketch::HistorySource::Region(*region),
            crate::surface_sketch::HistorySource::Curve(span.curve),
        ],
        PlacementFailure::Member(region) => {
            vec![crate::surface_sketch::HistorySource::Region(*region)]
        }
        PlacementFailure::SemanticDatum(key) => vec![failure_projection::binding_source(*key)],
        PlacementFailure::Budget(cause) => failure_projection::budget(cause).2,
        _ => vec![],
    }
}
pub(super) fn at<T>(step: &'static str, result: Result<T, Failure>) -> Checked<T> {
    result.map_err(|cause| fail(PlacementFailure::Native { step, cause }))
}
pub(super) fn budget<T>(result: Result<T, publication_budget::BudgetFailure>) -> Checked<T> {
    result.map_err(|cause| fail(PlacementFailure::Budget(cause)))
}

fn rotation_class(
    cause: &martensite_tkmath::predicates::roots::metric_bounds::FrameRotationFailure,
) -> WrappedFailureClass {
    use martensite_tkmath::predicates::roots::metric_bounds::FrameRotationFailure;
    match cause {
        FrameRotationFailure::InvalidAngle => WrappedFailureClass::InvalidInput,
        FrameRotationFailure::Frame(cause) => {
            classification(&PlacementFailure::Frame(cause.clone()))
        }
        FrameRotationFailure::Trigonometric(_) => WrappedFailureClass::Incomplete,
    }
}

#[cfg(test)]
#[path = "error_owner_tests.rs"]
mod owner_tests;
