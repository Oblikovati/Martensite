//! Recomputes edge crossings from neighbouring parameters and restriction geometry.
use super::TopOpeBRepFacesFiller;
use crate::top_ope_brep::{TopOpeBRepLineInter, TopOpeBRepTypeLineCurve, TopOpeBRepVPointInter};
use crate::top_ope_brep_ds::export::{fds_getupperlower, fds_lo_iinfsup};
use crate::top_ope_brep_ds::{TopOpeBRepDSKind, TopOpeBRepDSTransition};
use crate::top_ope_brep_tool::{
    TopOpeBRepToolMakeTransition, fun_tool_bounds, fun_tool_etg_f, fun_tool_par_e,
};
use martensite_tkbrep::{
    topexp::Explorer,
    topods::{self, Edge, Face, Shape},
};
use martensite_tkernel::{ncollection::List, precision, standard::Failure};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum, State};

pub(in crate::top_ope_brep::faces_filler) fn contains_shape(
    list: &List<Shape>,
    shape: &Shape,
) -> bool {
    list.iter().any(|entry| entry.is_same(shape))
}

fn restriction_on_face(
    face: &Face,
    line: &mut TopOpeBRepLineInter,
    restrictions: &List<Shape>,
    tolerance: f64,
) -> Result<Option<Edge>, Failure> {
    let mut explorer = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape)?;
    let mut edges = Vec::new();
    while explorer.more() {
        edges.push(explorer.current().clone());
        explorer.next()?;
    }
    let mut matching = Vec::new();
    for edge in restrictions
        .iter()
        .filter(|edge| edges.iter().any(|candidate| candidate.is_same(edge)))
    {
        let mut singleton = List::default();
        singleton.append(edge.clone());
        if TopOpeBRepFacesFiller::l_same_domain_erl(line, &singleton, tolerance)? {
            matching.push(topods::edge(edge)?);
        }
    }
    Ok((matching.len() == 1).then(|| matching.remove(0)))
}

fn neighbouring_parameters(
    filler: &TopOpeBRepFacesFiller<'_>,
    edge: &Edge,
    parameter: f64,
    kind: TopOpeBRepDSKind,
    index: i32,
) -> Result<(f64, f64), Failure> {
    let storage = filler.hds.get().ok_or_else(super::super::missing_storage)?;
    let edge_index = storage.ds().shape_index(edge, true);
    if edge_index == 0 {
        return fun_tool_bounds(edge);
    }
    if index == 0 {
        return fds_getupperlower(&filler.hds, edge_index, parameter);
    }
    let ds = storage.ds();
    let (first, last, _) = fds_lo_iinfsup(
        &ds,
        edge,
        parameter,
        kind,
        index,
        ds.shape_interferences(edge, true),
    )?;
    Ok((first, last))
}

fn restriction_for_transition(
    face: &Face,
    line: &mut TopOpeBRepLineInter,
    point: &TopOpeBRepVPointInter<'_>,
    edge: &Edge,
    parameter: f64,
    restrictions: &List<Shape>,
    tolerance: f64,
) -> Result<Option<(Edge, f64)>, Failure> {
    let restriction = if line.type_line_curve() == TopOpeBRepTypeLineCurve::Restriction {
        Some(topods::edge(&line.arc()?)?)
    } else {
        restriction_on_face(face, line, restrictions, tolerance)?
    };
    let Some(restriction) = restriction else {
        return Ok(None);
    };
    let mut on_restriction = 0.;
    if !point.par_on_edge(&restriction, &mut on_restriction)?
        && !fun_tool_par_e(edge, parameter, &restriction, &mut on_restriction)?
    {
        return Ok(None);
    }
    Ok(Some((restriction, on_restriction)))
}

/// What a new transition on an edge is computed from.
pub(in crate::top_ope_brep::faces_filler) struct EdgeTransitionQuery<'a, 'p> {
    pub line_on_restriction: bool,
    pub point: &'a TopOpeBRepVPointInter<'p>,
    pub kind: TopOpeBRepDSKind,
    pub index: i32,
    pub other_side: i32,
    pub edge: &'a Edge,
    pub restrictions: &'a List<Shape>,
}

pub(in crate::top_ope_brep::faces_filler) fn new_transition_edge(
    filler: &TopOpeBRepFacesFiller<'_>,
    line: &mut TopOpeBRepLineInter,
    query: &EdgeTransitionQuery<'_, '_>,
) -> Result<Option<TopOpeBRepDSTransition>, Failure> {
    let EdgeTransitionQuery {
        line_on_restriction,
        point,
        kind,
        index,
        other_side,
        edge,
        restrictions,
    } = *query;
    let face = filler.face(other_side)?;
    if matches!(
        face.orientation(),
        Orientation::Internal | Orientation::External
    ) {
        return Ok(Some(TopOpeBRepDSTransition::from_orientation(
            face.orientation(),
        )));
    }
    let mut parameter = 0.;
    if !point.par_on_edge(edge, &mut parameter)? {
        return Ok(None);
    }
    let (first, last) = neighbouring_parameters(filler, edge, parameter, kind, index)?;
    let uv = point.surface_parameters(other_side)?;
    let tangent = fun_tool_etg_f(parameter, edge, uv, face, precision::angular() * 1e4)?;
    let storage = filler.hds.get().ok_or_else(super::super::missing_storage)?;
    let rest = contains_shape(restrictions, edge) || storage.ds().is_section_edge(edge, true);
    let mut transition = TopOpeBRepToolMakeTransition::new();
    if !transition.initialize(edge, (first, last, parameter), face, uv, 0.01)? {
        return Ok(None);
    }
    if tangent && line_on_restriction && rest && transition.is_t2d() {
        let Some((restriction, par)) = restriction_for_transition(
            face,
            line,
            point,
            edge,
            parameter,
            restrictions,
            filler.face_tolerance,
        )?
        else {
            return Ok(None);
        };
        if !transition.set_rest(&restriction, par)? {
            return Ok(None);
        }
    } else if tangent && rest && !(line_on_restriction && rest) {
        let other = if point.shape_index() == 3 {
            Some((
                topods::edge(&point.edge(other_side)?)?,
                point.edge_parameter(other_side)?,
            ))
        } else if point.state(other_side)? == State::On {
            Some((
                topods::edge(point.edge_on(other_side)?)?,
                point.edge_on_parameter(other_side)?,
            ))
        } else {
            None
        };
        if let Some((edge, parameter)) = other
            && !transition.set_rest(&edge, parameter)?
        {
            return Ok(None);
        }
    }
    let (done, before, after) = transition.mk_ton_e()?;
    Ok(done.then(|| TopOpeBRepDSTransition::new(before, after, ShapeEnum::Face, ShapeEnum::Face)))
}
