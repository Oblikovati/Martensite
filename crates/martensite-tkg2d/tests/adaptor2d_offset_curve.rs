//! Planar geometry construction, evaluation and boundary contracts.
#![allow(non_snake_case)]
mod common;
use common::{EXPECT_DOUBLE_EQ, EXPECT_NEAR};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::adaptor2d::{Adaptor2dCurve2d, Adaptor2dLine2d, Adaptor2dOffsetCurve};
use martensite_tkmath::geom_abs::{CurveType, Shape};
use martensite_tkmath::gp::{Direction2, Point2};

fn dir(x: f64, y: f64) -> Direction2 {
    Direction2::new(x, y).unwrap()
}

// Adaptor2dOffsetCurveTest::SetUp: horizontal line through origin, bounds [0, 10].
fn setup() -> Handle<dyn Adaptor2dCurve2d> {
    Handle::new(Adaptor2dLine2d::new(
        Point2::new(0.0, 0.0),
        dir(1.0, 0.0),
        0.0,
        10.0,
    ))
    .cast()
}

#[test]
fn adapted_offset_curve_default_constructor() {
    let _myBaseLine = setup();
    let anOffset = Adaptor2dOffsetCurve::default();
    EXPECT_DOUBLE_EQ(anOffset.offset(), 0.0);
}

#[test]
fn adapted_offset_curve_construct_with_curve_offset_is_zero() {
    let myBaseLine = setup();
    // Constructor with only curve sets offset to 0 and bounds to 0,0
    let anOffset = Adaptor2dOffsetCurve::with_curve(&myBaseLine);
    EXPECT_DOUBLE_EQ(anOffset.offset(), 0.0);
    EXPECT_DOUBLE_EQ(anOffset.first_parameter().unwrap(), 0.0);
    EXPECT_DOUBLE_EQ(anOffset.last_parameter().unwrap(), 0.0);
}

#[test]
fn adapted_offset_curve_construct_with_offset() {
    let myBaseLine = setup();
    let anOffset = Adaptor2dOffsetCurve::with_offset(&myBaseLine, 3.0).unwrap();
    EXPECT_DOUBLE_EQ(anOffset.offset(), 3.0);
    EXPECT_DOUBLE_EQ(anOffset.first_parameter().unwrap(), 0.0);
    EXPECT_DOUBLE_EQ(anOffset.last_parameter().unwrap(), 10.0);
}

#[test]
fn adapted_offset_curve_construct_with_offset_and_bounds() {
    let myBaseLine = setup();
    let anOffset = Adaptor2dOffsetCurve::with_bounds(&myBaseLine, 2.0, 1.0, 8.0);
    EXPECT_DOUBLE_EQ(anOffset.offset(), 2.0);
    EXPECT_DOUBLE_EQ(anOffset.first_parameter().unwrap(), 1.0);
    EXPECT_DOUBLE_EQ(anOffset.last_parameter().unwrap(), 8.0);
}

#[test]
fn adapted_offset_curve_value_positive_offset() {
    let myBaseLine = setup();
    // Offsetting a horizontal +X line by +3 shifts Y by -3
    // (the geometry engine 2D offset uses right-hand normal: Z cross tangent)
    let anOffset = Adaptor2dOffsetCurve::with_offset(&myBaseLine, 3.0).unwrap();
    let aPnt = anOffset.value(5.0).unwrap();

    EXPECT_NEAR(aPnt.x(), 5.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), -3.0, precision::confusion());
}

#[test]
fn adapted_offset_curve_value_negative_offset() {
    let myBaseLine = setup();
    let anOffset = Adaptor2dOffsetCurve::with_offset(&myBaseLine, -2.0).unwrap();
    let aPnt = anOffset.value(5.0).unwrap();

    EXPECT_NEAR(aPnt.x(), 5.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 2.0, precision::confusion());
}

#[test]
fn adapted_offset_curve_value_zero_offset_matches_base_curve() {
    let myBaseLine = setup();
    let anOffset = Adaptor2dOffsetCurve::with_offset(&myBaseLine, 0.0).unwrap();

    let mut u: f64 = 0.0;
    while u <= 10.0 {
        let aOffPnt = anOffset.value(u).unwrap();
        let aBasePnt = myBaseLine.value(u).unwrap();
        assert!(aOffPnt.is_equal(aBasePnt, precision::confusion()));
        u += 2.0;
    }
}

#[test]
fn adapted_offset_curve_d1_offset_line() {
    let myBaseLine = setup();
    let anOffset = Adaptor2dOffsetCurve::with_offset(&myBaseLine, 3.0).unwrap();

    let (_aPnt, aVec) = anOffset.d1(5.0).unwrap();

    // First derivative of an offset line is same as the base line direction
    EXPECT_NEAR(aVec.x(), 1.0, precision::confusion());
    EXPECT_NEAR(aVec.y(), 0.0, precision::confusion());
}

#[test]
fn adapted_offset_curve_continuity() {
    let myBaseLine = setup();
    let anOffset = Adaptor2dOffsetCurve::with_offset(&myBaseLine, 3.0).unwrap();
    // Offset of CN line is CN
    assert_eq!(anOffset.continuity().unwrap(), Shape::CN);
}

#[test]
fn adapted_offset_curve_nb_intervals() {
    let myBaseLine = setup();
    let anOffset = Adaptor2dOffsetCurve::with_offset(&myBaseLine, 3.0).unwrap();
    assert_eq!(anOffset.nb_intervals(Shape::CN).unwrap(), 1);
}

#[test]
fn adapted_offset_curve_get_type_line() {
    let myBaseLine = setup();
    let anOffset = Adaptor2dOffsetCurve::with_offset(&myBaseLine, 3.0).unwrap();
    assert_eq!(anOffset.get_type().unwrap(), CurveType::Line);
}

#[test]
fn adapted_offset_curve_is_not_closed() {
    let myBaseLine = setup();
    let anOffset = Adaptor2dOffsetCurve::with_offset(&myBaseLine, 3.0).unwrap();
    assert!(!anOffset.is_closed().unwrap());
}

#[test]
fn adapted_offset_curve_is_not_periodic() {
    let myBaseLine = setup();
    let anOffset = Adaptor2dOffsetCurve::with_offset(&myBaseLine, 3.0).unwrap();
    assert!(!anOffset.is_periodic().unwrap());
}

#[test]
fn adapted_offset_curve_load_curve_resets_offset() {
    let myBaseLine = setup();
    let anOffset = Adaptor2dOffsetCurve::with_offset(&myBaseLine, 5.0).unwrap();
    EXPECT_DOUBLE_EQ(anOffset.offset(), 5.0);

    // Load a new curve resets offset to 0
    let aNewLine: Handle<dyn Adaptor2dCurve2d> = Handle::new(Adaptor2dLine2d::new(
        Point2::new(0.0, 0.0),
        dir(0.0, 1.0),
        0.0,
        5.0,
    ))
    .cast();
    anOffset.load_curve(&aNewLine);
    EXPECT_DOUBLE_EQ(anOffset.offset(), 0.0);
}

#[test]
fn adapted_offset_curve_load_offset() {
    let myBaseLine = setup();
    let anOffset = Adaptor2dOffsetCurve::with_curve(&myBaseLine);
    EXPECT_DOUBLE_EQ(anOffset.offset(), 0.0);

    anOffset.load_offset(7.5).unwrap();
    EXPECT_DOUBLE_EQ(anOffset.offset(), 7.5);
}

#[test]
fn adapted_offset_curve_load_offset_with_bounds() {
    let myBaseLine = setup();
    let anOffset = Adaptor2dOffsetCurve::with_curve(&myBaseLine);
    anOffset.load_offset_bounded(4.0, 2.0, 6.0);

    EXPECT_DOUBLE_EQ(anOffset.offset(), 4.0);
    EXPECT_DOUBLE_EQ(anOffset.first_parameter().unwrap(), 2.0);
    EXPECT_DOUBLE_EQ(anOffset.last_parameter().unwrap(), 6.0);
}

#[test]
fn adapted_offset_curve_curve_returns_base_curve() {
    let myBaseLine = setup();
    let anOffset = Adaptor2dOffsetCurve::with_offset(&myBaseLine, 3.0).unwrap();
    let aCurve = anOffset.curve();
    assert!(!aCurve.is_null());
    assert_eq!(aCurve.get_type().unwrap(), CurveType::Line);
}

#[test]
fn adapted_offset_curve_trim() {
    let myBaseLine = setup();
    let anOffset = Adaptor2dOffsetCurve::with_offset(&myBaseLine, 3.0).unwrap();
    let aTrimmed = anOffset.trim(2.0, 7.0, precision::confusion()).unwrap();

    EXPECT_DOUBLE_EQ(aTrimmed.first_parameter().unwrap(), 2.0);
    EXPECT_DOUBLE_EQ(aTrimmed.last_parameter().unwrap(), 7.0);
}

#[test]
fn adapted_offset_curve_shallow_copy() {
    let myBaseLine = setup();
    let anOffset = Adaptor2dOffsetCurve::with_bounds(&myBaseLine, 3.0, 1.0, 9.0);
    let aCopy = anOffset.shallow_copy().unwrap();

    EXPECT_DOUBLE_EQ(aCopy.first_parameter().unwrap(), 1.0);
    EXPECT_DOUBLE_EQ(aCopy.last_parameter().unwrap(), 9.0);

    let aPntOrig = anOffset.value(5.0).unwrap();
    let aPntCopy = aCopy.value(5.0).unwrap();
    assert!(aPntOrig.is_equal(aPntCopy, precision::confusion()));
}

#[test]
fn adapted_offset_curve_resolution() {
    let myBaseLine = setup();
    let anOffset = Adaptor2dOffsetCurve::with_offset(&myBaseLine, 3.0).unwrap();
    let aRes = anOffset.resolution(0.001).unwrap();
    assert!(aRes > 0.0);
}
