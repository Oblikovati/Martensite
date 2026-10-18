//! Planar geometry construction, evaluation and boundary contracts.
#![allow(non_snake_case)]
mod common;
use common::{EXPECT_DOUBLE_EQ, EXPECT_NEAR};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{
    Geom2dBoundedCurve, Geom2dCircle, Geom2dCurve, Geom2dGeometry, Geom2dLine, Geom2dTrimmedCurve,
};
use martensite_tkmath::geom_abs::Shape;
use martensite_tkmath::gp::{Axis2d, Circle2, Direction2, Point2, Transform2, Vector2};
use std::f64::consts::PI;

// Geom2dTrimmedCurveTest fixture: a line along X and a circle of radius 5 at the origin.
struct Fixture {
    myLine: Handle<Geom2dLine>,
    myCircle: Handle<Geom2dCircle>,
}

fn setup() -> Fixture {
    // Line through origin along X
    let myLine = Handle::new(Geom2dLine::from_point(
        Point2::new(0.0, 0.0),
        Direction2::new(1.0, 0.0).unwrap(),
    ));

    // Circle centered at origin, radius 5
    let aCirc = Circle2::from_x_axis(
        Axis2d::new(Point2::new(0.0, 0.0), Direction2::new(1.0, 0.0).unwrap()),
        5.0,
        true,
    )
    .unwrap();
    let myCircle = Handle::new(Geom2dCircle::from_circ2d(aCirc));
    Fixture { myLine, myCircle }
}

// `new Geom2dTrimmedCurve(C, U1, U2)` with the default Sense and theAdjustPeriodic.
fn trim<T: Geom2dCurve + martensite_tkernel::standard::RuntimeTyped>(
    curve: &Handle<T>,
    u1: f64,
    u2: f64,
) -> Handle<Geom2dTrimmedCurve> {
    Handle::new(Geom2dTrimmedCurve::new(&curve.cast(), u1, u2, true, true).unwrap())
}

#[test]
fn trimmed_curve_construct_from_line() {
    let f = setup();
    let aTrim = trim(&f.myLine, 0.0, 10.0);

    EXPECT_DOUBLE_EQ(aTrim.first_parameter(), 0.0);
    EXPECT_DOUBLE_EQ(aTrim.last_parameter(), 10.0);
}

#[test]
fn trimmed_curve_construct_from_circle() {
    let f = setup();
    let aTrim = trim(&f.myCircle, 0.0, PI);

    EXPECT_DOUBLE_EQ(aTrim.first_parameter(), 0.0);
    EXPECT_DOUBLE_EQ(aTrim.last_parameter(), PI);
}

#[test]
fn trimmed_curve_start_and_end_points() {
    let f = setup();
    let aTrim = trim(&f.myLine, 2.0, 8.0);

    let aStart: Point2 = aTrim.start_point().unwrap();
    let anEnd: Point2 = aTrim.end_point().unwrap();

    EXPECT_NEAR(aStart.x(), 2.0, precision::confusion());
    EXPECT_NEAR(anEnd.x(), 8.0, precision::confusion());
}

#[test]
fn trimmed_curve_basis_curve() {
    let f = setup();
    let aTrim = trim(&f.myCircle, 0.0, PI);

    let aBasis: Handle<dyn Geom2dCurve> = aTrim.basis_curve();
    assert!(!aBasis.is_null());
}

#[test]
fn trimmed_curve_eval_d0_matches_basis_curve() {
    let f = setup();
    let aTrim = trim(&f.myCircle, 0.0, PI);

    let mut u: f64 = 0.0;
    while u <= PI {
        let aTrimPnt: Point2 = aTrim.eval_d0(u).unwrap();
        let aBasePnt: Point2 = f.myCircle.eval_d0(u).unwrap();
        assert!(aTrimPnt.is_equal(aBasePnt, precision::confusion()));
        u += PI / 4.0;
    }
}

#[test]
fn trimmed_curve_eval_d1_matches_basis_curve() {
    let f = setup();
    let aTrim = trim(&f.myLine, 0.0, 10.0);

    let aTrimRes = aTrim.eval_d1(5.0).unwrap();
    let aBaseRes = f.myLine.eval_d1(5.0).unwrap();
    let (aTrimPnt, aTrimD1): (Point2, Vector2) = (aTrimRes.point, aTrimRes.d1);
    let (aBasePnt, aBaseD1): (Point2, Vector2) = (aBaseRes.point, aBaseRes.d1);

    assert!(aTrimPnt.is_equal(aBasePnt, precision::confusion()));
    EXPECT_NEAR(aTrimD1.x(), aBaseD1.x(), precision::confusion());
    EXPECT_NEAR(aTrimD1.y(), aBaseD1.y(), precision::confusion());
}

#[test]
fn trimmed_curve_continuity() {
    let f = setup();
    let aTrim = trim(&f.myLine, 0.0, 10.0);
    assert_eq!(aTrim.continuity(), Shape::CN);
}

#[test]
fn trimmed_curve_is_cn() {
    let f = setup();
    let aTrim = trim(&f.myLine, 0.0, 10.0);
    assert!(aTrim.is_cn(10).unwrap());
}

#[test]
fn trimmed_curve_is_closed_open_arc() {
    let f = setup();
    let aTrim = trim(&f.myCircle, 0.0, PI);
    assert!(!aTrim.is_closed().unwrap());
}

#[test]
fn trimmed_curve_is_closed_full_circle() {
    let f = setup();
    // RepCurveDescFull-period trim on a periodic curve should be reported as closed
    let aTrim = trim(&f.myCircle, 0.0, 2.0 * PI);

    assert!(aTrim.is_closed().unwrap());
}

#[test]
fn trimmed_curve_is_periodic_partial_arc() {
    let f = setup();
    let aTrim = trim(&f.myCircle, 0.0, PI);
    assert!(!aTrim.is_periodic());
}

#[test]
fn trimmed_curve_is_periodic_full_period() {
    let f = setup();
    // RepCurveDescFull-period trim on a periodic curve should be reported as periodic
    let aTrim = trim(&f.myCircle, 0.0, 2.0 * PI);
    assert!(aTrim.is_periodic());
}

#[test]
fn trimmed_curve_is_periodic_line() {
    let f = setup();
    // Line is not periodic, so trimmed line should not be periodic
    let aTrim = trim(&f.myLine, 0.0, 10.0);
    assert!(!aTrim.is_periodic());
}

#[test]
fn trimmed_curve_set_trim() {
    let f = setup();
    let aTrim = trim(&f.myLine, 0.0, 10.0);
    aTrim.set_trim(3.0, 7.0, true, true).unwrap();

    EXPECT_DOUBLE_EQ(aTrim.first_parameter(), 3.0);
    EXPECT_DOUBLE_EQ(aTrim.last_parameter(), 7.0);
}

#[test]
fn trimmed_curve_copy() {
    let f = setup();
    let aTrim = trim(&f.myCircle, 0.0, PI);
    let aCopyGeom: Handle<dyn Geom2dGeometry> = aTrim.copy().unwrap();
    let aCopy: Handle<Geom2dTrimmedCurve> = aCopyGeom.cast::<Geom2dTrimmedCurve>();

    assert!(!aCopy.is_null());
    EXPECT_DOUBLE_EQ(aCopy.first_parameter(), 0.0);
    EXPECT_DOUBLE_EQ(aCopy.last_parameter(), PI);
}

#[test]
fn trimmed_curve_transform_translation() {
    let f = setup();
    let aTrim = trim(&f.myLine, 0.0, 10.0);

    let mut aTrsf = Transform2::default();
    aTrsf.set_translation(Vector2::new(5.0, 5.0));
    aTrim.transform(aTrsf).unwrap();

    let aStart: Point2 = aTrim.start_point().unwrap();
    EXPECT_NEAR(aStart.x(), 5.0, precision::confusion());
    EXPECT_NEAR(aStart.y(), 5.0, precision::confusion());
}

#[test]
fn trimmed_curve_circle_arc_start_end_points() {
    let f = setup();
    // Quarter circle arc
    let aTrim = trim(&f.myCircle, 0.0, PI / 2.0);

    let aStart: Point2 = aTrim.start_point().unwrap();
    let anEnd: Point2 = aTrim.end_point().unwrap();

    EXPECT_NEAR(aStart.x(), 5.0, precision::confusion());
    EXPECT_NEAR(aStart.y(), 0.0, precision::confusion());
    EXPECT_NEAR(anEnd.x(), 0.0, precision::confusion());
    EXPECT_NEAR(anEnd.y(), 5.0, precision::confusion());
}
