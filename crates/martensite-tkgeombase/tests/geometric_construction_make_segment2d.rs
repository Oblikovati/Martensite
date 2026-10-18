//! Gc make segment2d contracts.
#![allow(non_snake_case)]
use martensite_tkgeombase::gc::{ErrorType, MakeSegment2d, Root};
use martensite_tkmath::gp::{self, Direction2, Point2};

fn x_dir() -> Direction2 {
    Direction2::new(1.0, 0.0).unwrap()
}

#[test]
fn make_segment2d_from_points_below_resolution_confused_points() {
    let aP1 = Point2::new(0.0, 0.0);
    let aP2 = Point2::new(0.5 * gp::resolution(), 0.0);

    let aMaker = MakeSegment2d::from_points(aP1, aP2).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::ConfusedPoints);
}

#[test]
fn make_segment2d_from_points_above_resolution_done() {
    let aP1 = Point2::new(0.0, 0.0);
    let aP2 = Point2::new(1.0e-150, 0.0);

    let aMaker = MakeSegment2d::from_points(aP1, aP2).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);
}

#[test]
fn make_segment2d_from_points_at_resolution_confused_points() {
    let aP1 = Point2::new(0.0, 0.0);
    let aP2 = Point2::new(gp::resolution(), 0.0);

    let aMaker = MakeSegment2d::from_points(aP1, aP2).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::ConfusedPoints);
}

#[test]
fn make_segment2d_from_point_dir_below_resolution_parameter_confused_points() {
    let aP1 = Point2::new(0.0, 0.0);
    let aDir = x_dir();
    let aP2 = Point2::new(0.5 * gp::resolution(), 3.0);

    let aMaker = MakeSegment2d::from_direction(aP1, aDir, aP2).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::ConfusedPoints);
}

#[test]
fn make_segment2d_from_point_dir_above_resolution_parameter_done() {
    let aP1 = Point2::new(0.0, 0.0);
    let aDir = x_dir();
    let aP2 = Point2::new(2.0 * gp::resolution(), 3.0);

    let aMaker = MakeSegment2d::from_direction(aP1, aDir, aP2).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);
}

#[test]
fn make_segment2d_from_point_dir_at_resolution_parameter_confused_points() {
    let aP1 = Point2::new(0.0, 0.0);
    let aDir = x_dir();
    let aP2 = Point2::new(gp::resolution(), 5.0);

    let aMaker = MakeSegment2d::from_direction(aP1, aDir, aP2).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::ConfusedPoints);
}
