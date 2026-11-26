// SPDX-License-Identifier: GPL-2.0-only
//! Exact queries on a located affine chart with a strictly convex outer trim.
mod chart;
mod cubic;
mod curve;
pub use cubic::{
    PlanarCubicCertificate, PlanarCubicOptions, PlanarCubicProofFailure, PlanarMonotoneAxis,
};
mod segment;
pub use curve::{PlanarCurveCertificate, PlanarCurvePath, PlanarCurveProofFailure};
mod trim;

use self::{chart::Chart, trim::Loop};
use super::super::certificate_arithmetic::{ExactRatio, rounding::outward_sqrt};
use super::segment_plane::SegmentPlaneFailure;
use crate::gp::{Axis2d, Axis3, Line3};
use crate::predicates::NonFiniteCoordinate;
pub use segment::PlanarSegmentContact;
use std::{cmp::Ordering, error::Error, fmt};

/// Maximum number of edge uses in each of the two admitted loops.
pub const PLANAR_ATTACHMENT_MAX_LOOP_EDGES: usize = 32;

/// Actual finite affine representations of one directed source trim.
#[derive(Clone, Copy, Debug)]
pub struct PlanarAttachmentEdge {
    /// 3D carrier before the common face placement.
    pub line: Line3,
    /// The stored increasing range on the 3D carrier.
    pub range: [f64; 2],
    /// Endpoints in directed topological order before the common placement.
    pub vertices: [[f64; 3]; 2],
    /// Whether to traverse from the 3D carrier's upper parameter.
    pub reversed: bool,
    /// The actual stored affine pcurve on this support, when present.
    pub pcurve: Option<(Axis2d, [f64; 2])>,
}

/// Exact trim position of the orthogonal projection onto the source support.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanarAttachmentLocation {
    /// Strictly inside the material.
    Material,
    /// On a closed outer boundary.
    OuterBoundary,
    /// On a closed hole boundary.
    HoleBoundary,
    /// Strictly outside the outer boundary.
    OutsideOuter,
    /// Strictly inside the hole.
    HoleInterior,
}

/// A classified projection and outward bounds before nominal coordinate conversion.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanarAttachmentQuery {
    /// Location of the exact projected UV in the complete trimmed domain.
    pub location: PlanarAttachmentLocation,
    /// Inclusive lower and upper UV bounds in the original face chart.
    pub uv: [[f64; 2]; 2],
    /// Inclusive lower and upper world-coordinate bounds of the projected point.
    pub projected: [[f64; 2]; 3],
    /// Upper bound on the world Euclidean projection distance.
    pub distance_bound: f64,
}

/// Nominal local data for a solver linearization at an accepted chart coordinate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanarAttachmentChart {
    /// Accepted chart coordinate used as the local evaluation anchor.
    pub uv: [f64; 2],
    /// Nominal world point at the anchor, rounded once from the exact affine map.
    pub world: [f64; 3],
    /// Nominal world tangents for one unit of each original chart coordinate.
    pub tangents: [[f64; 3]; 2],
}

/// Why exact planar trim preparation or classification could not complete.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum PlanarAttachmentError {
    /// An input coordinate must be finite.
    Coordinate(NonFiniteCoordinate),
    /// Exact segment admission or cut rounding did not complete.
    Segment(SegmentPlaneFailure),
    /// The original stored frame is inconsistent.
    Frame,
    /// The affine chart tangents lack a positive Gram determinant.
    DegenerateChart,
    /// A loop is outside the supported edge count.
    LoopSize(usize),
    /// Directed edge endpoints disagree with the carrier, vertices, or actual pcurve.
    Edge(usize),
    /// A directed loop does not close as an exact strictly convex polygon.
    Loop,
    /// The loops require opposite winding and a strictly nested, disjoint hole.
    Hole,
    /// The requested inclusive model distance must be finite and nonnegative.
    DistanceLimit(f64),
    /// The exact Euclidean residual exceeds the requested model distance.
    OffSupport {
        /// Certified upper bound when representable.
        bound: Option<f64>,
        /// Caller-supplied acceptance limit.
        limit: f64,
    },
    /// The query has no finite outward interval.
    Unrepresentable,
    /// The remaining computation was cancelled.
    Cancelled,
}

impl fmt::Display for PlanarAttachmentError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Coordinate(cause) => write!(output, "{cause}"),
            Self::Segment(cause) => cause.fmt(output),
            Self::Frame => output.write_str("attachment requires a consistent stored planar frame"),
            Self::DegenerateChart => {
                output.write_str("planar attachment requires independent chart tangents")
            }
            Self::LoopSize(count) => write!(
                output,
                "attachment loop with {count} edges requires 3 to {PLANAR_ATTACHMENT_MAX_LOOP_EDGES}"
            ),
            Self::Edge(index) => write!(
                output,
                "attachment edge {index} requires one consistent finite affine trim"
            ),
            Self::Loop => {
                output.write_str("attachment loop requires a closed strictly convex polygon")
            }
            Self::Hole => {
                output.write_str("attachment requires a strictly contained opposite-wound hole")
            }
            Self::DistanceLimit(limit) => write!(
                output,
                "attachment distance limit {limit} must be finite and nonnegative"
            ),
            Self::OffSupport { bound, limit } => write!(
                output,
                "attachment residual with upper bound {bound:?} exceeds {limit}"
            ),
            Self::Unrepresentable => {
                output.write_str("attachment requires finite outward intervals")
            }
            Self::Cancelled => output.write_str("planar attachment was cancelled"),
        }
    }
}

impl Error for PlanarAttachmentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Coordinate(cause) => Some(cause),
            Self::Segment(cause) => Some(cause),
            _ => None,
        }
    }
}

impl From<NonFiniteCoordinate> for PlanarAttachmentError {
    fn from(error: NonFiniteCoordinate) -> Self {
        Self::Coordinate(error)
    }
}

/// Immutable source chart and complete directed linear trim proof.
#[derive(Clone, Debug)]
pub struct CertifiedPlanarAttachment {
    world: Chart,
    outer: Loop,
    hole: Option<Loop>,
}

impl CertifiedPlanarAttachment {
    /// Prove two opposite-wound linear loops against their exact source representations.
    /// Example: CertifiedPlanarAttachment::capture(frame, map, [&a, &b], || false).
    pub fn capture(
        frame: Axis3,
        placement: [[f64; 4]; 3],
        [a, b]: [&[PlanarAttachmentEdge]; 2],
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<Self, PlanarAttachmentError> {
        poll(&mut cancelled)?;
        let frame = chart::retained_frame(frame)?;
        let local = Chart::local(frame)?;
        let world = Chart::placed(frame, placement)?;
        let a = Loop::capture(&local, a, &mut cancelled)?;
        let b = Loop::capture(&local, b, &mut cancelled)?;
        let (outer, hole) = match (a.sense, b.sense) {
            (Ordering::Greater, Ordering::Less) => (a, b),
            (Ordering::Less, Ordering::Greater) => (b, a),
            _ => return Err(PlanarAttachmentError::Hole),
        };
        hole.require_inside(&outer, &mut cancelled)?;
        Ok(Self {
            world,
            outer,
            hole: Some(hole),
        })
    }

    /// Prove a single positively wound finite outer trim.
    /// Example: `CertifiedPlanarAttachment::capture_outer(frame, map, &edges, || false)?`.
    pub fn capture_outer(
        frame: Axis3,
        placement: [[f64; 4]; 3],
        edges: &[PlanarAttachmentEdge],
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<Self, PlanarAttachmentError> {
        poll(&mut cancelled)?;
        let frame = chart::retained_frame(frame)?;
        let local = Chart::local(frame)?;
        let world = Chart::placed(frame, placement)?;
        let outer = Loop::capture(&local, edges, &mut cancelled)?;
        if outer.sense != Ordering::Greater {
            return Err(PlanarAttachmentError::Loop);
        }
        Ok(Self {
            world,
            outer,
            hole: None,
        })
    }

    /// Classify the exact projection if the point is within the model distance.
    /// Example: trim.classify([0., 0., 0.], 0., || false).
    pub fn classify(
        &self,
        point: [f64; 3],
        limit: f64,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<PlanarAttachmentQuery, PlanarAttachmentError> {
        use PlanarAttachmentError as Error;
        poll(&mut cancelled)?;
        if !limit.is_finite() || limit < 0. {
            return Err(Error::DistanceLimit(limit));
        }
        let projection = self.world.project(point)?;
        self.qualify(projection, limit, &mut cancelled)
    }

    /// Check a supplied solver UV lift against the complete trim and the proposed world point.
    /// Example: proof.classify_uv(uv, world, tolerance, || false).
    pub fn classify_uv(
        &self,
        uv: [f64; 2],
        world: [f64; 3],
        limit: f64,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<PlanarAttachmentQuery, PlanarAttachmentError> {
        poll(&mut cancelled)?;
        if !limit.is_finite() || limit < 0. {
            return Err(PlanarAttachmentError::DistanceLimit(limit));
        }
        self.qualify(self.world.proposed(uv, world)?, limit, &mut cancelled)
    }

    /// Read a finite nominal affine linearization at a caller-selected accepted UV anchor.
    /// Example: proof.solver_chart([0., 0.]).
    pub fn solver_chart(
        &self,
        anchor: [f64; 2],
    ) -> Result<PlanarAttachmentChart, PlanarAttachmentError> {
        self.world.solver_chart(anchor)
    }

    fn qualify(
        &self,
        projection: chart::Projection,
        limit: f64,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<PlanarAttachmentQuery, PlanarAttachmentError> {
        use PlanarAttachmentError as Error;
        let cap = ExactRatio::from_finite(limit);
        let distance_bound = if projection.distance_squared.sign() == Ordering::Equal {
            Some(0.)
        } else {
            outward_sqrt(&projection.distance_squared)
        };
        if projection.distance_squared > cap.multiply(&cap) {
            return Err(Error::OffSupport {
                bound: distance_bound,
                limit,
            });
        }
        let location = trim::locate(&self.outer, self.hole.as_ref(), &projection.uv, cancelled)?;
        let uv = chart::enclose(&projection.uv)?;
        let projected = chart::enclose(&projection.world)?;
        poll(cancelled)?;
        Ok(PlanarAttachmentQuery {
            location,
            uv,
            projected,
            distance_bound: distance_bound.ok_or(Error::Unrepresentable)?,
        })
    }
}

fn poll(cancelled: &mut impl FnMut() -> bool) -> Result<(), PlanarAttachmentError> {
    if cancelled() {
        return Err(PlanarAttachmentError::Cancelled);
    }
    Ok(())
}
