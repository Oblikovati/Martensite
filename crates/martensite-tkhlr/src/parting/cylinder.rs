// SPDX-License-Identifier: GPL-2.0-only
//! Full cylindrical trim admission and native Contap generators.
use super::{
    PartingCarrier, PartingCurve, PartingCylinderProblem as CylinderProblem, PartingEntity,
    PartingFaceUse, PartingFailureKind, PartingOrigin,
    failure::{Checked, native},
    geometry::{FaceSign, checked_curve, parallel, plane_sign},
};
use crate::contap::ContapContAna;
use martensite_tkbrep::{
    brep::BRepTool,
    brep_adaptor::{BRepAdaptorCurve, BRepAdaptorSurface},
    topods::{self, Edge, Face, ShapeIterator},
};
use martensite_tkg2d::geom2d::Geom2dLine;
use martensite_tkg3d::{
    adaptor3d::{Adaptor3dCurve, Adaptor3dSurface},
    top_abs::Orientation,
};
use martensite_tkmath::{
    elslib,
    geom_abs::CurveType,
    gp::{Cylinder3, Direction3, Line3, Plane3},
    predicates::roots::metric_bounds::exact_affine_coordinate,
};
use std::f64::consts::{FRAC_PI_2, TAU};

pub(super) fn evaluate(
    face: &Face,
    trace: PartingFaceUse,
    pull: Direction3,
) -> Checked<(FaceSign, Vec<PartingCurve>)> {
    let entity = PartingEntity::Face(trace.id);
    let face = Face::unchecked(face.oriented(Orientation::Forward));
    let surface = BRepAdaptorSurface::with_face(&face, true).map_err(native(entity))?;
    let cylinder = surface.cylinder().map_err(native(entity))?;
    let bounds = wall_bounds(&surface, trace, cylinder.radius())?;
    check_rectangle(&face, trace, bounds)?;
    let axis = cylinder.axis().direction();
    let plane = Plane3::from_position(cylinder.position());
    if plane_sign(plane, axis, Orientation::Forward, entity)? == FaceSign::Zero {
        return Err(problem(trace, CylinderProblem::Frame));
    }
    if parallel(axis, pull, entity)? {
        return Ok((FaceSign::Zero, Vec::new()));
    }
    if plane_sign(plane, pull, Orientation::Forward, entity)? != FaceSign::Zero {
        return Err(problem(trace, CylinderProblem::SkewPull));
    }
    let curves = generators(cylinder, trace, pull, bounds)?;
    Ok((FaceSign::Varying, curves))
}

fn wall_bounds(
    surface: &BRepAdaptorSurface,
    trace: PartingFaceUse,
    radius: f64,
) -> Checked<[f64; 4]> {
    let entity = PartingEntity::Face(trace.id);
    let bounds = [
        surface.first_u_parameter().map_err(native(entity))?,
        surface.last_u_parameter().map_err(native(entity))?,
        surface.first_v_parameter().map_err(native(entity))?,
        surface.last_v_parameter().map_err(native(entity))?,
    ];
    let [u0, u1, v0, v1] = bounds;
    if u0 == 0.
        && u1 == TAU
        && v0.is_finite()
        && v1.is_finite()
        && v0 < v1
        && radius.is_finite()
        && radius > 0.
    {
        return Ok(bounds);
    }
    Err(problem(trace, CylinderProblem::Bounds(bounds)))
}

fn check_rectangle(face: &Face, trace: PartingFaceUse, bounds: [f64; 4]) -> Checked<()> {
    let entity = PartingEntity::Face(trace.id);
    let mut wires = ShapeIterator::new(face, true, true);
    let wire = Iterator::next(&mut wires).ok_or_else(|| problem(trace, CylinderProblem::Trim))?;
    if Iterator::next(&mut wires).is_some() || wire.nb_children() != 4 {
        return Err(problem(trace, CylinderProblem::Trim));
    }
    let mut sides: [Option<Edge>; 4] = [None, None, None, None];
    for shape in ShapeIterator::new(&wire, true, true) {
        let edge = topods::edge(&shape).map_err(native(entity))?;
        let side = chart_side(&edge, face, trace, bounds)?;
        let curve = BRepAdaptorCurve::with_edge(&edge).map_err(native(entity))?;
        let expected = if side < 2 {
            CurveType::Line
        } else {
            CurveType::Circle
        };
        if curve.get_type().map_err(native(entity))? != expected
            || sides[side].replace(edge).is_some()
        {
            return Err(problem(trace, CylinderProblem::Trim));
        }
    }
    let [Some(seam0), Some(seam1), Some(rim0), Some(rim1)] = sides else {
        return Err(problem(trace, CylinderProblem::Trim));
    };
    if !seam0.is_same(&seam1) || seam0.orientation() == seam1.orientation() || rim0.is_same(&rim1) {
        return Err(problem(trace, CylinderProblem::Trim));
    }
    Ok(())
}

fn chart_side(
    edge: &Edge,
    face: &Face,
    trace: PartingFaceUse,
    [u0, u1, v0, v1]: [f64; 4],
) -> Checked<usize> {
    let entity = PartingEntity::Face(trace.id);
    let curve = BRepTool::curve_on_face(edge, face).map_err(native(entity))?;
    let carrier = curve.pcurve.cast::<Geom2dLine>();
    let line = carrier
        .get()
        .filter(|_| curve.is_stored)
        .ok_or_else(|| problem(trace, CylinderProblem::Trim))?;
    let ends = affine_ends(line, [curve.first, curve.last], trace)?;
    let p = [[u0, v0], [u0, v1], [u1, v0], [u1, v1]];
    [[p[0], p[1]], [p[2], p[3]], [p[0], p[2]], [p[1], p[3]]]
        .into_iter()
        .position(|[a, b]| ends == [a, b] || ends == [b, a])
        .ok_or_else(|| problem(trace, CylinderProblem::Trim))
}

fn affine_ends(
    line: &Geom2dLine,
    parameters: [f64; 2],
    trace: PartingFaceUse,
) -> Checked<[[f64; 2]; 2]> {
    let start = line.location();
    let direction = line.direction();
    let entity = PartingEntity::Face(trace.id);
    let at = |origin, component, parameter| {
        exact_affine_coordinate(origin, component, parameter)
            .map_err(|cause| PartingFailureKind::Coordinate { entity, cause })?
            .ok_or_else(|| problem(trace, CylinderProblem::Trim))
    };
    let endpoint = |t| {
        Ok([
            at(start.x(), direction.x(), t)?,
            at(start.y(), direction.y(), t)?,
        ])
    };
    Ok([endpoint(parameters[0])?, endpoint(parameters[1])?])
}

fn generators(
    cylinder: Cylinder3,
    trace: PartingFaceUse,
    pull: Direction3,
    bounds: [f64; 4],
) -> Checked<Vec<PartingCurve>> {
    let entity = PartingEntity::Face(trace.id);
    let mut contour = ContapContAna::new();
    contour
        .perform_cylinder(&cylinder, pull)
        .map_err(native(entity))?;
    if !contour.is_done()
        || contour.nb_contours().map_err(native(entity))? != 2
        || contour.type_contour().map_err(native(entity))? != CurveType::Line
    {
        return Err(problem(trace, CylinderProblem::Contour));
    }
    let make = |index| {
        generator(
            contour.line(index).map_err(native(entity))?,
            cylinder,
            trace,
            bounds,
        )
    };
    order_generators([make(1)?, make(2)?], trace)
}

fn generator(
    line: Line3,
    cylinder: Cylinder3,
    trace: PartingFaceUse,
    [u0, u1, v0, v1]: [f64; 4],
) -> Checked<(f64, PartingCurve)> {
    let entity = PartingEntity::Face(trace.id);
    let (u, _) =
        elslib::cylinder_parameters(cylinder.position(), cylinder.radius(), line.location());
    if !(u0..=u1).contains(&u) || line.direction() != cylinder.axis().direction() {
        return Err(problem(trace, CylinderProblem::Contour));
    }
    // Contap's origin is on the cylinder's V=0 section, with the same axial
    // parameter direction. The proved rectangular trim supplies both endpoints.
    let u = u.rem_euclid(TAU);
    let origin = PartingOrigin::Smooth {
        face: trace,
        uv_ends: [[u, v0], [u, v1]],
    };
    let curve = checked_curve(PartingCarrier::Line(line), [v0, v1], origin, entity)?;
    Ok((u, curve))
}

fn order_generators(
    mut found: [(f64, PartingCurve); 2],
    trace: PartingFaceUse,
) -> Checked<Vec<PartingCurve>> {
    found.sort_by(|a, b| a.0.total_cmp(&b.0));
    let [(low, first), (high, second)] = found;
    // Opposite regular roots occupy distinct half turns even if their returned angles round.
    if !(FRAC_PI_2..(TAU - FRAC_PI_2)).contains(&(high - low)) {
        return Err(problem(trace, CylinderProblem::Contour));
    }
    Ok(vec![first, second])
}

fn problem(trace: PartingFaceUse, problem: CylinderProblem) -> PartingFailureKind {
    PartingFailureKind::Cylinder {
        face: trace.id,
        problem,
    }
}
