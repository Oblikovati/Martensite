//! Gce make elips contracts.
#![allow(non_snake_case)]
#[path = "support/geometric_test_support/mod.rs"]
mod common;
use common::EXPECT_NEAR;
use martensite_tkernel::precision;
use martensite_tkgeombase::gce::{ErrorType, MakeElips, Root};
use martensite_tkmath::gp::{self, Ellipse3, Point3};

#[test]
fn make_elips_from_points_valid_done() {
    let aCenter = Point3::new(0.0, 0.0, 0.0);
    let aS1 = Point3::new(10.0, 0.0, 0.0);
    let aS2 = Point3::new(0.0, 5.0, 0.0);

    let aMaker = MakeElips::from_points(aS1, aS2, aCenter).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);

    let anElips: Ellipse3 = aMaker.value().unwrap();
    EXPECT_NEAR(anElips.major_radius(), 10.0, precision::confusion());
    EXPECT_NEAR(anElips.minor_radius(), 5.0, precision::confusion());
}

#[test]
fn make_elips_from_points_collinear_invert_axis() {
    let aCenter = Point3::new(0.0, 0.0, 0.0);
    let aS1 = Point3::new(10.0, 0.0, 0.0);
    let aS2 = Point3::new(5.0, 0.0, 0.0);

    let aMaker = MakeElips::from_points(aS1, aS2, aCenter).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::InvertAxis);
}

#[test]
fn make_elips_from_points_tiny_scale_null_axis() {
    let aRes: f64 = gp::resolution();
    let aCenter = Point3::new(0.0, 0.0, 0.0);
    let aS1 = Point3::new(2.0 * aRes, 0.0, 0.0);
    let aS2 = Point3::new(0.0, 1.1 * aRes, 0.0);

    let aMaker = MakeElips::from_points(aS1, aS2, aCenter).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::NullAxis);
}
