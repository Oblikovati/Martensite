//! Restriction midpoint classification retains endpoint precedence and periodic swaps.
use super::*;
use crate::top_ope_brep::TopOpeBRepFFTransitionTool;
use crate::top_ope_brep_tool::fsc_state_ponface;
use martensite_tkbrep::brep_adaptor::BRepAdaptorCurve;
use martensite_tkg3d::{
    adaptor3d::Adaptor3dCurve,
    top_abs::{Orientation, ShapeEnum, State},
};
use martensite_tkmath::geom_abs::CurveType;

impl TopOpeBRepFacesFiller<'_> {
    pub(super) fn st_bip_vpon_f(
        &mut self,
        first: &TopOpeBRepVPointInter<'_>,
        last: &TopOpeBRepVPointInter<'_>,
        line: &TopOpeBRepLineInter,
        first_edge: bool,
    ) -> Result<State, Failure> {
        let opposite = if first_edge { 2 } else { 1 };
        let a = first.state(opposite)?;
        let b = last.state(opposite)?;
        if a == State::Out || b == State::Out {
            return Ok(State::Out);
        }
        if a == State::In || b == State::In {
            return Ok(State::In);
        }
        let edge = topods::edge(&line.arc()?)?;
        let curve = BRepAdaptorCurve::with_edge(&edge)?;
        let periodic = matches!(curve.get_type()?, CurveType::Circle | CurveType::Ellipse);
        let Some((first, last)) = periodic_point_order(first, last, line, periodic)? else {
            return Ok(State::Unknown);
        };
        let (mut low, mut high) = (
            Self::vp_param_on_er(first, line)?,
            Self::vp_param_on_er(last, line)?,
        );
        let (bound_first, bound_last) = (curve.first_parameter()?, curve.last_parameter()?);
        if low > high && periodic {
            if low == bound_last {
                low = bound_first;
            }
            if high == bound_first {
                high = bound_last;
            }
        }
        let position = curve.value((1.0 - 0.789) * low + 0.789 * high)?;
        let face = self.faces[(opposite - 1) as usize].clone();
        fsc_state_ponface(position, face.as_ref(), self.shape_classifier_mut()?)
    }
}

fn periodic_point_order<'a, 'b>(
    first: &'a TopOpeBRepVPointInter<'b>,
    last: &'a TopOpeBRepVPointInter<'b>,
    line: &TopOpeBRepLineInter,
    periodic: bool,
) -> Result<Option<(&'a TopOpeBRepVPointInter<'b>, &'a TopOpeBRepVPointInter<'b>)>, Failure> {
    if !periodic {
        return Ok(Some((first, last)));
    }
    let arc_index = if line.arc_is_edge(2)? {
        2
    } else if line.arc_is_edge(1)? {
        1
    } else {
        return Ok(None);
    };
    let opposite = if arc_index == 1 { 2 } else { 1 };
    let a = first.shape_index();
    let b = last.shape_index();
    if (a == 3 || a == opposite) && (b == 3 || b == opposite) {
        let first_transition = TopOpeBRepFFTransitionTool::process_line_transition(
            first,
            opposite,
            first.edge(opposite)?.orientation(),
        )?;
        let last_transition = TopOpeBRepFFTransitionTool::process_line_transition(
            last,
            opposite,
            last.edge(opposite)?.orientation(),
        )?;
        if first_transition.orientation(State::In, ShapeEnum::Face) == Orientation::Reversed
            && last_transition.orientation(State::In, ShapeEnum::Face) == Orientation::Forward
        {
            return Ok(Some((last, first)));
        }
    }
    Ok(Some((first, last)))
}
