//! Planar geometry construction, evaluation and boundary contracts.
#![allow(non_snake_case)]
mod common;
use common::{EXPECT_DOUBLE_EQ, EXPECT_NEAR};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dConic, Geom2dCurve, Geom2dEllipse, Geom2dGeometry};
use martensite_tkmath::gp::{Axis2d, Direction2, Ellipse2, Point2, Transform2, Vector2};
use std::f64::consts::PI;

// Fixture: ellipse centered at origin, major radius 10, minor radius 5.
fn setup() -> Handle<Geom2dEllipse> {
    let anElips = Ellipse2::from_x_axis(
        Axis2d::new(Point2::new(0.0, 0.0), Direction2::new(1.0, 0.0).unwrap()),
        10.0,
        5.0,
        true,
    )
    .unwrap();
    Handle::new(Geom2dEllipse::from_elips2d(anElips))
}

#[test]
fn ellipse_radii() {
    let myEllipse = setup();
    EXPECT_NEAR(myEllipse.major_radius(), 10.0, precision::confusion());
    EXPECT_NEAR(myEllipse.minor_radius(), 5.0, precision::confusion());
}

#[test]
fn ellipse_parameter_bounds() {
    let myEllipse = setup();
    EXPECT_DOUBLE_EQ(myEllipse.first_parameter(), 0.0);
    EXPECT_DOUBLE_EQ(myEllipse.last_parameter(), 2.0 * PI);
}

#[test]
fn ellipse_is_closed_and_periodic() {
    let myEllipse = setup();
    assert!(myEllipse.is_closed().unwrap());
    assert!(myEllipse.is_periodic());
}

#[test]
fn ellipse_eccentricity() {
    let myEllipse = setup();
    // e = sqrt(1 - (b/a)^2) = sqrt(1 - 0.25) = sqrt(0.75)
    let anExpected: f64 = (1.0 - (5.0 * 5.0) / (10.0 * 10.0_f64)).sqrt();
    EXPECT_NEAR(
        myEllipse.eccentricity().unwrap(),
        anExpected,
        precision::confusion(),
    );
}

#[test]
fn ellipse_focal() {
    let myEllipse = setup();
    // Focal distance = sqrt(a^2 - b^2) * 2
    let anExpected: f64 = 2.0 * (10.0 * 10.0 - 5.0 * 5.0_f64).sqrt();
    EXPECT_NEAR(myEllipse.focal(), anExpected, precision::confusion());
}

#[test]
fn ellipse_foci() {
    let myEllipse = setup();
    let aFocus1: Point2 = myEllipse.focus1();
    let aFocus2: Point2 = myEllipse.focus2();
    let c: f64 = (100.0 - 25.0_f64).sqrt(); // sqrt(a^2 - b^2)

    EXPECT_NEAR(aFocus1.x(), c, precision::confusion());
    EXPECT_NEAR(aFocus1.y(), 0.0, precision::confusion());
    EXPECT_NEAR(aFocus2.x(), -c, precision::confusion());
    EXPECT_NEAR(aFocus2.y(), 0.0, precision::confusion());
}

#[test]
fn ellipse_parameter() {
    let myEllipse = setup();
    // Semi-latus rectum = b^2/a = 25/10 = 2.5
    EXPECT_NEAR(myEllipse.parameter(), 2.5, precision::confusion());
}

#[test]
fn ellipse_eval_d0_major_vertex() {
    let myEllipse = setup();
    // At U=0: point on major axis = (a, 0) = (10, 0)
    let aPnt: Point2 = myEllipse.eval_d0(0.0).unwrap();
    EXPECT_NEAR(aPnt.x(), 10.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 0.0, precision::confusion());
}

#[test]
fn ellipse_eval_d0_minor_vertex() {
    let myEllipse = setup();
    // At U=PI/2: point on minor axis = (0, b) = (0, 5)
    let aPnt: Point2 = myEllipse.eval_d0(PI / 2.0).unwrap();
    EXPECT_NEAR(aPnt.x(), 0.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 5.0, precision::confusion());
}

#[test]
fn ellipse_eval_d1_at_zero() {
    let myEllipse = setup();
    let aD1: Vector2 = myEllipse.eval_d1(0.0).unwrap().d1;
    // At U=0: tangent is (0, b) = (0, 5)
    EXPECT_NEAR(aD1.x(), 0.0, precision::confusion());
    EXPECT_NEAR(aD1.y(), 5.0, precision::confusion());
}

#[test]
fn ellipse_eval_d2_at_zero() {
    let myEllipse = setup();
    let aD2: Vector2 = myEllipse.eval_d2(0.0).unwrap().d2;
    // At U=0: second derivative is (-a, 0) = (-10, 0)
    EXPECT_NEAR(aD2.x(), -10.0, precision::confusion());
    EXPECT_NEAR(aD2.y(), 0.0, precision::confusion());
}

#[test]
fn ellipse_set_major_radius() {
    let myEllipse = setup();
    myEllipse.set_major_radius(20.0).unwrap();
    EXPECT_NEAR(myEllipse.major_radius(), 20.0, precision::confusion());
}

#[test]
fn ellipse_set_minor_radius() {
    let myEllipse = setup();
    myEllipse.set_minor_radius(3.0).unwrap();
    EXPECT_NEAR(myEllipse.minor_radius(), 3.0, precision::confusion());
}

#[test]
fn ellipse_point_on_ellipse_foci_distance_sum() {
    let myEllipse = setup();
    // For any point on ellipse: d(P, F1) + d(P, F2) = 2a
    let aF1: Point2 = myEllipse.focus1();
    let aF2: Point2 = myEllipse.focus2();

    let mut u: f64 = 0.0;
    while u < 2.0 * PI {
        let aPnt: Point2 = myEllipse.eval_d0(u).unwrap();
        let aSum: f64 = aPnt.distance(aF1) + aPnt.distance(aF2);
        EXPECT_NEAR(aSum, 2.0 * 10.0, precision::confusion());
        u += PI / 6.0;
    }
}

#[test]
fn ellipse_copy() {
    let myEllipse = setup();
    let aCopyGeom: Handle<dyn Geom2dGeometry> = myEllipse.copy().unwrap();
    let aCopy: Handle<Geom2dEllipse> = aCopyGeom.cast::<Geom2dEllipse>();

    assert!(!aCopy.is_null());
    EXPECT_NEAR(aCopy.major_radius(), 10.0, precision::confusion());
    EXPECT_NEAR(aCopy.minor_radius(), 5.0, precision::confusion());
}

#[test]
fn ellipse_transform_translation() {
    let myEllipse = setup();
    let mut aTrsf = Transform2::default();
    aTrsf.set_translation(Vector2::new(5.0, 10.0));
    myEllipse.transform(aTrsf).unwrap();

    EXPECT_NEAR(myEllipse.location().x(), 5.0, precision::confusion());
    EXPECT_NEAR(myEllipse.location().y(), 10.0, precision::confusion());
    EXPECT_NEAR(myEllipse.major_radius(), 10.0, precision::confusion());
}

#[test]
fn ellipse_reversed_parameter() {
    let myEllipse = setup();
    let aU: f64 = PI / 4.0;
    EXPECT_NEAR(
        myEllipse.reversed_parameter(aU).unwrap(),
        2.0 * PI - aU,
        precision::confusion(),
    );
}
