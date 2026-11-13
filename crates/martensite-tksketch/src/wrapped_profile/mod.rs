//! Rational walking skeleton; no accepted body or new public API is exposed (#418).
mod angular;
mod boundaries;
mod native;
mod pipeline;
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
}

#[derive(Clone)]
struct Request {
    region: SurfaceRegionId,
    loops: Vec<Vec<AuthoredPiece>>,
    radius: f64,
    depths: [f64; 2],
    tolerance: f64,
    fit_level: u8,
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
    Construction,
    Freeze,
    Validation,
    Measure,
}

#[derive(Debug)]
struct Refusal {
    stage: Stage,
    region: SurfaceRegionId,
    cause: Box<Failure>,
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
