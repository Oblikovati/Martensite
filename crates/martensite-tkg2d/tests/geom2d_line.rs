//! Planar geometry construction, evaluation and boundary contracts.
#![allow(non_snake_case)]
mod common;
use common::{EXPECT_DOUBLE_EQ, EXPECT_NEAR};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dGeometry, Geom2dLine};
use martensite_tkmath::geom_abs::Shape;
use martensite_tkmath::gp::{Axis2d, Direction2, Line2, Point2, Transform2, Vector2};
use std::f64::consts::PI;

fn dir(x: f64, y: f64) -> Direction2 {
    Direction2::new(x, y).unwrap()
}

// Fixture: horizontal line through origin.
fn setup() -> Handle<Geom2dLine> {
    Handle::new(Geom2dLine::from_point(Point2::new(0.0, 0.0), dir(1.0, 0.0)))
}

#[test]
fn line_construct_from_point_and_dir() {
    let myLine = setup();
    EXPECT_NEAR(myLine.direction().x(), 1.0, precision::confusion());
    EXPECT_NEAR(myLine.direction().y(), 0.0, precision::confusion());
    EXPECT_NEAR(myLine.location().x(), 0.0, precision::confusion());
    EXPECT_NEAR(myLine.location().y(), 0.0, precision::confusion());
}

#[test]
fn line_construct_from_lin2d() {
    let aLin = Line2::new(Point2::new(1.0, 2.0), dir(0.0, 1.0));
    let aLine = Handle::new(Geom2dLine::from_lin2d(aLin));

    EXPECT_NEAR(aLine.direction().x(), 0.0, precision::confusion());
    EXPECT_NEAR(aLine.direction().y(), 1.0, precision::confusion());
}

#[test]
fn line_construct_from_axis() {
    let anAxis = Axis2d::new(Point2::new(3.0, 4.0), dir(1.0, 0.0));
    let aLine = Handle::new(Geom2dLine::new(anAxis));

    EXPECT_NEAR(aLine.location().x(), 3.0, precision::confusion());
}

#[test]
fn line_parameter_bounds() {
    let myLine = setup();
    // Curve parameters use large finite bounds.
    assert!(myLine.first_parameter() < -1e+90);
    assert!(myLine.last_parameter() > 1e+90);
}

#[test]
fn line_is_not_closed_not_periodic() {
    let myLine = setup();
    assert!(!myLine.is_closed().unwrap());
    assert!(!myLine.is_periodic());
}

#[test]
fn line_continuity() {
    let myLine = setup();
    assert_eq!(myLine.continuity(), Shape::CN);
    assert!(myLine.is_cn(100).unwrap());
}

#[test]
fn line_eval_d0() {
    let myLine = setup();
    let aPnt: Point2 = myLine.eval_d0(5.0).unwrap();
    EXPECT_NEAR(aPnt.x(), 5.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 0.0, precision::confusion());
}

#[test]
fn line_eval_d1_direction_is_constant() {
    let myLine = setup();
    let aD1: Vector2 = myLine.eval_d1(5.0).unwrap().d1;
    EXPECT_NEAR(aD1.x(), 1.0, precision::confusion());
    EXPECT_NEAR(aD1.y(), 0.0, precision::confusion());
}

#[test]
fn line_eval_d2_is_zero() {
    let myLine = setup();
    let aD2: Vector2 = myLine.eval_d2(5.0).unwrap().d2;
    EXPECT_NEAR(aD2.magnitude(), 0.0, precision::confusion());
}

#[test]
fn line_eval_d3_is_zero() {
    let myLine = setup();
    let aD3: Vector2 = myLine.eval_d3(5.0).unwrap().d3;
    EXPECT_NEAR(aD3.magnitude(), 0.0, precision::confusion());
}

#[test]
fn line_eval_dn_higher_order() {
    let myLine = setup();
    let aD4: Vector2 = myLine.eval_dn(0.0, 4).unwrap();
    EXPECT_NEAR(aD4.magnitude(), 0.0, precision::confusion());
}

#[test]
fn line_distance() {
    let myLine = setup();
    EXPECT_NEAR(
        myLine.distance(Point2::new(5.0, 3.0)),
        3.0,
        precision::confusion(),
    );
    EXPECT_NEAR(
        myLine.distance(Point2::new(0.0, 0.0)),
        0.0,
        precision::confusion(),
    );
}

#[test]
fn line_set_direction() {
    let myLine = setup();
    myLine.set_direction(dir(0.0, 1.0));
    EXPECT_NEAR(myLine.direction().x(), 0.0, precision::confusion());
    EXPECT_NEAR(myLine.direction().y(), 1.0, precision::confusion());
}

#[test]
fn line_set_location() {
    let myLine = setup();
    myLine.set_location(Point2::new(3.0, 4.0));
    EXPECT_NEAR(myLine.location().x(), 3.0, precision::confusion());
    EXPECT_NEAR(myLine.location().y(), 4.0, precision::confusion());
}

#[test]
fn line_lin2d() {
    let myLine = setup();
    let aLin: Line2 = myLine.lin2d();
    EXPECT_NEAR(aLin.direction().x(), 1.0, precision::confusion());
    EXPECT_NEAR(aLin.location().x(), 0.0, precision::confusion());
}

#[test]
fn line_copy() {
    let myLine = setup();
    let aCopyGeom: Handle<dyn Geom2dGeometry> = myLine.copy().unwrap();
    let aCopy: Handle<Geom2dLine> = aCopyGeom.cast::<Geom2dLine>();

    assert!(!aCopy.is_null());
    EXPECT_NEAR(aCopy.direction().x(), 1.0, precision::confusion());

    aCopy.set_direction(dir(0.0, 1.0));
    EXPECT_NEAR(myLine.direction().x(), 1.0, precision::confusion());
}

#[test]
fn line_transform_translation() {
    let myLine = setup();
    let mut aTrsf = Transform2::default();
    aTrsf.set_translation(Vector2::new(5.0, 10.0));
    myLine.transform(aTrsf).unwrap();

    EXPECT_NEAR(myLine.location().x(), 5.0, precision::confusion());
    EXPECT_NEAR(myLine.location().y(), 10.0, precision::confusion());
}

#[test]
fn line_transform_rotation() {
    let myLine = setup();
    let mut aTrsf = Transform2::default();
    aTrsf.set_rotation(Point2::new(0.0, 0.0), PI / 2.0);
    myLine.transform(aTrsf).unwrap();

    // Horizontal line rotated 90 degrees becomes vertical
    EXPECT_NEAR(myLine.direction().x(), 0.0, precision::confusion());
    EXPECT_NEAR(myLine.direction().y().abs(), 1.0, precision::confusion());
}

#[test]
fn line_reversed_parameter() {
    let myLine = setup();
    EXPECT_DOUBLE_EQ(myLine.reversed_parameter(5.0).unwrap(), -5.0);
}

#[test]
fn line_diagonal_line_value() {
    let aDiag = Handle::new(Geom2dLine::from_point(Point2::new(0.0, 0.0), dir(1.0, 1.0)));

    let aPnt: Point2 = aDiag.eval_d0(2.0_f64.sqrt()).unwrap();
    EXPECT_NEAR(aPnt.x(), 1.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 1.0, precision::confusion());
}
