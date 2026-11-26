// SPDX-License-Identifier: GPL-2.0-only
//! Bind one exact periodic UV rectangle to its stored seam, rims and vertex identities.
mod hole;
pub(in crate::brep) mod sphere;

use super::{CylindricalFaceError as Error, numeric_failure, poll};
use crate::brep::{
    face_input_capture::{
        AnalyticCell, AnalyticFaceInput, CapturedFaceCell as Cell, CapturedOccurrence,
    },
    planar_attachment::{
        FaceAttachmentError,
        capture::{Captured, local, reversed},
    },
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::{
    gp::{Axis2d, Circle3, Cylinder3, Line3},
    predicates::roots::metric_bounds::{
        exact_affine_coordinate,
        segment_cylinder::{
            CylinderWallTraceInput, SegmentCylinderWall, certify_cylinder_wall_trace,
        },
    },
};
use std::f64::consts::TAU;

type Checked<T> = Result<T, Error>;

pub(super) fn prove(
    input: &AnalyticFaceInput,
    limit: f64,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<(SegmentCylinderWall, f64)> {
    let (wall, hole) = Wall::capture(input, cancelled)?;
    let proof = SegmentCylinderWall::capture(wall.cylinder, wall.placement, wall.axial)
        .map_err(numeric_failure)?;
    let upper_bound =
        certify_cylinder_wall_trace(wall, &mut *cancelled).map_err(numeric_failure)?;
    let (proof, upper_bound) = match hole {
        Some(hole) => hole.bind(wall, proof, upper_bound, cancelled)?,
        None => (proof, upper_bound),
    };
    if upper_bound > limit {
        return Err(Error::TraceError { upper_bound, limit });
    }
    Ok((proof, upper_bound))
}

struct Wall<'a> {
    values: Captured<'a>,
    carriers: &'a [(usize, AnalyticCell)],
    surface: usize,
}

impl<'a> Wall<'a> {
    fn capture(
        input: &'a AnalyticFaceInput,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Checked<(CylinderWallTraceInput, Option<hole::Hole>)> {
        let root = input.common.root();
        reversed(root).map_err(Error::Source)?;
        let values = Captured::index(&input.common);
        let surface = values.surface(root.owner).map_err(Error::Source)?;
        let wires = values
            .children(root.owner, ShapeEnum::Face)
            .map_err(Error::Source)?;
        if !(1..=2).contains(&wires.len()) {
            return Err(required(
                root.owner,
                "one full-period wire and at most one rectangular hole",
            ));
        }
        let trimmed = Self {
            values,
            carriers: &input.carriers,
            surface,
        };
        let cylinder = trimmed.cylinder()?;
        let placement = placement(root, cancelled)?;
        let (outer, hole) = trimmed.wall_wires(wires, cancelled)?;
        let (seam, rims) = outer.outer()?;
        let wall = trimmed.bind(cylinder, placement, seam, rims)?;
        let hole = hole
            .map(|wire| hole::Hole::capture(wire, &trimmed.values, seam[0].vertices))
            .transpose()?;
        Ok((wall, hole))
    }

    fn wall_wires(
        &self,
        wires: &[CapturedOccurrence],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Checked<(RawWire, Option<RawWire>)> {
        let first = self.wire(&wires[0], false, cancelled)?;
        let Some(second) = wires.get(1) else {
            return Ok((first, None));
        };
        let second = self.wire(second, false, cancelled)?;
        match [first.has_seam(), second.has_seam()] {
            [true, false] => Ok((first, Some(second))),
            [false, true] => Ok((second, Some(first))),
            _ => Err(required(
                first.owner,
                "one native outer paired seam and one hole wire",
            )),
        }
    }

    fn cylinder(&self) -> Checked<Cylinder3> {
        match self.analytic(self.surface) {
            Some(AnalyticCell::Cylinder(axis, radius)) => Cylinder3::new(*axis, *radius)
                .map_err(|_| required(self.surface, "a positive stored cylinder")),
            _ => Err(required(self.surface, "an original cylinder surface")),
        }
    }

    fn wire(
        &self,
        wire: &CapturedOccurrence,
        curved_seam: bool,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Checked<RawWire> {
        local(wire).map_err(Error::Source)?;
        let reverse = reversed(wire).map_err(Error::Source)?;
        let edges = self
            .values
            .children(wire.owner, ShapeEnum::Wire)
            .map_err(Error::Source)?;
        if edges.len() != 4 {
            return Err(required(wire.owner, "four native rectangular coedges"));
        }
        let mut uses = Vec::with_capacity(4);
        for edge in edges {
            poll(cancelled)?;
            uses.push(self.edge_use(edge, reverse, curved_seam)?);
        }
        Ok(RawWire {
            owner: wire.owner,
            uses,
        })
    }

    fn edge_use(
        &self,
        edge: &CapturedOccurrence,
        wire_reverse: bool,
        curved_seam: bool,
    ) -> Checked<RawUse> {
        local(edge).map_err(Error::Source)?;
        let reversed = wire_reverse ^ reversed(edge).map_err(Error::Source)?;
        let vertices = self
            .values
            .directed_vertices(edge.owner, false)
            .map_err(Error::Source)?;
        let (spatial, parametric) = self.representations(edge.owner)?;
        let curve =
            if let Some(AnalyticCell::SpatialCircle(axis, radius)) = self.analytic(spatial.curve) {
                if *radius <= 0. {
                    return Err(required(edge.owner, "a circular rim with positive radius"));
                }
                let circle = Circle3::new(*axis, *radius)
                    .map_err(|_| required(edge.owner, "a stored circular carrier"))?;
                if curved_seam && parametric.second.is_some() {
                    Curve::CurvedSeam(circle)
                } else {
                    Curve::Rim(circle)
                }
            } else {
                let [Cell::Line(axis)] = self.values.cells(spatial.curve).map_err(Error::Source)?
                else {
                    return Err(required(
                        edge.owner,
                        "a stored circular rim or spatial seam line",
                    ));
                };
                Curve::Seam(Line3::from_axis(*axis))
            };
        let (uv, range) = self
            .values
            .parametric(parametric.for_direction(reversed), parametric.owner)
            .map_err(Error::Source)?;
        if range != self.values.range(spatial.owner).map_err(Error::Source)? {
            return Err(required(
                edge.owner,
                "one actual parameter interval for the spatial and chart traces",
            ));
        }
        Ok(RawUse {
            owner: edge.owner,
            vertices,
            reversed,
            curve,
            uv,
            range,
            paired: parametric.second.is_some(),
        })
    }

    fn representations(&self, edge: usize) -> Checked<(StoredCurve, Pcurve)> {
        let reps = self
            .values
            .edge_representations(edge)
            .map_err(Error::Source)?;
        let mut spatial = None;
        let mut chart = None;
        for &owner in reps {
            let cells = self.values.cells(owner).map_err(Error::Source)?;
            if let Some(curve) = spatial_curve(cells, owner)? {
                unique(&mut spatial, StoredCurve { owner, curve }, owner)?;
            }
            if let Some(curve) = selected_pcurve(cells, owner, self.surface) {
                unique(&mut chart, curve, owner)?;
            }
        }
        match (spatial, chart) {
            (Some(a), Some(b)) => Ok((a, b)),
            _ => Err(required(
                edge,
                "one spatial and one selected-support representation",
            )),
        }
    }

    fn analytic(&self, owner: usize) -> Option<&AnalyticCell> {
        self.carriers
            .iter()
            .find_map(|(id, carrier)| (*id == owner).then_some(carrier))
    }

    fn bind(
        &self,
        cylinder: Cylinder3,
        placement: [[f64; 4]; 3],
        [start, end]: [Use; 2],
        [low, high]: [Use; 2],
    ) -> Checked<CylinderWallTraceInput> {
        let [Curve::Seam(generator), Curve::Seam(_)] = [start.curve, end.curve] else {
            return Err(required(start.owner, "two line coedges of the shared seam"));
        };
        let [Curve::Rim(a), Curve::Rim(b)] = [low.curve, high.curve] else {
            return Err(required(low.owner, "two complete circular rims"));
        };
        let [[_, v0], [_, v1]] = end.points;
        let [first, last] = start.vertices;
        Ok(CylinderWallTraceInput {
            cylinder,
            placement,
            axial: [v0, v1],
            rims: [a, b],
            generator,
            generator_range: start.range,
            anchors: [
                self.values.vertex(first).map_err(Error::Source)?,
                self.values.vertex(last).map_err(Error::Source)?,
            ],
        })
    }
}

fn placement(
    root: &CapturedOccurrence,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<[[f64; 4]; 3]> {
    let error = |cause| Error::Source(FaceAttachmentError::Placement(cause));
    for (_, _, datum) in root.location.steps() {
        poll(cancelled)?;
        datum.checked_unit_placement_rows().map_err(error)?;
    }
    root.location
        .effective_transform()
        .checked_unit_placement_rows()
        .map_err(error)
}

struct StoredCurve {
    owner: usize,
    curve: usize,
}

struct Pcurve {
    owner: usize,
    first: usize,
    second: Option<usize>,
}

impl Pcurve {
    fn for_direction(&self, reversed: bool) -> usize {
        if reversed {
            self.second.unwrap_or(self.first)
        } else {
            self.first
        }
    }
}

fn spatial_curve(cells: &[&Cell], owner: usize) -> Checked<Option<usize>> {
    for cell in cells {
        match cell {
            Cell::Spatial { curve } => return Ok(Some(*curve)),
            Cell::AbsentSpatial => return Err(required(owner, "a retained spatial curve")),
            _ => {}
        }
    }
    Ok(None)
}

fn selected_pcurve(cells: &[&Cell], owner: usize, support: usize) -> Option<Pcurve> {
    let first = cells.iter().find_map(|cell| match cell {
        Cell::Parametric { surface, curve, .. } if *surface == support => Some(*curve),
        _ => None,
    })?;
    let second = cells.iter().find_map(|cell| match cell {
        Cell::Seam { curve, .. } => Some(*curve),
        _ => None,
    });
    Some(Pcurve {
        owner,
        first,
        second,
    })
}

fn unique<T>(slot: &mut Option<T>, value: T, owner: usize) -> Checked<()> {
    if slot.replace(value).is_some() {
        return Err(required(owner, "one representation for this curve role"));
    }
    Ok(())
}

struct RawWire {
    owner: usize,
    uses: Vec<RawUse>,
}

impl RawWire {
    fn has_seam(&self) -> bool {
        self.uses.iter().any(|coedge| coedge.paired)
    }

    fn outer(self) -> Checked<([Use; 2], [Use; 2])> {
        let uses = self
            .uses
            .into_iter()
            .map(Use::capture)
            .collect::<Checked<Vec<_>>>()?;
        classify_uses(uses, self.owner)
    }
}

struct RawUse {
    owner: usize,
    vertices: [usize; 2],
    reversed: bool,
    curve: Curve,
    uv: Axis2d,
    range: [f64; 2],
    paired: bool,
}

#[derive(Clone, Copy)]
enum Curve {
    Rim(Circle3),
    Seam(Line3),
    CurvedSeam(Circle3),
}

#[derive(Clone, Copy)]
struct Use {
    owner: usize,
    vertices: [usize; 2],
    curve: Curve,
    range: [f64; 2],
    points: [[f64; 2]; 2],
    offset: f64,
}

impl Use {
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
        let [a, b] = [uv.location().x(), uv.location().y()];
        let direction = [uv.direction().x(), uv.direction().y()];
        let points = match curve {
            Curve::Rim(_)
                if !paired
                    && vertices[0] == vertices[1]
                    && a == 0.
                    && b.is_finite()
                    && direction == [1., 0.]
                    && range == [0., TAU] =>
            {
                [[0., b], [TAU, b]]
            }
            Curve::Seam(_) | Curve::CurvedSeam(_)
                if paired
                    && vertices[0] != vertices[1]
                    && direction == [0., 1.]
                    && (a == 0. || a == TAU) =>
            {
                seam_points(a, b, range)
                    .ok_or_else(|| required(owner, "an exact affine seam interval"))?
            }
            _ => {
                return Err(required(
                    owner,
                    "a directed full-circle rim or one branch of a stored paired seam",
                ));
            }
        };
        let points = if reversed {
            [points[1], points[0]]
        } else {
            points
        };
        Ok(Self {
            owner,
            vertices,
            curve,
            range,
            points,
            offset: b,
        })
    }
}

fn seam_points(u: f64, offset: f64, [low, high]: [f64; 2]) -> Option<[[f64; 2]; 2]> {
    let first = exact_affine_coordinate(offset, 1., low).ok()??;
    let last = exact_affine_coordinate(offset, 1., high).ok()??;
    (first < last).then_some([[u, first], [u, last]])
}

fn classify_uses(uses: Vec<Use>, wire: usize) -> Checked<([Use; 2], [Use; 2])> {
    let find = |predicate: fn(Use) -> bool| unique_use(&uses, predicate, wire);
    let start = find(|edge| {
        matches!(edge.curve, Curve::Seam(_) | Curve::CurvedSeam(_)) && edge.points[0][0] == 0.
    })?;
    let end = find(|edge| {
        matches!(edge.curve, Curve::Seam(_) | Curve::CurvedSeam(_)) && edge.points[0][0] == TAU
    })?;
    let low = find(|edge| matches!(edge.curve, Curve::Rim(_)) && edge.points[0][0] == 0.)?;
    let high = find(|edge| matches!(edge.curve, Curve::Rim(_)) && edge.points[0][0] == TAU)?;
    verify_rectangle(start, end, low, high, wire)?;
    Ok(([start, end], [low, high]))
}

fn unique_use(uses: &[Use], predicate: fn(Use) -> bool, wire: usize) -> Checked<Use> {
    let mut matching = uses.iter().copied().filter(|&item| predicate(item));
    let found = matching.next().filter(|_| matching.next().is_none());
    found.ok_or_else(|| required(wire, "one coedge at each directed rectangular side"))
}

fn verify_rectangle(start: Use, end: Use, low: Use, high: Use, wire: usize) -> Checked<()> {
    let [[_, v0], [_, v1]] = end.points;
    let [first, last] = start.vertices;
    let paired =
        start.owner == end.owner && start.vertices == end.vertices && start.range == end.range;
    let closed =
        low.owner != high.owner && low.vertices == [first; 2] && high.vertices == [last; 2];
    let corners = start.points == [[0., v1], [0., v0]]
        && low.points == [[0., v0], [TAU, v0]]
        && high.points == [[TAU, v1], [0., v1]]
        && v0 < v1;
    if !(paired && closed && corners) {
        return Err(required(
            wire,
            "a paired seam and two rims joined to exact directed UV corners and shared vertices",
        ));
    }
    Ok(())
}

fn required(owner: usize, required: &'static str) -> Error {
    Error::UnsupportedTrim { owner, required }
}
