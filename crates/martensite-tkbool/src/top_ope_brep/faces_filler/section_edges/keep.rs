//! Closed restrictions distinguish a seam endpoint from a genuine interior vertex.
use super::*;
use crate::top_ope_brep_tool::{TopOpeBRepToolShapeTool, TopOpeBRepToolTOOL, fun_tool_bounds};
use martensite_tkg3d::top_abs::State;

impl TopOpeBRepFacesFiller<'_> {
    /// Tests whether a restriction has a usable interval in either operand; `filler.keep_rline(&mut line, false)?`.
    pub fn keep_rline(
        &mut self,
        line: &mut TopOpeBRepLineInter,
        check_keep: bool,
    ) -> Result<bool, Failure> {
        if line.type_line_curve() != TopOpeBRepTypeLineCurve::Restriction {
            return Ok(false);
        }
        let edge = topods::edge(&line.arc()?)?;
        if BRepTool::degenerated(&edge)? {
            return Ok(false);
        }
        let indices = selected_points(line, check_keep)?;
        let Some(first_index) = indices.first().copied() else {
            return Ok(false);
        };
        let (same, last_index) = compare_restriction_points(line, &edge, &indices)?;
        if !same && !distinct_spatial_bounds_allowed(line, &edge, first_index, last_index)? {
            return Ok(false);
        }
        if same && !coincident_periodic_bounds_allowed(line, &edge)? {
            return Ok(false);
        }
        let first = line.vpoint(first_index)?;
        let last = line.vpoint(last_index)?;
        let state = self.st_bip_vpon_f(first, last, line, line.arc_is_edge(1)?)?;
        Ok(matches!(state, State::In | State::On))
    }
}

fn selected_points(line: &TopOpeBRepLineInter, check_keep: bool) -> Result<Vec<i32>, Failure> {
    let mut indices = Vec::new();
    for index in 1..=line.nb_vpoint() {
        if !check_keep || line.vpoint(index)?.keep() {
            indices.push(index);
        }
    }
    Ok(indices)
}

fn compare_restriction_points(
    line: &TopOpeBRepLineInter,
    edge: &Edge,
    indices: &[i32],
) -> Result<(bool, i32), Failure> {
    let first = line.vpoint(indices[0])?;
    if TopOpeBRepToolTOOL::closed_e(edge)?.is_some() {
        let (low, high) = fun_tool_bounds(edge)?;
        for index in indices.iter().skip(1) {
            let parameter = TopOpeBRepFacesFiller::vp_param_on_er(line.vpoint(*index)?, line)?;
            if low < parameter && parameter < high {
                return first_spatial_difference(line, indices);
            }
        }
    }
    let mut last = indices[0];
    for index in indices.iter().skip(1) {
        last = *index;
        if !same_parameter(line, first, line.vpoint(last)?)? {
            return Ok((false, last));
        }
    }
    Ok((true, last))
}

fn first_spatial_difference(
    line: &TopOpeBRepLineInter,
    indices: &[i32],
) -> Result<(bool, i32), Failure> {
    let first = line.vpoint(indices[0])?;
    let mut last = indices[0];
    for index in indices {
        last = *index;
        if !same_position(first, line.vpoint(last)?)? {
            break;
        }
    }
    Ok((false, last))
}

fn distinct_spatial_bounds_allowed(
    line: &TopOpeBRepLineInter,
    edge: &Edge,
    first: i32,
    last: i32,
) -> Result<bool, Failure> {
    let first = line.vpoint(first)?;
    if !same_position(first, line.vpoint(last)?)? {
        return Ok(true);
    }
    let Some(vertex) = TopOpeBRepToolTOOL::closed_e(edge)? else {
        return Ok(false);
    };
    let tolerance = BRepTool::vertex_tolerance(&vertex)?.max(first.tolerance()?);
    Ok(first.value()?.distance(&BRepTool::pnt(&vertex)?) < tolerance)
}

fn coincident_periodic_bounds_allowed(
    line: &mut TopOpeBRepLineInter,
    edge: &Edge,
) -> Result<bool, Failure> {
    let curve = TopOpeBRepToolShapeTool::basis_curve_of_edge(edge)?;
    let curve = curve.get().ok_or_else(|| {
        Failure::with_message(
            "Standard_NullObject",
            "KeepRLine: curve=None; expected restriction basis curve",
        )
    })?;
    let (first, last, count) = line.vp_bounds();
    if !curve.is_periodic() || count != 2 {
        return Ok(false);
    }
    let opposite = if line.arc_is_edge(1)? { 2 } else { 1 };
    let first = line.vpoint(first)?.state(opposite)?;
    let last = line.vpoint(last)?.state(opposite)?;
    let a = matches!(first, State::In | State::On);
    let b = matches!(last, State::In | State::On);
    Ok(if first == State::Unknown || last == State::Unknown {
        a || b
    } else {
        a && b
    })
}

pub(super) fn same_position(
    first: &TopOpeBRepVPointInter<'_>,
    last: &TopOpeBRepVPointInter<'_>,
) -> Result<bool, Failure> {
    let a = first.tolerance()?;
    let b = last.tolerance()?;
    let tolerance = if a > b { a } else { b };
    Ok(first.value()?.is_equal(&last.value()?, tolerance))
}

fn same_parameter(
    line: &TopOpeBRepLineInter,
    first: &TopOpeBRepVPointInter<'_>,
    last: &TopOpeBRepVPointInter<'_>,
) -> Result<bool, Failure> {
    Ok((TopOpeBRepFacesFiller::vp_param_on_er(first, line)?
        - TopOpeBRepFacesFiller::vp_param_on_er(last, line)?)
    .abs()
        < martensite_tkernel::precision::p_confusion())
}
