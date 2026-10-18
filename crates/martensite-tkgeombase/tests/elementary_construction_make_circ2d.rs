//! Gce make circ2d contracts.
#![allow(non_snake_case)]
#[path = "support/geometric_test_support/mod.rs"]
mod common;
use common::EXPECT_NEAR;
use martensite_tkernel::precision;
use martensite_tkgeombase::gce::{ErrorType, MakeCirc2d, Root};
use martensite_tkmath::gp::{self, Point2};

#[test]
fn make_circ2d_three_points_all_coincident_within_resolution_done_zero_radius() {
    // Intentional: points inside gp::Resolution() are treated as coincident
    // and should produce a valid zero-radius circle.
    let aDelta: f64 = 0.25 * gp::resolution();

    let aP1 = Point2::new(0.0, 0.0);
    let aP2 = Point2::new(aDelta, 0.0);
    let aP3 = Point2::new(0.0, aDelta);

    let aMaker = MakeCirc2d::through_points(aP1, aP2, aP3).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);
    EXPECT_NEAR(
        aMaker.value().unwrap().radius(),
        0.0,
        precision::confusion(),
    );
}

#[test]
fn make_circ2d_three_points_one_near_coincident_pair_done() {
    let aP1 = Point2::new(0.0, 0.0);
    let aP2 = Point2::new(1.0e-150, 0.0);
    let aP3 = Point2::new(0.0, 1.0);

    let aMaker = MakeCirc2d::through_points(aP1, aP2, aP3).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);
    assert!(aMaker.value().unwrap().radius() > 0.0);
}

#[test]
fn make_circ2d_three_points_small_distinct_done() {
    let aP1 = Point2::new(0.0, 0.0);
    let aP2 = Point2::new(2.0e-150, 0.0);
    let aP3 = Point2::new(0.0, 1.0);

    let aMaker = MakeCirc2d::through_points(aP1, aP2, aP3).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);
    assert!(aMaker.value().unwrap().radius() > 0.0);
}
