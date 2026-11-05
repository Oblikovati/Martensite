//! Restriction parameters prefer original vertex and arc incidences over projection.
use super::TopOpeBRepFacesFiller;
use crate::{
    top_ope_brep::{TopOpeBRepLineInter, TopOpeBRepVPointInter},
    top_ope_brep_tool::fun_tool_proj_pone_tol,
};
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, Edge},
};
use martensite_tkernel::{precision, standard::Failure};

impl TopOpeBRepFacesFiller<'_> {
    /// Finds a VPoint's restriction parameter using its exact source incidence first; `Self::vp_param_on_er(&point, &line)?`.
    pub fn vp_param_on_er(
        point: &TopOpeBRepVPointInter<'_>,
        line: &TopOpeBRepLineInter,
    ) -> Result<f64, Failure> {
        let edge = topods::edge(&line.arc()?)?;
        for index in [1, 2] {
            if line.arc_is_edge(index)? && point.is_vertex(index)? {
                return BRepTool::parameter(&topods::vertex(&point.vertex(index)?)?, &edge);
            }
        }
        for index in [1, 2] {
            if line.arc_is_edge(index)?
                && if index == 1 {
                    point.is_on_dom_s1()?
                } else {
                    point.is_on_dom_s2()?
                }
            {
                return point.edge_parameter(index);
            }
        }
        project_parameter(point, &edge)
    }

    /// Requires both spatial and strict parameter equality on the restriction; `Self::equalp_ponr(&line, &first, &second)?`.
    pub fn equalp_ponr(
        line: &TopOpeBRepLineInter,
        first: &TopOpeBRepVPointInter<'_>,
        second: &TopOpeBRepVPointInter<'_>,
    ) -> Result<bool, Failure> {
        let first_tolerance = first.tolerance()?;
        let second_tolerance = second.tolerance()?;
        let tolerance = if first_tolerance > second_tolerance {
            first_tolerance
        } else {
            second_tolerance
        };
        let same_position = first.value()?.is_equal(&second.value()?, tolerance);
        let first_parameter = Self::vp_param_on_er(first, line)?;
        let second_parameter = Self::vp_param_on_er(second, line)?;
        Ok(same_position && (first_parameter - second_parameter).abs() < precision::p_confusion())
    }
}

fn project_parameter(point: &TopOpeBRepVPointInter<'_>, edge: &Edge) -> Result<f64, Failure> {
    let mut parameter = 0.0;
    let mut distance = 0.0;
    let tolerance = BRepTool::edge_tolerance(edge)? * 1.0e2;
    if fun_tool_proj_pone_tol(
        point.value()?,
        tolerance,
        edge,
        &mut parameter,
        &mut distance,
    )? {
        return Ok(parameter);
    }
    Err(Failure::with_message(
        "Standard_ProgramError",
        "TopOpeBRepFacesFiller::VPParamOnER",
    ))
}

impl TopOpeBRepFacesFiller<'_> {
    /// Computes a line transition only for points on the requested domain; `Self::is_vp_trans_lok(&line, 1, 1, &mut transition)?`.
    pub fn is_vp_trans_lok(
        line: &TopOpeBRepLineInter,
        point_index: i32,
        shape_index: i32,
        transition: &mut crate::top_ope_brep_ds::TopOpeBRepDSTransition,
    ) -> Result<bool, Failure> {
        let point = line.vpoint(point_index)?;
        let on_domain = if shape_index == 1 {
            point.is_on_dom_s1()?
        } else {
            point.is_on_dom_s2()?
        };
        if !on_domain {
            return Ok(false);
        }
        let edge = topods::edge(&point.edge(shape_index)?)?;
        *transition = crate::top_ope_brep::TopOpeBRepFFTransitionTool::process_line_transition(
            point,
            shape_index,
            edge.orientation(),
        )?;
        Ok(!transition.is_unknown())
    }

    /// Requires the requested IN/OUT or OUT/IN crossing on a restriction; `Self::transvp_ok(&line, 1, 1, true)?`.
    pub fn transvp_ok(
        line: &TopOpeBRepLineInter,
        point_index: i32,
        shape_index: i32,
        in_out: bool,
    ) -> Result<bool, Failure> {
        use martensite_tkg3d::top_abs::State;
        let mut transition = crate::top_ope_brep_ds::TopOpeBRepDSTransition::default();
        if !Self::is_vp_trans_lok(line, point_index, shape_index, &mut transition)? {
            return Ok(false);
        }
        let expected = if in_out {
            (State::In, State::Out)
        } else {
            (State::Out, State::In)
        };
        Ok((transition.before(), transition.after()) == expected)
    }
}
