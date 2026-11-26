// SPDX-License-Identifier: GPL-2.0-only
//! Exact arc-centered chart and source IDs for a normally swept mixed loop.
use super::{Failure, Stage, poll};
use crate::spatial_prism::plane::vertex_id;
use martensite_tkbo::profile_extrusion::{
    ArcSense, LoopUse, ProfileCurve, ProfileCurveId, ProfilePlane, ProfileVertex, SketchProfile,
};
use martensite_tkmath::{
    gp::Direction3, predicates::roots::metric_bounds::exact_affine_coordinate,
};
use martensite_tksketch3d::{
    CurveId, PointId,
    wire::{
        LineDirection,
        mixed::{
            CardinalWireArc, DirectedCurveUse, MixedWireGeometry, MixedWireRecipe, MixedWireSegment,
        },
    },
};
use std::collections::BTreeMap;

pub(super) type Xyz = [f64; 3];
pub(super) type Uv = [f64; 2];

#[derive(Clone, Copy, Debug)]
pub(super) struct Chart {
    pub origins: [Xyz; 2],
    pub axes: [Xyz; 3],
    pub height: f64,
    axis_indices: [usize; 3],
}

#[derive(Clone, Copy, Debug)]
pub(super) struct SourcePoint {
    pub uv: Uv,
    pub xyz: [Xyz; 2],
}

#[derive(Clone, Copy, Debug)]
pub(super) struct SourceCurve {
    pub used: DirectedCurveUse,
    pub endpoints: [PointId; 2],
}

pub(super) struct Source {
    pub chart: Chart,
    pub arc: CardinalWireArc,
    pub points: BTreeMap<PointId, SourcePoint>,
    pub curves: BTreeMap<ProfileCurveId, SourceCurve>,
    pub profile: SketchProfile,
    cycle: Vec<ProfileCurveId>,
}

pub(super) fn lower(
    wire: &MixedWireRecipe,
    direction: Xyz,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Source, Failure> {
    let arc = only_arc(wire, cancelled)?;
    let chart = Chart::new(arc, direction)?;
    let mut points = BTreeMap::new();
    let mut curves = BTreeMap::new();
    let mut native_curves = Vec::with_capacity(wire.segments().len());
    for segment in wire.segments() {
        poll(cancelled, Stage::Plane)?;
        let (curve, geometry) = source_curve(segment, chart, &mut points)?;
        curves.insert(geometry.id(), curve);
        native_curves.push(geometry);
    }
    let cycle = native_curves.iter().map(ProfileCurve::id).collect();
    let arc_id = ProfileCurveId(arc.authored().recipe().id.get().get());
    canonicalize(&mut native_curves, arc_id, curves[&arc_id].used.direction);
    let profile = profile(chart, &points, native_curves);
    Ok(Source {
        chart,
        arc,
        points,
        curves,
        profile,
        cycle,
    })
}

fn canonicalize(curves: &mut [ProfileCurve], arc: ProfileCurveId, direction: LineDirection) {
    // Use the predecessor of the arc in its intrinsic direction, independent
    // of the caller's starting point or reversal. This keeps shared pair work
    // stable while retaining the literal caller cycle in the source receipt.
    if direction == LineDirection::Reversed {
        curves.reverse();
    }
    if let Some(index) = curves.iter().position(|curve| curve.id() == arc) {
        curves.rotate_left((index + curves.len() - 1) % curves.len());
    }
}

fn only_arc(
    wire: &MixedWireRecipe,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<CardinalWireArc, Failure> {
    let mut count = 0;
    let mut admitted = None;
    for segment in wire.segments() {
        poll(cancelled, Stage::Plane)?;
        if let MixedWireGeometry::Arc(arc) = segment.geometry() {
            count += 1;
            admitted = Some(*arc);
        }
    }
    match (count, admitted) {
        (1, Some(arc)) => Ok(arc),
        _ => Err(Failure::MixedArcCount(count)),
    }
}

impl Chart {
    fn new(arc: CardinalWireArc, direction: Xyz) -> Result<Self, Failure> {
        let frame = arc.authored().frame();
        let directions = [frame.x_direction(), frame.y_direction(), frame.direction()];
        let axes = directions.map(|axis| *axis.coordinates());
        let axis_indices = directions.map(cardinal_index);
        let origin = *frame.location().coordinates();
        let local = axis_indices.map(|axis| direction[axis]);
        if local[0] != 0. || local[1] != 0. || local[2] == 0. || !local[2].is_finite() {
            return Err(Failure::MixedDirection(direction));
        }
        let height = axes[2][axis_indices[2]] * local[2];
        let end = translate(origin, direction, arc.center())?;
        Ok(Self {
            origins: [origin, end],
            axes,
            height,
            axis_indices,
        })
    }

    fn source_point(self, point: PointId, xyz: Xyz) -> Result<SourcePoint, Failure> {
        let uv = self
            .local_coordinates(xyz)
            .filter(|[_, _, altitude]| *altitude == 0.)
            .ok_or_else(|| wrong_point(point, "an exact binary64 coordinate in the arc plane"))?;
        let mut direction = [0.; 3];
        let axis = self.axis_indices[2];
        direction[axis] = self.height * self.axes[2][axis];
        Ok(SourcePoint {
            uv: [uv[0], uv[1]],
            xyz: [xyz, translate(xyz, direction, point)?],
        })
    }

    fn local_coordinates(self, xyz: Xyz) -> Option<Xyz> {
        let mut local = [0.; 3];
        for (dimension, axis) in self.axis_indices.into_iter().enumerate() {
            let difference =
                exact_affine_coordinate(xyz[axis], -1., self.origins[0][axis]).ok()??;
            local[dimension] = difference * self.axes[dimension][axis];
        }
        Some(local)
    }
}

fn translate(point: Xyz, offset: Xyz, id: PointId) -> Result<Xyz, Failure> {
    let mut end = [0.; 3];
    for axis in 0..3 {
        end[axis] = exact_affine_coordinate(point[axis], 1., offset[axis])
            .ok()
            .flatten()
            .ok_or_else(|| wrong_point(id, "an exactly representable end image"))?;
    }
    Ok(end)
}

fn cardinal_index(direction: Direction3) -> usize {
    let [x, y, _] = *direction.coordinates();
    // CardinalWireArc admission proves that exactly one component has magnitude one.
    if x != 0. {
        0
    } else if y != 0. {
        1
    } else {
        2
    }
}

fn wrong_point(point: PointId, expected: &'static str) -> Failure {
    Failure::MixedPoint { point, expected }
}

fn source_curve(
    segment: &MixedWireSegment,
    chart: Chart,
    points: &mut BTreeMap<PointId, SourcePoint>,
) -> Result<(SourceCurve, ProfileCurve), Failure> {
    let used = segment.source();
    let (endpoints, xyz, id) = match segment.geometry() {
        MixedWireGeometry::Line(line) => (
            line.authored.line.endpoints,
            line.authored.points,
            ProfileCurveId(line.authored.line.id.get().get()),
        ),
        MixedWireGeometry::Arc(arc) => {
            let geometry = arc.authored();
            let [a, b, _] = geometry.points();
            (
                geometry.recipe().endpoints,
                [a, b],
                ProfileCurveId(geometry.recipe().id.get().get()),
            )
        }
    };
    for (point, xyz) in endpoints.into_iter().zip(xyz) {
        points.insert(point, chart.source_point(point, *xyz.coordinates())?);
    }
    let [start, end] = endpoints.map(vertex_id);
    let geometry = match used.curve {
        CurveId::Arc(_) => ProfileCurve::Arc {
            id,
            start,
            end,
            center: [0.; 2],
            sense: ArcSense::CounterClockwise,
        },
        _ => ProfileCurve::Line { id, start, end },
    };
    Ok((SourceCurve { used, endpoints }, geometry))
}

fn profile(
    chart: Chart,
    points: &BTreeMap<PointId, SourcePoint>,
    curves: Vec<ProfileCurve>,
) -> SketchProfile {
    let plane = ProfilePlane {
        origin: chart.origins[0],
        x_direction: chart.axes[0],
        normal: chart.axes[2],
    };
    let vertices = points
        .iter()
        .map(|(&point, position)| ProfileVertex {
            id: vertex_id(point),
            point: position.uv,
        })
        .collect();
    SketchProfile {
        plane,
        vertices,
        curves,
    }
}

pub(super) fn loop_matches(
    actual: &[LoopUse],
    source: &Source,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<bool, Failure> {
    let expected = &source.cycle;
    if actual.len() != expected.len() || actual.is_empty() {
        return Ok(false);
    }
    let Some(start) = actual.iter().position(|item| item.curve == expected[0]) else {
        return Ok(false);
    };
    let reversed = actual[start].reversed ^ used_reversed(source, expected[0]);
    for (offset, &curve) in expected.iter().enumerate() {
        poll(cancelled, Stage::Binding)?;
        let index = loop_index(start, offset, actual.len(), reversed);
        let item = actual[index];
        if item.curve != curve || (item.reversed ^ reversed) != used_reversed(source, curve) {
            return Ok(false);
        }
    }
    Ok(true)
}

fn loop_index(start: usize, offset: usize, len: usize, reversed: bool) -> usize {
    (start + if reversed { len - offset } else { offset }) % len
}

fn used_reversed(source: &Source, curve: ProfileCurveId) -> bool {
    source.curves[&curve].used.direction == LineDirection::Reversed
}
