//! Independent polynomial and analytic identities; none uses the captured corpus values.
use martensite_tkernel::ncollection::Array1;
use martensite_tkg2d::geom2d::{
    Geom2dBezierCurve, Geom2dCircle, Geom2dCurve, Geom2dLine, Geom2dParabola,
};
use martensite_tkmath::gp::{Axis2d, Direction2, Point2};

#[test]
fn line_affine_equation_and_higher_derivatives() {
    let line = Geom2dLine::from_point(Point2::new(2., -3.), Direction2::new(3., 4.).unwrap());
    for u in [-10., -5., 0., 5., 10.] {
        let evaluation = line.eval_d3(u).unwrap();
        assert_eq!(
            (evaluation.point.x(), evaluation.point.y()),
            (2. + 3. * u / 5., -3. + 4. * u / 5.)
        );
        assert_eq!((evaluation.d1.x(), evaluation.d1.y()), (3. / 5., 4. / 5.));
        assert_eq!(
            (
                evaluation.d2.x(),
                evaluation.d2.y(),
                evaluation.d3.x(),
                evaluation.d3.y()
            ),
            (0., 0., 0., 0.)
        );
    }
}

#[test]
fn circle_axis_point_and_cyclic_derivatives() {
    let axis = Axis2d::new(Point2::new(1., -1.), Direction2::new(1., 0.).unwrap());
    let circle = Geom2dCircle::from_axis(axis, 2., true).unwrap();
    let evaluation = circle.eval_d3(0.).unwrap();
    assert_eq!((evaluation.point.x(), evaluation.point.y()), (3., -1.));
    assert_eq!((evaluation.d1.x(), evaluation.d1.y()), (0., 2.));
    assert_eq!((evaluation.d2.x(), evaluation.d2.y()), (-2., 0.));
    assert_eq!((evaluation.d3.x(), evaluation.d3.y()), (0., -2.));
}

#[test]
fn parabola_focal_half_has_quadratic_coordinate() {
    let axis = Axis2d::new(Point2::new(0., 0.), Direction2::new(1., 0.).unwrap());
    let parabola = Geom2dParabola::from_axis(axis, 0.5, true).unwrap();
    for u in [-2., -1., 0., 0.5, 2.] {
        let evaluation = parabola.eval_d3(u).unwrap();
        assert_eq!(
            (evaluation.point.x(), evaluation.point.y()),
            (u * u / 2., u)
        );
        assert_eq!((evaluation.d1.x(), evaluation.d1.y()), (u, 1.));
        assert_eq!(
            (
                evaluation.d2.x(),
                evaluation.d2.y(),
                evaluation.d3.x(),
                evaluation.d3.y()
            ),
            (1., 0., 0., 0.)
        );
    }
}

#[test]
fn quadratic_bezier_matches_bernstein_polynomial() {
    let curve = quadratic_curve();
    for u in [-0.5, 0., 0.125, 0.5, 1., 1.5] {
        let evaluation = curve.eval_d3(u).unwrap();
        assert_eq!(
            (evaluation.point.x(), evaluation.point.y()),
            (2. * u, 4. * u * (1. - u))
        );
        assert_eq!((evaluation.d1.x(), evaluation.d1.y()), (2., 4. - 8. * u));
        assert_eq!(
            (
                evaluation.d2.x(),
                evaluation.d2.y(),
                evaluation.d3.x(),
                evaluation.d3.y()
            ),
            (0., -8., 0., 0.)
        );
    }
}

fn quadratic_curve() -> Geom2dBezierCurve {
    let mut poles = Array1::<Point2>::with_bounds(1, 3).unwrap();
    for (index, pole) in [
        (1, Point2::new(0., 0.)),
        (2, Point2::new(1., 2.)),
        (3, Point2::new(2., 0.)),
    ] {
        poles.set_value(index, pole).unwrap();
    }
    Geom2dBezierCurve::new(&poles).unwrap()
}
