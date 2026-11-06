//! Identify the source edge, check source gap samples, and reconstruct its 100-point polygon.
use super::super::Side;
use super::project::{closest, projector};
use martensite_tkbrep::{brep::BRepTool, topexp, topods::Edge};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkgeomalgo::{
    geom2d_int::TheProjPCurOfGInter,
    int_patch::{IntPatchIType, IntPatchLine, IntPatchPointLine, IntPatchWLine},
    int_start::TheArc,
    int_surf::{IntSurfLineOn2S, IntSurfPntOn2S},
};
use martensite_tkgeombase::extrema::{ExtPC, ExtPS};
use martensite_tkmath::gp::{Point2, Point3};

pub(super) fn find_arc(
    lines: &[Handle<dyn IntPatchLine>],
    side: Side<'_>,
    other: Side<'_>,
    test: Point3,
) -> Result<Option<(Handle<IntSurfLineOn2S>, f64)>, Failure> {
    let index = nearest_arc(side, test)?;
    if index == 0 {
        return Ok(None);
    }
    side.domain.init();
    for _ in 1..index {
        side.domain.next();
    }
    let arc = side.domain.value()?;
    let edge = side.domain.edge()?;
    let parameters = walking_parameters(lines, &arc, side.first)?;
    let (v1, v2) = topexp::vertices(&edge, false);
    let tolerance = native_max(
        BRepTool::vertex_tolerance(&v1)?,
        BRepTool::vertex_tolerance(&v2)?,
    );
    let check_tolerance = native_max(tolerance, BRepTool::edge_tolerance(&edge)?);
    let tool = &mut projector(other, check_tolerance)?;
    if !valid_gaps(&edge, &arc, &parameters, tolerance, check_tolerance, tool)? {
        return Ok(None);
    }
    let polygon = sample_arc(
        &edge,
        &parameters,
        &mut projector(side, check_tolerance)?,
        tool,
        check_tolerance,
    )?;
    Ok(Some((polygon, tolerance)))
}

fn nearest_arc(side: Side<'_>, point: Point3) -> Result<usize, Failure> {
    let (mut best, mut distance, mut current) = (0, 1e200, 0);
    side.domain.init();
    while side.domain.more() {
        current += 1;
        let edge = side.domain.edge()?;
        let (curve, _, _) = BRepTool::curve(&edge)?;
        if !curve.is_null() {
            let adaptor = GeomAdaptorCurve::with_curve(&curve)?;
            let extrema = ExtPC::with_curve(point, &adaptor, 1e-7)?;
            if extrema.is_done() {
                for index in 1..=extrema.nb_ext()? {
                    let square = extrema.square_distance(index)?;
                    if square < distance {
                        distance = square;
                        best = current;
                    }
                }
            }
        }
        side.domain.next();
    }
    Ok(best)
}

fn walking_parameters(
    lines: &[Handle<dyn IntPatchLine>],
    arc: &TheArc,
    first: bool,
) -> Result<Vec<f64>, Failure> {
    let mut parameters = Vec::new();
    for source in lines
        .iter()
        .filter(|line| line.arc_type() == IntPatchIType::Walking)
    {
        let line = source.cast::<IntPatchWLine>();
        for index in [1, line.nb_pnts()?] {
            let (u, v) = line.point(index)?.parameters_on_surface(first);
            parameters.push(TheProjPCurOfGInter::find_parameter(
                &**arc,
                Point2::new(u, v),
                1e-7,
            )?);
        }
    }
    for i in 0..parameters.len() {
        for j in i + 1..parameters.len() {
            if parameters[i] > parameters[j] {
                parameters.swap(i, j);
            }
        }
    }
    Ok(parameters)
}

fn valid_gaps(
    edge: &Edge,
    arc: &TheArc,
    parameters: &[f64],
    vertex_tolerance: f64,
    tolerance: f64,
    projector: &mut ExtPS<'_>,
) -> Result<bool, Failure> {
    let (curve, first, last) = BRepTool::curve(edge)?;
    let resolution = arc.resolution(vertex_tolerance)?;
    let mut gaps = vec![(first, parameters[0]), (last, *parameters.last().unwrap())];
    for index in 1..parameters.len() / 2 {
        gaps.push((parameters[2 * index - 1], parameters[2 * index]));
    }
    for (a, b) in gaps {
        if (a - b).abs() > resolution
            && closest(projector, curve.value((a + b) / 2.)?, tolerance)?.is_none()
        {
            return Ok(false);
        }
    }
    Ok(true)
}

fn sample_arc(
    edge: &Edge,
    parameters: &[f64],
    object: &mut ExtPS<'_>,
    tool: &mut ExtPS<'_>,
    tolerance: f64,
) -> Result<Handle<IntSurfLineOn2S>, Failure> {
    let (curve, _, _) = BRepTool::curve(edge)?;
    let (first, last) = (parameters[0], *parameters.last().unwrap());
    let step = (last - first).abs() / 100.;
    let mut parameter = first;
    let polygon = Handle::new(IntSurfLineOn2S::new(None));
    for index in 0..100 {
        if index == 99 {
            parameter = last;
        }
        let point = curve.value(parameter)?;
        if let (Some(p1), Some(p2)) = (
            closest(object, point, tolerance)?,
            closest(tool, point, tolerance)?,
        ) {
            let (u1, v1) = p1.parameter();
            let (u2, v2) = p2.parameter();
            let mut entry = IntSurfPntOn2S::default();
            entry.set_value_all(point, u1, v1, u2, v2);
            polygon.add(&entry);
        }
        parameter += step;
    }
    Ok(polygon)
}

fn native_max(first: f64, second: f64) -> f64 {
    if first < second { second } else { first }
}
