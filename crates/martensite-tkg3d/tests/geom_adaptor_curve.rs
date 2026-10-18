//! Adaptor curve construction and evaluation contracts.
#![allow(non_snake_case)]
#[path = "common/mod.rs"]
mod common;
use common::{EXPECT_DOUBLE_EQ, EXPECT_NEAR, EXPECT_THROW};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkg3d::geom::{GeomCircle, GeomCurve, GeomLine, GeomTrimmedCurve};
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkmath::geom_abs::CurveType;
use martensite_tkmath::gp::{Axis2, Circle3, Direction3, Line3, Point3};
use std::f64::consts::PI;

// The fixture: a line through the origin along (1, 1, 0) and a circle of radius 3
// centred at (5, 5, 0) in the XY plane.
struct Fixture {
    myLine: Handle<dyn GeomCurve>,
    myCircle: Handle<dyn GeomCurve>,
}

fn SetUp() -> Fixture {
    // Create a simple 3D line for testing
    let aP1 = Point3::new(0.0, 0.0, 0.0);
    let aLine = Line3::new(aP1, Direction3::new(1.0, 1.0, 0.0).unwrap());
    let myLine: Handle<dyn GeomCurve> = Handle::new(GeomLine::from_lin(aLine)).cast();

    // Create a 3D circle for testing
    let aCenter = Point3::new(5.0, 5.0, 0.0);
    let aCirc = Circle3::new(
        Axis2::new(aCenter, Direction3::new(0.0, 0.0, 1.0).unwrap()).unwrap(),
        3.0,
    )
    .unwrap();
    let myCircle: Handle<dyn GeomCurve> = Handle::new(GeomCircle::from_circ(aCirc)).cast();
    Fixture { myLine, myCircle }
}

#[test]
fn curve_load_valid_parameters_success() {
    let f = SetUp();
    // Test loading with valid parameters
    let anAdaptor = GeomAdaptorCurve::default();
    assert!(anAdaptor.load_range(&f.myLine, 0.0, 10.0).is_ok());

    EXPECT_DOUBLE_EQ(anAdaptor.first_parameter().unwrap(), 0.0);
    EXPECT_DOUBLE_EQ(anAdaptor.last_parameter().unwrap(), 10.0);
}

#[test]
fn curve_load_equal_parameters_success() {
    let f = SetUp();
    // Test loading with equal parameters (degenerated curve)
    // This should be allowed as it represents a point
    let anAdaptor = GeomAdaptorCurve::default();
    assert!(anAdaptor.load_range(&f.myLine, 5.0, 5.0).is_ok());

    EXPECT_DOUBLE_EQ(anAdaptor.first_parameter().unwrap(), 5.0);
    EXPECT_DOUBLE_EQ(anAdaptor.last_parameter().unwrap(), 5.0);

    // Verify it evaluates to a single point
    let aP1: Point3 = anAdaptor.value(5.0).unwrap();
    let aP2: Point3 = f.myLine.value(5.0).unwrap();
    assert!(aP1.is_equal(&aP2, precision::confusion()));
}

#[test]
fn curve_load_parameters_within_confusion_success() {
    let f = SetUp();
    // Test loading with parameters within Precision::Confusion()
    // This should be allowed (degenerated curve handling)
    let aParam1: f64 = 5.0;
    let aParam2: f64 = 5.0 + precision::confusion() * 0.5;

    let anAdaptor = GeomAdaptorCurve::default();
    assert!(anAdaptor.load_range(&f.myLine, aParam1, aParam2).is_ok());

    EXPECT_DOUBLE_EQ(anAdaptor.first_parameter().unwrap(), aParam1);
    EXPECT_DOUBLE_EQ(anAdaptor.last_parameter().unwrap(), aParam2);
}

#[test]
fn curve_load_parameters_at_confusion_boundary_success() {
    let f = SetUp();
    // Test loading with parameters exactly at the confusion tolerance boundary
    let aParam1: f64 = 5.0;
    let aParam2: f64 = 5.0 + precision::confusion();

    let anAdaptor = GeomAdaptorCurve::default();
    assert!(anAdaptor.load_range(&f.myLine, aParam1, aParam2).is_ok());

    EXPECT_DOUBLE_EQ(anAdaptor.first_parameter().unwrap(), aParam1);
    EXPECT_DOUBLE_EQ(anAdaptor.last_parameter().unwrap(), aParam2);
}

#[test]
fn curve_load_first_greater_than_last_within_confusion_success() {
    let f = SetUp();
    // Test loading with theUFirst > theULast but within Precision::Confusion()
    // This represents a degenerated curve and should be allowed
    let aParam1: f64 = 5.0 + precision::confusion() * 0.5;
    let aParam2: f64 = 5.0;

    let anAdaptor = GeomAdaptorCurve::default();
    assert!(anAdaptor.load_range(&f.myLine, aParam1, aParam2).is_ok());

    EXPECT_DOUBLE_EQ(anAdaptor.first_parameter().unwrap(), aParam1);
    EXPECT_DOUBLE_EQ(anAdaptor.last_parameter().unwrap(), aParam2);
}

#[test]
fn curve_load_first_greater_than_last_beyond_confusion_throws_exception() {
    let f = SetUp();
    // Test loading with UFirst > ULast + Precision::Confusion()
    // This should throw StandardConstructionError
    let aParam1: f64 = 10.0;
    let aParam2: f64 = 5.0;

    let anAdaptor = GeomAdaptorCurve::default();
    EXPECT_THROW(
        anAdaptor.load_range(&f.myLine, aParam1, aParam2),
        "Standard_ConstructionError",
    );
}

#[test]
fn curve_load_first_slightly_greater_than_last_throws_exception() {
    let f = SetUp();
    // Test loading with UFirst slightly greater than ULast beyond tolerance
    let aParam1: f64 = 5.0;
    let aParam2: f64 = 5.0 - precision::confusion() * 2.0;

    let anAdaptor = GeomAdaptorCurve::default();
    EXPECT_THROW(
        anAdaptor.load_range(&f.myLine, aParam1, aParam2),
        "Standard_ConstructionError",
    );
}

#[test]
fn curve_constructor_degenerated_curve_success() {
    let f = SetUp();
    // Test constructor with degenerated curve parameters
    assert!(GeomAdaptorCurve::with_range(&f.myCircle, 0.0, 0.0).is_ok());

    let anAdaptor = GeomAdaptorCurve::with_range(&f.myCircle, 0.0, 0.0).unwrap();
    EXPECT_DOUBLE_EQ(anAdaptor.first_parameter().unwrap(), 0.0);
    EXPECT_DOUBLE_EQ(anAdaptor.last_parameter().unwrap(), 0.0);
}

#[test]
fn curve_constructor_invalid_parameters_throws_exception() {
    let f = SetUp();
    // Test constructor with invalid parameters
    EXPECT_THROW(
        GeomAdaptorCurve::with_range(&f.myCircle, 10.0, 0.0),
        "Standard_ConstructionError",
    );
}

#[test]
fn curve_load_null_curve_throws_exception() {
    // Test loading with null curve
    let aNullCurve: Handle<dyn GeomCurve> = Handle::default();
    let anAdaptor = GeomAdaptorCurve::default();

    EXPECT_THROW(
        anAdaptor.load_range(&aNullCurve, 0.0, 10.0),
        "Standard_NullObject",
    );
}

#[test]
fn curve_degenerated_curve_circle_at_zero_length_success() {
    let f = SetUp();
    // Test degenerated circle (zero length arc)
    let aParam: f64 = PI;

    let anAdaptor = GeomAdaptorCurve::default();
    assert!(anAdaptor.load_range(&f.myCircle, aParam, aParam).is_ok());

    // Verify the curve represents a single point
    let aPoint: Point3 = anAdaptor.value(aParam).unwrap();
    let aExpectedPoint: Point3 = f.myCircle.value(aParam).unwrap();

    assert!(aPoint.is_equal(&aExpectedPoint, precision::confusion()));
    assert!(
        anAdaptor.is_closed().unwrap()
            || anAdaptor.first_parameter().unwrap() == anAdaptor.last_parameter().unwrap()
    );
}

#[test]
fn curve_degenerated_curve_trimmed_curve_success() {
    let f = SetUp();
    // Create a trimmed curve and test degenerated case
    let aTrimmedCurve: Handle<dyn GeomCurve> =
        Handle::new(GeomTrimmedCurve::new(&f.myLine, 0.0, 20.0, true, true).unwrap()).cast();

    let aParam: f64 = 10.0;
    let anAdaptor = GeomAdaptorCurve::default();
    assert!(anAdaptor.load_range(&aTrimmedCurve, aParam, aParam).is_ok());

    EXPECT_DOUBLE_EQ(anAdaptor.first_parameter().unwrap(), aParam);
    EXPECT_DOUBLE_EQ(anAdaptor.last_parameter().unwrap(), aParam);
}

#[test]
fn curve_tolerance_boundary_negative_case_throws_exception() {
    let f = SetUp();
    // Test parameters just beyond the negative tolerance boundary
    let aParam1: f64 = 5.0;
    let aParam2: f64 = 5.0 - precision::confusion() - 1e-10;

    let anAdaptor = GeomAdaptorCurve::default();
    EXPECT_THROW(
        anAdaptor.load_range(&f.myLine, aParam1, aParam2),
        "Standard_ConstructionError",
    );
}

#[test]
fn curve_load_without_parameters_success() {
    let f = SetUp();
    // Test loading curve without specifying parameters (uses curve's own parameters)
    let anAdaptor = GeomAdaptorCurve::default();
    assert!(anAdaptor.load(&f.myCircle).is_ok());

    EXPECT_NEAR(
        anAdaptor.first_parameter().unwrap(),
        f.myCircle.first_parameter(),
        precision::confusion(),
    );
    EXPECT_NEAR(
        anAdaptor.last_parameter().unwrap(),
        f.myCircle.last_parameter(),
        precision::confusion(),
    );
    assert!(anAdaptor.is_periodic().unwrap());
}

#[test]
fn curve_degenerated_curve_multiple_locations_success() {
    let f = SetUp();
    // Test degenerated curves at different parameter locations
    let aParams: [f64; 5] = [0.0, 1.0, -5.0, 100.0, PI];

    for aParam in aParams {
        let anAdaptor = GeomAdaptorCurve::default();
        assert!(anAdaptor.load_range(&f.myLine, aParam, aParam).is_ok());

        let aPoint1: Point3 = anAdaptor.value(aParam).unwrap();
        let aPoint2: Point3 = f.myLine.value(aParam).unwrap();
        assert!(aPoint1.is_equal(&aPoint2, precision::confusion()));
    }
}

#[test]
fn curve_boundary_conditions_very_small_interval_success() {
    let f = SetUp();
    // Test with very small but valid interval (just above tolerance)
    let aParam1: f64 = 5.0;
    let aParam2: f64 = 5.0 + precision::confusion() + 1e-12;

    let anAdaptor = GeomAdaptorCurve::default();
    assert!(anAdaptor.load_range(&f.myLine, aParam1, aParam2).is_ok());

    EXPECT_DOUBLE_EQ(anAdaptor.first_parameter().unwrap(), aParam1);
    EXPECT_DOUBLE_EQ(anAdaptor.last_parameter().unwrap(), aParam2);
}

#[test]
fn curve_constructor_with_valid_range_success() {
    let f = SetUp();
    // Test the 3-parameter constructor with valid range
    let aFirst: f64 = 0.0;
    let aLast: f64 = 2.0 * PI;

    assert!(GeomAdaptorCurve::with_range(&f.myCircle, aFirst, aLast).is_ok());

    let anAdaptor = GeomAdaptorCurve::with_range(&f.myCircle, aFirst, aLast).unwrap();
    EXPECT_DOUBLE_EQ(anAdaptor.first_parameter().unwrap(), aFirst);
    EXPECT_DOUBLE_EQ(anAdaptor.last_parameter().unwrap(), aLast);
    assert_eq!(anAdaptor.get_type().unwrap(), CurveType::Circle);
}
