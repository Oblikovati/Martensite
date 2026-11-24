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
