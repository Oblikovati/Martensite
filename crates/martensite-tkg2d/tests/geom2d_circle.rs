//! Planar geometry construction, evaluation and boundary contracts.
#![allow(non_snake_case)]
mod common;
use common::{EXPECT_DOUBLE_EQ, EXPECT_NEAR};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dConic, Geom2dCurve, Geom2dGeometry};
use martensite_tkmath::gp::{Axis2d, Axis22d, Circle2, Direction2, Point2, Transform2, Vector2};
use std::f64::consts::PI;

// Fixture: circle centered at origin, radius 5.
fn setup() -> Handle<Geom2dCircle> {
    let aCirc = Circle2::from_x_axis(
        Axis2d::new(Point2::new(0.0, 0.0), Direction2::new(1.0, 0.0).unwrap()),
        5.0,
        true,
    )
    .unwrap();
    Handle::new(Geom2dCircle::from_circ2d(aCirc))
}

#[test]
fn circle_construct_from_circ2d() {
    let myCircle = setup();
    EXPECT_NEAR(myCircle.radius(), 5.0, precision::confusion());
}

#[test]
fn circle_construct_from_axis_and_radius() {
    let aCircle = Handle::new(
        Geom2dCircle::from_axis(
            Axis2d::new(Point2::new(1.0, 2.0), Direction2::new(1.0, 0.0).unwrap()),
            3.0,
            true,
        )
        .unwrap(),
    );

    EXPECT_NEAR(aCircle.radius(), 3.0, precision::confusion());
    EXPECT_NEAR(aCircle.location().x(), 1.0, precision::confusion());
    EXPECT_NEAR(aCircle.location().y(), 2.0, precision::confusion());
}

#[test]
fn circle_construct_from_ax22d() {
    let anAxis = Axis22d::with_directions(
        Point2::new(3.0, 4.0),
        Direction2::new(1.0, 0.0).unwrap(),
        Direction2::new(0.0, 1.0).unwrap(),
    )
    .unwrap();
    let aCircle = Handle::new(Geom2dCircle::new(anAxis, 7.0).unwrap());

    EXPECT_NEAR(aCircle.radius(), 7.0, precision::confusion());
    EXPECT_NEAR(aCircle.location().x(), 3.0, precision::confusion());
}

#[test]
fn circle_parameter_bounds() {
    let myCircle = setup();
    EXPECT_DOUBLE_EQ(myCircle.first_parameter(), 0.0);
    EXPECT_DOUBLE_EQ(myCircle.last_parameter(), 2.0 * PI);
}

#[test]
fn circle_is_closed_and_periodic() {
    let myCircle = setup();
    assert!(myCircle.is_closed().unwrap());
    assert!(myCircle.is_periodic());
}

#[test]
fn circle_eccentricity() {
    let myCircle = setup();
    EXPECT_NEAR(
        myCircle.eccentricity().unwrap(),
        0.0,
        precision::confusion(),
    );
}

#[test]
fn circle_set_radius() {
    let myCircle = setup();
    myCircle.set_radius(10.0).unwrap();
    EXPECT_NEAR(myCircle.radius(), 10.0, precision::confusion());
}

#[test]
fn circle_circ2d() {
    let myCircle = setup();
    let aCirc: Circle2 = myCircle.circ2d().unwrap();
    EXPECT_NEAR(aCirc.radius(), 5.0, precision::confusion());
}

#[test]
fn circle_eval_d0_at_zero() {
    let myCircle = setup();
    let aPnt: Point2 = myCircle.eval_d0(0.0).unwrap();
    EXPECT_NEAR(aPnt.x(), 5.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 0.0, precision::confusion());
}

#[test]
fn circle_eval_d0_at_pi_half() {
    let myCircle = setup();
    let aPnt: Point2 = myCircle.eval_d0(PI / 2.0).unwrap();
    EXPECT_NEAR(aPnt.x(), 0.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 5.0, precision::confusion());
}

#[test]
fn circle_eval_d0_at_pi() {
    let myCircle = setup();
    let aPnt: Point2 = myCircle.eval_d0(PI).unwrap();
    EXPECT_NEAR(aPnt.x(), -5.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 0.0, precision::confusion());
}

#[test]
fn circle_eval_d1_at_zero() {
    let myCircle = setup();
    let aD1: Vector2 = myCircle.eval_d1(0.0).unwrap().d1;
    // Tangent at U=0 is (0, R) = (0, 5)
    EXPECT_NEAR(aD1.x(), 0.0, precision::confusion());
    EXPECT_NEAR(aD1.y(), 5.0, precision::confusion());
}

#[test]
fn circle_eval_d2_at_zero() {
    let myCircle = setup();
    let aD2: Vector2 = myCircle.eval_d2(0.0).unwrap().d2;
    // Second derivative at U=0 is (-R, 0) = (-5, 0)
    EXPECT_NEAR(aD2.x(), -5.0, precision::confusion());
    EXPECT_NEAR(aD2.y(), 0.0, precision::confusion());
}

#[test]
fn circle_eval_dn_order1() {
    let myCircle = setup();
    let aD1: Vector2 = myCircle.eval_dn(0.0, 1).unwrap();
    EXPECT_NEAR(aD1.x(), 0.0, precision::confusion());
    EXPECT_NEAR(aD1.y(), 5.0, precision::confusion());
}

#[test]
fn circle_reversed_parameter() {
    let myCircle = setup();
    let aU: f64 = PI / 3.0;
    let aRev: f64 = myCircle.reversed_parameter(aU).unwrap();
    EXPECT_NEAR(aRev, 2.0 * PI - aU, precision::confusion());
}

#[test]
fn circle_copy() {
    let myCircle = setup();
    let aCopyGeom: Handle<dyn Geom2dGeometry> = myCircle.copy().unwrap();
    let aCopy: Handle<Geom2dCircle> = aCopyGeom.cast::<Geom2dCircle>();

    assert!(!aCopy.is_null());
    EXPECT_NEAR(aCopy.radius(), 5.0, precision::confusion());

    // Verify independence
    aCopy.set_radius(1.0).unwrap();
    EXPECT_NEAR(myCircle.radius(), 5.0, precision::confusion());
}

#[test]
fn circle_transform_translation() {
    let myCircle = setup();
    let mut aTrsf = Transform2::default();
    aTrsf.set_translation(Vector2::new(10.0, 20.0));
    myCircle.transform(aTrsf).unwrap();

    EXPECT_NEAR(myCircle.location().x(), 10.0, precision::confusion());
    EXPECT_NEAR(myCircle.location().y(), 20.0, precision::confusion());
    EXPECT_NEAR(myCircle.radius(), 5.0, precision::confusion());
}

#[test]
fn circle_point_on_circle_distance_from_center() {
    let myCircle = setup();
    // All points on circle should be at distance R from center
    let mut u: f64 = 0.0;
    while u < 2.0 * PI {
        let aPnt: Point2 = myCircle.eval_d0(u).unwrap();
        let aDist: f64 = aPnt.distance(myCircle.location());
        EXPECT_NEAR(aDist, 5.0, precision::confusion());
        u += PI / 6.0;
    }
}
