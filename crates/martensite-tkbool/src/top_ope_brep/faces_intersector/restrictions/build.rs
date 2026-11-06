//! The source BuildRLine and BuildRLineBasedOnWLine conversions.
use super::Side;
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkgeomalgo::{
    int_patch::{IntPatchLine, IntPatchPoint, IntPatchPointLine, IntPatchRLine, IntPatchWLine},
    int_start::{TheArc, TheVertex},
    int_surf::{IntSurfLineOn2S, IntSurfTransition, IntSurfTypeTrans},
};
use martensite_tkmath::gp::Point2;

pub(super) fn build_group(
    group: &[Handle<dyn IntPatchLine>],
    side: Side<'_>,
) -> Result<Option<Handle<IntPatchRLine>>, Failure> {
    let first = group[0].cast::<IntPatchWLine>();
    let last = group[group.len() - 1].cast::<IntPatchWLine>();
    let endpoints = (first.vertex(1)?, last.vertex(last.nb_vertex())?);
    let v1 = vertex(&endpoints.0, side.first)?;
    let v2 = vertex(&endpoints.1, side.first)?;
    if v1.is_same(&v2)? {
        return Ok(None);
    }
    side.domain.init();
    while side.domain.more() {
        let arc = side.domain.value()?;
        if arc_has_vertices(side, &arc, &v1, &v2)? && middles_on_arc(group, side, &arc)? {
            let emulated = emulate_group(group, &endpoints)?;
            return build_based_on(&emulated, &arc, side);
        }
        side.domain.next();
    }
    Ok(None)
}

fn vertex(point: &IntPatchPoint, first: bool) -> Result<TheVertex, Failure> {
    if first {
        point.vertex_on_s1()
    } else {
        point.vertex_on_s2()
    }
}

fn arc_has_vertices(
    side: Side<'_>,
    arc: &TheArc,
    first: &TheVertex,
    last: &TheVertex,
) -> Result<bool, Failure> {
    side.domain.initialize_curve(arc)?;
    side.domain.init_vertex_iterator()?;
    let (mut found_first, mut found_last) = (false, false);
    while (!found_first || !found_last) && side.domain.more_vertex() {
        let current = side.domain.vertex()?;
        if !found_first {
            found_first = first.is_same(&current)?;
        }
        if !found_last {
            found_last = last.is_same(&current)?;
        }
        side.domain.next_vertex()?;
    }
    Ok(found_first && found_last)
}

fn middles_on_arc(
    group: &[Handle<dyn IntPatchLine>],
    side: Side<'_>,
    arc: &TheArc,
) -> Result<bool, Failure> {
    for source in group {
        let line = source.cast::<IntPatchWLine>();
        let index = line.nb_pnts()? / 2;
        if index < 1 {
            return Ok(false);
        }
        let (u, v) = line.point(index)?.parameters_on_surface(side.first);
        let point = Point2::new(u, v);
        let parameter = martensite_tkgeomalgo::geom2d_int::TheProjPCurOfGInter::find_parameter(
            &**arc, point, 1e-7,
        )?;
        let derivative = side.surface.d1(u, v)?;
        let tolerance = uv_tolerance(
            derivative.d1u.magnitude(),
            derivative.d1v.magnitude(),
            side.tolerance,
        );
        if arc.value(parameter)?.distance(point) > tolerance {
            return Ok(false);
        }
    }
    Ok(true)
}

fn uv_tolerance(du: f64, dv: f64, tolerance: f64) -> f64 {
    let u = if du > 1e-12 { tolerance / du } else { 0.1 };
    let v = if dv > 1e-12 { tolerance / dv } else { 0.1 };
    if u > v { v } else { u }
}

fn emulate_group(
    group: &[Handle<dyn IntPatchLine>],
    endpoints: &(IntPatchPoint, IntPatchPoint),
) -> Result<IntPatchWLine, Failure> {
    let points = Handle::new(IntSurfLineOn2S::new(None));
    let transitions = append_group_points(group, &points)?;
    let line = IntPatchWLine::new(&points, false, transitions.0, transitions.1);
    let (mut first, mut last) = endpoints.clone();
    first.set_parameter(1.);
    last.set_parameter(f64::from(points.nb_points()));
    line.add_vertex(&first, false);
    add_coincident_vertices(
        &line,
        &group[0].cast::<IntPatchWLine>(),
        &first,
        2,
        group[0].cast::<IntPatchWLine>().nb_vertex(),
    )?;
    let last_source = group[group.len() - 1].cast::<IntPatchWLine>();
    add_coincident_vertices(&line, &last_source, &last, 1, last_source.nb_vertex() - 1)?;
    line.add_vertex(&last, false);
    Ok(line)
}

fn append_group_points(
    group: &[Handle<dyn IntPatchLine>],
    points: &Handle<IntSurfLineOn2S>,
) -> Result<(IntSurfTypeTrans, IntSurfTypeTrans), Failure> {
    let mut transition = (IntSurfTypeTrans::Undecided, IntSurfTypeTrans::Undecided);
    for source in group {
        let line = source.cast::<IntPatchWLine>();
        update_transition(&mut transition.0, line.transition_on_s1());
        update_transition(&mut transition.1, line.transition_on_s2());
        let first = line.vertex(1)?.parameter_on_line() as i32;
        let last = line.vertex(line.nb_vertex())?.parameter_on_line() as i32;
        for index in first..=last {
            points.add(&line.curve().value(index)?);
        }
    }
    Ok(transition)
}

fn update_transition(target: &mut IntSurfTypeTrans, current: IntSurfTypeTrans) {
    if current != IntSurfTypeTrans::Undecided && current != IntSurfTypeTrans::Touch {
        *target = current;
    }
}

fn add_coincident_vertices(
    target: &IntPatchWLine,
    source: &IntPatchWLine,
    endpoint: &IntPatchPoint,
    first: i32,
    last: i32,
) -> Result<(), Failure> {
    for index in first..=last {
        let mut point = source.vertex(index)?;
        let tolerance = if point.tolerance() > endpoint.tolerance() {
            point.tolerance()
        } else {
            endpoint.tolerance()
        };
        if point.value().is_equal(&endpoint.value(), tolerance) {
            point.set_parameter(endpoint.parameter_on_line());
            target.add_vertex(&point, false);
        }
    }
    Ok(())
}

fn build_based_on(
    source: &IntPatchWLine,
    arc: &TheArc,
    side: Side<'_>,
) -> Result<Option<Handle<IntPatchRLine>>, Failure> {
    let count = source.nb_vertex();
    let first = source.vertex(1)?;
    let last = source.vertex(count)?;
    let (start, end) = (side.project(arc, &first)?, side.project(arc, &last)?);
    let tolerance = if first.tolerance() > last.tolerance() {
        first.tolerance()
    } else {
        last.tolerance()
    };
    if (start - end).abs() < arc.resolution(tolerance)? {
        return Ok(None);
    }
    let line = Handle::new(IntPatchRLine::new(
        false,
        source.transition_on_s1(),
        source.transition_on_s2(),
    ));
    if side.first {
        line.set_arc_on_s1(arc);
    } else {
        line.set_arc_on_s2(arc);
    }
    let forward = start < end;
    append_polygon(source, &line, forward)?;
    append_vertices(source, &line, arc, side, forward)?;
    line.set_first_point(1);
    line.set_last_point(count);
    line.compute_vertex_parameters(precision::confusion())?;
    Ok(Some(line))
}

fn append_polygon(
    source: &IntPatchWLine,
    target: &IntPatchRLine,
    forward: bool,
) -> Result<(), Failure> {
    let points = Handle::new(IntSurfLineOn2S::new(None));
    let curve = source.curve();
    for offset in 0..curve.nb_points() {
        let index = if forward {
            offset + 1
        } else {
            curve.nb_points() - offset
        };
        points.add(&curve.value(index)?);
    }
    target.add(&points);
    Ok(())
}

fn append_vertices(
    source: &IntPatchWLine,
    target: &IntPatchRLine,
    arc: &TheArc,
    side: Side<'_>,
    forward: bool,
) -> Result<(), Failure> {
    let count = source.nb_vertex();
    for offset in 0..count {
        let index = if forward { offset + 1 } else { count - offset };
        let mut point = source.vertex(index)?;
        if !forward && offset != 0 && offset != count - 1 {
            point.reverse_transition();
        }
        let parameter = side.project(arc, &point)?;
        point.set_parameter(parameter);
        if offset == 0 || offset == count - 1 {
            let transition = IntSurfTransition::default();
            point.set_arc(side.first, arc, parameter, &transition, &transition);
        }
        target.add_vertex(&point, false);
    }
    Ok(())
}
