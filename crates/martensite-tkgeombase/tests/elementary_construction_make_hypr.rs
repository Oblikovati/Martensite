//! Gce make hypr contracts.
#![allow(non_snake_case)]
#[path = "support/geometric_test_support/mod.rs"]
mod common;
use common::EXPECT_NEAR;
use martensite_tkernel::precision;
use martensite_tkgeombase::gce::{ErrorType, MakeHypr, MakeHypr2d, Root};
use martensite_tkmath::gp::{self, Axis2, Axis2d, Axis22d, Hyperbola2, Hyperbola3, Point2, Point3};

// ==================== MakeHypr (3D) ====================

#[test]
fn make_hypr_from_axis_valid_radii_done() {
    let aMaker = MakeHypr::new(Axis2::default(), 10.0, 5.0).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);

    let aHypr: Hyperbola3 = aMaker.value().unwrap();
    EXPECT_NEAR(aHypr.major_radius(), 10.0, precision::confusion());
    EXPECT_NEAR(aHypr.minor_radius(), 5.0, precision::confusion());
}

#[test]
fn make_hypr_from_axis_major_less_than_minor_done() {
    let aMaker = MakeHypr::new(Axis2::default(), 3.0, 7.0).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);

    let aHypr: Hyperbola3 = aMaker.value().unwrap();
    EXPECT_NEAR(aHypr.major_radius(), 3.0, precision::confusion());
    EXPECT_NEAR(aHypr.minor_radius(), 7.0, precision::confusion());
}

#[test]
fn make_hypr_from_axis_equal_radii_done() {
    let aMaker = MakeHypr::new(Axis2::default(), 5.0, 5.0).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);
}

#[test]
fn make_hypr_from_axis_zero_radii_done() {
    let aMaker = MakeHypr::new(Axis2::default(), 0.0, 0.0).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);
}

#[test]
fn make_hypr_from_axis_negative_major_negative_radius() {
    let aMaker = MakeHypr::new(Axis2::default(), -1.0, 5.0).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::NegativeRadius);
}

#[test]
fn make_hypr_from_axis_negative_minor_negative_radius() {
    let aMaker = MakeHypr::new(Axis2::default(), 5.0, -1.0).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::NegativeRadius);
}

#[test]
fn make_hypr_from_axis_both_negative_negative_radius() {
    let aMaker = MakeHypr::new(Axis2::default(), -3.0, -2.0).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::NegativeRadius);
}

#[test]
fn make_hypr_from_points_valid_done() {
    let aCenter = Point3::new(0.0, 0.0, 0.0);
    let aS1 = Point3::new(10.0, 0.0, 0.0);
    let aS2 = Point3::new(0.0, 5.0, 0.0);

    let aMaker = MakeHypr::from_points(aS1, aS2, aCenter).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);

    let aHypr: Hyperbola3 = aMaker.value().unwrap();
    EXPECT_NEAR(aHypr.major_radius(), 10.0, precision::confusion());
    EXPECT_NEAR(aHypr.minor_radius(), 5.0, precision::confusion());
}

#[test]
fn make_hypr_from_points_major_less_than_minor_done() {
    let aCenter = Point3::new(0.0, 0.0, 0.0);
    let aS1 = Point3::new(3.0, 0.0, 0.0);
    let aS2 = Point3::new(0.0, 7.0, 0.0);

    let aMaker = MakeHypr::from_points(aS1, aS2, aCenter).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);

    let aHypr: Hyperbola3 = aMaker.value().unwrap();
    EXPECT_NEAR(aHypr.major_radius(), 3.0, precision::confusion());
    EXPECT_NEAR(aHypr.minor_radius(), 7.0, precision::confusion());
}

#[test]
fn make_hypr_from_points_confused_s1_center_confused_points() {
    let aCenter = Point3::new(1.0, 2.0, 3.0);
    let aS1 = Point3::new(1.0, 2.0, 3.0);
    let aS2 = Point3::new(4.0, 5.0, 6.0);

    let aMaker = MakeHypr::from_points(aS1, aS2, aCenter).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::ConfusedPoints);
}

#[test]
fn make_hypr_from_points_confused_s2_center_confused_points() {
    let aCenter = Point3::new(1.0, 2.0, 3.0);
    let aS1 = Point3::new(4.0, 5.0, 6.0);
    let aS2 = Point3::new(1.0, 2.0, 3.0);

    let aMaker = MakeHypr::from_points(aS1, aS2, aCenter).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::ConfusedPoints);
}

#[test]
fn make_hypr_from_points_confused_s1_s2_confused_points() {
    let aCenter = Point3::new(0.0, 0.0, 0.0);
    let aS1 = Point3::new(5.0, 5.0, 5.0);
    let aS2 = Point3::new(5.0, 5.0, 5.0);

    let aMaker = MakeHypr::from_points(aS1, aS2, aCenter).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::ConfusedPoints);
}

#[test]
fn make_hypr_from_points_collinear_colinear_points() {
    let aCenter = Point3::new(0.0, 0.0, 0.0);
    let aS1 = Point3::new(10.0, 0.0, 0.0);
    let aS2 = Point3::new(5.0, 0.0, 0.0);

    let aMaker = MakeHypr::from_points(aS1, aS2, aCenter).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::ColinearPoints);
}

#[test]
fn make_hypr_from_points_tiny_scale_confused_points() {
    let aRes: f64 = gp::resolution();
    let aCenter = Point3::new(0.0, 0.0, 0.0);
    let aS1 = Point3::new(2.0 * aRes, 0.0, 0.0);
    let aS2 = Point3::new(0.0, 1.1 * aRes, 0.0);

    let aMaker = MakeHypr::from_points(aS1, aS2, aCenter).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::ConfusedPoints);
}

// ==================== MakeHypr2d (2D) ====================

#[test]
fn make_hypr2d_from_major_axis_valid_radii_done() {
    let aMaker = MakeHypr2d::new(Axis2d::default(), 10.0, 5.0, true).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);

    let aHypr: Hyperbola2 = aMaker.value().unwrap();
    EXPECT_NEAR(aHypr.major_radius(), 10.0, precision::confusion());
    EXPECT_NEAR(aHypr.minor_radius(), 5.0, precision::confusion());
}

#[test]
fn make_hypr2d_from_major_axis_major_less_than_minor_done() {
    let aMaker = MakeHypr2d::new(Axis2d::default(), 3.0, 7.0, true).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);

    let aHypr: Hyperbola2 = aMaker.value().unwrap();
    EXPECT_NEAR(aHypr.major_radius(), 3.0, precision::confusion());
    EXPECT_NEAR(aHypr.minor_radius(), 7.0, precision::confusion());
}

#[test]
fn make_hypr2d_from_major_axis_negative_radius_negative_radius() {
    let aMaker = MakeHypr2d::new(Axis2d::default(), -1.0, 5.0, true).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::NegativeRadius);
}

#[test]
fn make_hypr2d_from_ax22d_valid_radii_done() {
    let aMaker = MakeHypr2d::from_frame(Axis22d::default(), 8.0, 4.0).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);
}

#[test]
fn make_hypr2d_from_ax22d_negative_radius_negative_radius() {
    let aMaker = MakeHypr2d::from_frame(Axis22d::default(), 5.0, -2.0).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::NegativeRadius);
}

#[test]
fn make_hypr2d_from_points_valid_done() {
    let aCenter = Point2::new(0.0, 0.0);
    let aS1 = Point2::new(10.0, 0.0);
    let aS2 = Point2::new(0.0, 5.0);

    let aMaker = MakeHypr2d::from_points(aS1, aS2, aCenter).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);

    let aHypr: Hyperbola2 = aMaker.value().unwrap();
    EXPECT_NEAR(aHypr.major_radius(), 10.0, precision::confusion());
    EXPECT_NEAR(aHypr.minor_radius(), 5.0, precision::confusion());
}

#[test]
fn make_hypr2d_from_points_major_less_than_minor_done() {
    let aCenter = Point2::new(0.0, 0.0);
    let aS1 = Point2::new(3.0, 0.0);
    let aS2 = Point2::new(0.0, 7.0);

    let aMaker = MakeHypr2d::from_points(aS1, aS2, aCenter).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);

    let aHypr: Hyperbola2 = aMaker.value().unwrap();
    EXPECT_NEAR(aHypr.major_radius(), 3.0, precision::confusion());
    EXPECT_NEAR(aHypr.minor_radius(), 7.0, precision::confusion());
}

#[test]
fn make_hypr2d_from_points_confused_s1_center_confused_points() {
    let aCenter = Point2::new(1.0, 2.0);
    let aS1 = Point2::new(1.0, 2.0);
    let aS2 = Point2::new(4.0, 5.0);

    let aMaker = MakeHypr2d::from_points(aS1, aS2, aCenter).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::ConfusedPoints);
}

#[test]
fn make_hypr2d_from_points_confused_s2_center_confused_points() {
    let aCenter = Point2::new(1.0, 2.0);
    let aS1 = Point2::new(4.0, 5.0);
    let aS2 = Point2::new(1.0, 2.0);

    let aMaker = MakeHypr2d::from_points(aS1, aS2, aCenter).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::ConfusedPoints);
}

#[test]
fn make_hypr2d_from_points_collinear_colinear_points() {
    let aCenter = Point2::new(0.0, 0.0);
    let aS1 = Point2::new(10.0, 0.0);
    let aS2 = Point2::new(5.0, 0.0);

    let aMaker = MakeHypr2d::from_points(aS1, aS2, aCenter).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::ColinearPoints);
}
