// SPDX-License-Identifier: GPL-2.0-only
//! Owned native operands remain inspectable without borrowed construction queries.
use super::*;
/// Actual side source retained by the numeric certificate; fields have no proof constructor.
#[derive(Clone, Debug)]
pub struct ConicalActualSideSnapshot {
    pub(super) id: ConicalActualSideId,
    pub(super) member: ConicalActualMemberId,
    pub(super) caps: [ConicalActualCapId; 2],
    pub(super) rows: [Vec<[f64; 3]>; 2],
    pub(super) weights: Vec<f64>,
    pub(super) orientation: i8,
}
/// One actual intrinsic cap occurrence, including complete range and orientation.
#[derive(Clone, Debug)]
pub struct ConicalActualUvEdgeSnapshot {
    side: ConicalActualSideId,
    row: usize,
    poles: Vec<[f64; 2]>,
    weights: Vec<f64>,
    range: [f64; 2],
    reversed: bool,
}
/// Complete selected intrinsic wire; role is rebound independently by TKBO.
#[derive(Clone, Debug)]
pub struct ConicalActualUvContourSnapshot {
    role: ConicalActualContourRole,
    wire_orientation: i8,
    edges: Vec<ConicalActualUvEdgeSnapshot>,
}
/// Actual carrier and complete selected UV source retained for native rebinding.
#[derive(Clone, Debug)]
pub struct ConicalActualCapSnapshot {
    carrier: ConicalActualCapCarrier,
    face_orientation: i8,
    contours: Vec<ConicalActualUvContourSnapshot>,
}
impl ConicalActualSideSnapshot {
    /// Exact side source label; `side.id()`.
    pub fn id(&self) -> ConicalActualSideId {
        self.id
    }
    /// Exact member source label; `side.member()`.
    pub fn member(&self) -> ConicalActualMemberId {
        self.member
    }
    /// Exact physical row cap labels; `side.caps()`.
    pub fn caps(&self) -> [ConicalActualCapId; 2] {
        self.caps
    }
    /// Complete actual surface rows; `side.rows()`.
    pub fn rows(&self) -> &[Vec<[f64; 3]>; 2] {
        &self.rows
    }
    /// Actual common positive weights; `side.weights()`.
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }
    /// Actual shell face orientation; `side.orientation()`.
    pub fn orientation(&self) -> i8 {
        self.orientation
    }
}
impl ConicalActualUvEdgeSnapshot {
    /// Actual incident side label; `edge.side()`.
    pub fn side(&self) -> ConicalActualSideId {
        self.side
    }
    /// Actual physical row; `edge.row()`.
    pub fn row(&self) -> usize {
        self.row
    }
    /// Actual UV control points; `edge.poles()`.
    pub fn poles(&self) -> &[[f64; 2]] {
        &self.poles
    }
    /// Complete actual weights; `edge.weights()`.
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }
    /// Actual stored parameter range; `edge.range()`.
    pub fn range(&self) -> [f64; 2] {
        self.range
    }
    /// Intrinsic native edge-use reversal; `edge.reversed()`.
    pub fn reversed(&self) -> bool {
        self.reversed
    }
}
impl ConicalActualUvContourSnapshot {
    /// Selected contour role; `wire.role()`.
    pub fn role(&self) -> ConicalActualContourRole {
        self.role
    }
    /// Intrinsic face-Forward wire orientation; `wire.wire_orientation()`.
    pub fn wire_orientation(&self) -> i8 {
        self.wire_orientation
    }
    /// Complete ordered occurrences; `wire.edges()`.
    pub fn edges(&self) -> &[ConicalActualUvEdgeSnapshot] {
        &self.edges
    }
}
impl ConicalActualCapSnapshot {
    /// Complete actual analytic carrier; `cap.carrier()`.
    pub fn carrier(&self) -> ConicalActualCapCarrier {
        self.carrier
    }
    /// Actual shell face orientation; `cap.face_orientation()`.
    pub fn face_orientation(&self) -> i8 {
        self.face_orientation
    }
    /// Complete selected intrinsic contours; `cap.contours()`.
    pub fn contours(&self) -> &[ConicalActualUvContourSnapshot] {
        &self.contours
    }
    pub(super) fn capture(query: &ConicalActualCapQuery<'_>) -> Self {
        let contours = query.contours.iter().map(contour_snapshot).collect();
        Self {
            carrier: query.carrier,
            face_orientation: query.face_orientation,
            contours,
        }
    }
}
pub(super) fn side(query: &ConicalActualSideQuery<'_>) -> ConicalActualSideSnapshot {
    ConicalActualSideSnapshot {
        id: query.id,
        member: query.member,
        caps: query.caps,
        rows: query.current.edges.map(|e| e.stored_poles.to_vec()),
        weights: query.current.edges[0].stored_weights.to_vec(),
        orientation: query.orientation,
    }
}

fn contour_snapshot(wire: &ConicalActualUvContour<'_>) -> ConicalActualUvContourSnapshot {
    ConicalActualUvContourSnapshot {
        role: wire.role,
        wire_orientation: wire.wire_orientation,
        edges: wire.edges.iter().map(edge_snapshot).collect(),
    }
}
fn edge_snapshot(edge: &ConicalActualUvEdge<'_>) -> ConicalActualUvEdgeSnapshot {
    ConicalActualUvEdgeSnapshot {
        side: edge.side,
        row: edge.row,
        poles: edge.poles.to_vec(),
        weights: edge.weights.to_vec(),
        range: edge.range,
        reversed: edge.reversed,
    }
}
