// SPDX-License-Identifier: GPL-2.0-only
//! Detached capture of a located native line and its named endpoint occurrences.
mod failure;
mod storage;
pub use failure::{LineEdgeCaptureError, LineEdgeReadCell};

use crate::topods::{Edge, Vertex};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::{
    gp::{Direction3, Line3, Point3},
    predicates::roots::metric_bounds::{exact_placed_line_ends, exact_placed_point},
};

/// The actual retained line carrier and its endpoints, with no native handles.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativeLineEdgeCapture {
    /// Stored carrier in its own coordinates.
    pub carrier: Line3,
    /// Original curve representation's effective placement rows.
    pub placement: [[f64; 4]; 3],
    /// Original increasing spatial representation range.
    pub range: [f64; 2],
    /// Actual edge occurrence orientation.
    pub orientation: Orientation,
    /// Endpoint positions in occurrence direction.
    pub points: [Point3; 2],
    /// Original modelling tolerance of the edge.
    pub edge_tolerance: f64,
    /// Original vertex tolerances in occurrence direction.
    pub vertex_tolerances: [f64; 2],
}

/// Native line evidence with the correspondence to the caller's vertex nominations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativeLineEdgeBinding {
    /// Detached native evidence, independent of nomination order.
    pub geometry: NativeLineEdgeCapture,
    /// Which nominated vertex is bound to each directed endpoint.
    pub vertices: [usize; 2],
}

/// Capture the original active line and prove its two named vertices agree exactly.
/// Example: `capture_line_edge(&edge, [&first, &last], || false)?`.
///
/// Vertex nominations are matched by actual topology and location. The caller owns and
/// pins its source revision while resolving the edge and vertices. Native geometry,
/// topology, range and placement used by the proof are copied under simultaneous
/// nonblocking read leases. Source data can be changed after publication.
pub fn capture_line_edge(
    edge: &Edge,
    vertices: [&Vertex; 2],
    mut cancelled: impl FnMut() -> bool,
) -> Result<NativeLineEdgeBinding, LineEdgeCaptureError> {
    poll(&mut cancelled)?;
    let stored = storage::read(edge, vertices)?;
    poll(&mut cancelled)?;
    let capture = prove(stored)?;
    poll(&mut cancelled)?;
    Ok(capture)
}

fn poll(cancelled: &mut impl FnMut() -> bool) -> Result<(), LineEdgeCaptureError> {
    if cancelled() {
        return Err(LineEdgeCaptureError::Cancelled);
    }
    Ok(())
}

fn prove(stored: storage::StoredLine) -> Result<NativeLineEdgeBinding, LineEdgeCaptureError> {
    Direction3::from_stored_coefficients(*stored.carrier.direction().coordinates())?;
    let [a, b] = stored.range;
    if !a.is_finite()
        || !b.is_finite()
        || a >= b
        || [a, b]
            .into_iter()
            .any(martensite_tkernel::precision::is_infinite)
    {
        return Err(LineEdgeCaptureError::Range(stored.range));
    }
    let ends = exact_placed_line_ends(stored.carrier, stored.placement, stored.range)?
        .ok_or(LineEdgeCaptureError::Unrepresentable)?;
    if ends[0] == ends[1] {
        return Err(LineEdgeCaptureError::Collapsed);
    }
    stored.endpoints(ends)
}

impl storage::StoredLine {
    fn endpoints(
        self,
        mut ends: [[f64; 3]; 2],
    ) -> Result<NativeLineEdgeBinding, LineEdgeCaptureError> {
        if self.orientation == Orientation::Reversed {
            ends.swap(0, 1);
        }
        for (index, vertex) in self.vertices.iter().enumerate() {
            let observed = exact_placed_point(vertex.point, vertex.placement)?
                .ok_or(LineEdgeCaptureError::Unrepresentable)?;
            if observed != ends[index] {
                return Err(LineEdgeCaptureError::Endpoint {
                    index,
                    expected: ends[index],
                    observed,
                });
            }
        }
        let geometry = NativeLineEdgeCapture {
            carrier: self.carrier,
            placement: self.placement,
            range: self.range,
            orientation: self.orientation,
            points: ends.map(|[x, y, z]| Point3::new(x, y, z)),
            edge_tolerance: self.edge_tolerance,
            vertex_tolerances: self.vertices.each_ref().map(|value| value.tolerance),
        };
        Ok(NativeLineEdgeBinding {
            geometry,
            vertices: self.vertices.map(|value| value.binding),
        })
    }
}

const _: fn() = || {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<(
        NativeLineEdgeCapture,
        LineEdgeCaptureError,
        LineEdgeReadCell,
    )>();
};

#[cfg(test)]
mod tests;
