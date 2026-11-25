// SPDX-License-Identifier: GPL-2.0-only
//! Full cylindrical trim admission and native Contap generators.
use super::{
    PartingCarrier, PartingCurve, PartingCylinderProblem as CylinderProblem, PartingEntity,
    PartingFaceUse, PartingFailureKind, PartingOrigin, PartingPositionEvidence,
    failure::{Checked, native},
    geometry::{FaceSign, checked_curve},
};
use crate::contap::ContapContAna;
use martensite_tkbrep::{
    brep::{BRepTFace, BRepTool},
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
    gp::{Cylinder3, Direction3, Line3},
    predicates::roots::metric_bounds::{
        CylindricalGeneratorChart, CylindricalPullKind, exact_affine_coordinate,
    },
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
    let chart = capture_chart(&surface, trace, pull)?;
    match chart.kind() {
        CylindricalPullKind::Axial => return Ok((FaceSign::Zero, Vec::new())),
        CylindricalPullKind::Skew => return Err(problem(trace, CylinderProblem::SkewPull)),
        CylindricalPullKind::Radial => (),
    }
    let allowance = source_tolerance(&face, trace)?;
    let curves = generators(cylinder, trace, pull, bounds, allowance, &chart)?;
    Ok((FaceSign::Varying, curves))
}

fn capture_chart(
    surface: &BRepAdaptorSurface,
    trace: PartingFaceUse,
    pull: Direction3,
) -> Checked<CylindricalGeneratorChart> {
    let entity = PartingEntity::Face(trace.id);
    let placed = surface.transformed_surface();
    let local = placed.adaptor_surface_original();
    let local = local
        .get()
        .ok_or_else(|| problem(trace, CylinderProblem::Trim))?;
    let cylinder = local.cylinder().map_err(native(entity))?;
    let transform = placed.trsf();
    let mut rows = [[0.; 4]; 3];
    for (row, values) in rows.iter_mut().enumerate() {
        for (column, element) in values.iter_mut().enumerate() {
            *element = transform
                .value(row as i32 + 1, column as i32 + 1)
                .map_err(native(entity))?;
        }
    }
    CylindricalGeneratorChart::capture(cylinder, rows, pull).map_err(|cause| {
        PartingFailureKind::CylinderPosition {
            face: trace.id,
            cause,
        }
    })
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

fn source_tolerance(face: &Face, trace: PartingFaceUse) -> Checked<f64> {
    // The BRepTool accessor applies a confusion floor. Admission uses the literal stored value.
    let owner = face.tshape().cast::<BRepTFace>();
    let stored = owner
        .get()
        .ok_or_else(|| problem(trace, CylinderProblem::Trim))?;
    let tolerance = stored.tolerance();
    if tolerance.is_finite() && tolerance >= 0. {
        return Ok(tolerance);
    }
    Err(problem(trace, CylinderProblem::Tolerance(tolerance)))
}

fn generators(
    cylinder: Cylinder3,
    trace: PartingFaceUse,
    pull: Direction3,
    bounds: [f64; 4],
    source_tolerance: f64,
    chart: &CylindricalGeneratorChart,
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
    let lines = [
        contour.line(1).map_err(native(entity))?,
        contour.line(2).map_err(native(entity))?,
    ];
    let upper = chart
        .enclose(lines, [bounds[2], bounds[3]])
        .map_err(|cause| PartingFailureKind::CylinderPosition {
            face: trace.id,
            cause,
        })?;
    let make = |index: usize| {
        let upper_bound = upper[index];
        if upper_bound > source_tolerance {
            return Err(problem(
                trace,
                CylinderProblem::Position {
                    generator: index + 1,
                    upper_bound,
                    source_tolerance,
                },
            ));
        }
        let position = PartingPositionEvidence::CylindricalGenerator {
            upper_bound,
            source_tolerance,
        };
        generator(lines[index], cylinder, trace, bounds, position)
    };
    order_generators([make(0)?, make(1)?], trace)
}

fn generator(
    line: Line3,
    cylinder: Cylinder3,
    trace: PartingFaceUse,
    [u0, u1, v0, v1]: [f64; 4],
    position: PartingPositionEvidence,
) -> Checked<(f64, PartingCurve)> {
    let entity = PartingEntity::Face(trace.id);
    let (u, _) =
        elslib::cylinder_parameters(cylinder.position(), cylinder.radius(), line.location());
    if !(u0..=u1).contains(&u) || line.direction() != cylinder.axis().direction() {
        return Err(problem(trace, CylinderProblem::Contour));
    }
    // Contap uses the rebuilt world's V=0 section and axial line direction.
    // The full-interval comparison above binds this line to the stored source chart.
    let u = u.rem_euclid(TAU);
    let origin = PartingOrigin::Smooth {
        face: trace,
        uv_ends: [[u, v0], [u, v1]],
    };
    let curve = checked_curve(
        PartingCarrier::Line(line),
        [v0, v1],
        origin,
        position,
        entity,
    )?;
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
