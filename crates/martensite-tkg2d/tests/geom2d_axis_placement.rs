//! Planar geometry construction, evaluation and boundary contracts.
#![allow(non_snake_case)]
mod common;
use common::EXPECT_NEAR;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dAxisPlacement, Geom2dGeometry};
use martensite_tkmath::gp::{Axis2d, Direction2, Point2, Transform2, Vector2};
use std::f64::consts::PI;

fn dir(x: f64, y: f64) -> Direction2 {
    Direction2::new(x, y).unwrap()
}
fn placement(x: f64, y: f64, dx: f64, dy: f64) -> Handle<Geom2dAxisPlacement> {
    Handle::new(Geom2dAxisPlacement::from_location(
        Point2::new(x, y),
        dir(dx, dy),
    ))
}

#[test]
fn axis_placement_construct_from_ax2d() {
    let anAxis = Axis2d::new(Point2::new(1.0, 2.0), dir(1.0, 0.0));
    let anAP = Handle::new(Geom2dAxisPlacement::new(anAxis));

    EXPECT_NEAR(anAP.location().x(), 1.0, precision::confusion());
    EXPECT_NEAR(anAP.location().y(), 2.0, precision::confusion());
    EXPECT_NEAR(anAP.direction().x(), 1.0, precision::confusion());
    EXPECT_NEAR(anAP.direction().y(), 0.0, precision::confusion());
}

#[test]
fn axis_placement_construct_from_point_and_dir() {
    let anAP = placement(3.0, 4.0, 0.0, 1.0);

    EXPECT_NEAR(anAP.location().x(), 3.0, precision::confusion());
    EXPECT_NEAR(anAP.direction().y(), 1.0, precision::confusion());
}

#[test]
fn axis_placement_set_location() {
    let anAP = placement(0.0, 0.0, 1.0, 0.0);
    anAP.set_location(Point2::new(5.0, 6.0));

    EXPECT_NEAR(anAP.location().x(), 5.0, precision::confusion());
    EXPECT_NEAR(anAP.location().y(), 6.0, precision::confusion());
}

#[test]
fn axis_placement_set_direction() {
    let anAP = placement(0.0, 0.0, 1.0, 0.0);
    anAP.set_direction(dir(0.0, 1.0));

    EXPECT_NEAR(anAP.direction().x(), 0.0, precision::confusion());
    EXPECT_NEAR(anAP.direction().y(), 1.0, precision::confusion());
}

#[test]
fn axis_placement_set_axis() {
    let anAP = placement(0.0, 0.0, 1.0, 0.0);

    let aNewAxis = Axis2d::new(Point2::new(10.0, 20.0), dir(0.0, 1.0));
    anAP.set_axis(aNewAxis);

    EXPECT_NEAR(anAP.location().x(), 10.0, precision::confusion());
    EXPECT_NEAR(anAP.direction().y(), 1.0, precision::confusion());
}

#[test]
fn axis_placement_ax2d() {
    let anAP = placement(1.0, 2.0, 1.0, 0.0);

    let anAxis: Axis2d = anAP.ax2d();
    EXPECT_NEAR(anAxis.location().x(), 1.0, precision::confusion());
    EXPECT_NEAR(anAxis.direction().x(), 1.0, precision::confusion());
}

#[test]
fn axis_placement_angle_perpendicular() {
    let anAP1 = placement(0.0, 0.0, 1.0, 0.0);
    let anAP2 = placement(0.0, 0.0, 0.0, 1.0);

    EXPECT_NEAR(anAP1.angle(&anAP2), PI / 2.0, precision::confusion());
}

#[test]
fn axis_placement_angle_parallel() {
    let anAP1 = placement(0.0, 0.0, 1.0, 0.0);
    let anAP2 = placement(5.0, 5.0, 1.0, 0.0);

    EXPECT_NEAR(anAP1.angle(&anAP2), 0.0, precision::confusion());
}

#[test]
fn axis_placement_angle_opposite() {
    let anAP1 = placement(0.0, 0.0, 1.0, 0.0);
    let anAP2 = placement(0.0, 0.0, -1.0, 0.0);

    EXPECT_NEAR(anAP1.angle(&anAP2).abs(), PI, precision::confusion());
}

#[test]
fn axis_placement_reverse() {
    let anAP = placement(0.0, 0.0, 1.0, 0.0);
    anAP.reverse();

    EXPECT_NEAR(anAP.direction().x(), -1.0, precision::confusion());
    EXPECT_NEAR(anAP.direction().y(), 0.0, precision::confusion());
    // Location unchanged
    EXPECT_NEAR(anAP.location().x(), 0.0, precision::confusion());
}

#[test]
fn axis_placement_reversed() {
    let anAP = placement(0.0, 0.0, 1.0, 0.0);
    let aReversed: Handle<Geom2dAxisPlacement> = anAP.reversed();

    EXPECT_NEAR(aReversed.direction().x(), -1.0, precision::confusion());
    // Original unchanged
    EXPECT_NEAR(anAP.direction().x(), 1.0, precision::confusion());
}

#[test]
fn axis_placement_transform_translation() {
    let anAP = placement(1.0, 2.0, 1.0, 0.0);

    let mut aTrsf = Transform2::default();
    aTrsf.set_translation(Vector2::new(10.0, 20.0));
    anAP.transform(aTrsf).unwrap();

    EXPECT_NEAR(anAP.location().x(), 11.0, precision::confusion());
    EXPECT_NEAR(anAP.location().y(), 22.0, precision::confusion());
    // Direction unchanged
    EXPECT_NEAR(anAP.direction().x(), 1.0, precision::confusion());
}

#[test]
fn axis_placement_transform_rotation() {
    let anAP = placement(1.0, 0.0, 1.0, 0.0);

    let mut aTrsf = Transform2::default();
    aTrsf.set_rotation(Point2::new(0.0, 0.0), PI / 2.0);
    anAP.transform(aTrsf).unwrap();

    EXPECT_NEAR(anAP.location().x(), 0.0, precision::confusion());
    EXPECT_NEAR(anAP.location().y(), 1.0, precision::confusion());
    EXPECT_NEAR(anAP.direction().x(), 0.0, precision::confusion());
    EXPECT_NEAR(anAP.direction().y(), 1.0, precision::confusion());
}

#[test]
fn axis_placement_copy() {
    let anAP = placement(1.0, 2.0, 0.0, 1.0);
    let aCopyGeom: Handle<dyn Geom2dGeometry> = anAP.copy().unwrap();
    let aCopy: Handle<Geom2dAxisPlacement> = aCopyGeom.cast::<Geom2dAxisPlacement>();

    assert!(!aCopy.is_null());
    EXPECT_NEAR(aCopy.location().x(), 1.0, precision::confusion());
    EXPECT_NEAR(aCopy.direction().y(), 1.0, precision::confusion());

    // Verify independence
    aCopy.set_location(Point2::new(99.0, 99.0));
    EXPECT_NEAR(anAP.location().x(), 1.0, precision::confusion());
}
