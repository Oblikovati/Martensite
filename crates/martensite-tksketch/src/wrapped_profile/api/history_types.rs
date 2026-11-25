//! Public authored identities stay independent of proof partitions and native table order.
use crate::surface_sketch::{
    HistoryRole, HistorySource, SurfaceCurveId, SurfaceRegionId, SurfaceVertexId,
};

/// Canonical dyadic interval in the original curve's normalized `[0,1]` parameter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WrappedSourceSpan {
    /// Denominator exponent: the interval denominator is `2^level`.
    pub level: u8,
    /// Lower endpoint numerator; the upper numerator is `index+1`.
    pub index: u64,
    /// Traversal runs from the upper endpoint to the lower endpoint.
    pub reversed: bool,
}

/// Canonical interior parameter of an operation-created subdivision vertex.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WrappedSourcePoint {
    /// Denominator exponent in the normalized original curve parameter.
    pub level: u8,
    /// Reduced numerator in `(0, 2^level)`.
    pub numerator: u64,
}

/// Exact map from a conic quarter's dyadic stereographic parameter to its native angle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WrappedConicParameterMap {
    /// Start angle in signed multiples of pi/2.
    pub first_quarter: i8,
    /// `true` adds `2 atan(t)` to the starting angle; `false` subtracts it.
    pub increasing: bool,
}

/// One bound of an exact stereographic portion of the original analytic arc.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WrappedAnalyticArcEndpoint {
    /// Native start of the authored arc.
    Start,
    /// Native end of the authored arc.
    End,
}

/// The exact native-angle map of a stereographic interval on an original analytic arc.
/// If `u0` and `u1` are its exact unit endpoints in the carrier's affine frame, then
/// `angle(t) = arg(u0) + 2 atan(t * cross(u0,u1)/(1 + dot(u0,u1)))`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WrappedAnalyticArcMap {
    /// Exact first angular endpoint.
    pub first: WrappedAnalyticArcEndpoint,
    /// Exact last angular endpoint.
    pub last: WrappedAnalyticArcEndpoint,
    /// Increasing native angle on the original curve.
    pub increasing: bool,
}

pub use martensite_tkmath::predicates::roots::metric_bounds::ConicSpanParameter as WrappedConicParameterization;

/// Original admitted native angle and exact rational span parameter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WrappedAdmittedArcMap {
    pub(crate) angles: [u64; 2],
    /// Zero-based piece of the admitted sweep.
    pub index: u8,
    /// Number of pieces, all of equal native angular length.
    pub count: u8,
    /// Conversion from the local parameter to its native angle.
    pub parameter: WrappedConicParameterization,
}

impl WrappedAdmittedArcMap {
    /// Recover the admitted first angle and signed sweep in radians.
    /// The piece starts at the exact expression `first + sweep * index / count`.
    /// Example: `let [first, sweep] = map.sweep();`.
    pub fn sweep(self) -> [f64; 2] {
        self.angles.map(f64::from_bits)
    }
}

/// Exact binary64 parameter authored as a native B-spline knot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WrappedNativeKnot(pub(crate) u64);

impl WrappedNativeKnot {
    /// Recover the original knot without normalization. Example: `let u = knot.value();`.
    pub fn value(self) -> f64 {
        f64::from_bits(self.0)
    }
}

/// Exact affine map from a local span parameter to native B-spline knots.
/// If `[a,b] = map.knots.map(WrappedNativeKnot::value)`, the native parameter is
/// the real arithmetic expression `a + (b - a) t`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WrappedSplineParameterMap {
    /// The two original ordered native knots.
    pub knots: [WrappedNativeKnot; 2],
}

/// Authored source of one result entity; coordinates and traversal ranks carry no identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum WrappedEntitySource {
    /// The complete selected material region.
    Region(SurfaceRegionId),
    /// An interval of one original authored curve.
    Curve {
        /// Original curve identity.
        curve: SurfaceCurveId,
        /// Exact normalized source interval and traversal direction.
        span: WrappedSourceSpan,
    },
    /// A mapped interval of one original analytic conic arc.
    MappedCurve {
        /// Original authored curve identity.
        curve: SurfaceCurveId,
        /// Dyadic interval in the stereographic parameter used by `map`.
        span: WrappedSourceSpan,
        /// Exact conversion to the authored angular parameter.
        map: WrappedConicParameterMap,
    },
    /// A split of an original analytic conic arc.
    MappedSplitVertex {
        /// Original authored curve identity.
        curve: SurfaceCurveId,
        /// Dyadic point in the stereographic parameter used by `map`.
        point: WrappedSourcePoint,
        /// Exact conversion to the authored angular parameter.
        map: WrappedConicParameterMap,
    },
    /// A mapped interval on an exact authored analytic sweep.
    AnalyticArcCurve {
        /// Original authored curve identity.
        curve: SurfaceCurveId,
        /// Dyadic interval in the mapped stereographic parameter.
        span: WrappedSourceSpan,
        /// Exact conversion to the native angle.
        map: WrappedAnalyticArcMap,
    },
    /// A subdivision point of an original exact analytic sweep.
    AnalyticArcSplitVertex {
        /// Original authored curve identity.
        curve: SurfaceCurveId,
        /// Dyadic point in the mapped stereographic parameter.
        point: WrappedSourcePoint,
        /// Exact conversion to the native angle.
        map: WrappedAnalyticArcMap,
    },
    /// A portion of an admitted analytic sweep whose planar representative is sewn.
    AdmittedArcCurve {
        /// Original authored curve identity.
        curve: SurfaceCurveId,
        /// Exact dyadic interval in the local conic parameter.
        span: WrappedSourceSpan,
        /// Original native-angle map.
        map: WrappedAdmittedArcMap,
    },
    /// A subdivision point on a parametrized admitted analytic sweep.
    AdmittedArcSplitVertex {
        /// Original authored curve identity.
        curve: SurfaceCurveId,
        /// Exact dyadic point in the local conic parameter.
        point: WrappedSourcePoint,
        /// Original native-angle map.
        map: WrappedAdmittedArcMap,
    },
    /// An internal native angle separating two sewn portions of one sweep.
    AdmittedArcAnchor {
        /// Original authored curve identity.
        curve: SurfaceCurveId,
        /// The original map whose final angle defines this anchor.
        after: WrappedAdmittedArcMap,
    },
    /// A dyadic interval within one mapped native B-spline knot span.
    NativeSplineCurve {
        /// Original authored curve identity.
        curve: SurfaceCurveId,
        /// Exact interval in the local affine parameter and traversal direction.
        span: WrappedSourceSpan,
        /// Exact conversion to the authored B-spline parameter.
        map: WrappedSplineParameterMap,
    },
    /// A subdivision point within an original B-spline knot span.
    NativeSplineSplitVertex {
        /// Original authored curve identity.
        curve: SurfaceCurveId,
        /// Exact point in the local affine parameter.
        point: WrappedSourcePoint,
        /// Exact conversion to the native parameter.
        map: WrappedSplineParameterMap,
    },
    /// An interior original native knot, shared by two span images.
    NativeSplineKnot {
        /// Original authored curve identity.
        curve: SurfaceCurveId,
        /// Native knot identity before normalization.
        knot: WrappedNativeKnot,
    },
    /// Exact native quarter angle on a closed or multipart analytic conic.
    ConicAnchor {
        /// Original authored curve identity.
        curve: SurfaceCurveId,
        /// Signed native angle in multiples of pi/2; closed seam is zero.
        quarter: i8,
    },
    /// A caller-authored endpoint.
    Vertex(SurfaceVertexId),
    /// A new vertex derived from an exact original curve parameter.
    SplitVertex {
        /// Original curve identity.
        curve: SurfaceCurveId,
        /// Canonical source parameter.
        point: WrappedSourcePoint,
    },
}

/// Persistent identity relative to the caller's ordered normal-depth endpoints.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WrappedHistoryKey {
    /// Authored region, curve interval or vertex source.
    pub source: WrappedEntitySource,
    /// Role relative to the requested first and last depths.
    pub role: HistoryRole,
}

/// Relation from all authored parents to one result entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum WrappedHistoryRelation {
    /// An image of complete original sources.
    Derived,
    /// An image of a subdivided source interval.
    Split,
    /// A newly constructed entity with explicit authored parents.
    Created,
}

/// One complete relation, ordered by its persistent result key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WrappedHistoryRecord {
    /// Result entity identity.
    pub result: WrappedHistoryKey,
    /// Selected material owner.
    pub owner: SurfaceRegionId,
    /// Relation to the original inputs.
    pub relation: WrappedHistoryRelation,
    /// All parents, in total order without duplicates.
    pub parents: Vec<HistorySource>,
}
