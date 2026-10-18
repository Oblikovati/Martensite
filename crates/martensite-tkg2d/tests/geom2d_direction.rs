//! Direction construction, normalization, transformation and exceptional inputs.
#![allow(non_snake_case)]
mod common;
use common::EXPECT_NEAR;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{
    Geom2dDirection, Geom2dGeometry, Geom2dVector, Geom2dVectorWithMagnitude,
};
use martensite_tkmath::gp::{Direction2, Point2, Transform2};
use std::f64::consts::PI;

fn direction(x: f64, y: f64) -> Handle<Geom2dDirection> {
    Handle::new(Geom2dDirection::new(x, y).unwrap())
}

#[test]
fn direction_construct_from_coords() {
    let aDir = direction(3.0, 4.0);
    // Should be normalized
    EXPECT_NEAR(aDir.magnitude(), 1.0, precision::confusion());
    EXPECT_NEAR(aDir.x(), 0.6, precision::confusion());
    EXPECT_NEAR(aDir.y(), 0.8, precision::confusion());
}

#[test]
fn direction_construct_from_dir2d() {
    let aDir = Handle::new(Geom2dDirection::from_dir2d(
        Direction2::new(0.0, 1.0).unwrap(),
    ));
    EXPECT_NEAR(aDir.x(), 0.0, precision::confusion());
    EXPECT_NEAR(aDir.y(), 1.0, precision::confusion());
}

#[test]
fn direction_construct_from_zero_throws() {
    let aResult = Geom2dDirection::new(0.0, 0.0);
    assert_eq!(
        aResult.err().unwrap().exception_type(),
        "Standard_ConstructionError"
    );
}

#[test]
fn direction_magnitude_always_one() {
    let aDir = direction(100.0, 200.0);
    EXPECT_NEAR(aDir.magnitude(), 1.0, precision::confusion());
    EXPECT_NEAR(aDir.square_magnitude(), 1.0, precision::confusion());
}

#[test]
fn direction_set_coord() {
    let aDir = direction(1.0, 0.0);
    aDir.set_coord(0.0, 5.0).unwrap();
    EXPECT_NEAR(aDir.x(), 0.0, precision::confusion());
    EXPECT_NEAR(aDir.y(), 1.0, precision::confusion());
}

#[test]
fn direction_set_coord_zero_throws() {
    let aDir = direction(1.0, 0.0);
    let aResult = aDir.set_coord(0.0, 0.0);
    assert_eq!(
        aResult.err().unwrap().exception_type(),
        "Standard_ConstructionError"
    );
}

#[test]
fn direction_dir2d() {
    let aDir = direction(1.0, 0.0);
    let aGpDir: Direction2 = aDir.dir2d().unwrap();
    EXPECT_NEAR(aGpDir.x(), 1.0, precision::confusion());
    EXPECT_NEAR(aGpDir.y(), 0.0, precision::confusion());
}

#[test]
fn direction_crossed() {
    let aDirX = direction(1.0, 0.0);
    let aDirY = direction(0.0, 1.0);

    // (1,0) x (0,1) = 1
    EXPECT_NEAR(aDirX.crossed(&*aDirY), 1.0, precision::confusion());
    // (0,1) x (1,0) = -1
    EXPECT_NEAR(aDirY.crossed(&*aDirX), -1.0, precision::confusion());
}

#[test]
fn direction_angle() {
    let aDirX = direction(1.0, 0.0);
    let aDirY = direction(0.0, 1.0);

    EXPECT_NEAR(
        aDirX.angle(&*aDirY).unwrap(),
        PI / 2.0,
        precision::confusion(),
    );
}

#[test]
fn direction_angle_opposite_directions() {
    let aDirPos = direction(1.0, 0.0);
    let aDirNeg = direction(-1.0, 0.0);

    EXPECT_NEAR(
        aDirPos.angle(&*aDirNeg).unwrap().abs(),
        PI,
        precision::confusion(),
    );
}

#[test]
fn direction_dot() {
    let aDirX = direction(1.0, 0.0);
    let aDirY = direction(0.0, 1.0);

    EXPECT_NEAR(aDirX.dot(&*aDirY), 0.0, precision::confusion());
    EXPECT_NEAR(aDirX.dot(&*aDirX), 1.0, precision::confusion());
}

#[test]
fn direction_reverse() {
    let aDir = direction(1.0, 0.0);
    aDir.reverse();
    EXPECT_NEAR(aDir.x(), -1.0, precision::confusion());
    EXPECT_NEAR(aDir.y(), 0.0, precision::confusion());
}

#[test]
fn direction_crossed_with_vector_with_magnitude() {
    let aDir = direction(1.0, 0.0);
    let aVec = Handle::new(Geom2dVectorWithMagnitude::new(0.0, 5.0));

    EXPECT_NEAR(aDir.crossed(&*aVec), 5.0, precision::confusion());
}

#[test]
fn direction_copy() {
    let aDir = direction(1.0, 0.0);
    let aCopyGeom: Handle<dyn Geom2dGeometry> = aDir.copy().unwrap();
    let aCopy: Handle<Geom2dDirection> = aCopyGeom.cast::<Geom2dDirection>();

    assert!(!aCopy.is_null());
    EXPECT_NEAR(aCopy.x(), 1.0, precision::confusion());

    aCopy.set_coord(0.0, 1.0).unwrap();
    EXPECT_NEAR(aDir.x(), 1.0, precision::confusion());
}

#[test]
fn direction_transform_rotation() {
    let mut aTrsf = Transform2::default();
    aTrsf.set_rotation(Point2::new(0.0, 0.0), PI / 2.0);

    let aDir = direction(1.0, 0.0);
    aDir.transform(aTrsf).unwrap();

    EXPECT_NEAR(aDir.x(), 0.0, precision::confusion());
    EXPECT_NEAR(aDir.y(), 1.0, precision::confusion());
    EXPECT_NEAR(aDir.magnitude(), 1.0, precision::confusion());
}
