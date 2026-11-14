//! Input refusals retain caller identities and offending source values.
use super::*;
const BEZIER_SHAPE: &str = "degree 1..=3, degree+1 poles, matching weights and 2*(degree+1) knots";
#[path = "display.rs"]
mod display;
#[derive(Debug)]
pub(in super::super) enum ProfileInputFailure {
    Plane(ProfilePlane),
    DuplicateVertex(ProfileVertexId),
    DuplicateCurve(ProfileCurveId),
    MissingVertex {
        curve: ProfileCurveId,
        vertex: ProfileVertexId,
    },
    MissingCurve(ProfileCurveId),
    ReusedCurve(ProfileCurveId),
    EmptyLoop {
        region: SurfaceRegionId,
        contour: usize,
    },
    Unsupported(ProfileCurveId),
    Spline {
        curve: ProfileCurveId,
        degree: u32,
        poles: usize,
        weights: Option<usize>,
        knots: usize,
    },
    Weight {
        curve: ProfileCurveId,
        index: usize,
        value: f64,
    },
    Knot {
        curve: ProfileCurveId,
        index: usize,
        value: f64,
    },
    Endpoint {
        curve: ProfileCurveId,
        vertex: ProfileVertexId,
        point: [f64; 2],
        expected: [f64; 2],
    },
    NonFinite {
        curve: ProfileCurveId,
        control: usize,
        point: [f64; 2],
    },
    UnknownSource(SurfaceCurveId),
    Interval(CurveSpan),
}
impl ProfileInputFailure {
    fn expected(&self) -> &'static str {
        match self {
            Self::Plane(_) => "world-XY developed coordinates; support placement is separate",
            Self::DuplicateVertex(_) => "unique authored vertex IDs",
            Self::DuplicateCurve(_) => "unique authored curve IDs",
            Self::MissingVertex { .. } => "one supplied vertex for each referenced endpoint",
            Self::MissingCurve(_) => "one supplied curve for each directed use",
            Self::ReusedCurve(_) => "one selected boundary use per authored curve",
            Self::EmptyLoop { .. } => "at least one directed curve use per selected loop",
            Self::Unsupported(_) => "a line or a degree-1 through degree-3 polynomial Bezier span",
            Self::Spline { .. } => BEZIER_SHAPE,
            Self::Weight { .. } => "unit polynomial weights",
            Self::Knot { .. } => "degree+1 zero knots followed by degree+1 unit knots",
            Self::Endpoint { .. } => "endpoint control bits equal to the authored vertex bits",
            Self::NonFinite { .. } => "finite developed control coordinates",
            Self::UnknownSource(_) => "a selected authored curve",
            Self::Interval(_) => "a nonempty canonical dyadic interval with level below 64",
        }
    }
}
