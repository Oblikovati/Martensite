// SPDX-License-Identifier: GPL-2.0-only
//! Explicit cavity evidence for separate, strictly nested orthogonal box shells.
use super::{
    PartingEntity, PartingFailureKind, PartingOptions, PartingShell,
    failure::{Checked, native},
    geometry::{FaceGeometry, FaceSign, direction, plane_sign, point},
    observe_stop,
    topology::{BoundEdge, SourceTopology},
};
use martensite_tkbrep::brep_adaptor::BRepAdaptorCurve;
use martensite_tkg3d::adaptor3d::{Adaptor3dCurve, Adaptor3dSurface};
use martensite_tkmath::{
    geom_abs::{CurveType, SurfaceType},
    gp::Direction3,
    predicates::roots::metric_bounds::exact_affine_coordinate,
};

const AXES: [Direction3; 3] = [Direction3::X, Direction3::Y, Direction3::Z];

pub(super) fn classify_shells(
    topology: &SourceTopology<'_>,
    geometry: &[FaceGeometry],
    options: PartingOptions<'_>,
) -> Checked<Vec<PartingShell>> {
    let count = topology.shells.len();
    if count == 1 {
        return Ok(vec![PartingShell::Exterior]);
    }
    let mut boxes = Vec::with_capacity(count);
    for faces in &topology.shells {
        observe_stop(options)?;
        boxes.push(
            box_shell(topology, geometry, faces)?
                .ok_or(PartingFailureKind::ShellScope { shells: count })?,
        );
    }
    distinct_shell_roles(&boxes, options)
        .and_then(|roles| roles.ok_or(PartingFailureKind::ShellScope { shells: count }))
}

#[derive(Clone, Copy)]
struct BoxShell {
    limits: [[f64; 2]; 3],
    outward: bool,
}

#[derive(Clone, Copy)]
struct BoxSide {
    axis: usize,
    height: f64,
    sign: FaceSign,
}

fn box_shell(
    topology: &SourceTopology<'_>,
    geometry: &[FaceGeometry],
    faces: &[usize],
) -> Checked<Option<BoxShell>> {
    if faces.len() != 6 {
        return Ok(None);
    }
    let mut sides = Vec::with_capacity(6);
    for &index in faces {
        let entity = PartingEntity::Face(topology.faces[index].id);
        let orientation = topology.faces[index].face.orientation();
        let Some(side) = box_side(&geometry[index], orientation, entity)? else {
            return Ok(None);
        };
        sides.push(side);
    }
    let Some(bounds) = box_limits(&sides) else {
        return Ok(None);
    };
    for (&index, side) in faces.iter().zip(sides) {
        if !box_face_trim(topology, index, side, bounds.limits)? {
            return Ok(None);
        }
    }
    Ok(Some(bounds))
}

fn box_side(
    geometry: &FaceGeometry,
    orientation: martensite_tkg3d::top_abs::Orientation,
    entity: PartingEntity,
) -> Checked<Option<BoxSide>> {
    if geometry.kind != SurfaceType::Plane {
        return Ok(None);
    }
    let plane = geometry.surface.plane().map_err(native(entity))?;
    let normal = direction(plane.axis().direction());
    let Some(axis) = normal.iter().position(|component| component.abs() == 1.) else {
        return Ok(None);
    };
    if normal
        .iter()
        .enumerate()
        .any(|(i, &component)| i != axis && component != 0.)
    {
        return Ok(None);
    }
    let frame = plane.position();
    if direction(frame.x_direction())[axis] != 0. || direction(frame.y_direction())[axis] != 0. {
        return Ok(None);
    }
    let height = point(plane.location())[axis];
    let sign = plane_sign(plane, AXES[axis], orientation, entity)?;
    Ok((height.is_finite() && sign != FaceSign::Zero).then_some(BoxSide { axis, height, sign }))
}

fn box_limits(sides: &[BoxSide]) -> Option<BoxShell> {
    let mut sign = None;
    let mut limits = [[0.; 2]; 3];
    for (axis, limit) in limits.iter_mut().enumerate() {
        let mut along = sides.iter().filter(|side| side.axis == axis);
        let mut ends = [along.next()?, along.next()?];
        ends.sort_by(|a, b| a.height.total_cmp(&b.height));
        let [low, high] = ends;
        if along.next().is_some() || low.height >= high.height || low.sign == high.sign {
            return None;
        }
        let outward = low.sign == FaceSign::Negative;
        if sign.is_some_and(|earlier| earlier != outward) {
            return None;
        }
        sign = Some(outward);
        *limit = [low.height, high.height];
    }
    Some(BoxShell {
        limits,
        outward: sign?,
    })
}

fn box_face_trim(
    topology: &SourceTopology<'_>,
    face: usize,
    side: BoxSide,
    limits: [[f64; 2]; 3],
) -> Checked<bool> {
    if topology.faces[face].face.nb_children() != 1 {
        return Ok(false);
    }
    let mut rails = [false; 4];
    for edge in topology
        .edges
        .iter()
        .filter(|edge| edge.uses.iter().any(|member| member.face == face))
    {
        let Some(ends) = exact_line_ends(edge)? else {
            return Ok(false);
        };
        let Some(rail) = box_rail(ends, side, limits) else {
            return Ok(false);
        };
        if std::mem::replace(&mut rails[rail], true) {
            return Ok(false);
        }
    }
    Ok(rails.into_iter().all(|seen| seen))
}

fn exact_line_ends(edge: &BoundEdge<'_>) -> Checked<Option<[[f64; 3]; 2]>> {
    let entity = PartingEntity::Edge(edge.binding.id);
    let curve = BRepAdaptorCurve::with_edge(edge.binding.occurrence).map_err(native(entity))?;
    if curve.get_type().map_err(native(entity))? != CurveType::Line {
        return Ok(None);
    }
    let line = curve.line().map_err(native(entity))?;
    let origin = point(line.location());
    let direction = direction(line.direction());
    let range = [
        curve.first_parameter().map_err(native(entity))?,
        curve.last_parameter().map_err(native(entity))?,
    ];
    let mut ends = [[0.; 3]; 2];
    for (parameter, end) in range.into_iter().zip(&mut ends) {
        for axis in 0..3 {
            let coordinate = exact_affine_coordinate(origin[axis], direction[axis], parameter)
                .map_err(|cause| PartingFailureKind::Coordinate { entity, cause })?;
            let Some(value) = coordinate else {
                return Ok(None);
            };
            end[axis] = value;
        }
    }
    Ok(Some(ends))
}

fn box_rail(ends: [[f64; 3]; 2], side: BoxSide, limits: [[f64; 2]; 3]) -> Option<usize> {
    if ends.iter().any(|end| end[side.axis] != side.height) {
        return None;
    }
    let a = (side.axis + 1) % 3;
    let b = (side.axis + 2) % 3;
    fixed_rail(ends, limits, a, b).or_else(|| fixed_rail(ends, limits, b, a).map(|rail| rail + 2))
}

fn fixed_rail(
    [a, b]: [[f64; 3]; 2],
    limits: [[f64; 2]; 3],
    fixed: usize,
    varying: usize,
) -> Option<usize> {
    let varying_range = [a[varying].min(b[varying]), a[varying].max(b[varying])];
    if a[fixed] != b[fixed] || varying_range != limits[varying] {
        return None;
    }
    limits[fixed]
        .iter()
        .position(|&coordinate| coordinate == a[fixed])
}

fn distinct_shell_roles(
    boxes: &[BoxShell],
    options: PartingOptions<'_>,
) -> Checked<Option<Vec<PartingShell>>> {
    let Some(outer_index) = boxes.iter().position(|shell| shell.outward) else {
        return Ok(None);
    };
    let outer = boxes[outer_index];
    let mut roles = vec![PartingShell::Cavity; boxes.len()];
    roles[outer_index] = PartingShell::Exterior;
    for (index, cavity) in boxes.iter().enumerate().filter(|(i, _)| *i != outer_index) {
        observe_stop(options)?;
        if cavity.outward || !strictly_inside(*cavity, outer) {
            return Ok(None);
        }
        if boxes[..index]
            .iter()
            .enumerate()
            .any(|(other, shell)| other != outer_index && !disjoint(*cavity, *shell))
        {
            return Ok(None);
        }
    }
    Ok(Some(roles))
}

fn strictly_inside(inner: BoxShell, outer: BoxShell) -> bool {
    (0..3).all(|axis| {
        outer.limits[axis][0] < inner.limits[axis][0]
            && inner.limits[axis][1] < outer.limits[axis][1]
    })
}

fn disjoint(a: BoxShell, b: BoxShell) -> bool {
    (0..3)
        .any(|axis| a.limits[axis][1] < b.limits[axis][0] || b.limits[axis][1] < a.limits[axis][0])
}
