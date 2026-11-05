use super::TopOpeBRepEdgesIntersector;
use crate::top_ope_brep_tool::fun_tool_orient_v_in_e;
use crate::{
    top_ope_brep::TopOpeBRepPoint2d,
    top_ope_brep_tool::{TopOpeBRepToolShapeTool, TopOpeBRepToolTOOL},
};
use martensite_tkbrep::topods::{Edge, Vertex};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum, State};
impl TopOpeBRepEdgesIntersector {
    pub(super) fn correct_same_domain_vertices(&mut self) -> Result<(), Failure> {
        if !self.same_domain {
            return Ok(());
        }
        for index in 0..self.points.len() {
            let mut point = std::mem::take(&mut self.points[index]);
            if point.keep() {
                correct_point(&self.edges, &mut point)?;
            }
            self.points[index] = point;
        }
        Ok(())
    }
}
fn correct_point(edges: &[Edge; 2], point: &mut TopOpeBRepPoint2d) -> Result<(), Failure> {
    let (first, second) = (point.is_vertex(1)?, point.is_vertex(2)?);
    let first_internal =
        point.transition(1)?.orientation(State::In, ShapeEnum::Face) == Orientation::Internal;
    let second_orientation = point.transition(2)?.orientation(State::In, ShapeEnum::Face);
    if first_internal && second && !first {
        correct_transition(edges, point, 0)?;
    }
    if matches!(
        second_orientation,
        Orientation::Internal | Orientation::External
    ) && first
        && !second
    {
        correct_transition(edges, point, 1)?;
    }
    correct_closed(edges, point, 0, !first_internal && second && !first)?;
    correct_closed(
        edges,
        point,
        1,
        second_orientation != Orientation::Internal && first && !second,
    )
}
fn correct_transition(
    edges: &[Edge; 2],
    point: &mut TopOpeBRepPoint2d,
    index: usize,
) -> Result<(), Failure> {
    let vertex = point.vertex((2 - index) as i32)?;
    if let Some(orientation) = corrected_orientation(&edges[index], &edges[1 - index], vertex)? {
        point
            .change_transition(index as i32 + 1)?
            .set_orientation(orientation);
    }
    Ok(())
}
fn corrected_orientation(
    first: &Edge,
    second: &Edge,
    vertex: &Vertex,
) -> Result<Option<Orientation>, Failure> {
    let position = fun_tool_orient_v_in_e(vertex, second);
    if position == 0 {
        return Ok(None);
    }
    if position == 3 {
        return Ok(Some(Orientation::Internal));
    }
    let same = TopOpeBRepToolShapeTool::shapes_same_oriented(first, second)?;
    let same = same != (first.orientation() == Orientation::Reversed);
    let reversed = (same && position == 1) || (!same && position == 2);
    Ok(Some(if reversed {
        Orientation::Reversed
    } else {
        Orientation::Forward
    }))
}
fn correct_closed(
    edges: &[Edge; 2],
    point: &mut TopOpeBRepPoint2d,
    index: usize,
    enabled: bool,
) -> Result<(), Failure> {
    if !enabled {
        return Ok(());
    }
    let Some(closing) = TopOpeBRepToolTOOL::closed_e(&edges[1 - index])? else {
        return Ok(());
    };
    if closing.is_same(point.vertex((2 - index) as i32)?) {
        point
            .change_transition(index as i32 + 1)?
            .set_orientation(Orientation::Internal);
    }
    Ok(())
}
