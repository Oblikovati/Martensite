// SPDX-License-Identifier: GPL-2.0-only
//! Establish the plane with exact source XYZ before computing any UV coordinates.
use super::{SpatialPrismFailure as Failure, SpatialPrismStage as Stage, poll};
use martensite_tkbo::profile_extrusion::{
    ProfileCurve, ProfileCurveId, ProfilePlane, ProfileVertex, ProfileVertexId, SketchProfile,
};
use martensite_tkmath::predicates::{
    OrientationSign, compare_plane_direction_3d, compare_tetrahedron_orientation_3d, orient2d,
};
use martensite_tksketch3d::{
    PointId,
    wire::{WireRecipe, WireSegment},
};
use std::cmp::Ordering;

pub(crate) type Vertex = (PointId, [f64; 3]);

pub(crate) struct AuthoredPlane {
    pub points: Vec<Vertex>,
    pub anchors: [Vertex; 3],
}

pub(crate) fn certify_authored_plane(
    wire: &WireRecipe,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<AuthoredPlane, Failure> {
    let points = vertices(wire, cancelled)?;
    let anchors = find_anchors(&points, cancelled)?;
    prove_points(anchors, &points, cancelled)?;
    Ok(AuthoredPlane { points, anchors })
}

pub(super) struct Lowered {
    pub profile: SketchProfile,
    pub anchors: [PointId; 3],
    pub sweep_sign: Ordering,
}

pub(super) fn lower(
    wire: &WireRecipe,
    direction: [f64; 3],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Lowered, Failure> {
    let AuthoredPlane { points, anchors } = certify_authored_plane(wire, cancelled)?;
    let sweep_sign = direction_sign(anchors, direction)?;
    let profile = profile(wire, &points, anchors, cancelled)?;
    Ok(Lowered {
        profile,
        anchors: anchors.map(|anchor| anchor.0),
        sweep_sign,
    })
}

fn vertices(
    wire: &WireRecipe,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<Vertex>, Failure> {
    wire.segments()
        .iter()
        .map(|segment| {
            poll(cancelled, Stage::Plane)?;
            let start = segment.endpoints()[0];
            let index = usize::from(start != segment.authored.line.endpoints[0]);
            Ok((start, *segment.authored.points[index].coordinates()))
        })
        .collect()
}

fn find_anchors(
    points: &[Vertex],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<[Vertex; 3], Failure> {
    let [first, second, remaining @ ..] = points else {
        return Err(Failure::DegeneratePlane);
    };
    for &candidate in remaining {
        poll(cancelled, Stage::Plane)?;
        let triangle = [*first, *second, candidate];
        if noncollinear(triangle)? {
            return Ok(triangle);
        }
    }
    Err(Failure::DegeneratePlane)
}

fn noncollinear(vertices: [Vertex; 3]) -> Result<bool, Failure> {
    for [x, y] in [[0, 1], [1, 2], [2, 0]] {
        let [a, b, c] = vertices.map(|(_, xyz)| [xyz[x], xyz[y]]);
        let orientation = orient2d(a, b, c).map_err(|source| Failure::Predicate {
            point: vertices.get(source.point).map(|vertex| vertex.0),
            source,
        })?;
        if orientation != OrientationSign::Zero {
            return Ok(true);
        }
    }
    Ok(false)
}

fn prove_points(
    anchors: [Vertex; 3],
    points: &[Vertex],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), Failure> {
    let [a, b, c] = anchors.map(|anchor| anchor.1);
    for &(point, xyz) in points {
        poll(cancelled, Stage::Plane)?;
        let sign = compare_tetrahedron_orientation_3d([a, b, c, xyz]).map_err(|source| {
            Failure::Predicate {
                point: Some(point),
                source,
            }
        })?;
        if sign != Ordering::Equal {
            return Err(Failure::Skew {
                anchors: anchors.map(|anchor| anchor.0),
                point,
                sign,
            });
        }
    }
    Ok(())
}

fn direction_sign(anchors: [Vertex; 3], direction: [f64; 3]) -> Result<Ordering, Failure> {
    let sign = compare_plane_direction_3d(anchors.map(|anchor| anchor.1), direction).map_err(
        |source| Failure::Predicate {
            point: None,
            source,
        },
    )?;
    if sign == Ordering::Equal {
        return Err(Failure::TangentDirection(direction));
    }
    Ok(sign)
}

fn profile(
    wire: &WireRecipe,
    points: &[Vertex],
    anchors: [Vertex; 3],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<SketchProfile, Failure> {
    let chart = Chart::through(anchors).ok_or_else(|| chart_failure(anchors))?;
    let [a, b, c] = anchors.map(|anchor| chart.project(anchor.1));
    if orient2d(a, b, c) != Ok(OrientationSign::Positive) {
        return Err(chart_failure(anchors));
    }
    let vertices = points
        .iter()
        .map(|&(id, xyz)| {
            poll(cancelled, Stage::Plane)?;
            let point = chart.project(xyz);
            if point.iter().any(|component| !component.is_finite()) {
                return Err(chart_failure(anchors));
            }
            Ok(ProfileVertex {
                id: vertex_id(id),
                point,
            })
        })
        .collect::<Result<_, _>>()?;
    let curves = wire
        .segments()
        .iter()
        .map(|segment| {
            poll(cancelled, Stage::Plane)?;
            Ok(profile_line(segment))
        })
        .collect::<Result<_, Failure>>()?;
    Ok(SketchProfile {
        plane: chart.plane,
        vertices,
        curves,
    })
}

fn chart_failure(anchors: [Vertex; 3]) -> Failure {
    Failure::UnboundChart(anchors.map(|anchor| anchor.0))
}

pub(super) fn vertex_id(id: PointId) -> ProfileVertexId {
    ProfileVertexId(id.get().get())
}

fn profile_line(segment: &WireSegment) -> ProfileCurve {
    let [start, end] = segment.authored.line.endpoints.map(vertex_id);
    ProfileCurve::Line {
        id: ProfileCurveId(segment.source.line.get().get()),
        start,
        end,
    }
}

struct Chart {
    plane: ProfilePlane,
    y: [f64; 3],
}

impl Chart {
    fn through([a, b, c]: [Vertex; 3]) -> Option<Self> {
        let x = unit(difference(b.1, a.1))?;
        let diagonal = unit(difference(c.1, a.1))?;
        let normal = unit(cross(x, diagonal))?;
        let y = unit(cross(normal, x))?;
        Some(Self {
            plane: ProfilePlane {
                origin: a.1,
                x_direction: x,
                normal,
            },
            y,
        })
    }

    fn project(&self, xyz: [f64; 3]) -> [f64; 2] {
        let delta = difference(xyz, self.plane.origin);
        [dot(delta, self.plane.x_direction), dot(delta, self.y)]
    }
}

fn difference(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|axis| a[axis] - b[axis])
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|axis| {
        let i = (axis + 1) % 3;
        let j = (axis + 2) % 3;
        a[i].mul_add(b[j], -(a[j] * b[i]))
    })
}

fn unit(vector: [f64; 3]) -> Option<[f64; 3]> {
    let scale = vector.into_iter().map(f64::abs).fold(0.0, f64::max);
    if !scale.is_finite() || scale == 0.0 {
        return None;
    }
    let scaled = vector.map(|value| value / scale);
    let length = dot(scaled, scaled).sqrt();
    (length.is_finite() && length > 0.0).then(|| scaled.map(|value| value / length))
}
