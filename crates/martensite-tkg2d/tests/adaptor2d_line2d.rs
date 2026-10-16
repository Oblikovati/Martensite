//! Planar geometry construction, evaluation and boundary contracts.
#![allow(non_snake_case)]
mod common;
use common::{EXPECT_DOUBLE_EQ, EXPECT_NEAR};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::adaptor2d::{Adaptor2dCurve2d, Adaptor2dLine2d};
use martensite_tkmath::geom_abs::{CurveType, Shape};
use martensite_tkmath::gp::{Direction2, Line2, Point2};

fn dir(x: f64, y: f64) -> Direction2 {
    Direction2::new(x, y).unwrap()
}

// Adaptor2dLine2dTest::SetUp: the fixture lines.
struct Fixture {
    myLine: Handle<Adaptor2dLine2d>,
    myDiagLine: Handle<Adaptor2dLine2d>,
}

fn setup() -> Fixture {
    // Line through origin along direction (1, 0) with bounds [0, 10]
    let myLine = Handle::new(Adaptor2dLine2d::new(
        Point2::new(0.0, 0.0),
        dir(1.0, 0.0),
        0.0,
        10.0,
    ));
    // Line through (1, 2) along direction (3, 4)/5 with bounds [-5, 5]
    let myDiagLine = Handle::new(Adaptor2dLine2d::new(
        Point2::new(1.0, 2.0),
        dir(3.0 / 5.0, 4.0 / 5.0),
        -5.0,
        5.0,
    ));
    Fixture { myLine, myDiagLine }
}

#[test]
fn adapted_line2d_default_constructor() {
    let _fixture = setup();
    let aLine = Adaptor2dLine2d::default();
    assert_eq!(aLine.get_type().unwrap(), CurveType::Line);
}

#[test]
fn adapted_line2d_parameter_bounds() {
    let f = setup();
    EXPECT_DOUBLE_EQ(f.myLine.first_parameter().unwrap(), 0.0);
    EXPECT_DOUBLE_EQ(f.myLine.last_parameter().unwrap(), 10.0);
    EXPECT_DOUBLE_EQ(f.myDiagLine.first_parameter().unwrap(), -5.0);
    EXPECT_DOUBLE_EQ(f.myDiagLine.last_parameter().unwrap(), 5.0);
}

#[test]
fn adapted_line2d_continuity() {
    let f = setup();
    assert_eq!(f.myLine.continuity().unwrap(), Shape::CN);
}

#[test]
fn adapted_line2d_nb_intervals() {
    let f = setup();
    assert_eq!(f.myLine.nb_intervals(Shape::C0).unwrap(), 1);
    assert_eq!(f.myLine.nb_intervals(Shape::CN).unwrap(), 1);
}

#[test]
fn adapted_line2d_intervals() {
    let f = setup();
    let mut anIntervals = Array1::<f64>::with_bounds(1, 2).unwrap();
    f.myLine.intervals(&mut anIntervals, Shape::CN).unwrap();
    EXPECT_DOUBLE_EQ(anIntervals[1], 0.0);
    EXPECT_DOUBLE_EQ(anIntervals[2], 10.0);
}

#[test]
fn adapted_line2d_is_not_closed() {
    let f = setup();
    assert!(!f.myLine.is_closed().unwrap());
}

#[test]
fn adapted_line2d_is_not_periodic() {
    let f = setup();
    assert!(!f.myLine.is_periodic().unwrap());
}

#[test]
fn adapted_line2d_get_type() {
    let f = setup();
    assert_eq!(f.myLine.get_type().unwrap(), CurveType::Line);
}

#[test]
fn adapted_line2d_value_at_origin() {
    let f = setup();
    let aPnt = f.myLine.value(0.0).unwrap();
    EXPECT_NEAR(aPnt.x(), 0.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 0.0, precision::confusion());
}

#[test]
fn adapted_line2d_value_at_parameter() {
    let f = setup();
    let aPnt = f.myLine.value(5.0).unwrap();
    EXPECT_NEAR(aPnt.x(), 5.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 0.0, precision::confusion());
}

#[test]
fn adapted_line2d_value_diagonal_line() {
    let f = setup();
    // At U=0, point should be origin (1, 2)
    let aPnt0 = f.myDiagLine.value(0.0).unwrap();
    EXPECT_NEAR(aPnt0.x(), 1.0, precision::confusion());
    EXPECT_NEAR(aPnt0.y(), 2.0, precision::confusion());

    // At U=5, point should be (1 + 3, 2 + 4) = (4, 6)
    let aPnt5 = f.myDiagLine.value(5.0).unwrap();
    EXPECT_NEAR(aPnt5.x(), 4.0, precision::confusion());
    EXPECT_NEAR(aPnt5.y(), 6.0, precision::confusion());
}

#[test]
fn adapted_line2d_d0() {
    let f = setup();
    let aPnt = f.myLine.d0(3.0).unwrap();
    EXPECT_NEAR(aPnt.x(), 3.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 0.0, precision::confusion());
}

#[test]
fn adapted_line2d_d1_first_derivative_is_direction() {
    let f = setup();
    let (aPnt, aVec) = f.myLine.d1(5.0).unwrap();

    EXPECT_NEAR(aPnt.x(), 5.0, precision::confusion());
    EXPECT_NEAR(aVec.x(), 1.0, precision::confusion());
    EXPECT_NEAR(aVec.y(), 0.0, precision::confusion());
}

#[test]
fn adapted_line2d_d2_second_derivative_is_zero() {
    let f = setup();
    let (_aPnt, _aV1, aV2) = f.myLine.d2(5.0).unwrap();

    EXPECT_NEAR(aV2.magnitude(), 0.0, precision::confusion());
}

#[test]
fn adapted_line2d_d3_third_derivative_is_zero() {
    let f = setup();
    let (_aPnt, _aV1, _aV2, aV3) = f.myLine.d3(5.0).unwrap();

    EXPECT_NEAR(aV3.magnitude(), 0.0, precision::confusion());
}

#[test]
fn adapted_line2d_dn_first_order_matches_d1() {
    let f = setup();
    let aDN1 = f.myLine.dn(5.0, 1).unwrap();
    EXPECT_NEAR(aDN1.x(), 1.0, precision::confusion());
    EXPECT_NEAR(aDN1.y(), 0.0, precision::confusion());
}

#[test]
fn adapted_line2d_dn_higher_order_is_zero() {
    let f = setup();
    let aDN2 = f.myLine.dn(5.0, 2).unwrap();
    EXPECT_NEAR(aDN2.magnitude(), 0.0, precision::confusion());

    let aDN3 = f.myLine.dn(5.0, 3).unwrap();
    EXPECT_NEAR(aDN3.magnitude(), 0.0, precision::confusion());
}

#[test]
fn adapted_line2d_resolution() {
    let f = setup();
    // For a unit-speed line, resolution should equal the input tolerance
    let aResolution = f.myLine.resolution(0.001).unwrap();
    EXPECT_NEAR(aResolution, 0.001, precision::confusion());
}

#[test]
fn adapted_line2d_line_geometry() {
    let f = setup();
    let aLin = f.myLine.line().unwrap();
    EXPECT_NEAR(aLin.direction().x(), 1.0, precision::confusion());
    EXPECT_NEAR(aLin.direction().y(), 0.0, precision::confusion());
    EXPECT_NEAR(aLin.location().x(), 0.0, precision::confusion());
    EXPECT_NEAR(aLin.location().y(), 0.0, precision::confusion());
}

#[test]
fn adapted_line2d_load_with_line() {
    let _fixture = setup();
    let aLine = Adaptor2dLine2d::default();
    let aLin = Line2::new(Point2::new(1.0, 1.0), dir(0.0, 1.0));
    aLine.load(aLin);

    let aPnt = aLine.value(3.0).unwrap();
    EXPECT_NEAR(aPnt.x(), 1.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 4.0, precision::confusion());
}

#[test]
fn adapted_line2d_load_with_bounds() {
    let _fixture = setup();
    let aLine = Adaptor2dLine2d::default();
    let aLin = Line2::new(Point2::new(0.0, 0.0), dir(1.0, 0.0));
    aLine.load_bounded(aLin, 2.0, 8.0);

    EXPECT_DOUBLE_EQ(aLine.first_parameter().unwrap(), 2.0);
    EXPECT_DOUBLE_EQ(aLine.last_parameter().unwrap(), 8.0);
}

#[test]
fn adapted_line2d_trim() {
    let f = setup();
    let aTrimmed = f.myLine.trim(2.0, 7.0, precision::confusion()).unwrap();
    EXPECT_DOUBLE_EQ(aTrimmed.first_parameter().unwrap(), 2.0);
    EXPECT_DOUBLE_EQ(aTrimmed.last_parameter().unwrap(), 7.0);

    let aPnt = aTrimmed.value(2.0).unwrap();
    EXPECT_NEAR(aPnt.x(), 2.0, precision::confusion());
}

#[test]
fn adapted_line2d_shallow_copy() {
    let f = setup();
    let aCopy = f.myLine.shallow_copy().unwrap();
    EXPECT_DOUBLE_EQ(aCopy.first_parameter().unwrap(), 0.0);
    EXPECT_DOUBLE_EQ(aCopy.last_parameter().unwrap(), 10.0);
    assert_eq!(aCopy.get_type().unwrap(), CurveType::Line);

    let aPnt = aCopy.value(5.0).unwrap();
    EXPECT_NEAR(aPnt.x(), 5.0, precision::confusion());
}
