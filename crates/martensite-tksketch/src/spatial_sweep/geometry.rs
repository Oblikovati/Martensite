// SPDX-License-Identifier: GPL-2.0-only
//! Bounds on decoded ordinary PipeShell supports and source-bound terminal vertices.
use super::{
    SweepFailure as Failure, SweepOptions, SweepStage as Stage, SweepSupportEvidence,
    admission::{Admission, Axis, direction},
    lineage::{Face, TerminalVertex},
    native, poll, trim_proof,
};
use crate::spatial_prism::volume::{
    affine_curve,
    interval::{Point, Span, l1_difference},
};
use martensite_tkbrep::brep::{BRepTool, EdgeCurve3D, EdgePCurve};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dTrimmedCurve};
use martensite_tkg3d::geom::{
    GeomRectangularTrimmedSurface, GeomSurface, GeomSurfaceOfRevolution, GeomSweptSurface,
};
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
    let mut trig = trim_proof::Trig::new();
    for (index, face) in faces.iter().enumerate() {
        poll(cancelled, Stage::Binding)?;
        let revolved = is_revolved(admission, face)?;
        trim_proof::verify_lateral(
            admission, face, revolved, index, limit, &mut trig, cancelled,
        )?;
        if revolved {
            revolved_source(admission, face, limit)?;
            receipt.revolved_corners += 1;
            corner_counts[face.station - 1][1] += 1;
        } else if face.corner {
            receipt.planar_corners += 1;
            corner_counts[face.station - 1][0] += 1;
        } else {
            receipt.bands += 1;
        }
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

fn is_revolved(admission: &Admission, face: &Face) -> Result<bool, Failure> {
    if !face.corner {
        return Ok(false);
    }
    let [a, b] = face_endpoints(admission, face);
    let line = direction(a, b).ok_or(Failure::Geometry("a straight authored support edge"))?;
    let turn = admission.axes[face.station - 1].cross(admission.axes[face.station]);
    Ok(admission.frames[face.station - 1][line.index].index == turn.index)
}

fn revolved_source(admission: &Admission, face: &Face, limit: f64) -> Result<(), Failure> {
    let (surface, location) = native(BRepTool::surface_with_location(&face.shape), Stage::Binding)?;
    if !location.is_identity() {
        return Err(Failure::Geometry("an unlocated literal revolved support"));
    }
    let station = face.station;
    let turn = admission.axes[station - 1].cross(admission.axes[station]);
    verify_revolution(
        surface.downcast_ref(),
        admission,
        face_endpoints(admission, face),
        station,
        turn,
        limit,
    )
}

fn face_endpoints(admission: &Admission, face: &Face) -> [[f64; 3]; 2] {
    let points = &admission.profile.points;
    [
        points[face.profile].1,
        points[(face.profile + 1) % points.len()].1,
    ]
}

pub(super) fn world(
    admission: &Admission,
    source: [f64; 3],
    frame: usize,
    station: usize,
) -> Point {
    let offset = transported(admission, source, frame);
    std::array::from_fn(|i| offset[i] + Span::at(admission.stations[station].xyz[i]))
}

pub(super) fn transported(admission: &Admission, source: [f64; 3], frame: usize) -> Point {
    let mut result = [Span::ZERO; 3];
    for (i, axis) in admission.frames[frame].iter().enumerate() {
        let difference = Span::at(source[i]) - Span::at(admission.stations[0].xyz[i]);
        result[axis.index] = Span::at(axis.sign) * difference;
    }
    result
}

pub(super) fn trim_basis(mut trim: EdgePCurve) -> Option<EdgePCurve> {
    if let Some(wrapper) = trim.pcurve.downcast_ref::<Geom2dTrimmedCurve>() {
        if trim.first < wrapper.first_parameter() || trim.last > wrapper.last_parameter() {
            return None;
        }
        trim.pcurve = wrapper.basis_curve();
    }
    if !trim.first.is_finite() || !trim.last.is_finite() || trim.first >= trim.last {
        return None;
    }
    Some(trim)
}

pub(super) fn cardinal(direction: Direction3) -> Option<Axis> {
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
    if u0 != 0. || !u1.is_finite() || u1 <= 0. || u1 > std::f64::consts::PI {
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
    let angle_error = short_quarter_error(desired, turn.index, u1);
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

pub(super) fn meridian_error(actual: [Point; 2], expected: [Point; 2]) -> f64 {
    let paired = |i: usize, j: usize| {
        l1_difference(actual[0], expected[i]).max(l1_difference(actual[1], expected[j]))
    };
    // A geometric affine segment can have either representation orientation.
    paired(0, 1).min(paired(1, 0))
}

fn short_quarter_error(meridian: [Point; 2], axis: usize, actual: f64) -> Span {
    let radial = |point: Point| {
        point
            .into_iter()
            .enumerate()
            .filter(|(i, _)| *i != axis)
            .fold(0., |bound: f64, (_, coordinate)| {
                (bound + coordinate.magnitude()).next_up()
            })
    };
    let quarter = Span {
        lo: FRAC_PI_2.next_down(),
        hi: FRAC_PI_2.next_up(),
    };
    let angle_gap = (Span::at(actual) - quarter).magnitude();
    Span::at(angle_gap) * Span::at(radial(meridian[0]).max(radial(meridian[1])))
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

pub(super) fn require_bound(
    bound: f64,
    limit: f64,
    description: &'static str,
) -> Result<(), Failure> {
    if !bound.is_finite() || bound > limit {
        return Err(Failure::Trace {
            description,
            bound,
            limit,
        });
    }
    Ok(())
}
