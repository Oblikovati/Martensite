//! Unknown line transitions use retained endpoints and later walking crossings.
use super::{TopOpeBRepFacesFiller, context::Context};
use crate::top_ope_brep::{
    TopOpeBRepFFTransitionTool, TopOpeBRepLineInter, TopOpeBRepTypeLineCurve, TopOpeBRepVPointInter,
};
use crate::top_ope_brep_ds::{TopOpeBRepDSTransition, export::fds_parameter};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::Orientation;

fn following_transition(
    line: &TopOpeBRepLineInter,
    point_index: i32,
    side: i32,
) -> Result<TopOpeBRepDSTransition, Failure> {
    for index in point_index + 1..=line.nb_vpoint() {
        let point = line.vpoint(index)?;
        if !point.keep() {
            continue;
        }
        let mask = point.shape_index();
        let incident = if mask == 3 { side } else { mask };
        if incident == 0 {
            continue;
        }
        let edge = point.edge(incident)?;
        let transition = TopOpeBRepFFTransitionTool::process_line_transition(
            point,
            incident,
            edge.orientation(),
        )?;
        if !transition.is_unknown() {
            return Ok(transition.complement());
        }
    }
    Ok(TopOpeBRepDSTransition::default())
}

fn same_endpoint_vertex(
    point: &TopOpeBRepVPointInter<'_>,
    endpoint: &TopOpeBRepVPointInter<'_>,
    side: i32,
) -> Result<bool, Failure> {
    Ok(point.is_vertex(side)?
        && endpoint.is_vertex(side)?
        && point.vertex(side)?.is_same(&endpoint.vertex(side)?))
}

fn keep_geometric_point(
    line: &TopOpeBRepLineInter,
    point: &TopOpeBRepVPointInter<'_>,
    context: &Context,
    bounds: (i32, i32, i32),
    empty: bool,
) -> Result<bool, Failure> {
    if point.index() == bounds.0 {
        return Ok(true);
    }
    if point.index() == bounds.1 {
        return Ok(!empty);
    }
    if context.current.degenerate || context.other_degenerate() {
        return Ok(false);
    }
    for side in [1, 2] {
        if same_endpoint_vertex(point, line.vpoint(bounds.0)?, side)? {
            return Ok(true);
        }
        if same_endpoint_vertex(point, line.vpoint(bounds.1)?, side)? && !empty {
            return Ok(true);
        }
    }
    Ok(false)
}

pub(super) fn line_transition(
    filler: &TopOpeBRepFacesFiller<'_>,
    line: &mut TopOpeBRepLineInter,
    point: &TopOpeBRepVPointInter<'_>,
    context: &Context,
) -> Result<Option<(TopOpeBRepDSTransition, bool)>, Failure> {
    let mut transition = TopOpeBRepFFTransitionTool::process_line_transition(
        point,
        context.side,
        context.current.edge.orientation(),
    )?;
    let bounds = line.vp_bounds();
    if !transition.is_unknown() {
        return Ok(Some((transition, false)));
    }
    let empty = filler.curve_interferences.is_empty();
    if line.type_line_curve() == TopOpeBRepTypeLineCurve::Walking {
        let previous = filler.curve_interferences.iter().last();
        transition = match previous {
            Some(previous) if fds_parameter(previous)? != point.parameter_on_line()? => previous
                .get()
                .ok_or_else(super::super::missing_interference)?
                .transition()
                .complement(),
            _ => following_transition(line, point.index(), context.side)?,
        };
        let keep_first = empty
            && point.index() == bounds.0
            && bounds.2 == 2
            && (point.shape_index() != 3 || filler.last_point_on_zero);
        if transition.is_unknown() && keep_first {
            transition.set_orientation(Orientation::Forward);
        }
        return Ok(Some((transition, false)));
    }
    let geometric =
        line.type_line_curve() != TopOpeBRepTypeLineCurve::Restriction && !filler.line_is_on_edge;
    if geometric && keep_geometric_point(line, point, context, bounds, empty)? {
        return Ok(Some((transition, true)));
    }
    Ok(None)
}
