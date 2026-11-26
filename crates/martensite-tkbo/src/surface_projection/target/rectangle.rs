// SPDX-License-Identifier: GPL-2.0-only
//! Exact rectangle sides and closed directed native wire incidence.
use super::{BoundaryObservation, Checked, ProjectionHole, Scope};

#[derive(Clone, Copy)]
pub(super) enum Side {
    LowerU = 0,
    UpperU = 1,
    LowerV = 2,
    UpperV = 3,
}

pub(super) struct Rectangle {
    pub angular: [f64; 2],
    pub axial: [f64; 2],
    pub uses: [BoundaryObservation; 4],
    pub forward: bool,
}

impl Rectangle {
    pub fn capture(observed: Vec<BoundaryObservation>) -> Checked<Self> {
        let [angular, axial] = spans(&observed);
        if angular[0] >= angular[1] || axial[0] >= axial[1] {
            return Err(Scope::Topology.into());
        }
        let mut slots = [None, None, None, None];
        for boundary in observed {
            let side = side_of(boundary.uv, angular, axial)?;
            if slots[side as usize].replace(boundary).is_some() {
                return Err(Scope::Topology.into());
            }
        }
        let [Some(lo_u), Some(hi_u), Some(lo_v), Some(hi_v)] = slots else {
            return Err(Scope::Topology.into());
        };
        let uses = [lo_u, hi_u, lo_v, hi_v];
        require_incidences(&uses)?;
        let forward = uses[Side::LowerV as usize].uv[0][0] < uses[Side::LowerV as usize].uv[1][0];
        Ok(Self {
            angular,
            axial,
            uses,
            forward,
        })
    }

    pub fn as_hole(&self) -> ProjectionHole {
        ProjectionHole {
            angle: self.angular,
            axial: self.axial,
        }
    }
}

fn spans(edges: &[BoundaryObservation]) -> [[f64; 2]; 2] {
    let mut spans = [[f64::INFINITY, f64::NEG_INFINITY]; 2];
    for edge in edges {
        for point in edge.uv {
            for (axis, coordinate) in point.into_iter().enumerate() {
                spans[axis][0] = spans[axis][0].min(coordinate);
                spans[axis][1] = spans[axis][1].max(coordinate);
            }
        }
    }
    spans
}

fn side_of([a, b]: [[f64; 2]; 2], u: [f64; 2], v: [f64; 2]) -> Checked<Side> {
    let interval = |p: f64, q: f64, end: [f64; 2]| [p.min(q), p.max(q)] == end;
    if a[0] == b[0] && interval(a[1], b[1], v) {
        if a[0] == u[0] {
            return Ok(Side::LowerU);
        }
        if a[0] == u[1] {
            return Ok(Side::UpperU);
        }
    }
    if a[1] == b[1] && interval(a[0], b[0], u) {
        if a[1] == v[0] {
            return Ok(Side::LowerV);
        }
        if a[1] == v[1] {
            return Ok(Side::UpperV);
        }
    }
    Err(Scope::Topology.into())
}

fn require_incidences(edges: &[BoundaryObservation; 4]) -> Checked<()> {
    for edge in edges {
        let mut successors = edges.iter().filter(|other| edge.uv[1] == other.uv[0]);
        let next = successors.next().ok_or(Scope::Topology)?;
        if successors.next().is_some() || !edge.vertices[1].is_same(&next.vertices[0]) {
            return Err(Scope::Topology.into());
        }
    }
    Ok(())
}
