//! Source-bound rational normal extrusion with frozen publication evidence (#418).
mod angular;
mod api;
pub use api::*;
mod boundaries;
mod cap_coordinates;
mod coverage;
mod developed_request;
mod family;
mod frozen_placement;
mod history;
mod math_cancellation;
mod native;
mod normal_measure;
mod operation;
mod physical_frame;
mod pipeline;
mod planar;
mod planar_clearance;
mod planar_cones;
mod profile_input;
mod publication_budget;
mod selection;
mod storage;
mod subdivision;
#[cfg(test)]
mod tests;

use crate::surface_sketch::{SurfaceCurveId, SurfaceRegionId, SurfaceVertexId};
use developed_request::DevelopedRegionRequest;
use martensite_tkbrep::topods::{Edge, Face, Shape};
use martensite_tkernel::standard::Failure;
type CancelPoll<'a> = dyn FnMut() -> bool + Send + 'a;

// Retained typed causes determine cancellation without polling again (#526).
pub(super) trait CancellationCause {
    fn cancelled(&self) -> bool;
}
impl<T: CancellationCause + ?Sized> CancellationCause for Box<T> {
    fn cancelled(&self) -> bool {
        self.as_ref().cancelled()
    }
}

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
    source: DevelopedRegionRequest,
    radius: f64,
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
    #[cfg(test)]
    Admission,
    #[cfg(test)]
    Planar,
    #[cfg(test)]
    Planning,
    #[cfg(test)]
    Clearance,
    #[cfg(test)]
    Construction,
    #[cfg(test)]
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
    #[cfg(test)]
    Planning(Box<subdivision::PlanningFailure>),
    #[cfg(test)]
    Planar(Box<planar::PlanarFailure>),
    Cones(Box<planar_cones::ConeFailure>),
    #[cfg(test)]
    Clearance(Box<planar_clearance::ClearanceFailure>),
}
impl std::fmt::Display for RefusalCause {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Coverage(cause) => write!(formatter, "{cause}"),
            Self::NormalMeasure(cause) => write!(formatter, "{cause}"),
            Self::Native(cause) => write!(formatter, "{cause}"),
            Self::Storage(cause) => write!(formatter, "{cause}"),
            #[cfg(test)]
            Self::Planning(cause) => write!(formatter, "{cause}"),
            #[cfg(test)]
            Self::Planar(cause) => write!(formatter, "{cause}"),
            Self::Cones(cause) => write!(formatter, "{cause}"),
            #[cfg(test)]
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
