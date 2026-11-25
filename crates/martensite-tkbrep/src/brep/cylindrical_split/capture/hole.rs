// SPDX-License-Identifier: GPL-2.0-only
//! Bind a clockwise exact rectangle to original arc/generator and vertex owners.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::segment_cylinder::{
    CylinderHoleCarrier, CylinderHoleTraceEdge, certify_cylinder_hole_trace,
};

pub(super) struct Hole {
    angular: [f64; 2],
    axial: [f64; 2],
    edges: [CylinderHoleTraceEdge; 4],
}

impl Hole {
    pub(super) fn capture(
        wire: RawWire,
        values: &Captured<'_>,
        outer_vertices: [usize; 2],
    ) -> Checked<Self> {
        let owner = wire.owner;
        let uses = wire
            .uses
            .into_iter()
            .map(HoleUse::capture)
            .collect::<Checked<Vec<_>>>()?;
        let [left, high, right, low] = directed_sides(&uses, owner)?;
        let [u0, u1] = low.trace.range;
        let [v0, v1] = left.trace.range;
        let expected = [[u0, v0], [u0, v1], [u1, v1], [u1, v0]];
        let ordered = [left, high, right, low];
        verify_cycle(ordered, expected, outer_vertices, owner)?;
        let edges = ordered.map(|coedge| coedge.with_vertices(values));
        let [a, b, c, d] = edges;
        Ok(Self {
            angular: [u0, u1],
            axial: [v0, v1],
            edges: [a?, b?, c?, d?],
        })
    }

    pub(super) fn bind(
        self,
        wall: CylinderWallTraceInput,
        proof: SegmentCylinderWall,
        outer_trace: f64,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Checked<(SegmentCylinderWall, f64)> {
        let proof = proof
            .with_rectangular_hole(self.angular, self.axial, &mut *cancelled)
            .map_err(numeric_failure)?;
        let trace =
            certify_cylinder_hole_trace(wall, self.edges, cancelled).map_err(numeric_failure)?;
        Ok((proof, outer_trace.max(trace)))
    }
}

#[derive(Clone, Copy)]
struct HoleUse {
    owner: usize,
    reverse: bool,
    ordered_vertices: [usize; 2],
    points: [[f64; 2]; 2],
    trace: CylinderHoleTraceEdge,
}

impl HoleUse {
    fn capture(raw: RawUse) -> Checked<Self> {
        let RawUse {
            owner,
            vertices,
            reversed,
            curve,
            uv,
            range,
            paired,
        } = raw;
        let [u, v] = [uv.location().x(), uv.location().y()];
        let direction = [uv.direction().x(), uv.direction().y()];
        let [low, high] = range;
        let invalid = || {
            required(
                owner,
                "a finite hole coedge with matched original chart and spatial parameter",
            )
        };
        if paired
            || vertices[0] == vertices[1]
            || !(low.is_finite() && high.is_finite() && low < high)
        {
            return Err(invalid());
        }
        let (carrier, fixed, points) = match curve {
            Curve::Rim(circle) if v.is_finite() && u == 0. && direction == [1., 0.] => {
                (CylinderHoleCarrier::Arc(circle), v, [[low, v], [high, v]])
            }
            Curve::Seam(line) if u.is_finite() && v == 0. && direction == [0., 1.] => (
                CylinderHoleCarrier::Generator(line),
                u,
                [[u, low], [u, high]],
            ),
            _ => return Err(invalid()),
        };
        let directed = |[a, b]: [[f64; 2]; 2]| if reversed { [b, a] } else { [a, b] };
        let ordered_vertices = if reversed {
            [vertices[1], vertices[0]]
        } else {
            vertices
        };
        let trace = CylinderHoleTraceEdge {
            carrier,
            range,
            fixed,
            vertices: [[0.; 3]; 2],
        };
        Ok(Self {
            owner,
            reverse: reversed,
            ordered_vertices,
            points: directed(points),
            trace,
        })
    }

    fn with_vertices(self, values: &Captured<'_>) -> Checked<CylinderHoleTraceEdge> {
        let [a, b] = self.ordered_vertices;
        let [first, last] = if self.reverse { [b, a] } else { [a, b] };
        let vertices = [
            values.vertex(first).map_err(Error::Source)?,
            values.vertex(last).map_err(Error::Source)?,
        ];
        if vertices[0] == vertices[1] {
            return Err(required(
                self.owner,
                "distinct actual hole coedge endpoint positions",
            ));
        }
        Ok(CylinderHoleTraceEdge {
            vertices,
            ..self.trace
        })
    }
}

fn directed_sides(uses: &[HoleUse], wire: usize) -> Checked<[HoleUse; 4]> {
    let find = |arc, reverse| {
        let mut matching = uses.iter().copied().filter(|coedge| {
            matches!(coedge.trace.carrier, CylinderHoleCarrier::Arc(_)) == arc
                && coedge.reverse == reverse
        });
        matching
            .next()
            .filter(|_| matching.next().is_none())
            .ok_or_else(|| {
                required(
                    wire,
                    "one directed coedge at each clockwise rectangular hole side",
                )
            })
    };
    Ok([
        find(false, false)?,
        find(true, false)?,
        find(false, true)?,
        find(true, true)?,
    ])
}

fn verify_cycle(
    sides: [HoleUse; 4],
    corners: [[f64; 2]; 4],
    outer_vertices: [usize; 2],
    owner: usize,
) -> Checked<()> {
    for (i, side) in sides.iter().enumerate() {
        let j = (i + 1) % sides.len();
        let duplicate = sides[..i].iter().any(|earlier| {
            earlier.owner == side.owner || earlier.ordered_vertices[0] == side.ordered_vertices[0]
        });
        if duplicate
            || outer_vertices.contains(&side.ordered_vertices[0])
            || side.points != [corners[i], corners[j]]
            || side.ordered_vertices[1] != sides[j].ordered_vertices[0]
        {
            return Err(required(
                owner,
                "four original coedges and vertices with exact clockwise UV corner joins",
            ));
        }
    }
    Ok(())
}
