use super::{TopOpeBRepEdgesIntersector, TopOpeBRepPoint2d};
use crate::{top_ope_brep::TopOpeBRepP2DStatus, top_ope_brep_ds::TopOpeBRepDSTransition};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Point2, Point3};
impl TopOpeBRepEdgesIntersector {
    /// Reduces paired segment endpoints into their midpoint; `state.reduce_segment(&mut a, &mut b, &mut midpoint)?`.
    pub fn reduce_segment(
        &self,
        first: &mut TopOpeBRepPoint2d,
        last: &mut TopOpeBRepPoint2d,
        midpoint: &mut TopOpeBRepPoint2d,
    ) -> Result<bool, Failure> {
        if !first.is_point_of_segment() || first.status() != TopOpeBRepP2DStatus::SegmentFirst {
            return Ok(false);
        }
        if !last.is_point_of_segment() || last.status() != TopOpeBRepP2DStatus::SegmentLast {
            return Ok(false);
        }
        reduce_geometry(first, last, midpoint);
        for index in [1, 2] {
            reduce_edge(index, first, last, midpoint)?;
        }
        reduce_bookkeeping(first, last, midpoint);
        Ok(true)
    }
}
fn reduce_geometry(
    first: &TopOpeBRepPoint2d,
    last: &TopOpeBRepPoint2d,
    midpoint: &mut TopOpeBRepPoint2d,
) {
    let (a, b) = (first.value(), last.value());
    midpoint.set_value(&Point3::new(
        (a.x() + b.x()) / 2.,
        (a.y() + b.y()) / 2.,
        (a.z() + b.z()) / 2.,
    ));
    let (a, b) = (first.value2d(), last.value2d());
    midpoint.set_value2d(&Point2::new((a.x() + b.x()) / 2., (a.y() + b.y()) / 2.));
}
fn reduce_edge(
    index: i32,
    first: &TopOpeBRepPoint2d,
    last: &TopOpeBRepPoint2d,
    midpoint: &mut TopOpeBRepPoint2d,
) -> Result<(), Failure> {
    midpoint.set_parameter(
        index,
        (first.parameter(index)? + last.parameter(index)?) / 2.,
    )?;
    let (a, b) = (first.transition(index)?, last.transition(index)?);
    let mut transition = TopOpeBRepDSTransition::default();
    transition.set_before(a.before(), a.shape_before());
    transition.set_after(b.after(), b.shape_after());
    midpoint.set_transition(index, &transition)?;
    for endpoint in [first, last] {
        if endpoint.is_vertex(index)? {
            midpoint.set_vertex(index, endpoint.vertex(index)?)?;
        }
    }
    Ok(())
}

fn reduce_bookkeeping(
    first: &mut TopOpeBRepPoint2d,
    last: &mut TopOpeBRepPoint2d,
    midpoint: &mut TopOpeBRepPoint2d,
) {
    midpoint.set_tolerance((first.tolerance() + last.tolerance()) * 1.5);
    midpoint.set_is_point_of_segment(false);
    midpoint.set_segment_ancestors(first.index(), last.index());
    midpoint.set_edges_config(first.edges_config());
    first.set_keep(false);
    last.set_keep(false);
}
