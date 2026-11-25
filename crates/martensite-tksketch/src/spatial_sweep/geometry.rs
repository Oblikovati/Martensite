// SPDX-License-Identifier: GPL-2.0-only
//! Bounds on decoded ordinary PipeShell supports and source-bound terminal vertices.
use super::{
    SweepFailure as Failure, SweepOptions, SweepStage as Stage, SweepSupportEvidence,
    admission::{Admission, Axis, direction},
    lineage::{Face, TerminalVertex},
    native, poll,
};
use crate::spatial_prism::volume::{
    affine_curve,
    interval::{Point, Span, l1_difference},
};
use martensite_tkbrep::{
    brep::{BRepTool, EdgeCurve3D, EdgePCurve},
    topexp::Explorer,
    topods,
};
use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dCurve, Geom2dTrimmedCurve};
use martensite_tkg3d::geom::{
    GeomPlane, GeomRectangularTrimmedSurface, GeomSurface, GeomSurfaceOfRevolution,
    GeomSweptSurface,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis1, Direction3};
use std::f64::consts::FRAC_PI_2;

pub(super) fn verify(
    admission: &Admission,
    faces: &[Face],
    terminals: &[TerminalVertex],
    options: SweepOptions,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<SweepSupportEvidence, Failure> {
    let limit = options.source_trace_limit;
    let mut receipt = SweepSupportEvidence {
        bands: 0,
        planar_corners: 0,
        revolved_corners: 0,
        terminal_vertices: 0,
        trace_limit: limit,
    };
    let mut corner_counts = [[0, 0]; 2];
    for face in faces {
        poll(cancelled, Stage::Binding)?;
        face_support(admission, face, &mut receipt, &mut corner_counts)?;
    }
    if receipt.bands != 12 || corner_counts != [[2, 1]; 2] || terminals.len() != 8 {
        return Err(Failure::Geometry(
            "twelve band planes, two planar and one revolved patch per corner, and eight terminal points",
        ));
    }
    for terminal in terminals {
        poll(cancelled, Stage::Binding)?;
        terminal_point(admission, terminal, limit)?;
        receipt.terminal_vertices += 1;
    }
    Ok(receipt)
}

fn face_support(
    admission: &Admission,
    face: &Face,
    receipt: &mut SweepSupportEvidence,
    corner_counts: &mut [[usize; 2]; 2],
) -> Result<(), Failure> {
    let (surface, location) = native(BRepTool::surface_with_location(&face.shape), Stage::Binding)?;
    if !location.is_identity() {
        return Err(Failure::Geometry("an unlocated literal sweep surface"));
    }
    let [a, b] = face_endpoints(admission, face);
    let line = direction(a, b).ok_or(Failure::Geometry("a straight authored support edge"))?;
    let station = face.station;
    let limit = receipt.trace_limit;
    if !face.corner {
        let normal = admission.frames[station][line.index].cross(admission.axes[station]);
        verify_plane(
            surface.downcast_ref(),
            &face.shape,
            normal,
            world(admission, a, station, station),
            limit,
        )?;
        receipt.bands += 1;
        return Ok(());
    }
    let turn = admission.axes[station - 1].cross(admission.axes[station]);
    if admission.frames[station - 1][line.index].index == turn.index {
        verify_revolution(
            surface.downcast_ref(),
            admission,
            [a, b],
            station,
            turn,
            limit,
        )?;
        receipt.revolved_corners += 1;
        corner_counts[station - 1][1] += 1;
    } else {
        verify_plane(
            surface.downcast_ref(),
            &face.shape,
            turn,
            world(admission, a, station - 1, station),
            limit,
        )?;
        receipt.planar_corners += 1;
        corner_counts[station - 1][0] += 1;
    }
    Ok(())
}

fn face_endpoints(admission: &Admission, face: &Face) -> [[f64; 3]; 2] {
    let points = &admission.profile.points;
    [
        points[face.profile].1,
        points[(face.profile + 1) % points.len()].1,
    ]
}

fn world(admission: &Admission, source: [f64; 3], frame: usize, station: usize) -> Point {
    let offset = transported(admission, source, frame);
    std::array::from_fn(|i| offset[i] + Span::at(admission.stations[station].xyz[i]))
}

fn transported(admission: &Admission, source: [f64; 3], frame: usize) -> Point {
    let mut result = [Span::ZERO; 3];
    for (i, axis) in admission.frames[frame].iter().enumerate() {
        let difference = Span::at(source[i]) - Span::at(admission.stations[0].xyz[i]);
        result[axis.index] = Span::at(axis.sign) * difference;
    }
    result
}

fn verify_plane(
    plane: Option<&GeomPlane>,
    face: &topods::Face,
    normal: Axis,
    expected: Point,
    limit: f64,
) -> Result<(), Failure> {
    let expected_kind = "a decoded plane within its authored support coordinate bound";
    let plane = plane
        .ok_or(Failure::Geometry(expected_kind))?
        .pln()
        .position();
    let axis = normal.index;
    let offset = Span::at(plane.location().coordinates()[axis]) - expected[axis];
    let tangents = [plane.x_direction(), plane.y_direction()]
        .map(|direction| Span::at(direction.coordinates()[axis]));
    let bound = if tangents == [Span::ZERO; 2] {
        offset.magnitude()
    } else {
        require_planar_trim_bound(face, offset, tangents)?
    };
    require_bound(bound, limit, expected_kind)
}

fn require_planar_trim_bound(
    face: &topods::Face,
    offset: Span,
    tangents: [Span; 2],
) -> Result<f64, Failure> {
    let expected = "a closed decoded planar trim with literal affine or circular boundary spans";
    let wires = native(
        Explorer::new(face, ShapeEnum::Wire, ShapeEnum::Shape),
        Stage::Binding,
    )?;
    if wires.count() != 1 {
        return Err(Failure::Geometry(expected));
    }
    let edges = native(
        Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape),
        Stage::Binding,
    )?;
    let mut maximum: Option<f64> = None;
    for edge in edges {
        let edge = native(topods::edge(&edge), Stage::Binding)?;
        let pcurve = native(BRepTool::curve_on_face(&edge, face), Stage::Binding)?;
        let bound =
            planar_trim_bound(pcurve, offset, tangents).ok_or(Failure::Geometry(expected))?;
        maximum = Some(maximum.unwrap_or(0.).max(bound));
    }
    maximum.ok_or(Failure::Geometry(expected))
}

fn planar_trim_bound(mut trim: EdgePCurve, offset: Span, tangents: [Span; 2]) -> Option<f64> {
    if let Some(wrapper) = trim.pcurve.downcast_ref::<Geom2dTrimmedCurve>() {
        if trim.first < wrapper.first_parameter() || trim.last > wrapper.last_parameter() {
            return None;
        }
        trim.pcurve = wrapper.basis_curve();
    }
    if !trim.first.is_finite() || !trim.last.is_finite() || trim.first >= trim.last {
        return None;
    }
    // An affine functional on the compact trimmed region has its extrema on the boundary.
    let value = |[u, v]: [Span; 2]| offset + tangents[0] * u + tangents[1] * v;
    if let Some([a, b]) = affine_curve::planar(&trim, false, true) {
        let hull = value(a).hull(value(b));
        return hull.finite().then_some(hull.magnitude());
    }
    let circle = trim.pcurve.downcast_ref::<Geom2dCircle>()?.circ2d().ok()?;
    let center = circle.location();
    let relative_radius = Span::at(circle.radius())
        * Span::at((tangents[0].magnitude() + tangents[1].magnitude()).next_up());
    let bound = value([Span::at(center.x()), Span::at(center.y())]).magnitude();
    let total = Span::at(bound) + relative_radius;
    total.finite().then_some(total.hi)
}

fn cardinal(direction: Direction3) -> Option<Axis> {
    let coordinate = *direction.coordinates();
    let index = coordinate.iter().position(|n| n.abs() == 1.)?;
    if (0..3).any(|i| i != index && coordinate[i] != 0.) {
        return None;
    }
    Some(Axis {
        index,
        sign: coordinate[index],
    })
}

fn verify_revolution(
    trimmed: Option<&GeomRectangularTrimmedSurface>,
    admission: &Admission,
    source: [[f64; 3]; 2],
    station: usize,
    turn: Axis,
    limit: f64,
) -> Result<(), Failure> {
    let expected_kind = "a literal cardinal quarter revolution with an affine meridian";
    let trimmed = trimmed.ok_or(Failure::Geometry(expected_kind))?;
    let (u0, u1, v0, v1) = trimmed.bounds();
    if u0 != 0. || u1 != FRAC_PI_2 {
        return Err(Failure::Geometry(expected_kind));
    }
    let basis = trimmed.basis_surface();
    let rev = basis
        .downcast_ref::<GeomSurfaceOfRevolution>()
        .filter(|surface| !surface.has_eval_representation())
        .ok_or(Failure::Geometry(expected_kind))?;
    let (frame, axis_error) = rotation_frame(rev.axis(), admission, station, turn)?;
    let origin = admission.stations[station].xyz;
    let meridian = EdgeCurve3D {
        curve: rev.basis_curve(),
        first: v0,
        last: v1,
        ..Default::default()
    };
    let actual = affine_curve::spatial(&meridian, false, true, origin)
        .ok_or(Failure::Geometry(expected_kind))?;
    let desired = source.map(|p| transported(admission, p, frame));
    let matching = meridian_error(actual, desired);
    let angle_error = short_quarter_error(desired, turn.index);
    let total = Span::at(matching) + Span::at(2.) * Span::at(axis_error) + angle_error;
    require_bound(total.hi, limit, expected_kind)
}

fn rotation_frame(
    axis: Axis1,
    admission: &Admission,
    station: usize,
    turn: Axis,
) -> Result<(usize, f64), Failure> {
    let actual = cardinal(axis.direction())
        .filter(|actual| actual.index == turn.index)
        .ok_or(Failure::Geometry("a cardinal decoded revolution axis"))?;
    let center = *axis.location().coordinates();
    let desired = admission.stations[station].xyz;
    let relative = std::array::from_fn(|i| {
        if i == turn.index {
            Span::ZERO
        } else {
            Span::at(center[i]) - Span::at(desired[i])
        }
    });
    let frame = if actual.sign == turn.sign {
        station - 1
    } else {
        station
    };
    Ok((frame, l1_difference(relative, [Span::ZERO; 3])))
}

fn meridian_error(actual: [Point; 2], expected: [Point; 2]) -> f64 {
    let paired = |i: usize, j: usize| {
        l1_difference(actual[0], expected[i]).max(l1_difference(actual[1], expected[j]))
    };
    // A geometric affine segment can have either representation orientation.
    paired(0, 1).min(paired(1, 0))
}

fn short_quarter_error(meridian: [Point; 2], axis: usize) -> Span {
    let radial = |point: Point| {
        point
            .into_iter()
            .enumerate()
            .filter(|(i, _)| *i != axis)
            .fold(0., |bound: f64, (_, coordinate)| {
                (bound + coordinate.magnitude()).next_up()
            })
    };
    // The binary64 quarter-turn differs from the mathematical one by less than 2^-52.
    Span::at(f64::EPSILON) * Span::at(radial(meridian[0]).max(radial(meridian[1])))
}

fn terminal_point(
    admission: &Admission,
    terminal: &TerminalVertex,
    limit: f64,
) -> Result<(), Failure> {
    let (_, source) = admission
        .profile
        .points
        .iter()
        .find(|(id, _)| *id == terminal.profile_point)
        .ok_or(Failure::Geometry("a recorded authored terminal point"))?;
    if !terminal.actual.location().is_identity() {
        return Err(Failure::Geometry("an unlocated decoded terminal vertex"));
    }
    let actual = native(BRepTool::pnt(&terminal.actual), Stage::Binding)?;
    let frame = if terminal.station == 0 { 0 } else { 2 };
    let expected = world(admission, *source, frame, terminal.station);
    require_bound(
        l1_difference(actual.coordinates().map(Span::at), expected),
        limit,
        "a decoded terminal point at its transported authored address",
    )
}

fn require_bound(bound: f64, limit: f64, description: &'static str) -> Result<(), Failure> {
    if !bound.is_finite() || bound > limit {
        return Err(Failure::Trace {
            description,
            bound,
            limit,
        });
    }
    Ok(())
}
