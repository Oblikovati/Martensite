//! Merge all-boundary walking pieces through the source arc-gap criterion.
mod arc;
mod project;
use super::Side;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkgeomalgo::{
    int_patch::{IntPatchIType, IntPatchLine, IntPatchPoint, IntPatchPointLine, IntPatchWLine},
    int_surf::{IntSurfLineOn2S, IntSurfTypeTrans},
};
use martensite_tkmath::gp::Point3;

pub(in super::super) fn merge_restriction_segments(
    lines: &mut Vec<Handle<dyn IntPatchLine>>,
    sides: [Side<'_>; 2],
) -> Result<(), Failure> {
    let Some((side, other, points)) = boundary_side(lines, sides)? else {
        return Ok(());
    };
    if points.len() <= 2 || points[0].is_equal(points.last().unwrap(), 1e-9) {
        return Ok(());
    }
    let index = if points.len() > 3 {
        points.len() / 2
    } else {
        2
    };
    let Some((polygon, tolerance)) = arc::find_arc(lines, side, other, points[index - 1])? else {
        return Ok(());
    };
    if polygon.nb_points() == 0 {
        return Ok(());
    }
    let merged = merged_line(lines, &polygon, tolerance)?;
    lines.clear();
    lines.push(Handle::new(merged).cast());
    Ok(())
}

fn boundary_side<'a>(
    lines: &[Handle<dyn IntPatchLine>],
    sides: [Side<'a>; 2],
) -> Result<Option<(Side<'a>, Side<'a>, Vec<Point3>)>, Failure> {
    let (mut on_first, mut on_second) = (0, 0);
    let mut endpoints = Vec::new();
    for rank in 0..2 {
        let mut count = 0;
        for source in lines
            .iter()
            .filter(|line| line.arc_type() == IntPatchIType::Walking)
        {
            count += 1;
            let line = source.cast::<IntPatchWLine>();
            if rank == 0 {
                endpoints.extend([
                    line.vertex(1)?.value(),
                    line.vertex(line.nb_vertex())?.value(),
                ]);
                if sides[0].on_restriction(&line)? {
                    on_first += 1;
                }
            } else if sides[1].on_restriction(&line)? {
                on_second += 1;
            }
        }
        if count == on_first || count == on_second {
            let chosen = if on_first == count { 0 } else { 1 };
            return Ok(Some((sides[chosen], sides[1 - chosen], endpoints)));
        }
    }
    Ok(None)
}

fn merged_line(
    lines: &[Handle<dyn IntPatchLine>],
    polygon: &Handle<IntSurfLineOn2S>,
    tolerance: f64,
) -> Result<IntPatchWLine, Failure> {
    let mut transitions = (IntSurfTypeTrans::Undecided, IntSurfTypeTrans::Undecided);
    for source in lines
        .iter()
        .filter(|line| line.arc_type() == IntPatchIType::Walking)
    {
        let line = source.cast::<IntPatchWLine>();
        update_transition(&mut transitions.0, line.transition_on_s1());
        update_transition(&mut transitions.1, line.transition_on_s2());
    }
    let line = IntPatchWLine::new(polygon, false, transitions.0, transitions.1);
    for index in [1, polygon.nb_points()] {
        let point = polygon.value(index)?;
        let mut vertex = IntPatchPoint::new();
        vertex.set_value_tangent(point.value(), tolerance, false);
        let (u1, v1, u2, v2) = point.parameters();
        vertex.set_parameters(u1, v1, u2, v2);
        vertex.set_parameter(f64::from(index));
        line.add_vertex(&vertex, false);
    }
    line.compute_vertex_parameters(tolerance)?;
    Ok(line)
}

fn update_transition(target: &mut IntSurfTypeTrans, current: IntSurfTypeTrans) {
    if current != IntSurfTypeTrans::Undecided && current != IntSurfTypeTrans::Touch {
        *target = current;
    }
}
