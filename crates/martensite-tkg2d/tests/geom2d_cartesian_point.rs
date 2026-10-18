//! Planar geometry construction, evaluation and boundary contracts.
#![allow(non_snake_case)]
mod common;
use common::EXPECT_NEAR;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dCartesianPoint, Geom2dGeometry, Geom2dPoint};
use martensite_tkmath::gp::{Point2, Transform2, Vector2};
use std::f64::consts::PI;

#[test]
fn cartesian_point_construct_from_coords() {
    let aPnt = Handle::new(Geom2dCartesianPoint::new(3.0, 4.0));
    EXPECT_NEAR(aPnt.x(), 3.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 4.0, precision::confusion());
}

#[test]
fn cartesian_point_construct_from_pnt2d() {
    let aPnt = Handle::new(Geom2dCartesianPoint::from_pnt2d(Point2::new(5.0, 6.0)));
    EXPECT_NEAR(aPnt.x(), 5.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 6.0, precision::confusion());
}

#[test]
fn cartesian_point_set_coord() {
    let aPnt = Handle::new(Geom2dCartesianPoint::new(0.0, 0.0));
    aPnt.set_coord(7.0, 8.0);
    EXPECT_NEAR(aPnt.x(), 7.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 8.0, precision::confusion());
}

#[test]
fn cartesian_point_set_pnt2d() {
    let aPnt = Handle::new(Geom2dCartesianPoint::new(0.0, 0.0));
    aPnt.set_pnt2d(Point2::new(1.0, 2.0));
    EXPECT_NEAR(aPnt.x(), 1.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 2.0, precision::confusion());
}

#[test]
fn cartesian_point_set_x_set_y() {
    let aPnt = Handle::new(Geom2dCartesianPoint::new(0.0, 0.0));
    aPnt.set_x(10.0);
    aPnt.set_y(20.0);
    EXPECT_NEAR(aPnt.x(), 10.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 20.0, precision::confusion());
}

#[test]
fn cartesian_point_coord() {
    let aPnt = Handle::new(Geom2dCartesianPoint::new(3.0, 4.0));
    let (aX, aY): (f64, f64) = aPnt.coord();
    EXPECT_NEAR(aX, 3.0, precision::confusion());
    EXPECT_NEAR(aY, 4.0, precision::confusion());
}

#[test]
fn cartesian_point_pnt2d() {
    let aPnt = Handle::new(Geom2dCartesianPoint::new(3.0, 4.0));
    let aGpPnt: Point2 = aPnt.pnt2d();
    EXPECT_NEAR(aGpPnt.x(), 3.0, precision::confusion());
    EXPECT_NEAR(aGpPnt.y(), 4.0, precision::confusion());
}

#[test]
fn cartesian_point_distance() {
    let aP1 = Handle::new(Geom2dCartesianPoint::new(0.0, 0.0));
    let aP2 = Handle::new(Geom2dCartesianPoint::new(3.0, 4.0));
    EXPECT_NEAR(aP1.distance(&*aP2), 5.0, precision::confusion());
}

#[test]
fn cartesian_point_square_distance() {
    let aP1 = Handle::new(Geom2dCartesianPoint::new(0.0, 0.0));
    let aP2 = Handle::new(Geom2dCartesianPoint::new(3.0, 4.0));
    EXPECT_NEAR(aP1.square_distance(&*aP2), 25.0, precision::confusion());
}

#[test]
fn cartesian_point_distance_same_point() {
    let aPnt = Handle::new(Geom2dCartesianPoint::new(5.0, 5.0));
    EXPECT_NEAR(aPnt.distance(&*aPnt), 0.0, precision::confusion());
}

#[test]
fn cartesian_point_transform_translation() {
    let aPnt = Handle::new(Geom2dCartesianPoint::new(1.0, 2.0));

    let mut aTrsf = Transform2::default();
    aTrsf.set_translation(Vector2::new(3.0, 4.0));
    aPnt.transform(aTrsf).unwrap();

    EXPECT_NEAR(aPnt.x(), 4.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 6.0, precision::confusion());
}

#[test]
fn cartesian_point_transform_rotation() {
    let aPnt = Handle::new(Geom2dCartesianPoint::new(1.0, 0.0));

    let mut aTrsf = Transform2::default();
    aTrsf.set_rotation(Point2::new(0.0, 0.0), PI / 2.0);
    aPnt.transform(aTrsf).unwrap();

    EXPECT_NEAR(aPnt.x(), 0.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 1.0, precision::confusion());
}

#[test]
fn cartesian_point_copy() {
    let aPnt = Handle::new(Geom2dCartesianPoint::new(3.0, 4.0));
    let aCopyGeom: Handle<dyn Geom2dGeometry> = aPnt.copy().unwrap();
    let aCopy: Handle<Geom2dCartesianPoint> = aCopyGeom.cast::<Geom2dCartesianPoint>();

    assert!(!aCopy.is_null());
    EXPECT_NEAR(aCopy.x(), 3.0, precision::confusion());
    EXPECT_NEAR(aCopy.y(), 4.0, precision::confusion());

    aCopy.set_coord(0.0, 0.0);
    EXPECT_NEAR(aPnt.x(), 3.0, precision::confusion());
}
