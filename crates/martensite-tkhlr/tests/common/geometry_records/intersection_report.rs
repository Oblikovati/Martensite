//! One ordered intersection report shared by owned sinks and historical stdout.
use martensite_tkernel::standard::Failure;
use martensite_tkgeomalgo::int_res2d::{
    Intersection, IntersectionPoint, IntersectionSegment, Transition, TypeTrans,
};
use martensite_tkmath::gp::Point2;

/// Typed report fields, preserving partial emission on failure; `emit.flag(true)`.
pub trait IntersectionEmitter {
    fn flag(&mut self, value: bool);
    fn int(&mut self, value: i32);
    fn real(&mut self, value: f64);
    fn pnt2d(&mut self, value: Point2);
}

/// Emit only defined transition fields; `transition(emit, &transition)`.
pub fn transition(emit: &mut impl IntersectionEmitter, t: &Transition) -> Result<(), Failure> {
    emit.int(t.position_on_curve() as i32);
    emit.int(t.transition_type() as i32);
    if t.transition_type() != TypeTrans::Undecided {
        emit.flag(t.is_tangent()?);
    }
    if t.transition_type() == TypeTrans::Touch {
        emit.int(t.situation()? as i32);
        emit.flag(t.is_opposite()?);
    }
    Ok(())
}
/// Emit coordinates, parameters and ordered transitions; `point(emit, &point)`.
pub fn point(emit: &mut impl IntersectionEmitter, p: &IntersectionPoint) -> Result<(), Failure> {
    emit.pnt2d(p.value());
    emit.real(p.param_on_first());
    emit.real(p.param_on_second());
    transition(emit, &p.transition_of_first())?;
    transition(emit, &p.transition_of_second())
}
/// Emit only present segment endpoints; `segment(emit, &segment)`.
pub fn segment(
    emit: &mut impl IntersectionEmitter,
    s: &IntersectionSegment,
) -> Result<(), Failure> {
    emit.flag(s.is_opposite());
    emit.flag(s.has_first_point());
    if s.has_first_point() {
        point(emit, &s.first_point()?)?;
    }
    emit.flag(s.has_last_point());
    if s.has_last_point() {
        point(emit, &s.last_point()?)?;
    }
    Ok(())
}
/// Emit completion, then ordered points and segments; `intersection(emit, &result)`.
pub fn intersection(emit: &mut impl IntersectionEmitter, r: &Intersection) -> Result<(), Failure> {
    emit.flag(r.is_done());
    if !r.is_done() {
        return Ok(());
    }
    emit.int(r.nb_points()?);
    for i in 1..=r.nb_points()? {
        point(emit, &r.point(i)?)?;
    }
    emit.int(r.nb_segments()?);
    for i in 1..=r.nb_segments()? {
        segment(emit, &r.segment(i)?)?;
    }
    Ok(())
}
