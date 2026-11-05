use martensite_tkernel::standard::Failure;
use martensite_tkg2d::{adaptor2d::Adaptor2dCurve2d, geom2d_adaptor::Geom2dAdaptorCurve};
use martensite_tkgeomalgo::int_res2d::{IntersectionPoint, Position, Transition, TypeTrans};
pub(super) fn filter_points(
    points: &mut Vec<IntersectionPoint>,
    curves: &[Geom2dAdaptorCurve; 2],
    tolerance: f64,
) -> Result<(), Failure> {
    loop {
        let mut remove = None;
        for (index, pair) in points.windows(2).enumerate() {
            let (a, b) = (pair[0], pair[1]);
            if same_extremity(a.transition_of_first(), b.transition_of_first())?
                || same_extremity(a.transition_of_second(), b.transition_of_second())?
            {
                remove = Some(index);
                break;
            }
            if tangent_segment(a, b, curves, tolerance)? {
                remove = Some(
                    index
                        + usize::from(
                            b.transition_of_first().transition_type() != TypeTrans::Touch,
                        ),
                );
                break;
            }
        }
        let Some(index) = remove else {
            return Ok(());
        };
        points.remove(index);
    }
}
fn same_extremity(a: Transition, b: Transition) -> Result<bool, Failure> {
    if !matches!(a.position_on_curve(), Position::Head | Position::End) {
        return Ok(false);
    }
    if a.position_on_curve() != b.position_on_curve() || a.transition_type() != b.transition_type()
    {
        return Ok(false);
    }
    if a.transition_type() != TypeTrans::Touch {
        return Ok(true);
    }
    Ok(a.is_tangent()? == b.is_tangent()?
        && a.situation()? == b.situation()?
        && a.is_opposite()? == b.is_opposite()?)
}
fn tangent_segment(
    a: IntersectionPoint,
    b: IntersectionPoint,
    curves: &[Geom2dAdaptorCurve; 2],
    tolerance: f64,
) -> Result<bool, Failure> {
    if a.transition_of_first().transition_type() != TypeTrans::Touch
        && b.transition_of_first().transition_type() != TypeTrans::Touch
    {
        return Ok(false);
    }
    if a.value().square_distance(b.value()) > tolerance {
        return Ok(false);
    }
    let delta = [
        a.param_on_first() - b.param_on_first(),
        a.param_on_second() - b.param_on_second(),
    ];
    Ok(delta[0] * delta[0] <= curves[0].resolution(tolerance)?
        && delta[1] * delta[1] <= curves[1].resolution(tolerance)?)
}
#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkgeomalgo::int_res2d::Situation;
    #[test]
    fn endpoint_filter_requires_matching_types_positions_and_touch_flags() -> Result<(), Failure> {
        let t = Transition::crossing(false, Position::Head, TypeTrans::In);
        assert!(same_extremity(t, t)?);
        let other = Transition::crossing(false, Position::End, TypeTrans::In);
        assert!(!same_extremity(t, other)?);
        let other = Transition::crossing(false, Position::Head, TypeTrans::Out);
        assert!(!same_extremity(t, other)?);
        let t = Transition::touch(true, Position::End, Situation::Inside, false);
        assert!(same_extremity(t, t)?);
        let other = Transition::touch(true, Position::End, Situation::Inside, true);
        assert!(!same_extremity(t, other)?);
        let middle = Transition::touch(true, Position::Middle, Situation::Inside, false);
        assert!(!same_extremity(middle, middle)?);
        Ok(())
    }
    #[test]
    fn tangent_filter_keeps_the_touch_record_and_requires_close_parameters() -> Result<(), Failure>
    {
        use martensite_tkernel::standard::Handle;
        use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
        use martensite_tkmath::gp::{Direction2, Point2};
        let line: Handle<dyn Geom2dCurve> =
            Handle::new(Geom2dLine::from_point(Point2::default(), Direction2::X)).cast();
        let curves = [Geom2dAdaptorCurve::default(), Geom2dAdaptorCurve::default()];
        for curve in &curves {
            curve.load(&line)?;
        }
        let touch = Transition::touch(true, Position::Middle, Situation::Inside, false);
        let crossing = Transition::crossing(false, Position::Middle, TypeTrans::In);
        let a = IntersectionPoint::new(Point2::default(), 0., 0., crossing, crossing, false);
        let b = IntersectionPoint::new(Point2::new(1e-6, 0.), 1e-6, 1e-6, touch, touch, false);
        let mut points = vec![a, b];
        filter_points(&mut points, &curves, 1e-4)?;
        assert_eq!(points, vec![b]);
        let mut points = vec![b, a];
        filter_points(&mut points, &curves, 1e-4)?;
        assert_eq!(points, vec![b]);
        let far = IntersectionPoint::new(Point2::default(), 2., 2., touch, touch, false);
        assert!(!tangent_segment(a, far, &curves, 1e-4)?);
        Ok(())
    }
}
