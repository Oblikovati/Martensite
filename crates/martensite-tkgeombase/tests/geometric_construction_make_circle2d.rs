//! Gc make circle2d contracts.
#![allow(non_snake_case)]
#[path = "support/geometric_test_support/mod.rs"]
mod common;
use common::EXPECT_NEAR;
use martensite_tkernel::precision;
use martensite_tkgeombase::gc::{ErrorType, MakeCircle2d, Root};
use martensite_tkmath::gp::{Axis2d, Circle2, Direction2, Point2};

fn base_circle() -> Circle2 {
    let axis = Axis2d::new(Point2::new(0.0, 0.0), Direction2::new(1.0, 0.0).unwrap());
    Circle2::from_x_axis(axis, 5.0, true).unwrap()
}

#[test]
fn make_circle2d_from_circ_and_point_done() {
    let aBase = base_circle();
    let aPoint = Point2::new(0.0, 10.0);

    let aMaker = MakeCircle2d::from_circ_point(aBase, aPoint).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);
    let aRadius: f64 = aMaker.value().unwrap().circ2d().unwrap().radius();
    EXPECT_NEAR(aRadius, 10.0, precision::confusion());
}

#[test]
fn make_circle2d_from_circ_and_dist_done() {
    let aBase = base_circle();

    let aMaker = MakeCircle2d::from_circ_distance(aBase, -2.0).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);
    let aRadius: f64 = aMaker.value().unwrap().circ2d().unwrap().radius();
    EXPECT_NEAR(aRadius, 3.0, precision::confusion());
}

#[test]
fn make_circle2d_from_circ_and_point_at_center_done_zero_radius() {
    let aBase = base_circle();

    let aMaker = MakeCircle2d::from_circ_point(aBase, Point2::new(0.0, 0.0)).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);
    assert!(!aMaker.value().unwrap().is_null());
    let aRadius: f64 = aMaker.value().unwrap().circ2d().unwrap().radius();
    EXPECT_NEAR(aRadius, 0.0, precision::confusion());
}
