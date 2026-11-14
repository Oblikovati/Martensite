//! Rational walking skeleton; no accepted body or new public API is exposed (#418).
mod angular;
mod boundaries;
mod coverage;
mod family;
mod native;
mod normal_measure;
mod pipeline;
mod planar;
mod planar_clearance;
mod planar_cones;
mod selection;
mod storage;
mod subdivision;
mod tests;

use crate::surface_sketch::{SurfaceCurveId, SurfaceRegionId, SurfaceVertexId};
use martensite_tkbrep::topods::{Edge, Face, Shape};
use martensite_tkernel::standard::Failure;

#[derive(Clone)]
struct AuthoredPiece {
    span: CurveSpan,
    start: CornerSource,
    end: CornerSource,
    poles: Vec<[f64; 2]>,
    chart_center: f64,
}

#[derive(Clone)]
struct Request {
    region: SurfaceRegionId,
    loops: Vec<Vec<AuthoredPiece>>,
    radius: f64,
    depths: [f64; 2],
    tolerance: f64,
    max_fit_level: u8,
}

struct NativePiece {
    edges: [Edge; 2],
    connectors: [Edge; 2],
    side: Face,
}

struct Constructed {
    solid: Shape,
    bindings: Vec<(BindingKey, Shape)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stage {
    Admission,
    Planar,
    Planning,
    Clearance,
    Construction,
    Freeze,
    Coverage,
    Storage,
    Cones,
    Validation,
    Measure,
}

#[derive(Debug)]
struct Refusal {
    stage: Stage,
    region: SurfaceRegionId,
    cause: RefusalCause,
}

#[derive(Debug)]
enum RefusalCause {
    Native(Box<Failure>),
    NormalMeasure(Box<normal_measure::NormalMeasureFailure>),
    Coverage(Box<coverage::CoverageFailure>),
    Storage(Box<storage::StorageFailure>),
    Planning(Box<subdivision::PlanningFailure>),
    Planar(Box<planar::PlanarFailure>),
    Cones(Box<planar_cones::ConeFailure>),
    Clearance(Box<planar_clearance::ClearanceFailure>),
}
impl std::fmt::Display for RefusalCause {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Coverage(cause) => write!(formatter, "{cause}"),
            Self::NormalMeasure(cause) => write!(formatter, "{cause}"),
            Self::Native(cause) => write!(formatter, "{cause}"),
            Self::Storage(cause) => write!(formatter, "{cause}"),
            Self::Planning(cause) => write!(formatter, "{cause}"),
            Self::Planar(cause) => write!(formatter, "{cause}"),
            Self::Cones(cause) => write!(formatter, "{cause}"),
            Self::Clearance(cause) => write!(formatter, "{cause}"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CurveSpan {
    curve: SurfaceCurveId,
    level: u8,
    index: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CornerSource {
    Authored(SurfaceVertexId),
    Split {
        curve: SurfaceCurveId,
        numerator: u32,
        level: u8,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Source {
    Region(SurfaceRegionId),
    Span(CurveSpan),
    Corner(CornerSource),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BindingKey {
    source: Source,
    role: crate::surface_sketch::HistoryRole,
}
