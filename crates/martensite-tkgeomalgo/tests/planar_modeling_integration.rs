//! Geometry regression fixtures separated by their actual algorithm dependencies.
//! Original inputs/assertions.
#![allow(non_snake_case)]
#[path = "common/mod.rs"]
mod common;
use common::{EXPECT_NEAR as legacy_near, modeling_circle as legacy_circle};
use martensite_tkernel::{
    ncollection::{Array1, List},
    precision,
    standard::Handle,
};
use martensite_tkg2d::{
    geom2d::{
        Geom2dBSplineCurve, Geom2dBezierCurve, Geom2dCurve, Geom2dEllipse, Geom2dLine,
        Geom2dTrimmedCurve,
    },
    geom2d_adaptor::Geom2dAdaptorCurve,
};
use martensite_tkgeomalgo::{
    gcc_ent::Position,
    geom2d_api::{ExtremaCurveCurve, InterCurveCurve, PointsToBSpline, ProjectPointOnCurve},
    geom2d_gcc::{Circ2d2TanOn, Circ2d2TanRad, QualifiedCurve},
};
use martensite_tkmath::{
    geom_abs::Shape,
    gp::{Axis2d, Circle2, Direction2, Point2, Vector2},
};

fn legacy_ellipse(center: Point2) -> Handle<dyn Geom2dCurve> {
    Handle::new(
        Geom2dEllipse::from_axis(Axis2d::new(center, Direction2::X), 2.0, 1.0, true).unwrap(),
    )
    .cast()
}

fn legacy_extrema(
    first: &Handle<dyn Geom2dCurve>,
    second: &Handle<dyn Geom2dCurve>,
) -> ExtremaCurveCurve {
    ExtremaCurveCurve::new(
        first,
        second,
        first.first_parameter(),
        first.last_parameter(),
        second.first_parameter(),
        second.last_parameter(),
    )
    .unwrap()
}

fn legacy_qualified(curve: &Handle<dyn Geom2dCurve>) -> QualifiedCurve {
    QualifiedCurve::new(
        &Geom2dAdaptorCurve::with_curve(curve).unwrap(),
        Position::Unqualified,
    )
    .unwrap()
}

fn legacy_bezier(poles: &Array1<'_, Point2>) -> Handle<dyn Geom2dCurve> {
    let curve = Geom2dBezierCurve::new(poles).unwrap();
    Handle::new(curve).cast()
}

fn tangent_solutions_at(
    first: &Handle<dyn Geom2dCurve>,
    second: &Handle<dyn Geom2dCurve>,
    u: f64,
) -> i32 {
    let (point, tangent) = first.d1(u).unwrap();
    let normal = Direction2::from_vector(Vector2::new(-tangent.y(), tangent.x())).unwrap();
    let line: Handle<dyn Geom2dCurve> = Handle::new(Geom2dLine::from_point(point, normal)).cast();
    let q1 = legacy_qualified(first);
    let q2 = legacy_qualified(second);
    let g1 = ProjectPointOnCurve::with_curve(point, first)
        .unwrap()
        .lower_distance_parameter()
        .unwrap();
    let g3 = ProjectPointOnCurve::with_curve(point, &line)
        .unwrap()
        .lower_distance_parameter()
        .unwrap();
    Circ2d2TanOn::new(
        &q1,
        &q2,
        &Geom2dAdaptorCurve::with_curve(&line).unwrap(),
        1e-9,
        [g1, g1, g3],
    )
    .unwrap()
    .nb_solutions()
}

fn legacy_interpolated(points: &[Point2]) -> Handle<Geom2dBSplineCurve> {
    let interpolation =
        PointsToBSpline::with_points(&Array1::from_shared(points, 1), 3, 8, Shape::C2, 1e-6)
            .unwrap();
    assert!(interpolation.is_done());
    interpolation.curve().unwrap()
}

#[test]
// The case preserves the original decimal boundary, which differs from FRAC_PI_2.
#[allow(clippy::approx_constant)]
fn Geom2dAPI_InterCurveCurve_Test__OCC24889_IntersectionParameterWithinLimits() {
    // Intersection parameters must remain within both trimmed domains.
    let first = legacy_circle(Point2::new(25.0, -25.0), 155.0);
    let second = legacy_circle(Point2::new(25.0, 25.0), 155.0);
    let first: Handle<dyn Geom2dCurve> = Handle::new(
        Geom2dTrimmedCurve::new(&first, 1.57079632679490, 2.97959469729228, true, true).unwrap(),
    )
    .cast();
    let second: Handle<dyn Geom2dCurve> = Handle::new(
        Geom2dTrimmedCurve::new(&second, 3.30359060633978, 4.71238898038469, true, true).unwrap(),
    )
    .cast();
    let tool = InterCurveCurve::between(&first, &second, precision::confusion()).unwrap();
    assert!(tool.nb_points() > 0);
    let point = tool.intersector().point(1).unwrap();
    let (u, v) = (point.param_on_first(), point.param_on_second());
    assert!(u >= first.first_parameter());
    assert!(u <= first.last_parameter());
    assert!(v >= second.first_parameter());
    assert!(v <= second.last_parameter());
    assert!(
        first
            .value(u)
            .unwrap()
            .square_distance(second.value(v).unwrap())
            < 1e-14
    );
}

#[test]
fn Geom2dAPI_InterCurveCurve_Test__FClassesBug_25635_1_TangentEllipses() {
    let first = legacy_ellipse(Point2::default());
    let second = legacy_ellipse(Point2::new(4.0, 0.0));
    let extrema = legacy_extrema(&first, &second);
    assert!(extrema.nb_extrema() > 0);
    legacy_near(extrema.distance(1).unwrap(), 0.0, 7e-5);
}

#[test]
fn Geom2dAPI_InterCurveCurve_Test__FClassesBug_25635_2_TangentCircles() {
    let first = legacy_circle(Point2::default(), 2.0);
    let second = legacy_circle(Point2::new(4.0, 0.0), 2.0);
    let extrema = legacy_extrema(&first, &second);
    assert!(extrema.nb_extrema() >= 4);
    assert!(extrema.distance(2).unwrap() <= precision::p_confusion());
    let (first, second) = extrema.points(2).unwrap();
    assert!(first.is_equal(second, precision::p_confusion()));
}

#[test]
fn Geom2dGcc_Circ2d2TanOn_Test__OCC27357_NoExceptions() {
    // Each of the 100 sub-algorithm calls must complete without exception.
    // Reusing caller storage preserves the original constructor-copy boundary.
    let mut poles = Array1::with_bounds(1, 3).unwrap();
    poles[1] = Point2::default();
    poles[2] = Point2::new(0.0, 1.0);
    poles[3] = Point2::new(6.0, 0.0);
    let first = legacy_bezier(&poles);
    poles[2] = Point2::new(0.0, 1.5);
    let second = legacy_bezier(&poles);
    let mut dummy = List::default();
    for i in 0..100 {
        dummy.append(tangent_solutions_at(&first, &second, f64::from(i) / 99.0));
    }
}

#[test]
fn Geom2dGcc_Circ2d2TanRad_Test__OCC24303_CircleTangentToTwoEllipses() {
    let first = legacy_ellipse(Point2::default());
    let second = legacy_ellipse(Point2::new(4.0, 0.0));
    let theoretical =
        Circle2::from_x_axis(Axis2d::new(Point2::new(5.0, 0.0), Direction2::X), 3.0, true).unwrap();
    let calculator = Circ2d2TanRad::new(
        &legacy_qualified(&first),
        &legacy_qualified(&second),
        3.0,
        1e-9,
    )
    .unwrap();
    let count = calculator.nb_solutions();
    assert!(count > 0);
    for index in 1..=count {
        legacy_near(calculator.this_solution(index).unwrap().radius(), 3.0, 1e-6);
    }
    if count > 0 {
        assert!(
            theoretical
                .location()
                .distance(calculator.this_solution(1).unwrap().location())
                < 10.0
        );
    }
}

#[test]
fn Geom2d_CurveEvalTest__BSplineCurve_EvalD1_ConsistentWithOldAPI() {
    let curve = legacy_interpolated(&[
        Point2::default(),
        Point2::new(1.0, 2.0),
        Point2::new(3.0, 1.0),
        Point2::new(5.0, 3.0),
        Point2::new(7.0, 0.0),
    ]);
    let mid = (curve.first_parameter() + curve.last_parameter()) / 2.0;
    let result = curve.eval_d1(mid).unwrap();
    let (point, derivative) = curve.d1(mid).unwrap();
    legacy_near(result.point.distance(point), 0.0, precision::confusion());
    legacy_near(
        (result.d1 - derivative).magnitude(),
        0.0,
        precision::confusion(),
    );
}

#[test]
fn Geom2d_CurveEvalTest__BSplineCurve_EvalDN_InvalidN_Throws() {
    let curve = legacy_interpolated(&[
        Point2::default(),
        Point2::new(1.0, 1.0),
        Point2::new(2.0, 0.0),
    ]);
    let mid = (curve.first_parameter() + curve.last_parameter()) / 2.0;
    assert_eq!(
        curve.eval_dn(mid, 0).unwrap_err().exception_type(),
        "Geom2d_UndefinedDerivative"
    );
    assert_eq!(
        curve.eval_dn(mid, -1).unwrap_err().exception_type(),
        "Geom2d_UndefinedDerivative"
    );
}
