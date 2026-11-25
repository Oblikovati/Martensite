// SPDX-License-Identifier: GPL-2.0-only
//! Exact convex planar face clipping and detached affine section evidence.
mod exact;
mod trim;
mod verify;

use crate::{
    gp::{Line2, Line3, Plane3, Point3},
    predicates::NonFiniteCoordinate,
};
use exact::{Chart, Hit};
use std::{error::Error, fmt};

/// The first planar section behavior admits at most this many trim edges.
pub const PLANAR_SECTION_MAX_EDGES: usize = 32;

/// Literal affine placement rows, including translation.
pub type SectionPlacement = [[f64; 4]; 3];
/// Outward endpoint coverage for one native two-coordinate chart.
pub type SectionUvEndpoints = [[[f64; 2]; 2]; 2];

/// A stored plane and its effective placement, without rebuilding its frame.
#[derive(Clone, Copy, Debug)]
pub struct SectionChartSource {
    /// Original support chart.
    pub plane: Plane3,
    /// Actual effective affine placement.
    pub placement: SectionPlacement,
}

/// An actual stored vertex and its accumulated location.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SectionVertexSource {
    /// Local Cartesian vertex.
    pub point: Point3,
    /// Actual effective affine placement.
    pub placement: SectionPlacement,
}

/// A finite straight-line representation on its native domain.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SectionLineSource {
    /// Original 3D carrier.
    pub line: Line3,
    /// Effective curve placement.
    pub placement: SectionPlacement,
    /// Increasing native range.
    pub range: [f64; 2],
}

/// A stored affine curve in a support chart.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SectionUvLine {
    /// The stored, possibly trimmed line's parameterization.
    pub line: Line2,
    /// Native range of the representation.
    pub range: [f64; 2],
}

/// One directed use of a trim edge in the intrinsic forward face loop.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SectionBorder {
    /// Native 3D line and trim.
    pub space: SectionLineSource,
    /// Directed topological vertices.
    pub vertices: [SectionVertexSource; 2],
    /// The source use runs from the native last parameter towards the first.
    pub reversed: bool,
    /// Stored face pcurve if one exists. Absent planar pcurves use analytic pullback.
    pub pcurve: Option<SectionUvLine>,
}

/// Whether a completed pair has no regular curve inside the admitted trim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanarSectionEmpty {
    /// Distinct parallel support planes.
    ParallelDistinct,
    /// Transverse support line misses the closed trim.
    TrimMiss,
}

/// Local trim incidence of a certified endpoint.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlanarSectionAnchor {
    /// An interior point of the indexed border use.
    Edge {
        /// Index in the supplied directed loop.
        index: usize,
        /// Outward interval of the original edge's native parameter.
        parameter: [f64; 2],
    },
    /// The start vertex of the indexed border use.
    Vertex(usize),
}

/// An independently classified intersection of two placed planar supports.
#[derive(Clone, Debug)]
pub enum PlanarSectionTrim {
    /// A completed, classified empty cut.
    Empty(PlanarSectionEmpty),
    /// The complete planar supports coincide.
    CoincidentSupports,
    /// The intersection follows a trim boundary edge, indexed by its use.
    BoundaryOverlap(usize),
    /// A single tangent point at a trim vertex, indexed by its outgoing use.
    PointOnly(usize),
    /// A unique directed positive-length cut through the convex trim.
    Segment(Box<PlanarSectionWitness>),
}

/// Source-bound exact evidence used to qualify a real native candidate.
#[derive(Clone, Debug)]
pub struct PlanarSectionWitness {
    charts: [Chart; 2],
    hits: [Hit; 2],
}

/// Detached output from qualifying an actual native section edge.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanarSectionLineEvidence {
    /// Rounded endpoints of the actual native carrier in the proved direction.
    pub endpoints: [[f64; 3]; 2],
    /// Corresponding original native section parameter endpoints, possibly descending.
    pub native_parameters: [f64; 2],
    /// Exact clipped endpoint positions in each original support chart.
    /// The map is affine along the common normalized section parameter.
    pub parent_uv: [SectionUvEndpoints; 2],
    /// Actual trim ancestry in the same order as the endpoints.
    pub anchors: [PlanarSectionAnchor; 2],
    /// Certified distance from the full native curve to the exact clipped segment.
    pub native_error_bound: f64,
    /// Whole-interval vertex chord to actual native carrier bound.
    pub vertex_error_bound: f64,
    /// Certified distance from the published line to the exact clipped segment.
    pub position_error_bound: f64,
    /// Whole-interval distances from actual stored pcurves to the native carrier.
    /// Absent stored pcurves are covered by the exact analytic chart maps instead.
    pub stored_pcurve_error_bounds: [Option<f64>; 2],
}

/// A numeric or structural obligation that prevented publication.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum PlanarSectionError {
    /// Nonfinite raw coefficient.
    Coordinate(NonFiniteCoordinate),
    /// One source chart has dependent placed basis vectors.
    DegenerateChart(usize),
    /// The number of border uses lies outside the finite admitted domain.
    BorderCount(usize),
    /// The native line trim does not match its vertices, support or stored pcurve.
    InconsistentBorder(usize),
    /// The forward outer loop is not strictly convex with bounded interior.
    NonConvexTrim,
    /// A finite exact enclosure or rounded endpoint could not be supplied.
    Unrepresentable,
    /// Native candidate line range, orientation, vertices or pcurve is unresolved.
    CandidateDomain,
    /// Requested maximum spatial error is not finite and nonnegative.
    ErrorLimit(f64),
    /// The certificate exceeded the supplied positional allowance.
    Position {
        /// Verified upper bound.
        upper: f64,
        /// Requested limit.
        limit: f64,
    },
    /// An operation-local cancellation was observed.
    Cancelled,
}

impl fmt::Display for PlanarSectionError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Coordinate(cause) => cause.fmt(output),
            Self::DegenerateChart(index) => write!(
                output,
                "chart {index} requires two independent placed directions"
            ),
            Self::BorderCount(count) => write!(
                output,
                "border has {count} uses, expected 3 through {PLANAR_SECTION_MAX_EDGES}"
            ),
            Self::InconsistentBorder(index) => write!(
                output,
                "border use {index} requires consistent exact spatial, trim and vertex coefficients"
            ),
            Self::NonConvexTrim => write!(
                output,
                "face trim requires one positively oriented strictly convex loop"
            ),
            Self::Unrepresentable => write!(
                output,
                "section requires finite rounded coordinates and outward bounds"
            ),
            Self::CandidateDomain => write!(
                output,
                "native candidate requires one regular affine line and compatible trim representations"
            ),
            Self::ErrorLimit(limit) => write!(
                output,
                "position limit {limit} requires a finite nonnegative value"
            ),
            Self::Position { upper, limit } => {
                write!(output, "section position bound {upper} exceeds {limit}")
            }
            Self::Cancelled => write!(output, "planar section cancelled"),
        }
    }
}

impl Error for PlanarSectionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Coordinate(cause) => Some(cause),
            _ => None,
        }
    }
}

impl From<NonFiniteCoordinate> for PlanarSectionError {
    fn from(cause: NonFiniteCoordinate) -> Self {
        Self::Coordinate(cause)
    }
}

type Checked<T> = Result<T, PlanarSectionError>;

/// Classify an actual directed convex linear face trim against an unbounded plane.
/// Example: `certify_planar_section_trim([face, work_plane], false, &border, || false)?`.
pub fn certify_planar_section_trim(
    supports: [SectionChartSource; 2],
    reverse_face: bool,
    border: &[SectionBorder],
    mut cancelled: impl FnMut() -> bool,
) -> Checked<PlanarSectionTrim> {
    trim::classify(supports, reverse_face, border, &mut cancelled)
}

impl PlanarSectionWitness {
    /// Verify the private copy preserved the same placed border over its native range.
    /// Example: `witness.verify_copy_border(index, &source_border, &copied_border)?`.
    pub fn verify_copy_border(
        &self,
        index: usize,
        source: &SectionBorder,
        copied: &SectionBorder,
    ) -> Checked<()> {
        if source == copied {
            return Ok(());
        }
        if source.reversed != copied.reversed
            || source.space.range != copied.space.range
            || (source.pcurve.is_some() && source.pcurve != copied.pcurve)
        {
            return Err(PlanarSectionError::InconsistentBorder(index));
        }
        let chart = &self.charts[0];
        if trim::validate_edge(chart, source, index)? != trim::validate_edge(chart, copied, index)?
        {
            return Err(PlanarSectionError::InconsistentBorder(index));
        }
        Ok(())
    }

    /// Qualify the actual native 3D line, oriented vertices and optional stored line pcurves.
    /// Example: `witness.certify_native(line, vertices, pcurves, limit, || false)?`.
    pub fn certify_native(
        &self,
        line: SectionLineSource,
        vertices: [SectionVertexSource; 2],
        pcurves: [Option<SectionUvLine>; 2],
        limit: f64,
        mut cancelled: impl FnMut() -> bool,
    ) -> Checked<PlanarSectionLineEvidence> {
        verify::qualify(self, line, vertices, pcurves, limit, &mut cancelled)
    }
}

fn poll(cancelled: &mut impl FnMut() -> bool) -> Checked<()> {
    if cancelled() {
        return Err(PlanarSectionError::Cancelled);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
